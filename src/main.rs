extern crate regex;
extern crate irsc;
extern crate openssl;
mod github;

use regex::Regex;
use openssl::ssl::{ Ssl, SslContext, SslMethod };
use irsc::client::Client;

fn parse_post(post: &str) -> Option<String> {
    let postregex = Regex::new(r"(\S+)/(\S+)#(\d+)").unwrap();
    let cap = match postregex.captures(post) {
        Some(x) => x,
        None => { return None; }
    };
    let user = cap.at(1).unwrap();
    let repo = cap.at(2).unwrap();
    let number = cap.at(3).unwrap();
    return github::get_display_text(user, repo, number).ok();
}

fn main() {
    let mut args = std::env::args();
    let username = args.nth(1).expect("No username provided (argument 1)");
    let password = args.next().expect("No password provided (argument 2)");
    println!("IRC BOT\nUsername: {}\nPassword: {}\n", username, password);

    println!("Connecting to server...");
    let mut ircclient = irsc::client::OwnedClient::new();
    let ssl = Ssl::new(&SslContext::new(SslMethod::Tlsv1).unwrap()).unwrap();
    ircclient.connect_ssl("irc.freenode.org", 6697, ssl);
    println!("Registering nick...");
    ircclient.register(&username, &username,
                       "Rust Powered IRC Bot", Some(&password));

    let mut shared = ircclient.into_shared();

    let _a = shared.commands()
        .map(move |(mut cl, msg, c)| {
            if let irsc::command::Command::PRIVMSG(to, content) = c {
                let from = msg.ident().unwrap().nickname;

                if to == username && &from == "ids1024" {
                    let mut words = content.split_whitespace();
                    let command = words.next().unwrap_or("");
                    let parameter = words.next().unwrap_or("");
                    match command {
                        "join" => {
                            println!("Joining '{}'...", parameter);
                            cl.join(parameter, None);
                        }
                        _ => {},
                    }
                }

                match parse_post(&content) {
                    Some(x) => {
                        for line in x.lines() {
                            cl.msg(&to, &line);
                        }
                    },
                    None => {}
                };
            }
        });

    let _b = shared.replies()
        .map(|(mut cl, _msg, r)| {
            if let irsc::reply::Reply::RPL_WELCOME(_) = r {
                println!("Joining channel(s)...");
                cl.join("#idstest1024", None);
            }
        });

    shared.listen_with_events();
}

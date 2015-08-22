extern crate regex;
extern crate irc;
mod github;

use regex::Regex;
use std::default::Default;
use irc::client::prelude::*;

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

    let config = Config {
        nickname: Some(username.clone()),
        password: Some(password),
        server: Some(format!("irc.freenode.org")),
        channels: Some(vec![format!("#idstest1024")]),
        port: Some(6697),
        use_ssl: Some(true),
        .. Default::default()
    };

    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap();
        if &message.command[..] == "PRIVMSG" {
            let from = message.get_source_nickname().unwrap().to_owned();
            let to = message.args[0].to_owned();
            let content = message.suffix.unwrap();

            if to == username && from == "ids1024" {
                let mut words = content.split_whitespace();
                let command = words.next().unwrap_or("");
                let parameter = words.next().unwrap_or("");
                match command {
                    "join" => {
                        server.send_join(parameter).unwrap();
                    }
                    _ => {},
                }
            }

            match parse_post(&content) {
                Some(x) => {
                    for line in x.lines() {
                        server.send_privmsg(&to, &line).unwrap();
                    }
                },
                None => {}
            };
        }
    }

}

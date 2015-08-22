extern crate regex;
extern crate irc;
extern crate xdg_basedir;

mod github;

use regex::Regex;
use irc::client::prelude::*;
use irc::client::server::NetIrcServer;
use xdg_basedir::*;

fn parse_post(post: &str) -> Option<String> {
    let postregex = Regex::new(r"(\S+)/(\S+)#(\d+)").unwrap();
    let cap = match postregex.captures(post) {
        Some(x) => x,
        None => { return None; }
    };
    let user = cap.at(1).unwrap();
    let repo = cap.at(2).unwrap();
    let number = cap.at(3).unwrap();

    github::get_display_text(user, repo, number).ok()
}

fn handle_message(server: &NetIrcServer, from: &str, to: &str, message: &str) {
    let nickname = server.config().nickname.as_ref().unwrap();

    if to == nickname && from == "ids1024" {
        let mut words = message.split_whitespace();
        let command = words.next().unwrap_or("");
        let parameter = words.next().unwrap_or("");
        match command {
            "join" => {
                server.send_join(parameter).unwrap();
            },
            "part" => {
                server.send(Command::PART(parameter.to_string(), None)).unwrap();
            },
            "quit" => {
                server.send_quit("").unwrap();
            }
            _ => {},
        }
    }

    match parse_post(&message) {
        Some(x) => {
            for line in x.lines() {
                server.send_privmsg(&to, &line).unwrap();
            }
        },
        None => {}
    };
}

fn main() {
    let mut configpath = get_config_home().unwrap();
    configpath.push("idsbot/config.json");

    let config = Config::load(configpath).unwrap();
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap();
        print!("{}", message.into_string());
        if message.command == "PRIVMSG" {
            let from = message.get_source_nickname().unwrap().to_owned();
            let to = message.args[0].to_owned();
            let content = message.suffix.unwrap();
            handle_message(&server, &from, &to, &content);
        }
    }
}

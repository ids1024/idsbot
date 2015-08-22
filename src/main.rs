extern crate regex;
extern crate irc;
extern crate xdg_basedir;
mod github;

use regex::Regex;
use irc::client::prelude::*;
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
    return github::get_display_text(user, repo, number).ok();
}

fn main() {
    let mut configpath = get_config_home().unwrap();
    configpath.push("idsbot/config.json");

    let config = Config::load(configpath).unwrap();
    let nickname = config.nickname.clone().expect("Must provide nickname.");

    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap();
        if &message.command[..] == "PRIVMSG" {
            let from = message.get_source_nickname().unwrap().to_owned();
            let to = message.args[0].to_owned();
            let content = message.suffix.unwrap();

            if to == nickname && from == "ids1024" {
                let mut words = content.split_whitespace();
                let command = words.next().unwrap_or("");
                let parameter = words.next().unwrap_or("");
                match command {
                    "join" => {
                        server.send_join(parameter).unwrap();
                    },
                    "part" => {
                        server.send(Message::new(None, "PART", Some(vec![parameter]), None)).unwrap();
                    },
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

extern crate regex;
extern crate irc;
extern crate xdg_basedir;
extern crate hyper;
extern crate url;

mod github;

use regex::Regex;
use irc::client::prelude::*;
use xdg_basedir::*;
use std::io::Read;
use std::vec::Vec;
use std::iter::FromIterator;
use url::Url;

fn parse_post(post: &str) -> Option<String> {
    let client = hyper::client::Client::new();
    let urlregex = Regex::new(r"https?://\S+\.[:alpha:]{2,}\S+").unwrap();

    if let Some(x) = urlregex.captures(post) {
        if let Ok(parsedurl) = Url::parse(x.at(0).unwrap()) {
            if parsedurl.domain().unwrap().ends_with("github.com") {
                let urlpath = Vec::from_iter(parsedurl.path_segments().unwrap());
                if urlpath.len() == 4 &&
                    (urlpath[2] == "issues" || urlpath[2] == "pull") {
                        return github::get_display_text(&urlpath[0],
                                                        &urlpath[1],
                                                        &urlpath[3],
                                                        false).ok();
                    }
            }
        }
        if let Ok(mut resp) = client.get(x.at(0).unwrap()).send() {
            let mut body = String::new();
            if let Err(..) = resp.read_to_string(&mut body) {
                return None; // Not UTF8 (binary file?)
            }
            let titleregex = Regex::new(r"<title>(.+)</title>").unwrap();
            if let Some(cap) = titleregex.captures(&body) {
                return Some(format!("Title: {}", cap.at(1).unwrap()));
            }
        }
    }

    let issueregex = Regex::new(r"([^ ()]+)/([^ ()]+)#(\d+)").unwrap();

    if let Some(cap) = issueregex.captures(post) {
        let user = cap.at(1).unwrap();
        let repo = cap.at(2).unwrap();
        let number = cap.at(3).unwrap();
        return github::get_display_text(user, repo, number, true).ok();
    }
    None
}

fn handle_message(server: &IrcServer, from: &str, to: &str, message: &str) {
    let nickname = server.config().nickname.as_ref().unwrap();

    if to == nickname && from == "ids1024" {
        let mut words = message.split_whitespace();
        let command = words.next().unwrap_or("");
        let parameter = words.next().unwrap_or("");
        match command {
            "join" => server.send_join(parameter).unwrap(),
            "part" => server.send(
                Command::PART(parameter.to_string(), None)
                ).unwrap(),
            "quit" => server.send_quit("").unwrap(),
            _ => {},
        }
    }

    if let Some(x) = parse_post(&message) {
        for line in x.lines() {
            server.send_notice(&to, &line).unwrap();
        }
    }
}

fn main() {
    let mut configpath = get_config_home().unwrap();
    configpath.push("idsbot/config.json");

    let config = Config::load(configpath).unwrap();
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap();
        print!("{}", message.to_string());
        let from = message.source_nickname().map(|s| s.to_owned());
        if let Command::PRIVMSG(to, content) = message.command {
            handle_message(&server, &from.unwrap(), &to, &content);
        }
    }
}

extern crate regex;
mod github;

use regex::Regex;

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
    println!("{}", parse_post("mps-youtube/mps-youtube#1").unwrap());
}

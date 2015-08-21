extern crate hyper;
extern crate serde_json;

use std::string::String;
use std::io::Read;
use serde_json::Value;

fn get_title(user: &str, repo: &str, issue: &str) -> String {
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}",
                      user, repo, issue);
    let client = hyper::client::Client::new();
    let mut resp = client
        .get(&url)
        .header(hyper::header::UserAgent("ids1024".to_owned()))
        .send()
        .unwrap();

    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();
    let data: Value = serde_json::from_str(&body).unwrap();
    let obj = data.as_object().unwrap();

    let title = obj.get("title").unwrap().as_string().unwrap();
    let state = obj.get("state").unwrap().as_string().unwrap();
    let url = obj.get("html_url").unwrap().as_string().unwrap();
    let issuetype = 
        if obj.get("pull_request") != None {
            "pull request"
        } else {
            "issue"
        };

    return format!("[{}] [{}] {}\n{}", issuetype, state, title, url);
}

fn main() {
    println!("{}", get_title("mps-youtube", "mps-youtube", "1"));
}

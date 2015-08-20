extern crate curl;
extern crate serde_json;

use std::str;
use curl::http;
use serde_json::Value;

fn get_title(user: &str, repo: &str, issue: &str) -> String {
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}",
                      user, repo, issue);
    let resp = http::handle()
        .get(url)
        .header("User-Agent", "ids1024")
        .exec()
        .unwrap();

    let body = str::from_utf8(&resp.get_body()).unwrap();
    let data: Value = serde_json::from_str(body).unwrap();
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

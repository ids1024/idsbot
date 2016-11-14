extern crate std;
extern crate hyper;
extern crate serde_json;

use std::string::String;
use std::io::Read;
use self::serde_json::Value;

#[derive(Debug)]
pub struct GithubError;

impl From<hyper::error::Error> for GithubError {
    fn from(_: hyper::error::Error) -> Self {
        GithubError
    }
}

impl From<serde_json::error::Error> for GithubError {
    fn from(_: serde_json::error::Error) -> Self {
        GithubError
    }
}

impl From<std::io::Error> for GithubError {
    fn from(_: std::io::Error) -> Self {
        GithubError
    }
}

pub fn get_display_text(user: &str,
                        repo: &str,
                        issue: &str,
                        printurl: bool)
                        -> Result<String, GithubError> {

    let url = format!("https://api.github.com/repos/{}/{}/issues/{}",
                      user,
                      repo,
                      issue);
    let client = hyper::client::Client::new();
    let mut resp = client.get(&url)
        .header(hyper::header::UserAgent("idsbot".to_owned()))
        .send()?;

    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    let data: Value = serde_json::from_str(&body)?;
    let obj = data.as_object().ok_or(GithubError)?;

    let title = obj.get("title").ok_or(GithubError)?
        .as_str()
        .ok_or(GithubError)?;
    let state = obj.get("state").ok_or(GithubError)?
        .as_str()
        .ok_or(GithubError)?;
    let url = obj.get("html_url").ok_or(GithubError)?
        .as_str()
        .ok_or(GithubError)?;
    let issuetype = match obj.get("pull_request") {
        Some(..) => "pull request",
        None => "issue",
    };

    if printurl {
        Ok(format!("[{}] [{}] {}\n{}", issuetype, state, title, url))
    } else {
        Ok(format!("[{}] [{}] {}", issuetype, state, title))
    }
}

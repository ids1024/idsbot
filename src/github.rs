use std::string::String;
use std::io::{self, Read};
use serde_json::{self, Value};
use reqwest;
use reqwest::header::UserAgent;
use url;

#[derive(Debug)]
pub struct GithubError;

impl From<reqwest::Error> for GithubError {
    fn from(_: reqwest::Error) -> Self {
        GithubError
    }
}

impl From<serde_json::error::Error> for GithubError {
    fn from(_: serde_json::error::Error) -> Self {
        GithubError
    }
}

impl From<io::Error> for GithubError {
    fn from(_: io::Error) -> Self {
        GithubError
    }
}

impl From<url::ParseError> for GithubError {
    fn from(_: url::ParseError) -> Self {
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
    let mut resp = reqwest::Client::new()?.get(&url)?
                         .header(UserAgent::new("idsbot"))
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

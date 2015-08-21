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

pub fn get_display_text(user: &str, repo: &str, issue: &str)
    -> Result<String, GithubError> {

    let url = format!("https://api.github.com/repos/{}/{}/issues/{}",
                      user, repo, issue);
    let client = hyper::client::Client::new();
    let mut resp = try!(client
        .get(&url)
        .header(hyper::header::UserAgent("ids1024".to_owned()))
        .send());

    let mut body = String::new();
    try!(resp.read_to_string(&mut body));
    let data: Value = try!(serde_json::from_str(&body));
    let obj = try!(data.as_object().ok_or(GithubError));

    let title = try!(try!(obj.get("title").ok_or(GithubError))
                     .as_string().ok_or(GithubError));
    let state = try!(try!(obj.get("state").ok_or(GithubError))
                     .as_string().ok_or(GithubError));
    let url = try!(try!(obj.get("html_url").ok_or(GithubError))
                        .as_string().ok_or(GithubError));
    let issuetype = 
        if obj.get("pull_request") != None {
            "pull request"
        } else {
            "issue"
        };

    return Ok(format!("[{}] [{}] {}\n{}", issuetype, state, title, url));
}

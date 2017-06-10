#[macro_use] extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use hyper::Client;
use hyper::header;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::{Map, Value as Json};

const UA: &str = "https://github.com/notriddle/show-recent-activity-for-github-bot (0.1)";

fn main() {
    let matches = clap_app!(show_recent_activity_for_github_bot =>
        (version: "0.1")
        (author: "Michael Howell <michael@notriddle.com")
        (about: "Shows comments that have been recently left by a GitHub App")
        (@arg BOT: +required +takes_value "The name of the GitHub app")
        (@arg pages: -p +takes_value "Set the number of pages to go back in history (1-64)")
    ).get_matches();
    let bot = matches.value_of("BOT").unwrap();
    let mut pages: u32 = matches.value_of("pages").map(str::parse).and_then(Result::ok).unwrap_or(1);
    let client = Client::with_connector(
        HttpsConnector::new(
            NativeTlsClient::new().unwrap()
        )
    );
    let mut res = client.get(&format!("https://api.github.com/users/{}%5Bbot%5D/events", bot))
        .header(header::UserAgent(UA.to_owned()))
        .send().unwrap();
    assert_eq!(hyper::Ok, res.status);
    'main: loop {
        let mut next = None;
        if let (Some(link), 1...64) = (res.headers.get::<header::Link>(), pages) {
            for value in link.values() {
                if value.rel() == Some(&[header::RelationType::Next]) {
                    next = Some(value.link().to_owned());
                }
            }
        }
        let events: Vec<Map<String, Json>> = serde_json::from_reader(res).unwrap();
        for event in &events {
            if let (Some(&Json::String(ref ty)), Some(&Json::Object(ref p))) = (event.get("type"), event.get("payload")) {
                if ty == "IssueCommentEvent" {
                    if let Some(&Json::Object(ref c)) = p.get("comment") {
                        if let Some(&Json::String(ref h)) = c.get("html_url") {
                            println!("{}", h);
                        }
                    }
                }
            }
        }
        if let Some(next) = next {
            res = client.get(&next)
                .header(header::UserAgent(UA.to_owned()))
                .send().unwrap();
            pages -= 1;
        } else {
            break;
        }
    }
}

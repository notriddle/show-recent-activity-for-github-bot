use clap::clap_app;
use reqwest;
use serde_json::{Map, Value as Json};

const UA: &str = "https://github.com/notriddle/show-recent-activity-for-github-bot (0.1)";

#[tokio::main]
async fn main() {
    let matches = clap_app!(show_recent_activity_for_github_bot =>
        (version: "0.1")
        (author: "Michael Howell <michael@notriddle.com")
        (about: "Shows comments that have been recently left by a GitHub App")
        (@arg BOT: +required +takes_value "The name of the GitHub app")
    ).get_matches();
    let bot = matches.value_of("BOT").unwrap();
    let client = reqwest::Client::builder().user_agent(UA).build().unwrap();
    let res = client.get(format!("https://api.github.com/users/{}%5Bbot%5D/events", bot).parse::<reqwest::Url>().unwrap()).send()
        .await.unwrap();
    let events: Vec<Map<String, Json>> = res.json().await.unwrap();
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
}

use chrono::prelude::*;
use log::info;
use reqwest::blocking as reqwest;
use serde::Deserialize;

use crate::error::{RMError, RMErrorKind};

const USER_AGENT: &str = "macos:reddit-mailer:0.1.0 (by /u/GeneralMaximus)";

#[derive(Deserialize, Debug)]
struct StoryListingResponse {
    data: StoryListingResponseData,
}

#[derive(Deserialize, Debug)]
struct StoryListingResponseData {
    children: Vec<StoryListingResponseDataChildData>,
}

#[derive(Deserialize, Debug)]
struct StoryListingResponseDataChildData {
    data: Story,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Story {
    pub permalink: String,
    pub subreddit: String,
    pub title: String,
    pub score: u32,
    pub created_utc: f64,
    pub author: String,
    pub num_comments: u32,
    pub url: String,
    pub was_mailed: Option<u8>
}

impl Story {
    pub fn get_created_utc_iso8601(&self) -> String {
        let naive = NaiveDateTime::from_timestamp(self.created_utc as i64, 0);
        DateTime::<Utc>::from_utc(naive, Utc).to_rfc3339()
    }

    pub fn render_as_list_item(&self) -> String {
        format!(
            r#"<li><a href="{}">{}</a> (<a href="https://www.reddit.com{}">comments</a>) from /r/{}</li>"#,
            self.url, self.title, self.permalink, self.subreddit
        )
    }

    pub fn render_list(stories: &[Self]) -> String {
        let stories_list = stories
            .iter()
            .map(|s| s.render_as_list_item())
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r#"
        <!DOCTYPE html>
        <html>
        <body>
            <h1>Recent Top Reddit Stories</h1>
            <ol>
                {}
            </ol>
        </body>
        </html>
    "#,
            stories_list
        )
    }
}

pub fn get_hot_stories(subreddits: &[String], bearer_token: &str) -> Result<Vec<Story>, RMError> {
    info!("Getting hot stories");

    let http_client = reqwest::Client::new();
    let res = http_client
        .get(format!(
            "https://oauth.reddit.com/r/{}/hot",
            join_subreddits(subreddits)
        ))
        .bearer_auth(bearer_token)
        .header("User-Agent", USER_AGENT)
        .send()
        .map_err(|_| RMError {
            kind: RMErrorKind::RedditNetwork,
            message: "Failed to get hot stories from reddit".to_string(),
        })?;

    let res = res.text().map_err(|_| RMError {
        kind: RMErrorKind::Io,
        message: "Could not read reddit response body".to_string(),
    })?;

    let res: StoryListingResponse = serde_json::from_str(&res).map_err(|e| RMError {
        kind: RMErrorKind::RedditResponseParse,
        message: format!("Could not parse stories returned by reddit: {}", e),
    })?;

    let stories: Vec<Story> = res.data.children.into_iter().map(|c| c.data).collect();

    Ok(stories)
}

fn join_subreddits(subreddits: &[String]) -> String {
    subreddits.join("+")
}

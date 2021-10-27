use reqwest::blocking as reqwest;
use serde::Deserialize;
use std::error::Error;

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
    pub stickied: bool,
    pub author: String,
    pub num_comments: u32,
    pub url: String,
    pub subreddit_subscribers: u32,
}

pub fn get_hot_stories(
    subreddits: &[String],
    bearer_token: &str,
) -> Result<Vec<Story>, Box<dyn Error>> {
    let http_client = reqwest::Client::new();
    let res = http_client
        .get(format!(
            "https://oauth.reddit.com/r/{}/hot",
            join_subreddits(subreddits)
        ))
        .bearer_auth(bearer_token)
        .header("User-Agent", USER_AGENT)
        .send()?;
    let res = res.text()?;
    let res: StoryListingResponse = serde_json::from_str(&res)?;
    let stories: Vec<Story> = res.data.children.into_iter().map(|c| c.data).collect();

    Ok(stories)
}

fn join_subreddits(subreddits: &[String]) -> String {
    subreddits.join("+")
}

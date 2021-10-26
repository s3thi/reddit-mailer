mod stories;
mod token;

use stories::get_hot_stories;
use token::get_bearer_token;

use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    subreddits: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: read config path from command line arguments.
    let config = read_to_string("reddit-mailer.json")?;
    let config: AppConfig = serde_json::from_str(&config)?;
    let bearer_token = get_bearer_token(
        &config.client_id,
        &config.client_secret,
        &config.username,
        &config.password,
    )?;

    let stories = get_hot_stories(&config.subreddits, &bearer_token)?;
    for s in stories {
        println!("{:?}", s);
    }

    Ok(())
}

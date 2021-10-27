use librm::db::DB;
use librm::stories::get_hot_stories;
use librm::token::get_bearer_token;

use serde::{Deserialize, Serialize};
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
    let mut db = DB::new()?;
    for s in stories {
        db.save_story(&s)?;
    }

    Ok(())
}

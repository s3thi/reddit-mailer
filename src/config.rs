use std::error::Error;
use std::fs::read_to_string;

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub subreddits: Vec<String>,
}

impl AppConfig {
    pub fn read() -> Result<Self, Box<dyn Error>> {
        info!("Reading configuration file");
        let config = read_to_string("reddit-mailer.json")?;
        let config = serde_json::from_str(&config)?;
        Ok(config)
    }
}

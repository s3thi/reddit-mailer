use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::PathBuf;

use dirs::config_dir;
use log::info;
use serde::{Deserialize, Serialize};

use crate::error::{RMError, RMErrorKind};

const CONFIG_FILE_NAME: &str = "config.json";
const DB_FILE_NAME: &str = "db.sqlite";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,
    pub subreddits: Vec<String>,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
    pub reply_to_address: String,
    pub to_address: String,
    pub subject_line: String,
}

impl AppConfig {
    pub fn read() -> Result<Self, RMError> {
        info!("Reading configuration file");

        let mut path = Self::get_config_dir();
        path.push(CONFIG_FILE_NAME);
        let config = read_to_string(&path).map_err(|e| match e.kind() {
            ErrorKind::NotFound => RMError {
                kind: RMErrorKind::ConfigAccess,
                message: format!("Config file doesn't exist: {}", path.display()),
            },
            _ => RMError {
                kind: RMErrorKind::Io,
                message: format!("Could not read config file: {}", path.display()),
            },
        })?;

        // TODO: add more details about missing fields, etc.
        let config = serde_json::from_str(&config).map_err(|_| RMError {
            kind: RMErrorKind::ConfigParse,
            message: format!("Could not parse config file: {}", path.display()),
        })?;

        Ok(config)
    }

    fn get_config_dir() -> PathBuf {
        let mut config_dir = config_dir().unwrap();
        config_dir.push("reddit-mailer");
        config_dir
    }

    pub fn get_db_path() -> PathBuf {
        let mut db_path = Self::get_config_dir();
        db_path.push(DB_FILE_NAME);
        db_path
    }
}

use std::fs::read_to_string;
use std::io::ErrorKind;

use dirs::config_dir;
use log::info;
use serde::{Deserialize, Serialize};

use crate::error::{RMError, RMErrorKind};

const CONFIG_FILE_NAME: &str = "reddit-mailer.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub subreddits: Vec<String>,
}

impl AppConfig {
    pub fn read() -> Result<Self, RMError> {
        info!("Reading configuration file");

        let mut path = config_dir().unwrap();
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
}

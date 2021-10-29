use std::collections::HashMap;

use log::info;
use reqwest::blocking as reqwest;
use serde::Deserialize;

use crate::error::{RMError, RMErrorKind};

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
}

pub fn get_bearer_token(
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
) -> Result<String, RMError> {
    info!("Getting a bearer token from reddit");

    let mut auth_req_body = HashMap::<&str, &str>::new();
    auth_req_body.insert("grant_type", "password");
    auth_req_body.insert("username", username);
    auth_req_body.insert("password", password);

    let http_client = reqwest::Client::new();
    let res = http_client
        .post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(client_id, Some(client_secret))
        .form(&auth_req_body)
        .send()
        .map_err(|_| RMError {
            kind: RMErrorKind::RedditNetwork,
            message: "Error fetching bearer token from reddit".to_string(),
        })?;

    let res: AccessTokenResponse = res.json().map_err(|_| RMError {
        kind: RMErrorKind::RedditResponseParse,
        message: "Error parsing json response from reddit".to_string(),
    })?;

    Ok(res.access_token)
}

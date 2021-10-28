use log::info;
use reqwest::blocking as reqwest;
use serde::Deserialize;

use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
}

pub fn get_bearer_token(
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
) -> Result<String, Box<dyn Error>> {
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
        .send()?;
    let res: AccessTokenResponse = res.json()?;
    Ok(res.access_token)
}

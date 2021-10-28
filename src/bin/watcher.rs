use librm::config::AppConfig;
use librm::db::DB;
use librm::stories::get_hot_stories;
use librm::token::get_bearer_token;

use env_logger::Env;
use log::info;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = AppConfig::read()?;

    let bearer_token = get_bearer_token(
        &config.client_id,
        &config.client_secret,
        &config.username,
        &config.password,
    )?;

    let stories = get_hot_stories(&config.subreddits, &bearer_token)?;
    let mut db = DB::new()?;
    db.save_stories(&stories)?;

    info!("Success!");
    Ok(())
}

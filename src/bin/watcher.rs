use librm::db::DB;
use librm::stories::{get_hot_stories};
use librm::token::get_bearer_token;
use librm::{config::AppConfig, error::RMError};

use env_logger::Env;
use log::{error, info};

fn watch() -> Result<(), RMError> {
    let config = AppConfig::read()?;

    let bearer_token = get_bearer_token(
        &config.client_id,
        &config.client_secret,
        &config.reddit_username,
        &config.reddit_password,
    )?;

    let stories = get_hot_stories(&config.subreddits, &bearer_token)?;
    let mut db = DB::new()?;
    db.save_stories(&stories)?; 

    info!("Success!");

    Ok(())
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    match watch() {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e.message);
            std::process::exit(1);
        }
    };
}

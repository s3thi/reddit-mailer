use env_logger::Env;
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};

use librm::config::AppConfig;
use librm::db::DB;
use librm::error::RMError;
use librm::stories::Story;

fn make_email_message(body: &str, config: &AppConfig) -> Message {
    info!("Creating an email message");
    Message::builder()
        .from(config.from_address.parse().unwrap())
        .reply_to(config.reply_to_address.parse().unwrap())
        .to(config.to_address.parse().unwrap())
        .subject(config.subject_line.to_string())
        .header(header::ContentType::TEXT_HTML)
        .body(body.to_string())
        .unwrap()
}

fn make_mailer(config: &AppConfig) -> SmtpTransport {
    info!("Creating SMTP configuration");
    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
    SmtpTransport::relay("smtp.fastmail.com")
        .unwrap()
        .credentials(creds)
        .build()
}

fn send() -> Result<(), RMError> {
    let config = AppConfig::read()?;

    let mut db = DB::new()?;
    let stories = db.get_highest_scoring_stories()?;
    let stories_markup = Story::render_list(&stories);

    let email = make_email_message(&stories_markup, &config);
    let mailer = make_mailer(&config);

    info!("Sending newsletter");
    mailer.send(&email)?;

    info!("Done!");
    Ok(())
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    match send() {
        Ok(_) => {}
        Err(e) => error!("{}", e.message),
    }
}

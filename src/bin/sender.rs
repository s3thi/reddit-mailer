use env_logger::Env;
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};

use librm::config::AppConfig;
use librm::db::DB;
use librm::error::RMError;
use librm::stories::Story;

fn render_stories(stories: &[Story]) -> String {
    info!("Rendering newsletter");
    let stories_list = stories
        .iter()
        .map(|s| {
            format!(
                r#"<li><a href="{}">{}</a> (<a href="https://www.reddit.com{}">comments</a>)</li>"#,
                s.url, s.title, s.permalink
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <h1>Recent Top Reddit Stories</h1>
            <ol>
                {}
            </ol>
        </body>
        </html>
    "#,
        stories_list
    )
}

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

fn send() -> Result<(), RMError> {
    let config = AppConfig::read()?;

    let mut db = DB::new()?;
    let stories = db.get_highest_scoring_stories()?;
    let stories_markup = render_stories(&stories);

    let email = make_email_message(&stories_markup, &config);
    let creds = Credentials::new(config.smtp_username, config.smtp_password);

    info!("Creating SMTP configuration");
    let mailer = SmtpTransport::relay("smtp.fastmail.com")
        .unwrap()
        .credentials(creds)
        .build();

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

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum RMErrorKind {
    Io,
    Database,
    ConfigAccess,
    ConfigParse,
    RedditNetwork,
    RedditResponseParse,
    SMTP,
}

#[derive(Debug)]
pub struct RMError {
    pub kind: RMErrorKind,
    pub message: String,
}

impl Display for RMError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RMError {}

impl From<rusqlite::Error> for RMError {
    // TODO: add more context to this conversion.
    fn from(_: rusqlite::Error) -> Self {
        RMError {
            kind: RMErrorKind::Database,
            message: "Database error".to_string(),
        }
    }
}

impl From<lettre::transport::smtp::Error> for RMError {
    // TODO: add more context to this conversion.
    fn from(_: lettre::transport::smtp::Error) -> Self {
        RMError {
            kind: RMErrorKind::SMTP,
            message: "Error sending email".to_string(),
        }
    }
}

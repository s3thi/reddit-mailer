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
    fn from(error: rusqlite::Error) -> Self {
        RMError {
            kind: RMErrorKind::Database,
            message: format!("Database error: {}", error)
        }
    }
}

impl From<lettre::transport::smtp::Error> for RMError {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        RMError {
            kind: RMErrorKind::SMTP,
            message: format!("Error sending email: {}", error)
        }
    }
}

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
    // TODO: make this conversion more useful
    fn from(_: rusqlite::Error) -> Self {
        RMError {
            kind: RMErrorKind::Database,
            message: "Database error".to_string(),
        }
    }
}

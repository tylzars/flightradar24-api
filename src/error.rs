use std::fmt;

#[derive(Debug)]
pub enum FlightradarError {
    /// Errors returned by the HTTP client.
    Http(reqwest::Error),
    /// Errors that occur during parsing.
    Parsing(String),
    /// A general error with a message.
    General(String),
}

impl fmt::Display for FlightradarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlightradarError::Http(err) => write!(f, "HTTP error: {}", err),
            FlightradarError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
            FlightradarError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for FlightradarError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FlightradarError::Http(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for FlightradarError {
    fn from(err: reqwest::Error) -> Self {
        FlightradarError::Http(err)
    }
}

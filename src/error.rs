use std::fmt;

#[derive(Debug)]
pub enum FlightRadarError {
    /// Errors returned by the HTTP client.
    Http(reqwest::Error),
    /// Errors that occur during parsing.
    Parsing(String),
    /// A general error with a message.
    General(String),
    /// Invalid Parameter Passed to API.
    Parameter(String),
}

impl fmt::Display for FlightRadarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlightRadarError::Http(err) => write!(f, "HTTP error: {}", err),
            FlightRadarError::Parsing(msg) => write!(f, "Parsing error: {}", msg),
            FlightRadarError::General(msg) => write!(f, "Error: {}", msg),
            FlightRadarError::Parameter(msg) => write!(f, "Invalid Parameter: {}", msg),
        }
    }
}

impl std::error::Error for FlightRadarError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FlightRadarError::Http(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for FlightRadarError {
    fn from(err: reqwest::Error) -> Self {
        FlightRadarError::Http(err)
    }
}

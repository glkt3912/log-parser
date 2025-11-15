use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogParserError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Parse error: {message}")]
    Parse { message: String },

    #[error("Filter error: {message}")]
    Filter { message: String },

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Invalid log level: {level}")]
    InvalidLogLevel { level: String },

    #[error("Invalid date format: {date}")]
    InvalidDateFormat { date: String },
}

pub type Result<T> = std::result::Result<T, LogParserError>;

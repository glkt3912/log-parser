use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: Option<DateTime<Utc>>,
    pub level: Option<LogLevel>,
    pub message: String,
    pub raw_line: String,
}

impl LogEntry {
    pub fn new(raw_line: String) -> Self {
        Self {
            timestamp: None,
            level: None,
            message: raw_line.clone(),
            raw_line,
        }
    }

    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
}

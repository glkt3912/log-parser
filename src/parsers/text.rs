use crate::core::{LogEntry, LogLevel, Result};
use crate::parsers::Parser;
use chrono::{DateTime, Utc};
use regex::Regex;

pub struct TextParser {
    timestamp_regex: Regex,
    level_regex: Regex,
}

impl TextParser {
    pub fn new() -> Result<Self> {
        let timestamp_regex = Regex::new(r"(\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2})")?;
        let level_regex = Regex::new(r"\[(ERROR|WARN|INFO|DEBUG)\]")?;

        Ok(Self {
            timestamp_regex,
            level_regex,
        })
    }
}

impl Default for TextParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default TextParser")
    }
}

impl Parser for TextParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        if line.trim().is_empty() {
            return Ok(None);
        }

        let mut entry = LogEntry::new(line.to_string());

        // Extract timestamp
        if let Some(captures) = self.timestamp_regex.captures(line) {
            if let Some(timestamp_str) = captures.get(1) {
                if let Ok(timestamp) = timestamp_str.as_str().parse::<DateTime<Utc>>() {
                    entry = entry.with_timestamp(timestamp);
                }
            }
        }

        // Extract log level
        if let Some(captures) = self.level_regex.captures(line) {
            if let Some(level_str) = captures.get(1) {
                if let Ok(level) = level_str.as_str().parse::<LogLevel>() {
                    entry = entry.with_level(level);
                }
            }
        }

        // Extract message (everything after the level, or the whole line if no level found)
        let message = if let Some(level_match) = self.level_regex.find(line) {
            line[level_match.end()..].trim().to_string()
        } else {
            line.to_string()
        };
        entry = entry.with_message(message);

        Ok(Some(entry))
    }

    fn name(&self) -> &'static str {
        "text"
    }
}

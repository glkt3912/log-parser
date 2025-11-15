use crate::core::{LogEntry, LogLevel, Result};
use crate::filters::Filter;

pub struct LevelFilter {
    target_level: LogLevel,
}

impl LevelFilter {
    pub fn new(level: LogLevel) -> Self {
        Self {
            target_level: level,
        }
    }
}

impl Filter for LevelFilter {
    fn apply(&self, entry: &LogEntry) -> Result<bool> {
        match &entry.level {
            Some(level) => Ok(*level == self.target_level),
            None => Ok(false), // No level means we can't match
        }
    }

    fn name(&self) -> &'static str {
        "level"
    }
}

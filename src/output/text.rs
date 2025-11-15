use crate::core::{LogEntry, Result};
use crate::output::OutputFormatter;
use colored::*;

pub struct TextFormatter {
    use_colors: bool,
}

impl TextFormatter {
    pub fn new(use_colors: bool) -> Self {
        Self { use_colors }
    }

    fn format_entry(&self, entry: &LogEntry) -> String {
        if !self.use_colors {
            return entry.raw_line.clone();
        }

        // Apply colors based on log level
        match &entry.level {
            Some(level) => {
                let colored_line = match level {
                    crate::core::LogLevel::Error => entry.raw_line.red(),
                    crate::core::LogLevel::Warn => entry.raw_line.yellow(),
                    crate::core::LogLevel::Info => entry.raw_line.white(),
                    crate::core::LogLevel::Debug => entry.raw_line.bright_black(),
                };
                colored_line.to_string()
            }
            None => entry.raw_line.clone(),
        }
    }
}

impl Default for TextFormatter {
    fn default() -> Self {
        Self::new(true)
    }
}

impl OutputFormatter for TextFormatter {
    fn format(&self, entries: &[LogEntry]) -> Result<String> {
        let formatted: Vec<String> = entries
            .iter()
            .map(|entry| self.format_entry(entry))
            .collect();

        Ok(formatted.join("\n"))
    }

    fn format_single(&self, entry: &LogEntry) -> Result<String> {
        Ok(self.format_entry(entry))
    }

    fn name(&self) -> &'static str {
        "text"
    }
}

// Stub implementation - to be implemented in later phases
use crate::core::{LogEntry, Result};
use crate::output::OutputFormatter;

pub struct CsvFormatter;

impl OutputFormatter for CsvFormatter {
    fn format(&self, _entries: &[LogEntry]) -> Result<String> {
        // Placeholder implementation
        Ok("timestamp,level,message\n".to_string())
    }

    fn format_single(&self, _entry: &LogEntry) -> Result<String> {
        // Placeholder implementation
        Ok("".to_string())
    }

    fn name(&self) -> &'static str {
        "csv"
    }
}

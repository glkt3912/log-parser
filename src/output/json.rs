// Stub implementation - to be implemented in later phases
use crate::core::{LogEntry, Result};
use crate::output::OutputFormatter;

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, entries: &[LogEntry]) -> Result<String> {
        serde_json::to_string_pretty(entries).map_err(|e| e.into())
    }

    fn format_single(&self, entry: &LogEntry) -> Result<String> {
        serde_json::to_string_pretty(entry).map_err(|e| e.into())
    }

    fn name(&self) -> &'static str {
        "json"
    }
}

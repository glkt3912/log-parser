use crate::core::{LogEntry, Result};

pub trait OutputFormatter {
    fn format(&self, entries: &[LogEntry]) -> Result<String>;
    fn format_single(&self, entry: &LogEntry) -> Result<String>;
    fn name(&self) -> &'static str;
}

// Output formatter implementations
pub mod csv;
pub mod json;
pub mod stats;
pub mod text;

pub use text::TextFormatter;

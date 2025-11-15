use crate::core::{LogEntry, Result};

pub trait Parser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>>;
    fn name(&self) -> &'static str;
}

// Parser implementations will be added in subsequent phases
pub mod text;

pub use text::TextParser;

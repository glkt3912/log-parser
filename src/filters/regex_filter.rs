// Stub implementation - to be implemented in later phases
use crate::core::{LogEntry, Result};
use crate::filters::Filter;

pub struct RegexFilter;

impl Filter for RegexFilter {
    fn apply(&self, _entry: &LogEntry) -> Result<bool> {
        // Placeholder implementation
        Ok(true)
    }

    fn name(&self) -> &'static str {
        "regex"
    }
}

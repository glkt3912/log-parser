// Stub implementation - to be implemented in TimeFilter phase
use crate::core::{LogEntry, Result};
use crate::filters::Filter;

pub struct TimeFilter;

impl Filter for TimeFilter {
    fn apply(&self, _entry: &LogEntry) -> Result<bool> {
        // Placeholder implementation
        Ok(true)
    }

    fn name(&self) -> &'static str {
        "time"
    }
}

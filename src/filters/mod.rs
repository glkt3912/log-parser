use crate::core::{LogEntry, Result};

pub trait Filter {
    fn apply(&self, entry: &LogEntry) -> Result<bool>;
    fn name(&self) -> &'static str;
}

// Filter implementations will be added in subsequent phases
pub mod composite;
pub mod level;
pub mod regex_filter;
pub mod time;

pub use level::LevelFilter;

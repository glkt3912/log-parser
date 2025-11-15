pub mod config;
pub mod core;
pub mod filters;
pub mod output;
pub mod parsers;

#[cfg(feature = "tui")]
pub mod ui;

// Re-export core types
pub use crate::config::Config;
pub use crate::core::{LogEntry, LogLevel, StreamProcessor};
pub use crate::filters::Filter;
pub use crate::output::OutputFormatter;
pub use crate::parsers::Parser;

// Result type for the entire crate
pub type Result<T> = anyhow::Result<T>;

// Main application struct
pub struct LogParser {
    config: Config,
}

impl LogParser {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn run(&mut self) -> Result<()> {
        // Basic implementation for MVP - will be enhanced in Foundation phase
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(&self.config.file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            println!("{}", line);
        }

        Ok(())
    }
}

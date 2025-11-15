use crate::core::{LogEntry, Result};
use std::path::Path;

pub trait StreamProcessor {
    fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<LogEntry>>;
    fn process_lines<I>(&self, lines: I) -> Result<Vec<LogEntry>>
    where
        I: Iterator<Item = String>;
}

pub struct BasicStreamProcessor;

impl BasicStreamProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BasicStreamProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamProcessor for BasicStreamProcessor {
    fn process_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<LogEntry>> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines().collect::<std::io::Result<Vec<_>>>()?;

        self.process_lines(lines.into_iter())
    }

    fn process_lines<I>(&self, lines: I) -> Result<Vec<LogEntry>>
    where
        I: Iterator<Item = String>,
    {
        let entries: Vec<LogEntry> = lines.map(LogEntry::new).collect();

        Ok(entries)
    }
}

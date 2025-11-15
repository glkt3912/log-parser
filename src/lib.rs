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
        use crate::parsers::TextParser;
        use crate::filters::{LevelFilter, TimeFilter};
        use crate::output::{TextFormatter, json::JsonFormatter};
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        // Initialize parser
        let parser = TextParser::new()?;

        // Initialize filter based on config
        let level_filter = if let Some(ref level_str) = self.config.level_filter {
            match level_str.parse::<LogLevel>() {
                Ok(level) => Some(LevelFilter::new(level)),
                Err(_) => {
                    eprintln!("警告: 無効なログレベル '{}' - フィルタなしで処理を続行", level_str);
                    None
                }
            }
        } else {
            None
        };

        // Initialize time filter based on config
        let time_filter = if self.config.since.is_some() || self.config.until.is_some() {
            match TimeFilter::new(
                self.config.since.as_deref(),
                self.config.until.as_deref(),
            ) {
                Ok(filter) => Some(filter),
                Err(e) => {
                    eprintln!("警告: 時刻フィルタエラー: {} - 時刻フィルタなしで処理を続行", e);
                    None
                }
            }
        } else {
            None
        };

        // Initialize output formatter based on config
        let formatter: Box<dyn OutputFormatter> = match self.config.output_format.as_str() {
            "json" => Box::new(JsonFormatter),
            "text" => Box::new(TextFormatter::default()),
            _ => {
                eprintln!("警告: 未対応の出力形式 '{}' - テキスト形式を使用", self.config.output_format);
                Box::new(TextFormatter::default())
            }
        };

        // Process file
        let file = File::open(&self.config.file_path)?;
        let reader = BufReader::new(file);
        let mut parsed_entries = Vec::new();
        let mut line_count = 0;
        let mut error_count = 0;

        for line_result in reader.lines() {
            let line = line_result?;
            line_count += 1;

            match parser.parse_line(&line) {
                Ok(Some(entry)) => {
                    // Apply filters
                    let mut should_include = true;

                    // Apply level filter if specified
                    if should_include {
                        if let Some(ref filter) = level_filter {
                            match filter.apply(&entry) {
                                Ok(result) => should_include = result,
                                Err(e) => {
                                    eprintln!("レベルフィルタエラー ({}行目): {}", line_count, e);
                                    error_count += 1;
                                    should_include = false;
                                }
                            }
                        }
                    }

                    // Apply time filter if specified
                    if should_include {
                        if let Some(ref filter) = time_filter {
                            match filter.apply(&entry) {
                                Ok(result) => should_include = result,
                                Err(e) => {
                                    eprintln!("時刻フィルタエラー ({}行目): {}", line_count, e);
                                    error_count += 1;
                                    should_include = false;
                                }
                            }
                        }
                    }

                    if should_include {
                        parsed_entries.push(entry);
                    }
                },
                Ok(None) => {
                    // Empty line or comment - skip silently
                },
                Err(e) => {
                    eprintln!("解析エラー ({}行目): {}", line_count, e);
                    error_count += 1;
                }
            }
        }

        // Output results
        if parsed_entries.is_empty() {
            if error_count > 0 {
                eprintln!("エラー: {}個の解析エラーが発生しました", error_count);
            } else {
                eprintln!("該当するログエントリが見つかりませんでした");
            }
        } else {
            match formatter.format(&parsed_entries) {
                Ok(output) => println!("{}", output),
                Err(e) => {
                    eprintln!("出力フォーマットエラー: {}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }
}

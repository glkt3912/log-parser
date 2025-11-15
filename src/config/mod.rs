use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub file_path: PathBuf,
    pub level_filter: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub grep_pattern: Option<String>,
    pub output_format: String,
    pub follow: bool,
    pub show_stats: bool,
    pub tui_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_path: PathBuf::new(),
            level_filter: None,
            since: None,
            until: None,
            grep_pattern: None,
            output_format: "text".to_string(),
            follow: false,
            show_stats: false,
            tui_mode: false,
        }
    }
}

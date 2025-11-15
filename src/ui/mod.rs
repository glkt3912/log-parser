// TUI module - will be implemented in TUI phase
#[cfg(feature = "tui")]
pub mod tui;

// Placeholder exports
#[cfg(feature = "tui")]
pub use tui::TuiInterface;

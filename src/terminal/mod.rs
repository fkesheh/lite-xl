//! Terminal emulator integration for Lite XL
//!
//! This module provides an integrated terminal panel with support for:
//! - Multiple terminal tabs
//! - ANSI escape sequence processing
//! - Configurable shell and environment
//! - Cross-platform PTY support (Unix/Windows)
//! - Selection and clipboard operations
//! - Thread-safe state management

pub mod backend;
pub mod buffer;
pub mod clipboard;
pub mod commands;
pub mod config;
pub mod manager;
pub mod parser;
pub mod state;

// Re-export backend types
pub use backend::{
    detect_available_shells, ShellConfig, ShellType, TerminalBackend, TerminalEvent as BackendEvent,
};

#[cfg(feature = "pty")]
pub use backend::{Pty, PtyError, PtyResult};

// Re-export buffer types
pub use buffer::{
    Attributes, Cell, Color, Cursor, CursorShape, Grid, Scrollback, TerminalBuffer,
};

// Re-export higher-level types
pub use clipboard::{Selection, SelectionMode, TerminalClipboard};
pub use commands::TerminalCommand;
pub use config::{CursorStyle, TerminalColors, TerminalConfig};
pub use manager::{Terminal, TerminalId, TerminalManager};
pub use state::{SharedTerminalState, TerminalEvent, TerminalState, TerminalStateHandle};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Terminal manager wrapper for use in reactive contexts
pub type SharedTerminalManager = Arc<RwLock<TerminalManager>>;

/// Create a new terminal manager
pub fn create_terminal_manager(config: TerminalConfig) -> SharedTerminalManager {
    Arc::new(RwLock::new(TerminalManager::new(config)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_manager_creation() {
        let config = TerminalConfig::default();
        let manager = TerminalManager::new(config);
        assert_eq!(manager.terminal_count(), 0);
    }

    #[test]
    fn test_config_default() {
        let config = TerminalConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.scrollback_lines, 10000);
    }

    #[test]
    fn test_clipboard_creation() {
        let clipboard = TerminalClipboard::new();
        assert!(!clipboard.has_selection());
    }
}

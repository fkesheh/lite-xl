//! Clipboard Integration
//!
//! This module provides system clipboard integration for cut, copy, and paste
//! operations. It supports:
//! - Text clipboard operations
//! - Multi-cursor clipboard (each selection stored separately)
//! - Clipboard history
//! - Cross-platform clipboard access
//!
//! # Example
//!
//! ```
//! use clipboard::Clipboard;
//!
//! let mut clipboard = Clipboard::new();
//! clipboard.set_text("Hello, world!");
//! let text = clipboard.get_text();
//! ```

use std::collections::VecDeque;

/// Result type for clipboard operations
pub type ClipboardResult<T> = Result<T, ClipboardError>;

/// Clipboard errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardError {
    /// Failed to access system clipboard
    AccessError(String),

    /// Clipboard is empty
    Empty,

    /// Unsupported clipboard format
    UnsupportedFormat(String),

    /// Platform not supported
    PlatformNotSupported,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::AccessError(msg) => write!(f, "Clipboard access error: {}", msg),
            ClipboardError::Empty => write!(f, "Clipboard is empty"),
            ClipboardError::UnsupportedFormat(fmt) => {
                write!(f, "Unsupported clipboard format: {}", fmt)
            }
            ClipboardError::PlatformNotSupported => write!(f, "Platform not supported"),
        }
    }
}

impl std::error::Error for ClipboardError {}

/// Clipboard manager
///
/// Handles clipboard operations with support for history and multi-cursor editing.
pub struct Clipboard {
    /// Internal clipboard for when system clipboard is unavailable
    internal_clipboard: String,

    /// Clipboard history
    history: VecDeque<String>,

    /// Maximum history size
    max_history: usize,

    /// Multi-cursor clipboard entries
    multi_entries: Vec<String>,
}

impl Clipboard {
    /// Create a new clipboard manager
    pub fn new() -> Self {
        Self {
            internal_clipboard: String::new(),
            history: VecDeque::new(),
            max_history: 100,
            multi_entries: Vec::new(),
        }
    }

    /// Create a clipboard manager with custom history size
    pub fn with_history_size(max_history: usize) -> Self {
        Self {
            internal_clipboard: String::new(),
            history: VecDeque::new(),
            max_history,
            multi_entries: Vec::new(),
        }
    }

    /// Get text from system clipboard
    ///
    /// Falls back to internal clipboard if system clipboard is unavailable.
    pub fn get_text(&self) -> ClipboardResult<String> {
        #[cfg(feature = "system-clipboard")]
        {
            match Self::get_system_clipboard() {
                Ok(text) => Ok(text),
                Err(_) => Ok(self.internal_clipboard.clone()),
            }
        }

        #[cfg(not(feature = "system-clipboard"))]
        {
            Ok(self.internal_clipboard.clone())
        }
    }

    /// Set text to system clipboard
    ///
    /// Also stores in internal clipboard and history.
    pub fn set_text(&mut self, text: String) -> ClipboardResult<()> {
        // Add to history
        if !text.is_empty() && self.history.front() != Some(&text) {
            self.history.push_front(text.clone());
            if self.history.len() > self.max_history {
                self.history.pop_back();
            }
        }

        // Update internal clipboard
        self.internal_clipboard = text.clone();

        // Try to set system clipboard
        #[cfg(feature = "system-clipboard")]
        {
            let _ = Self::set_system_clipboard(&text);
        }

        Ok(())
    }

    /// Get clipboard history
    pub fn get_history(&self) -> &VecDeque<String> {
        &self.history
    }

    /// Get entry from history by index
    pub fn get_history_entry(&self, index: usize) -> Option<&String> {
        self.history.get(index)
    }

    /// Clear clipboard history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Set multi-cursor clipboard entries
    ///
    /// Each entry corresponds to a cursor's copied text.
    pub fn set_multi(&mut self, entries: Vec<String>) {
        self.multi_entries = entries;
    }

    /// Get multi-cursor clipboard entries
    pub fn get_multi(&self) -> &[String] {
        &self.multi_entries
    }

    /// Check if multi-cursor clipboard has entries
    pub fn has_multi(&self) -> bool {
        !self.multi_entries.is_empty()
    }

    /// Clear multi-cursor clipboard
    pub fn clear_multi(&mut self) {
        self.multi_entries.clear();
    }

    /// Get system clipboard text (platform-specific)
    #[cfg(feature = "system-clipboard")]
    fn get_system_clipboard() -> ClipboardResult<String> {
        // In a real implementation, this would use a clipboard library
        // For now, this is a placeholder
        Err(ClipboardError::PlatformNotSupported)
    }

    /// Set system clipboard text (platform-specific)
    #[cfg(feature = "system-clipboard")]
    fn set_system_clipboard(_text: &str) -> ClipboardResult<()> {
        // In a real implementation, this would use a clipboard library
        // For now, this is a placeholder
        Err(ClipboardError::PlatformNotSupported)
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Clipboard context for command execution
///
/// This trait abstracts clipboard operations for testing and different implementations.
pub trait ClipboardContext {
    /// Copy selected text to clipboard
    fn copy(&mut self) -> ClipboardResult<()>;

    /// Cut selected text to clipboard
    fn cut(&mut self) -> ClipboardResult<()>;

    /// Paste text from clipboard
    fn paste(&mut self) -> ClipboardResult<()>;

    /// Get clipboard text
    fn get_clipboard_text(&self) -> ClipboardResult<String>;

    /// Set clipboard text
    fn set_clipboard_text(&mut self, text: String) -> ClipboardResult<()>;
}

/// Execute a copy command
pub fn execute_copy(ctx: &mut impl ClipboardContext) -> ClipboardResult<()> {
    ctx.copy()
}

/// Execute a cut command
pub fn execute_cut(ctx: &mut impl ClipboardContext) -> ClipboardResult<()> {
    ctx.cut()
}

/// Execute a paste command
pub fn execute_paste(ctx: &mut impl ClipboardContext) -> ClipboardResult<()> {
    ctx.paste()
}

/// Rich clipboard content
///
/// Supports different content types beyond plain text.
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    /// Plain text
    Text(String),

    /// Rich text (HTML)
    RichText {
        html: String,
        plain: String,
    },

    /// Code with syntax information
    Code {
        text: String,
        language: Option<String>,
    },

    /// Custom content
    Custom {
        mime_type: String,
        data: Vec<u8>,
    },
}

impl ClipboardContent {
    /// Get plain text representation
    pub fn as_text(&self) -> &str {
        match self {
            ClipboardContent::Text(text) => text,
            ClipboardContent::RichText { plain, .. } => plain,
            ClipboardContent::Code { text, .. } => text,
            ClipboardContent::Custom { .. } => "",
        }
    }

    /// Create from plain text
    pub fn from_text(text: String) -> Self {
        ClipboardContent::Text(text)
    }

    /// Create code content
    pub fn from_code(text: String, language: Option<String>) -> Self {
        ClipboardContent::Code { text, language }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_basic() {
        let mut clipboard = Clipboard::new();

        clipboard.set_text("Hello, world!".to_string()).unwrap();
        let text = clipboard.get_text().unwrap();

        assert_eq!(text, "Hello, world!");
    }

    #[test]
    fn test_clipboard_history() {
        let mut clipboard = Clipboard::with_history_size(3);

        clipboard.set_text("First".to_string()).unwrap();
        clipboard.set_text("Second".to_string()).unwrap();
        clipboard.set_text("Third".to_string()).unwrap();

        assert_eq!(clipboard.get_history().len(), 3);
        assert_eq!(clipboard.get_history_entry(0), Some(&"Third".to_string()));
        assert_eq!(clipboard.get_history_entry(1), Some(&"Second".to_string()));
        assert_eq!(clipboard.get_history_entry(2), Some(&"First".to_string()));
    }

    #[test]
    fn test_clipboard_multi() {
        let mut clipboard = Clipboard::new();

        let entries = vec!["cursor1".to_string(), "cursor2".to_string()];
        clipboard.set_multi(entries.clone());

        assert!(clipboard.has_multi());
        assert_eq!(clipboard.get_multi(), &entries[..]);

        clipboard.clear_multi();
        assert!(!clipboard.has_multi());
    }

    #[test]
    fn test_clipboard_content() {
        let content = ClipboardContent::from_text("test".to_string());
        assert_eq!(content.as_text(), "test");

        let code = ClipboardContent::from_code("fn main() {}".to_string(), Some("rust".to_string()));
        assert_eq!(code.as_text(), "fn main() {}");
    }

    #[test]
    fn test_clipboard_error_display() {
        let error = ClipboardError::Empty;
        assert_eq!(error.to_string(), "Clipboard is empty");

        let error = ClipboardError::AccessError("test".to_string());
        assert_eq!(error.to_string(), "Clipboard access error: test");
    }
}

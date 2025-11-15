//! Lite XL - A lightweight, fast, and extensible text editor.
//!
//! This library provides the core text editing functionality for Lite XL,
//! including buffer management, multi-cursor editing, undo/redo, and document
//! abstraction.
//!
//! # Architecture
//!
//! The library is organized into several key modules:
//!
//! - **buffer**: Low-level text buffer using rope data structure
//! - **document**: High-level document abstraction with editing state
//! - **undo**: Undo/redo system with time-based grouping
//!
//! # Examples
//!
//! ## Basic Document Editing
//!
//! ```
//! use lite_xl::document::Document;
//! use lite_xl::buffer::Position;
//!
//! // Create a new document
//! let mut doc = Document::new();
//!
//! // Insert some text
//! doc.insert("Hello, world!");
//!
//! // The buffer contains the text
//! assert_eq!(doc.buffer().to_string(), "Hello, world!");
//!
//! // Undo the insertion
//! doc.undo();
//! assert_eq!(doc.buffer().to_string(), "");
//!
//! // Redo
//! doc.redo();
//! assert_eq!(doc.buffer().to_string(), "Hello, world!");
//! ```
//!
//! ## Multi-Cursor Editing
//!
//! ```
//! use lite_xl::document::{Document, Selections, Selection};
//! use lite_xl::buffer::Position;
//!
//! let mut doc = Document::from_str("line1\nline2\nline3");
//!
//! // Create multiple cursors
//! let mut selections = Selections::from_vec(vec![
//!     Selection::cursor(Position::new(0, 0)),
//!     Selection::cursor(Position::new(1, 0)),
//!     Selection::cursor(Position::new(2, 0)),
//! ]);
//!
//! doc.set_selections(selections);
//!
//! // Insert at all cursors
//! doc.insert("> ");
//!
//! assert_eq!(doc.buffer().to_string(), "> line1\n> line2\n> line3");
//! ```
//!
//! ## Working with Buffers
//!
//! ```
//! use lite_xl::buffer::{Buffer, Position, Range};
//!
//! let mut buffer = Buffer::from_str("Hello, world!");
//!
//! // Query buffer
//! assert_eq!(buffer.line_count(), 1);
//! assert_eq!(buffer.len_chars(), 13);
//!
//! // Insert text
//! buffer.insert(Position::new(0, 7), "beautiful ").unwrap();
//! assert_eq!(buffer.to_string(), "Hello, beautiful world!");
//!
//! // Delete text
//! buffer.delete(Range::new(
//!     Position::new(0, 5),
//!     Position::new(0, 7)
//! )).unwrap();
//! assert_eq!(buffer.to_string(), "Hellobeautiful world!");
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod buffer;
pub mod clipboard;
pub mod commands;
pub mod config;
pub mod document;
pub mod events;
pub mod terminal;
pub mod undo;

// Re-export commonly used types
pub use buffer::{Buffer, BufferId, LineEnding, Position, Range};
pub use clipboard::{Clipboard, ClipboardContext};
pub use commands::{Command, Key, KeyBinding, KeyMap, Modifiers, Movement as CommandMovement};
pub use config::{Config, EditorConfig, KeymapConfig, LanguageConfig, UiConfig};
pub use document::{Document, DocumentSettings, Movement, Selection, Selections};
pub use events::{EditorEvent, EventDispatcher, EventHandler, KeyEvent, MouseEvent};
pub use terminal::{
    // Backend types
    detect_available_shells, ShellConfig, ShellType, TerminalBackend,
    // Buffer types
    Attributes as TerminalAttributes, Cell as TerminalCell, Color as TerminalColor,
    Cursor as TerminalCursor, CursorShape, Grid as TerminalGrid,
    Scrollback, TerminalBuffer,
    // Manager types
    TerminalCommand, TerminalConfig, TerminalId, TerminalManager,
    SharedTerminalManager, create_terminal_manager,
};

#[cfg(feature = "pty")]
pub use terminal::{Pty, PtyError, PtyResult};
pub use undo::{Edit, UndoConfig, UndoStack};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_basic_workflow() {
        let mut doc = Document::new();
        doc.insert("Hello");
        assert_eq!(doc.buffer().to_string(), "Hello");
        
        assert!(doc.undo());
        assert_eq!(doc.buffer().to_string(), "");
        
        assert!(doc.redo());
        assert_eq!(doc.buffer().to_string(), "Hello");
    }

    #[test]
    fn test_multi_cursor_editing() {
        let mut doc = Document::from_str("a\nb\nc");
        
        let selections = Selections::from_vec(vec![
            Selection::cursor(Position::new(0, 0)),
            Selection::cursor(Position::new(1, 0)),
            Selection::cursor(Position::new(2, 0)),
        ]);
        doc.set_selections(selections);
        
        doc.insert(">");

        let buffer_text = doc.buffer().to_string();
        let lines: Vec<_> = buffer_text.lines().collect();
        assert_eq!(lines, vec![">a", ">b", ">c"]);
    }
}

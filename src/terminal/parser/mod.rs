//! Terminal ANSI escape sequence parser.
//!
//! This module provides comprehensive ANSI/VT escape sequence parsing using the `vte` crate.
//! It handles all common terminal sequences including:
//!
//! - **CSI sequences**: Cursor movement, colors, text attributes, screen clearing
//! - **OSC sequences**: Window title, hyperlinks, color palette manipulation
//! - **SGR (Select Graphic Rendition)**: Text formatting and colors
//!   - Named colors (0-15)
//!   - 256-color palette (0-255)
//!   - Truecolor/24-bit RGB
//!   - Text attributes: bold, italic, underline, strikethrough, blink, etc.
//!
//! # Architecture
//!
//! The parser is built on top of the `vte` crate, which provides a robust state machine
//! for parsing VT/ANSI escape sequences. The architecture consists of:
//!
//! - **AnsiParser**: High-level parser interface
//! - **AnsiPerformer**: Implementation of `vte::Perform` trait
//! - **TerminalAction**: High-level terminal actions
//! - **Sequences**: Terminal sequence type definitions
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```
//! use lite_xl::terminal::parser::{AnsiParser, TerminalAction};
//!
//! let mut parser = AnsiParser::new();
//!
//! // Parse some ANSI escape sequences
//! let actions = parser.parse(b"Hello \x1b[31mworld\x1b[0m!");
//!
//! // Process the actions
//! for action in actions {
//!     match action {
//!         TerminalAction::Print(c) => {
//!             // Print character to terminal
//!             print!("{}", c);
//!         }
//!         TerminalAction::SetForeground(color) => {
//!             // Change text color
//!             println!("Set foreground to {:?}", color);
//!         }
//!         TerminalAction::ResetAttributes => {
//!             // Reset text attributes
//!             println!("Reset attributes");
//!         }
//!         _ => {}
//!     }
//! }
//! ```
//!
//! ## Cursor Movement
//!
//! ```
//! use lite_xl::terminal::parser::{AnsiParser, TerminalAction};
//!
//! let mut parser = AnsiParser::new();
//!
//! // Move cursor to position (10, 20)
//! let actions = parser.parse(b"\x1b[10;20H");
//! assert!(matches!(
//!     actions[0],
//!     TerminalAction::CursorGoTo { line: 10, col: 20 }
//! ));
//!
//! // Move cursor up 5 lines
//! let actions = parser.parse(b"\x1b[5A");
//! assert!(matches!(actions[0], TerminalAction::CursorUp(5)));
//! ```
//!
//! ## Colors
//!
//! ```
//! use lite_xl::terminal::parser::{AnsiParser, TerminalAction, Color, NamedColor, Rgb};
//!
//! let mut parser = AnsiParser::new();
//!
//! // Named color (red)
//! let actions = parser.parse(b"\x1b[31m");
//! assert!(matches!(
//!     actions[0],
//!     TerminalAction::SetForeground(Color::Named(NamedColor::Red))
//! ));
//!
//! // 256-color palette
//! let actions = parser.parse(b"\x1b[38;5;196m");
//! assert!(matches!(
//!     actions[0],
//!     TerminalAction::SetForeground(Color::Indexed(196))
//! ));
//!
//! // 24-bit RGB color
//! let actions = parser.parse(b"\x1b[38;2;255;128;64m");
//! if let TerminalAction::SetForeground(Color::Rgb(rgb)) = &actions[0] {
//!     assert_eq!(rgb.r, 255);
//!     assert_eq!(rgb.g, 128);
//!     assert_eq!(rgb.b, 64);
//! }
//! ```
//!
//! ## Text Attributes
//!
//! ```
//! use lite_xl::terminal::parser::{AnsiParser, TerminalAction, AttributeChange};
//!
//! let mut parser = AnsiParser::new();
//!
//! // Bold, italic, and underline
//! let actions = parser.parse(b"\x1b[1;3;4m");
//! assert_eq!(actions.len(), 3);
//! ```
//!
//! ## OSC Sequences
//!
//! ```
//! use lite_xl::terminal::parser::{AnsiParser, TerminalAction};
//!
//! let mut parser = AnsiParser::new();
//!
//! // Set window title
//! let actions = parser.parse(b"\x1b]0;My Terminal\x07");
//! if let TerminalAction::SetTitle(title) = &actions[0] {
//!     assert_eq!(title, "My Terminal");
//! }
//!
//! // Set hyperlink
//! let actions = parser.parse(b"\x1b]8;;https://example.com\x07");
//! if let TerminalAction::SetHyperlink { url, .. } = &actions[0] {
//!     assert_eq!(url.as_ref().unwrap(), "https://example.com");
//! }
//! ```
//!
//! ## Streaming Input
//!
//! ```
//! use lite_xl::terminal::parser::AnsiParser;
//!
//! let mut parser = AnsiParser::new();
//! let input = b"\x1b[31mHello";
//!
//! // Process byte by byte
//! for &byte in input {
//!     let actions = parser.parse_byte(byte);
//!     // Handle actions as they come
//!     for action in actions {
//!         println!("{:?}", action);
//!     }
//! }
//! ```

pub mod ansi;
pub mod performer;
pub mod sequences;

// Re-export commonly used types
pub use ansi::{parse, parse_str, AnsiParser};
pub use performer::AnsiPerformer;
pub use sequences::{
    Attributes, AttributeChange, Color, CursorShape, Mode, NamedColor, Rgb, TerminalAction,
    UnderlineStyle,
};

// Type alias for backward compatibility
pub type Action = TerminalAction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basic() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"Hello");
        assert_eq!(actions.len(), 5);
    }

    #[test]
    fn test_convenience_functions() {
        let actions = parse(b"Test");
        assert_eq!(actions.len(), 4);

        let actions = parse_str("Test");
        assert_eq!(actions.len(), 4);
    }

    #[test]
    fn test_comprehensive_sequence() {
        let mut parser = AnsiParser::new();

        // Complex terminal sequence combining multiple features
        let input = concat!(
            "\x1b[2J",           // Clear screen
            "\x1b[1;1H",         // Move to home
            "\x1b[1;31m",        // Bold red
            "Error: ",
            "\x1b[0m",           // Reset
            "\x1b[2mFile not found\x1b[22m", // Dim text
            "\r\n",              // New line
            "\x1b]8;;https://example.com\x07", // Hyperlink
            "Click here",
            "\x1b]8;;\x07",      // End hyperlink
        );

        let actions = parser.parse(input.as_bytes());
        assert!(!actions.is_empty());

        // Verify we got the major actions
        let has_clear = actions.iter().any(|a| matches!(a, TerminalAction::ClearScreen));
        let has_cursor = actions.iter().any(|a| matches!(a, TerminalAction::CursorGoTo { .. }));
        let has_color = actions.iter().any(|a| matches!(a, TerminalAction::SetForeground(_)));
        let has_reset = actions.iter().any(|a| matches!(a, TerminalAction::ResetAttributes));

        assert!(has_clear, "Should have clear screen action");
        assert!(has_cursor, "Should have cursor positioning action");
        assert!(has_color, "Should have color action");
        assert!(has_reset, "Should have reset action");
    }

    #[test]
    fn test_all_named_colors() {
        let mut parser = AnsiParser::new();

        // Test all 16 named colors
        for i in 0..8 {
            // Normal colors (30-37)
            let seq = format!("\x1b[{}m", 30 + i);
            let actions = parser.parse(seq.as_bytes());
            assert_eq!(actions.len(), 1);

            // Bright colors (90-97)
            let seq = format!("\x1b[{}m", 90 + i);
            let actions = parser.parse(seq.as_bytes());
            assert_eq!(actions.len(), 1);
        }
    }

    #[test]
    fn test_256_color_range() {
        let mut parser = AnsiParser::new();

        // Test various 256-color indices
        for &index in &[0, 16, 128, 196, 255] {
            let seq = format!("\x1b[38;5;{}m", index);
            let actions = parser.parse(seq.as_bytes());
            assert_eq!(actions.len(), 1);
            if let TerminalAction::SetForeground(Color::Indexed(idx)) = actions[0] {
                assert_eq!(idx, index);
            } else {
                panic!("Expected indexed color");
            }
        }
    }

    #[test]
    fn test_rgb_colors() {
        let mut parser = AnsiParser::new();

        let test_cases = [
            (0, 0, 0),           // Black
            (255, 255, 255),     // White
            (255, 0, 0),         // Red
            (0, 255, 0),         // Green
            (0, 0, 255),         // Blue
            (128, 128, 128),     // Gray
        ];

        for (r, g, b) in test_cases {
            let seq = format!("\x1b[38;2;{};{};{}m", r, g, b);
            let actions = parser.parse(seq.as_bytes());
            assert_eq!(actions.len(), 1);
            if let TerminalAction::SetForeground(Color::Rgb(rgb)) = &actions[0] {
                assert_eq!(rgb.r, r);
                assert_eq!(rgb.g, g);
                assert_eq!(rgb.b, b);
            } else {
                panic!("Expected RGB color");
            }
        }
    }

    #[test]
    fn test_all_cursor_movements() {
        let mut parser = AnsiParser::new();

        // Up, down, forward, backward
        let sequences = [
            (b"\x1b[A".as_slice(), "CursorUp"),
            (b"\x1b[B", "CursorDown"),
            (b"\x1b[C", "CursorForward"),
            (b"\x1b[D", "CursorBackward"),
            (b"\x1b[H", "CursorGoTo"),
            (b"\x1b[G", "CursorGoToColumn"),
        ];

        for (seq, name) in sequences {
            let actions = parser.parse(seq);
            assert!(!actions.is_empty(), "No actions for {}", name);
        }
    }

    #[test]
    fn test_all_clear_operations() {
        let mut parser = AnsiParser::new();

        let sequences = [
            (b"\x1b[J", "ClearToEndOfScreen"),
            (b"\x1b[1J", "ClearToBeginningOfScreen"),
            (b"\x1b[2J", "ClearScreen"),
            (b"\x1b[K", "ClearToEndOfLine"),
            (b"\x1b[1K", "ClearToBeginningOfLine"),
            (b"\x1b[2K", "ClearLine"),
        ];

        for (seq, _name) in sequences {
            let actions = parser.parse(seq);
            assert_eq!(actions.len(), 1);
        }
    }

    #[test]
    fn test_text_attributes() {
        let mut parser = AnsiParser::new();

        let attributes = [
            (b"\x1b[1m", "Bold"),
            (b"\x1b[2m", "Dim"),
            (b"\x1b[3m", "Italic"),
            (b"\x1b[4m", "Underline"),
            (b"\x1b[5m", "Blink"),
            (b"\x1b[7m", "Reverse"),
            (b"\x1b[8m", "Hidden"),
            (b"\x1b[9m", "Strikethrough"),
        ];

        for (seq, _name) in attributes {
            let actions = parser.parse(seq);
            assert_eq!(actions.len(), 1);
        }
    }

    #[test]
    fn test_attribute_reset() {
        let mut parser = AnsiParser::new();

        let resets = [
            (b"\x1b[22m", "NormalIntensity"),
            (b"\x1b[23m", "NoItalic"),
            (b"\x1b[24m", "NoUnderline"),
            (b"\x1b[25m", "NoBlink"),
            (b"\x1b[27m", "NoReverse"),
            (b"\x1b[28m", "NoHidden"),
            (b"\x1b[29m", "NoStrikethrough"),
        ];

        for (seq, _name) in resets {
            let actions = parser.parse(seq);
            assert_eq!(actions.len(), 1);
        }
    }

    #[test]
    fn test_control_characters() {
        let mut parser = AnsiParser::new();

        let controls = [
            (b"\x07", "Bell"),
            (b"\x08", "Backspace"),
            (b"\x09", "Tab"),
            (b"\x0a", "LineFeed"),
            (b"\x0d", "CarriageReturn"),
        ];

        for (seq, _name) in controls {
            let actions = parser.parse(seq);
            assert_eq!(actions.len(), 1);
        }
    }

    #[test]
    fn test_real_world_output() {
        let mut parser = AnsiParser::new();

        // Simulated output from a real terminal application
        let input = b"\x1b[1;1H\x1b[2J\x1b[1;32m$\x1b[0m ls -la\r\n\
                      total 48\r\n\
                      drwxr-xr-x  12 user  staff   384 Nov 15 10:00 \x1b[1;34m.\x1b[0m\r\n\
                      drwxr-xr-x   3 user  staff    96 Nov 14 15:30 \x1b[1;34m..\x1b[0m\r\n\
                      -rw-r--r--   1 user  staff  1234 Nov 15 09:45 \x1b[0mfile.txt\x1b[0m\r\n";

        let actions = parser.parse(input);
        assert!(!actions.is_empty());

        // Should contain various action types
        let has_print = actions.iter().any(|a| matches!(a, TerminalAction::Print(_)));
        let has_color = actions.iter().any(|a| matches!(a, TerminalAction::SetForeground(_)));
        let has_control = actions.iter().any(|a| matches!(a, TerminalAction::LineFeed));

        assert!(has_print);
        assert!(has_color);
        assert!(has_control);
    }
}

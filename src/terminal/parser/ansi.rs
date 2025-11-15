//! ANSI escape sequence parser and handler.
//!
//! This module provides the main ANSI parser that processes terminal input
//! and generates terminal actions.

use super::performer::AnsiPerformer;
use super::sequences::TerminalAction;
use vte::Parser;

/// ANSI escape sequence parser.
///
/// Wraps the vte parser and provides a high-level interface for parsing
/// terminal input and generating terminal actions.
///
/// # Examples
///
/// ```
/// use lite_xl::terminal::parser::AnsiParser;
///
/// let mut parser = AnsiParser::new();
///
/// // Parse some input
/// let actions = parser.parse(b"Hello, \x1b[31mworld\x1b[0m!");
///
/// // Process the actions
/// for action in actions {
///     println!("{:?}", action);
/// }
/// ```
pub struct AnsiParser {
    /// VTE state machine
    parser: Parser,
    /// Action performer
    performer: AnsiPerformer,
}

impl std::fmt::Debug for AnsiParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnsiParser")
            .field("performer", &self.performer)
            .finish_non_exhaustive()
    }
}

impl Default for AnsiParser {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsiParser {
    /// Create a new ANSI parser
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            performer: AnsiPerformer::new(),
        }
    }

    /// Parse a byte slice and return the generated actions.
    ///
    /// This method processes the input bytes through the VTE state machine
    /// and returns all terminal actions that were generated.
    ///
    /// # Arguments
    ///
    /// * `input` - The bytes to parse
    ///
    /// # Returns
    ///
    /// A vector of terminal actions generated from the input
    pub fn parse(&mut self, input: &[u8]) -> Vec<TerminalAction> {
        for &byte in input {
            self.parser.advance(&mut self.performer, byte);
        }
        self.performer.take_actions()
    }

    /// Parse a single byte and return any generated actions.
    ///
    /// This is useful for streaming input processing.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to parse
    ///
    /// # Returns
    ///
    /// A vector of terminal actions generated from this byte
    pub fn parse_byte(&mut self, byte: u8) -> Vec<TerminalAction> {
        self.parser.advance(&mut self.performer, byte);
        self.performer.take_actions()
    }

    /// Parse a string and return the generated actions.
    ///
    /// Convenience method for parsing UTF-8 strings.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to parse
    ///
    /// # Returns
    ///
    /// A vector of terminal actions generated from the input
    pub fn parse_str(&mut self, input: &str) -> Vec<TerminalAction> {
        self.parse(input.as_bytes())
    }

    /// Get reference to pending actions without clearing them
    pub fn peek_actions(&self) -> &[TerminalAction] {
        self.performer.actions()
    }

    /// Clear all pending actions
    pub fn clear_actions(&mut self) {
        self.performer.take_actions();
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.parser = Parser::new();
        self.clear_actions();
    }
}

/// Helper function to parse ANSI escape sequences from a byte slice
///
/// This is a convenience function that creates a parser, parses the input,
/// and returns the actions.
///
/// # Arguments
///
/// * `input` - The bytes to parse
///
/// # Returns
///
/// A vector of terminal actions
pub fn parse(input: &[u8]) -> Vec<TerminalAction> {
    let mut parser = AnsiParser::new();
    parser.parse(input)
}

/// Helper function to parse ANSI escape sequences from a string
///
/// This is a convenience function that creates a parser, parses the input,
/// and returns the actions.
///
/// # Arguments
///
/// * `input` - The string to parse
///
/// # Returns
///
/// A vector of terminal actions
pub fn parse_str(input: &str) -> Vec<TerminalAction> {
    parse(input.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::parser::sequences::{Color, NamedColor};

    #[test]
    fn test_parse_plain_text() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"Hello");

        assert_eq!(actions.len(), 5);
        assert!(matches!(actions[0], TerminalAction::Print('H')));
        assert!(matches!(actions[1], TerminalAction::Print('e')));
        assert!(matches!(actions[2], TerminalAction::Print('l')));
        assert!(matches!(actions[3], TerminalAction::Print('l')));
        assert!(matches!(actions[4], TerminalAction::Print('o')));
    }

    #[test]
    fn test_parse_simple_color() {
        let mut parser = AnsiParser::new();
        // ESC[31m = red foreground
        let actions = parser.parse(b"\x1b[31m");

        assert_eq!(actions.len(), 1);
        assert!(matches!(
            actions[0],
            TerminalAction::SetForeground(Color::Named(NamedColor::Red))
        ));
    }

    #[test]
    fn test_parse_text_with_color() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"Hello \x1b[31mworld\x1b[0m!");

        // "Hello " = 6 chars
        // ESC[31m = 1 action (set red)
        // "world" = 5 chars
        // ESC[0m = 1 action (reset)
        // "!" = 1 char
        assert_eq!(actions.len(), 13);
    }

    #[test]
    fn test_parse_cursor_movement() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"\x1b[5A\x1b[3B\x1b[2C\x1b[4D");

        assert_eq!(actions.len(), 4);
        assert!(matches!(actions[0], TerminalAction::CursorUp(5)));
        assert!(matches!(actions[1], TerminalAction::CursorDown(3)));
        assert!(matches!(actions[2], TerminalAction::CursorForward(2)));
        assert!(matches!(actions[3], TerminalAction::CursorBackward(4)));
    }

    #[test]
    fn test_parse_cursor_position() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"\x1b[10;20H");

        assert_eq!(actions.len(), 1);
        assert!(matches!(
            actions[0],
            TerminalAction::CursorGoTo { line: 10, col: 20 }
        ));
    }

    #[test]
    fn test_parse_clear_screen() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"\x1b[2J");

        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], TerminalAction::ClearScreen));
    }

    #[test]
    fn test_parse_clear_line() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"\x1b[2K");

        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], TerminalAction::ClearLine));
    }

    #[test]
    fn test_parse_256_color() {
        let mut parser = AnsiParser::new();
        // ESC[38;5;196m = 256-color foreground (bright red)
        let actions = parser.parse(b"\x1b[38;5;196m");

        assert_eq!(actions.len(), 1);
        assert!(matches!(
            actions[0],
            TerminalAction::SetForeground(Color::Indexed(196))
        ));
    }

    #[test]
    fn test_parse_rgb_color() {
        let mut parser = AnsiParser::new();
        // ESC[38;2;255;128;64m = RGB foreground
        let actions = parser.parse(b"\x1b[38;2;255;128;64m");

        assert_eq!(actions.len(), 1);
        if let TerminalAction::SetForeground(Color::Rgb(rgb)) = &actions[0] {
            assert_eq!(rgb.r, 255);
            assert_eq!(rgb.g, 128);
            assert_eq!(rgb.b, 64);
        } else {
            panic!("Expected RGB color");
        }
    }

    #[test]
    fn test_parse_control_chars() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"\r\n\t");

        assert_eq!(actions.len(), 3);
        assert!(matches!(actions[0], TerminalAction::CarriageReturn));
        assert!(matches!(actions[1], TerminalAction::LineFeed));
        assert!(matches!(actions[2], TerminalAction::Tab));
    }

    #[test]
    fn test_parse_bell() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"\x07");

        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], TerminalAction::Bell));
    }

    #[test]
    fn test_parse_title() {
        let mut parser = AnsiParser::new();
        // OSC 0;title BEL
        let actions = parser.parse(b"\x1b]0;Test Title\x07");

        assert_eq!(actions.len(), 1);
        if let TerminalAction::SetTitle(title) = &actions[0] {
            assert_eq!(title, "Test Title");
        } else {
            panic!("Expected SetTitle action");
        }
    }

    #[test]
    fn test_parse_hyperlink() {
        let mut parser = AnsiParser::new();
        // OSC 8;;url BEL
        let actions = parser.parse(b"\x1b]8;;https://example.com\x07");

        assert_eq!(actions.len(), 1);
        if let TerminalAction::SetHyperlink { url, id } = &actions[0] {
            assert_eq!(url.as_ref().unwrap(), "https://example.com");
            assert!(id.is_none());
        } else {
            panic!("Expected SetHyperlink action");
        }
    }

    #[test]
    fn test_parse_multiple_sgr() {
        let mut parser = AnsiParser::new();
        // ESC[1;3;4m = bold, italic, underline
        let actions = parser.parse(b"\x1b[1;3;4m");

        assert_eq!(actions.len(), 3);
    }

    #[test]
    fn test_parse_reset() {
        let mut parser = AnsiParser::new();
        let actions = parser.parse(b"\x1b[0m");

        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], TerminalAction::ResetAttributes));
    }

    #[test]
    fn test_parse_byte_by_byte() {
        let mut parser = AnsiParser::new();
        let input = b"\x1b[31mHello\x1b[0m";

        let mut all_actions = Vec::new();
        for &byte in input {
            all_actions.extend(parser.parse_byte(byte));
        }

        // Should have: set red, 5 chars, reset
        assert_eq!(all_actions.len(), 7);
    }

    #[test]
    fn test_parse_str_convenience() {
        let actions = parse_str("Hello\x1b[31m World");
        // 5 chars + color change + space + 5 chars
        assert!(actions.len() >= 11);
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = AnsiParser::new();
        parser.parse(b"Hello");
        parser.reset();

        let actions = parser.peek_actions();
        assert_eq!(actions.len(), 0);
    }

    #[test]
    fn test_complex_sequence() {
        let mut parser = AnsiParser::new();
        // Complex sequence: position cursor, set color, print text, reset
        let actions = parser.parse(b"\x1b[1;1H\x1b[1;31;4mHello\x1b[0m");

        // Should have: cursor position, bold, red, underline, 5 chars, reset
        assert!(actions.len() >= 8);
    }
}

//! Terminal cell representation.
//!
//! This module provides the [`Cell`] type which represents a single character
//! cell in the terminal grid, including its character, foreground/background
//! colors, and text attributes.

use serde::{Deserialize, Serialize};

/// ANSI color codes for terminal output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    /// Default terminal color.
    Default,
    /// Black (ANSI color 0).
    Black,
    /// Red (ANSI color 1).
    Red,
    /// Green (ANSI color 2).
    Green,
    /// Yellow (ANSI color 3).
    Yellow,
    /// Blue (ANSI color 4).
    Blue,
    /// Magenta (ANSI color 5).
    Magenta,
    /// Cyan (ANSI color 6).
    Cyan,
    /// White (ANSI color 7).
    White,
    /// Bright black (ANSI color 8).
    BrightBlack,
    /// Bright red (ANSI color 9).
    BrightRed,
    /// Bright green (ANSI color 10).
    BrightGreen,
    /// Bright yellow (ANSI color 11).
    BrightYellow,
    /// Bright blue (ANSI color 12).
    BrightBlue,
    /// Bright magenta (ANSI color 13).
    BrightMagenta,
    /// Bright cyan (ANSI color 14).
    BrightCyan,
    /// Bright white (ANSI color 15).
    BrightWhite,
    /// 8-bit indexed color (0-255).
    Indexed(u8),
    /// 24-bit RGB color.
    Rgb(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Self {
        Color::Default
    }
}

impl Color {
    /// Converts an ANSI color index to a Color.
    pub fn from_ansi(index: u8) -> Self {
        match index {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,
            8 => Color::BrightBlack,
            9 => Color::BrightRed,
            10 => Color::BrightGreen,
            11 => Color::BrightYellow,
            12 => Color::BrightBlue,
            13 => Color::BrightMagenta,
            14 => Color::BrightCyan,
            15 => Color::BrightWhite,
            _ => Color::Indexed(index),
        }
    }
}

/// Text attributes for terminal cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Attributes {
    /// Bold text.
    pub bold: bool,
    /// Dim/faint text.
    pub dim: bool,
    /// Italic text.
    pub italic: bool,
    /// Underlined text.
    pub underline: bool,
    /// Blinking text.
    pub blink: bool,
    /// Reverse video (swap fg/bg colors).
    pub reverse: bool,
    /// Hidden/invisible text.
    pub hidden: bool,
    /// Strikethrough text.
    pub strikethrough: bool,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            hidden: false,
            strikethrough: false,
        }
    }
}

impl Attributes {
    /// Creates a new Attributes with all attributes set to false.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets all attributes to their default values.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Checks if any attributes are set.
    pub fn is_empty(&self) -> bool {
        !self.bold
            && !self.dim
            && !self.italic
            && !self.underline
            && !self.blink
            && !self.reverse
            && !self.hidden
            && !self.strikethrough
    }
}

/// A single cell in the terminal grid.
///
/// Each cell contains a character and its associated styling attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    /// The character displayed in this cell.
    pub c: char,
    /// Foreground color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Text attributes (bold, italic, etc.).
    pub attrs: Attributes,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            fg: Color::Default,
            bg: Color::Default,
            attrs: Attributes::default(),
        }
    }
}

impl Cell {
    /// Creates a new cell with the given character and default styling.
    pub fn new(c: char) -> Self {
        Self {
            c,
            ..Default::default()
        }
    }

    /// Creates a new cell with the given character and styling.
    pub fn with_style(c: char, fg: Color, bg: Color, attrs: Attributes) -> Self {
        Self { c, fg, bg, attrs }
    }

    /// Resets the cell to its default state (space with default colors).
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Checks if this cell is empty (contains a space with default styling).
    pub fn is_empty(&self) -> bool {
        self.c == ' '
            && self.fg == Color::Default
            && self.bg == Color::Default
            && self.attrs.is_empty()
    }

    /// Clears the cell (sets it to a space with default styling).
    pub fn clear(&mut self) {
        self.reset();
    }

    /// Sets the character of this cell.
    pub fn set_char(&mut self, c: char) {
        self.c = c;
    }

    /// Sets the foreground color of this cell.
    pub fn set_fg(&mut self, fg: Color) {
        self.fg = fg;
    }

    /// Sets the background color of this cell.
    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }

    /// Sets the attributes of this cell.
    pub fn set_attrs(&mut self, attrs: Attributes) {
        self.attrs = attrs;
    }

    /// Applies the given attributes to this cell (merges with existing attributes).
    pub fn apply_attrs(&mut self, attrs: Attributes) {
        self.attrs.bold |= attrs.bold;
        self.attrs.dim |= attrs.dim;
        self.attrs.italic |= attrs.italic;
        self.attrs.underline |= attrs.underline;
        self.attrs.blink |= attrs.blink;
        self.attrs.reverse |= attrs.reverse;
        self.attrs.hidden |= attrs.hidden;
        self.attrs.strikethrough |= attrs.strikethrough;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.c, ' ');
        assert_eq!(cell.fg, Color::Default);
        assert_eq!(cell.bg, Color::Default);
        assert!(cell.attrs.is_empty());
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new('a');
        assert_eq!(cell.c, 'a');
        assert_eq!(cell.fg, Color::Default);
        assert_eq!(cell.bg, Color::Default);
    }

    #[test]
    fn test_cell_with_style() {
        let mut attrs = Attributes::new();
        attrs.bold = true;
        let cell = Cell::with_style('b', Color::Red, Color::Blue, attrs);
        assert_eq!(cell.c, 'b');
        assert_eq!(cell.fg, Color::Red);
        assert_eq!(cell.bg, Color::Blue);
        assert!(cell.attrs.bold);
    }

    #[test]
    fn test_cell_reset() {
        let mut cell = Cell::with_style('x', Color::Green, Color::Yellow, Attributes::new());
        cell.reset();
        assert_eq!(cell, Cell::default());
    }

    #[test]
    fn test_color_from_ansi() {
        assert_eq!(Color::from_ansi(0), Color::Black);
        assert_eq!(Color::from_ansi(1), Color::Red);
        assert_eq!(Color::from_ansi(9), Color::BrightRed);
        assert_eq!(Color::from_ansi(255), Color::Indexed(255));
    }

    #[test]
    fn test_attributes_reset() {
        let mut attrs = Attributes::new();
        attrs.bold = true;
        attrs.italic = true;
        attrs.reset();
        assert!(attrs.is_empty());
    }
}

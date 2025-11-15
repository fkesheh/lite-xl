//! Terminal escape sequence definitions.
//!
//! This module defines all the terminal escape sequences that can be parsed
//! and executed by the ANSI parser.

use std::fmt;

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
}

impl Rgb {
    /// Create a new RGB color
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create from hex color code
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// ANSI color modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Named color (0-15)
    Named(NamedColor),
    /// 256-color palette index (0-255)
    Indexed(u8),
    /// 24-bit RGB color
    Rgb(Rgb),
}

impl Color {
    /// Create an indexed color
    pub const fn indexed(index: u8) -> Self {
        Self::Indexed(index)
    }

    /// Create an RGB color
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(Rgb::new(r, g, b))
    }
}

/// Named ANSI colors (0-15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NamedColor {
    /// Black
    Black = 0,
    /// Red
    Red = 1,
    /// Green
    Green = 2,
    /// Yellow
    Yellow = 3,
    /// Blue
    Blue = 4,
    /// Magenta
    Magenta = 5,
    /// Cyan
    Cyan = 6,
    /// White
    White = 7,
    /// Bright Black (Gray)
    BrightBlack = 8,
    /// Bright Red
    BrightRed = 9,
    /// Bright Green
    BrightGreen = 10,
    /// Bright Yellow
    BrightYellow = 11,
    /// Bright Blue
    BrightBlue = 12,
    /// Bright Magenta
    BrightMagenta = 13,
    /// Bright Cyan
    BrightCyan = 14,
    /// Bright White
    BrightWhite = 15,
}

impl NamedColor {
    /// Convert from ANSI color code
    pub fn from_ansi(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Black),
            1 => Some(Self::Red),
            2 => Some(Self::Green),
            3 => Some(Self::Yellow),
            4 => Some(Self::Blue),
            5 => Some(Self::Magenta),
            6 => Some(Self::Cyan),
            7 => Some(Self::White),
            8 => Some(Self::BrightBlack),
            9 => Some(Self::BrightRed),
            10 => Some(Self::BrightGreen),
            11 => Some(Self::BrightYellow),
            12 => Some(Self::BrightBlue),
            13 => Some(Self::BrightMagenta),
            14 => Some(Self::BrightCyan),
            15 => Some(Self::BrightWhite),
            _ => None,
        }
    }
}

/// Text attributes (SGR - Select Graphic Rendition)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attributes {
    /// Bold/increased intensity
    pub bold: bool,
    /// Faint/decreased intensity
    pub dim: bool,
    /// Italic
    pub italic: bool,
    /// Underline
    pub underline: UnderlineStyle,
    /// Slow blink
    pub blink: bool,
    /// Rapid blink
    pub blink_fast: bool,
    /// Reverse video (swap fg/bg)
    pub reverse: bool,
    /// Concealed/hidden
    pub hidden: bool,
    /// Strikethrough
    pub strikethrough: bool,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            bold: false,
            dim: false,
            italic: false,
            underline: UnderlineStyle::None,
            blink: false,
            blink_fast: false,
            reverse: false,
            hidden: false,
            strikethrough: false,
        }
    }
}

impl Attributes {
    /// Create a new attributes set with all attributes off
    pub const fn new() -> Self {
        Self {
            bold: false,
            dim: false,
            italic: false,
            underline: UnderlineStyle::None,
            blink: false,
            blink_fast: false,
            reverse: false,
            hidden: false,
            strikethrough: false,
        }
    }

    /// Reset all attributes to default
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Underline styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnderlineStyle {
    /// No underline
    None,
    /// Single underline
    Single,
    /// Double underline
    Double,
    /// Curly underline
    Curly,
    /// Dotted underline
    Dotted,
    /// Dashed underline
    Dashed,
}

/// Cursor shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorShape {
    /// Block cursor
    Block,
    /// Underline cursor
    Underline,
    /// Vertical bar cursor
    Bar,
}

/// Terminal mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Application cursor keys
    AppCursor,
    /// Application keypad
    AppKeypad,
    /// Bracketed paste mode
    BracketedPaste,
    /// Show cursor
    ShowCursor,
    /// Line wrap
    LineWrap,
    /// Origin mode (relative/absolute positioning)
    Origin,
    /// Insert mode
    Insert,
    /// Alternate screen buffer
    AltScreen,
    /// Mouse reporting (X10 compatibility mode)
    MouseX10,
    /// Mouse reporting (VT200 mode)
    MouseVt200,
    /// Mouse reporting (VT200 highlight mode)
    MouseVt200Highlight,
    /// Mouse reporting (button event tracking)
    MouseButtonEvent,
    /// Mouse reporting (any event tracking)
    MouseAnyEvent,
    /// Mouse reporting (focus events)
    MouseFocus,
    /// Mouse reporting (SGR extended mode)
    MouseSgr,
    /// Mouse reporting (URXVT extended mode)
    MouseUrxvt,
    /// UTF-8 mouse mode
    MouseUtf8,
}

/// Terminal action that can be performed
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalAction {
    /// Print a character to the screen
    Print(char),

    /// Execute a control character
    Execute(u8),

    /// Move cursor up by n lines
    CursorUp(usize),

    /// Move cursor down by n lines
    CursorDown(usize),

    /// Move cursor forward by n columns
    CursorForward(usize),

    /// Move cursor backward by n columns
    CursorBackward(usize),

    /// Move cursor to specific line (1-indexed)
    CursorGoToLine(usize),

    /// Move cursor to specific column (1-indexed)
    CursorGoToColumn(usize),

    /// Move cursor to specific position (line, column) (1-indexed)
    CursorGoTo { line: usize, col: usize },

    /// Save cursor position
    CursorSave,

    /// Restore cursor position
    CursorRestore,

    /// Set cursor shape
    SetCursorShape(CursorShape),

    /// Clear from cursor to end of screen
    ClearToEndOfScreen,

    /// Clear from cursor to beginning of screen
    ClearToBeginningOfScreen,

    /// Clear entire screen
    ClearScreen,

    /// Clear from cursor to end of line
    ClearToEndOfLine,

    /// Clear from cursor to beginning of line
    ClearToBeginningOfLine,

    /// Clear entire line
    ClearLine,

    /// Insert n blank lines
    InsertLines(usize),

    /// Delete n lines
    DeleteLines(usize),

    /// Erase n characters
    EraseChars(usize),

    /// Delete n characters
    DeleteChars(usize),

    /// Scroll up by n lines
    ScrollUp(usize),

    /// Scroll down by n lines
    ScrollDown(usize),

    /// Set scrolling region (top, bottom) (1-indexed)
    SetScrollRegion { top: usize, bottom: usize },

    /// Reset scrolling region to full screen
    ResetScrollRegion,

    /// Set foreground color
    SetForeground(Color),

    /// Set background color
    SetBackground(Color),

    /// Reset foreground color to default
    ResetForeground,

    /// Reset background color to default
    ResetBackground,

    /// Set attribute
    SetAttribute(AttributeChange),

    /// Reset all attributes
    ResetAttributes,

    /// Set terminal mode
    SetMode(Mode),

    /// Unset terminal mode
    UnsetMode(Mode),

    /// Set window title
    SetTitle(String),

    /// Ring bell
    Bell,

    /// Carriage return
    CarriageReturn,

    /// Line feed
    LineFeed,

    /// Backspace
    Backspace,

    /// Tab
    Tab,

    /// Reverse index (move cursor up and scroll if at top)
    ReverseIndex,

    /// Reset terminal state
    Reset,

    /// Set hyperlink
    SetHyperlink { url: Option<String>, id: Option<String> },
}

/// Changes to text attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeChange {
    /// Enable bold
    Bold,
    /// Disable bold (normal intensity)
    NormalIntensity,
    /// Enable dim
    Dim,
    /// Enable italic
    Italic,
    /// Disable italic
    NoItalic,
    /// Enable underline
    Underline(UnderlineStyle),
    /// Disable underline
    NoUnderline,
    /// Enable blink
    Blink,
    /// Enable fast blink
    BlinkFast,
    /// Disable blink
    NoBlink,
    /// Enable reverse video
    Reverse,
    /// Disable reverse video
    NoReverse,
    /// Enable hidden
    Hidden,
    /// Disable hidden
    NoHidden,
    /// Enable strikethrough
    Strikethrough,
    /// Disable strikethrough
    NoStrikethrough,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_creation() {
        let color = Rgb::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_rgb_from_hex() {
        let color = Rgb::from_hex(0xFF8040);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.to_string(), "#ff8040");
    }

    #[test]
    fn test_named_color_from_ansi() {
        assert_eq!(NamedColor::from_ansi(0), Some(NamedColor::Black));
        assert_eq!(NamedColor::from_ansi(7), Some(NamedColor::White));
        assert_eq!(NamedColor::from_ansi(15), Some(NamedColor::BrightWhite));
        assert_eq!(NamedColor::from_ansi(16), None);
    }

    #[test]
    fn test_attributes_default() {
        let attrs = Attributes::default();
        assert!(!attrs.bold);
        assert!(!attrs.italic);
        assert_eq!(attrs.underline, UnderlineStyle::None);
    }

    #[test]
    fn test_attributes_reset() {
        let mut attrs = Attributes {
            bold: true,
            italic: true,
            ..Default::default()
        };
        attrs.reset();
        assert!(!attrs.bold);
        assert!(!attrs.italic);
    }
}

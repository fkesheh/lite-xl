# ANSI Parser Quick Reference

## Importing

```rust
use lite_xl::terminal::parser::{
    AnsiParser,
    TerminalAction,
    Color,
    NamedColor,
    Rgb,
    AttributeChange,
    Attributes,
    UnderlineStyle,
    CursorShape,
    Mode,
};
```

## Creating a Parser

```rust
let mut parser = AnsiParser::new();
```

## Parsing Methods

```rust
// Parse byte slice
let actions: Vec<TerminalAction> = parser.parse(b"text\x1b[31m");

// Parse single byte (streaming)
let actions: Vec<TerminalAction> = parser.parse_byte(0x1b);

// Parse UTF-8 string
let actions: Vec<TerminalAction> = parser.parse_str("hello");

// Peek at pending actions without consuming
let actions: &[TerminalAction] = parser.peek_actions();

// Clear pending actions
parser.clear_actions();

// Reset parser state
parser.reset();
```

## Terminal Actions

### Printing Text
```rust
TerminalAction::Print(char)
```

### Cursor Movement
```rust
TerminalAction::CursorUp(n)              // Move up n lines
TerminalAction::CursorDown(n)            // Move down n lines
TerminalAction::CursorForward(n)         // Move forward n columns
TerminalAction::CursorBackward(n)        // Move backward n columns
TerminalAction::CursorGoToLine(line)     // Move to line (1-indexed)
TerminalAction::CursorGoToColumn(col)    // Move to column (1-indexed)
TerminalAction::CursorGoTo { line, col } // Move to position (1-indexed)
TerminalAction::CursorSave               // Save cursor position
TerminalAction::CursorRestore            // Restore cursor position
TerminalAction::SetCursorShape(shape)    // Change cursor shape
```

### Screen/Line Clearing
```rust
TerminalAction::ClearScreen              // Clear entire screen
TerminalAction::ClearToEndOfScreen       // Clear from cursor to end
TerminalAction::ClearToBeginningOfScreen // Clear from cursor to beginning
TerminalAction::ClearLine                // Clear entire line
TerminalAction::ClearToEndOfLine         // Clear from cursor to end of line
TerminalAction::ClearToBeginningOfLine   // Clear from cursor to beginning of line
```

### Colors
```rust
// Foreground
TerminalAction::SetForeground(Color::Named(NamedColor::Red))
TerminalAction::SetForeground(Color::Indexed(196))
TerminalAction::SetForeground(Color::Rgb(Rgb::new(255, 128, 64)))
TerminalAction::ResetForeground

// Background
TerminalAction::SetBackground(Color::Named(NamedColor::Blue))
TerminalAction::SetBackground(Color::Indexed(21))
TerminalAction::SetBackground(Color::Rgb(Rgb::new(0, 0, 255)))
TerminalAction::ResetBackground
```

### Text Attributes
```rust
TerminalAction::SetAttribute(AttributeChange::Bold)
TerminalAction::SetAttribute(AttributeChange::Dim)
TerminalAction::SetAttribute(AttributeChange::Italic)
TerminalAction::SetAttribute(AttributeChange::NoItalic)
TerminalAction::SetAttribute(AttributeChange::Underline(UnderlineStyle::Single))
TerminalAction::SetAttribute(AttributeChange::NoUnderline)
TerminalAction::SetAttribute(AttributeChange::Blink)
TerminalAction::SetAttribute(AttributeChange::BlinkFast)
TerminalAction::SetAttribute(AttributeChange::NoBlink)
TerminalAction::SetAttribute(AttributeChange::Reverse)
TerminalAction::SetAttribute(AttributeChange::NoReverse)
TerminalAction::SetAttribute(AttributeChange::Hidden)
TerminalAction::SetAttribute(AttributeChange::NoHidden)
TerminalAction::SetAttribute(AttributeChange::Strikethrough)
TerminalAction::SetAttribute(AttributeChange::NoStrikethrough)
TerminalAction::SetAttribute(AttributeChange::NormalIntensity)
TerminalAction::ResetAttributes
```

### Scrolling
```rust
TerminalAction::ScrollUp(n)                         // Scroll up n lines
TerminalAction::ScrollDown(n)                       // Scroll down n lines
TerminalAction::SetScrollRegion { top, bottom }     // Set scrolling region
TerminalAction::ResetScrollRegion                   // Reset to full screen
```

### Line/Character Operations
```rust
TerminalAction::InsertLines(n)   // Insert n blank lines
TerminalAction::DeleteLines(n)   // Delete n lines
TerminalAction::EraseChars(n)    // Erase n characters
TerminalAction::DeleteChars(n)   // Delete n characters
```

### OSC Sequences
```rust
TerminalAction::SetTitle(String)                          // Set window title
TerminalAction::SetHyperlink { url: Option<String>,       // Set/clear hyperlink
                               id: Option<String> }
```

### Control Characters
```rust
TerminalAction::Execute(u8)      // Raw control character
TerminalAction::Bell             // Bell/beep
TerminalAction::Backspace        // Move cursor back
TerminalAction::Tab              // Tab
TerminalAction::LineFeed         // Line feed / newline
TerminalAction::CarriageReturn   // Carriage return
TerminalAction::ReverseIndex     // Reverse line feed
```

### Terminal Modes
```rust
TerminalAction::SetMode(Mode)    // Enable mode
TerminalAction::UnsetMode(Mode)  // Disable mode
```

### Other
```rust
TerminalAction::Reset            // Reset terminal state
```

## Common ANSI Escape Sequences

### Colors
```
ESC[30m - ESC[37m   Black, Red, Green, Yellow, Blue, Magenta, Cyan, White
ESC[90m - ESC[97m   Bright variants
ESC[40m - ESC[47m   Background colors
ESC[100m - ESC[107m Bright background colors
ESC[38;5;Nm         256-color foreground
ESC[48;5;Nm         256-color background
ESC[38;2;R;G;Bm     RGB foreground
ESC[48;2;R;G;Bm     RGB background
ESC[39m             Default foreground
ESC[49m             Default background
```

### Text Attributes
```
ESC[0m   Reset all
ESC[1m   Bold
ESC[2m   Dim
ESC[3m   Italic
ESC[4m   Underline
ESC[5m   Blink
ESC[7m   Reverse
ESC[8m   Hidden
ESC[9m   Strikethrough
ESC[22m  Normal intensity
ESC[23m  Not italic
ESC[24m  Not underlined
ESC[25m  Not blinking
ESC[27m  Not reversed
ESC[28m  Not hidden
ESC[29m  Not strikethrough
```

### Cursor Movement
```
ESC[nA   Cursor up n lines
ESC[nB   Cursor down n lines
ESC[nC   Cursor forward n columns
ESC[nD   Cursor backward n columns
ESC[nE   Cursor next line (down n, column 0)
ESC[nF   Cursor previous line (up n, column 0)
ESC[nG   Cursor to column n
ESC[nd   Cursor to line n
ESC[n;mH Cursor to position (line n, column m)
ESC[n;mf Same as H
ESC[s    Save cursor position
ESC[u    Restore cursor position
ESC7     Save cursor (alternate)
ESC8     Restore cursor (alternate)
```

### Erase Functions
```
ESC[J    Clear from cursor to end of screen
ESC[1J   Clear from cursor to beginning of screen
ESC[2J   Clear entire screen
ESC[3J   Clear entire screen including scrollback
ESC[K    Clear from cursor to end of line
ESC[1K   Clear from cursor to beginning of line
ESC[2K   Clear entire line
```

### Scrolling
```
ESC[nS   Scroll up n lines
ESC[nT   Scroll down n lines
ESC[n;mr Set scrolling region (top=n, bottom=m)
```

### OSC Sequences
```
ESC]0;textBEL    Set window title
ESC]1;textBEL    Set icon name
ESC]2;textBEL    Set window title
ESC]8;;urlBEL    Set hyperlink (url)
ESC]8;;BEL       Clear hyperlink
```

## Named Colors

```rust
NamedColor::Black          // 0
NamedColor::Red            // 1
NamedColor::Green          // 2
NamedColor::Yellow         // 3
NamedColor::Blue           // 4
NamedColor::Magenta        // 5
NamedColor::Cyan           // 6
NamedColor::White          // 7
NamedColor::BrightBlack    // 8 (gray)
NamedColor::BrightRed      // 9
NamedColor::BrightGreen    // 10
NamedColor::BrightYellow   // 11
NamedColor::BrightBlue     // 12
NamedColor::BrightMagenta  // 13
NamedColor::BrightCyan     // 14
NamedColor::BrightWhite    // 15
```

## Underline Styles

```rust
UnderlineStyle::None
UnderlineStyle::Single
UnderlineStyle::Double
UnderlineStyle::Curly
UnderlineStyle::Dotted
UnderlineStyle::Dashed
```

## Cursor Shapes

```rust
CursorShape::Block
CursorShape::Underline
CursorShape::Bar
```

## Terminal Modes

```rust
Mode::AppCursor           // Application cursor keys
Mode::AppKeypad           // Application keypad
Mode::BracketedPaste      // Bracketed paste mode
Mode::ShowCursor          // Show/hide cursor
Mode::LineWrap            // Line wrap
Mode::Origin              // Origin mode
Mode::Insert              // Insert mode
Mode::AltScreen           // Alternate screen buffer
Mode::MouseX10            // Mouse reporting X10
Mode::MouseVt200          // Mouse reporting VT200
Mode::MouseButtonEvent    // Mouse button event tracking
Mode::MouseAnyEvent       // Mouse any event tracking
Mode::MouseFocus          // Mouse focus events
Mode::MouseSgr            // Mouse SGR extended mode
Mode::MouseUrxvt          // Mouse URXVT extended mode
Mode::MouseUtf8           // UTF-8 mouse mode
```

## Example: Processing Terminal Output

```rust
use lite_xl::terminal::parser::{AnsiParser, TerminalAction, Color};

fn process_terminal_output(data: &[u8]) {
    let mut parser = AnsiParser::new();
    let actions = parser.parse(data);

    for action in actions {
        match action {
            TerminalAction::Print(c) => {
                // Print character to screen
                print_char(c);
            }
            TerminalAction::SetForeground(color) => {
                // Change text color
                set_color(color);
            }
            TerminalAction::CursorGoTo { line, col } => {
                // Move cursor (note: 1-indexed)
                move_cursor(line - 1, col - 1);
            }
            TerminalAction::ClearScreen => {
                // Clear the screen
                clear_screen();
            }
            _ => {
                // Handle other actions
            }
        }
    }
}
```

## Convenience Functions

```rust
// Parse without creating a parser instance
use lite_xl::terminal::parser::{parse, parse_str};

let actions = parse(b"\x1b[31mRed text\x1b[0m");
let actions = parse_str("Normal text");
```

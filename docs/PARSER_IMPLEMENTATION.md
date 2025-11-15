# ANSI Parser Implementation

## Overview

This document describes the ANSI parser module implementation for the Lite XL text editor's terminal emulation support.

## Architecture

The ANSI parser module is built on the `vte` crate and provides a comprehensive implementation for parsing ANSI/VT escape sequences. The module consists of four main components:

### Components

1. **`src/terminal/parser/sequences.rs`** - Terminal sequence definitions
   - Defines all terminal actions (`TerminalAction` enum)
   - Color types (`Color`, `NamedColor`, `Rgb`)
   - Text attributes (`Attributes`, `AttributeChange`, `UnderlineStyle`)
   - Cursor shapes and terminal modes
   - Type-safe representations of all ANSI sequences

2. **`src/terminal/parser/performer.rs`** - vte::Perform implementation
   - Implements the `vte::Perform` trait
   - Parses all ANSI/VT escape sequences
   - Converts low-level byte sequences to high-level `TerminalAction`s
   - Handles CSI, OSC, ESC sequences
   - SGR (Select Graphic Rendition) parsing for colors and attributes

3. **`src/terminal/parser/ansi.rs`** - High-level parser interface
   - `AnsiParser` - Main parser struct
   - Wraps the vte state machine
   - Provides convenient methods: `parse()`, `parse_byte()`, `parse_str()`
   - Returns `Vec<TerminalAction>` for easy processing

4. **`src/terminal/parser/mod.rs`** - Module root
   - Re-exports public API
   - Comprehensive documentation with examples
   - Backward compatibility type alias (`Action = TerminalAction`)

## Features

### Supported ANSI Sequences

#### CSI (Control Sequence Introducer) Sequences
- Cursor movement (up, down, forward, backward)
- Cursor positioning (absolute and relative)
- Scrolling regions
- Line/character insertion and deletion
- Screen clearing (full, to end, to beginning)
- Line clearing (full, to end, to beginning)
- Save/restore cursor position

#### SGR (Select Graphic Rendition)
- Named colors (0-15): Black, Red, Green, Yellow, Blue, Magenta, Cyan, White + bright variants
- 256-color palette (0-255)
- 24-bit RGB truecolor
- Text attributes:
  - Bold/increased intensity
  - Dim/decreased intensity
  - Italic
  - Underline (single, double, curly, dotted, dashed)
  - Blink (slow and fast)
  - Reverse video
  - Hidden/concealed
  - Strikethrough

#### OSC (Operating System Command) Sequences
- Window title (OSC 0, 1, 2)
- Hyperlinks (OSC 8) with optional IDs
- Color palette manipulation (OSC 4, 10, 11, 12)

#### Control Characters
- Bell (BEL)
- Backspace (BS)
- Tab (HT)
- Line feed (LF)
- Carriage return (CR)
- And others

#### Terminal Modes
- Application cursor keys
- Application keypad
- Bracketed paste
- Show/hide cursor
- Line wrap
- Insert mode
- Alternate screen buffer
- Mouse reporting (X10, VT200, button events, any events, focus, SGR, URXVT, UTF-8)

## Usage

### Basic Parsing

```rust
use lite_xl::terminal::parser::{AnsiParser, TerminalAction};

let mut parser = AnsiParser::new();

// Parse ANSI sequences
let actions = parser.parse(b"Hello \x1b[31mWorld\x1b[0m!");

// Process actions
for action in actions {
    match action {
        TerminalAction::Print(c) => print!("{}", c),
        TerminalAction::SetForeground(color) => {
            println!("Set color: {:?}", color);
        }
        TerminalAction::ResetAttributes => {
            println!("Reset attributes");
        }
        _ => {}
    }
}
```

### Streaming Input

```rust
let mut parser = AnsiParser::new();

// Process byte by byte
for &byte in input {
    let actions = parser.parse_byte(byte);
    for action in actions {
        handle_action(action);
    }
}
```

### Color Handling

```rust
use lite_xl::terminal::parser::{Color, NamedColor, Rgb};

// Named color
parser.parse(b"\x1b[31m"); // Red

// 256-color
parser.parse(b"\x1b[38;5;196m"); // Bright red

// RGB color
parser.parse(b"\x1b[38;2;255;128;64m"); // Custom RGB
```

### Text Attributes

```rust
// Bold, italic, underline
parser.parse(b"\x1b[1;3;4m");

// Reset all attributes
parser.parse(b"\x1b[0m");
```

### OSC Sequences

```rust
// Set window title
parser.parse(b"\x1b]0;My Terminal\x07");

// Create hyperlink
parser.parse(b"\x1b]8;;https://example.com\x07Link Text\x1b]8;;\x07");
```

## Implementation Details

### vte Integration

The parser uses the `vte` crate (version 0.13) which provides a robust state machine for parsing VT/ANSI escape sequences. The integration:

1. `AnsiParser` wraps a `vte::Parser` instance
2. `AnsiPerformer` implements `vte::Perform` trait
3. Parser feeds bytes to vte state machine
4. Performer converts vte callbacks to `TerminalAction`s
5. Actions are queued and returned to caller

### Color Parsing

The parser supports three color modes:

1. **Named colors** (0-15): Directly mapped from ANSI codes 30-37, 40-47, 90-97, 100-107
2. **256-color palette**: CSI 38;5;N (foreground) or CSI 48;5;N (background)
3. **RGB truecolor**: CSI 38;2;R;G;B (foreground) or CSI 48;2;R;G;B (background)

### Error Handling

The parser is resilient to malformed sequences:
- Unknown sequences are ignored
- Invalid parameters are skipped
- Parser state is maintained even with bad input

## Testing

### Unit Tests

The implementation includes comprehensive unit tests in each module:

- `sequences.rs`: Tests for color types, RGB conversion, attributes
- `performer.rs`: Tests for SGR parsing, control characters, color parsing
- `ansi.rs`: Integration tests for complete parsing scenarios
- `mod.rs`: Tests for real-world terminal output

### Test Coverage

Tests cover:
- Plain text parsing
- All color modes (named, 256-color, RGB)
- Cursor movement sequences
- Screen/line clearing
- Text attributes
- OSC sequences
- Control characters
- Complex multi-sequence input
- Real-world terminal output

## Performance

The parser is designed for efficiency:
- Zero-copy parsing where possible
- Minimal allocations (only for strings in OSC sequences)
- Stateful parsing allows streaming input
- vte state machine is highly optimized

## Future Enhancements

Potential improvements:
1. DCS (Device Control String) sequence handling
2. Additional OSC sequences (color queries, etc.)
3. Performance profiling and optimization
4. Custom escape sequence extensions
5. Parser state serialization/deserialization

## Dependencies

- `vte = "0.13"` - VT/ANSI escape sequence parser

## Files Created

1. `/home/user/lite-xl/src/terminal/parser/mod.rs` (159 lines)
2. `/home/user/lite-xl/src/terminal/parser/sequences.rs` (459 lines)
3. `/home/user/lite-xl/src/terminal/parser/performer.rs` (630 lines)
4. `/home/user/lite-xl/src/terminal/parser/ansi.rs` (315 lines)
5. `/home/user/lite-xl/examples/parser_demo.rs` (157 lines)
6. Updated `/home/user/lite-xl/Cargo.toml` to add vte dependency

**Total**: ~1,720 lines of well-documented, tested code

## Integration

The parser module is already integrated with the terminal module:
- `src/terminal/mod.rs` exports parser types
- `src/lib.rs` includes terminal module
- Manager and state modules use the parser (backward compatible via type alias)

## Conclusion

The ANSI parser module provides comprehensive, robust, and efficient parsing of terminal escape sequences. It supports all major ANSI/VT features including colors, text formatting, cursor control, and OSC sequences, making it suitable for a full-featured terminal emulator within Lite XL.

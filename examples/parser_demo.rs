//! Demo of the ANSI parser module
//!
//! This example demonstrates the ANSI parser functionality without relying
//! on the terminal infrastructure.

use lite_xl::terminal::parser::{AnsiParser, Color, NamedColor, TerminalAction};

fn main() {
    let mut parser = AnsiParser::new();

    println!("=== ANSI Parser Demo ===\n");

    // Test 1: Plain text
    println!("Test 1: Plain text");
    let actions = parser.parse(b"Hello, World!");
    println!("  Input: 'Hello, World!'");
    println!("  Actions: {} print actions", actions.len());
    println!();

    // Test 2: Simple color
    println!("Test 2: Red color");
    let actions = parser.parse(b"\x1b[31m");
    println!("  Input: ESC[31m");
    for action in &actions {
        println!("  Action: {:?}", action);
    }
    println!();

    // Test 3: Text with color
    println!("Test 3: Text with color");
    let actions = parser.parse(b"Normal \x1b[31mRed\x1b[0m Normal");
    println!("  Input: 'Normal ESC[31mRedESC[0m Normal'");
    println!("  Total actions: {}", actions.len());
    for action in &actions {
        match action {
            TerminalAction::SetForeground(color) => {
                println!("  - Set foreground: {:?}", color);
            }
            TerminalAction::ResetAttributes => {
                println!("  - Reset attributes");
            }
            TerminalAction::Print(c) => {
                print!("{}", c);
            }
            _ => {}
        }
    }
    println!("\n");

    // Test 4: 256-color mode
    println!("Test 4: 256-color mode");
    let actions = parser.parse(b"\x1b[38;5;196m");
    println!("  Input: ESC[38;5;196m (256-color red)");
    for action in &actions {
        println!("  Action: {:?}", action);
    }
    println!();

    // Test 5: RGB color
    println!("Test 5: RGB color");
    let actions = parser.parse(b"\x1b[38;2;255;128;64m");
    println!("  Input: ESC[38;2;255;128;64m (RGB color)");
    for action in &actions {
        if let TerminalAction::SetForeground(Color::Rgb(rgb)) = action {
            println!("  RGB: r={}, g={}, b={}", rgb.r, rgb.g, rgb.b);
        }
    }
    println!();

    // Test 6: Cursor movement
    println!("Test 6: Cursor movement");
    let actions = parser.parse(b"\x1b[10;20H");
    println!("  Input: ESC[10;20H (move to row 10, col 20)");
    for action in &actions {
        println!("  Action: {:?}", action);
    }
    println!();

    // Test 7: Clear screen
    println!("Test 7: Clear screen");
    let actions = parser.parse(b"\x1b[2J");
    println!("  Input: ESC[2J (clear screen)");
    for action in &actions {
        println!("  Action: {:?}", action);
    }
    println!();

    // Test 8: Bold and italic
    println!("Test 8: Text attributes");
    let actions = parser.parse(b"\x1b[1;3;4m");
    println!("  Input: ESC[1;3;4m (bold, italic, underline)");
    println!("  Actions: {} attribute changes", actions.len());
    for action in &actions {
        println!("  - {:?}", action);
    }
    println!();

    // Test 9: Window title
    println!("Test 9: OSC sequence - window title");
    let actions = parser.parse(b"\x1b]0;My Terminal Window\x07");
    println!("  Input: ESC]0;My Terminal WindowBEL");
    for action in &actions {
        if let TerminalAction::SetTitle(title) = action {
            println!("  Title: '{}'", title);
        }
    }
    println!();

    // Test 10: Hyperlink
    println!("Test 10: OSC sequence - hyperlink");
    let actions = parser.parse(b"\x1b]8;;https://example.com\x07link text\x1b]8;;\x07");
    println!("  Input: ESC]8;;https://example.comBELlink textESC]8;;BEL");
    for action in &actions {
        match action {
            TerminalAction::SetHyperlink { url, id } => {
                println!("  Hyperlink: url={:?}, id={:?}", url, id);
            }
            TerminalAction::Print(c) => print!("{}", c),
            _ => {}
        }
    }
    println!("\n");

    // Test 11: Control characters
    println!("Test 11: Control characters");
    let actions = parser.parse(b"Line 1\r\nLine 2\t\tTabbed");
    println!("  Input: 'Line 1\\r\\nLine 2\\t\\tTabbed'");
    println!("  Total actions: {}", actions.len());
    let has_cr = actions.iter().any(|a| matches!(a, TerminalAction::CarriageReturn));
    let has_lf = actions.iter().any(|a| matches!(a, TerminalAction::LineFeed));
    let has_tab = actions.iter().any(|a| matches!(a, TerminalAction::Tab));
    println!("  Has CarriageReturn: {}", has_cr);
    println!("  Has LineFeed: {}", has_lf);
    println!("  Has Tab: {}", has_tab);
    println!();

    // Test 12: Complex real-world example
    println!("Test 12: Real-world terminal output");
    let complex = b"\x1b[1;1H\x1b[2J\x1b[1;32m$ \x1b[0mls -la\r\n\
                     total 48\r\n\
                     drwxr-xr-x  12 user  staff   384 Nov 15 10:00 \x1b[1;34m.\x1b[0m\r\n\
                     -rw-r--r--   1 user  staff  1234 Nov 15 09:45 \x1b[0mREADME.md\x1b[0m\r\n";

    let actions = parser.parse(complex);
    println!("  Simulated 'ls -la' output");
    println!("  Total actions: {}", actions.len());

    let num_colors = actions.iter().filter(|a| matches!(a, TerminalAction::SetForeground(_))).count();
    let num_resets = actions.iter().filter(|a| matches!(a, TerminalAction::ResetAttributes)).count();
    let num_cursor = actions.iter().filter(|a| matches!(a, TerminalAction::CursorGoTo { .. })).count();
    let num_clear = actions.iter().filter(|a| matches!(a, TerminalAction::ClearScreen)).count();

    println!("  Color changes: {}", num_colors);
    println!("  Attribute resets: {}", num_resets);
    println!("  Cursor movements: {}", num_cursor);
    println!("  Screen clears: {}", num_clear);

    println!("\n=== All tests completed successfully! ===");
}

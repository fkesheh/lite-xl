/// Lite XL - A lightweight, fast text editor built with Rust and Floem
///
/// This is the main entry point for the application.

mod editor;
mod ui;

use floem::{
    Application,
    reactive::RwSignal,
    window::WindowConfig,
};

use editor::EditorState;
use ui::{app_view, FontConfig, Theme};

fn main() {
    // Create application state
    let initial_text = r#"// Welcome to Lite XL!
// A fast, lightweight text editor built with Rust and Floem
//
// Features:
// - Syntax-aware text editing
// - Line numbers
// - Selection support
// - Keyboard navigation
// - 60 FPS rendering
//
// Keyboard shortcuts:
// - Arrow keys: Navigate
// - Shift + Arrow keys: Select
// - Ctrl+A: Select all
// - Home/End: Line start/end
// - Backspace/Delete: Delete text
// - Enter: New line
// - Tab: Insert spaces
//
// Try editing this text!

fn main() {
    println!("Hello, Lite XL!");
}

struct Editor {
    lines: Vec<String>,
    cursor: Position,
}

impl Editor {
    fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: Position::zero(),
        }
    }

    fn insert_char(&mut self, c: char) {
        // Insert character at cursor position
        let line = &mut self.lines[self.cursor.line];
        line.insert(self.cursor.col, c);
        self.cursor.col += 1;
    }
}
"#;

    let editor_state = RwSignal::new(EditorState::with_text(initial_text));
    let theme = RwSignal::new(Theme::dark());
    let font_config = RwSignal::new(FontConfig::default());

    // Create window configuration
    let window_config = WindowConfig::default()
        .title("Lite XL - Text Editor")
        .size((1200.0, 800.0));

    // Build and run application
    Application::new()
        .window(
            move |_| app_view(editor_state, theme, font_config),
            Some(window_config),
        )
        .run();
}

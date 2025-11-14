# Lite XL - Floem UI Implementation

This document describes the UI layer implementation using the Floem reactive framework.

## Overview

A complete text editor UI built with Floem featuring:
- Reactive state management with RwSignal
- 60 FPS rendering capability
- Full keyboard input handling
- Line number gutter
- Status bar with cursor position
- Text selection support
- Dark theme by default
- Monospace font rendering

## Architecture

### Module Structure

```
src/
├── main.rs                 # Application entry point
├── editor/
│   └── mod.rs             # Editor state management
└── ui/
    ├── mod.rs             # UI module exports
    ├── theme.rs           # Theme system (dark/light/solarized)
    ├── gutter.rs          # Line number gutter component
    ├── statusbar.rs       # Status bar component
    └── editor_view.rs     # Main editor view with text rendering
```

## Components

### 1. Editor State (`src/editor/mod.rs`)

**Purpose**: Manages the core text buffer and editing state

**Key Features**:
- Line-based text storage
- Cursor position tracking
- Text selection support
- Editing operations (insert, delete, backspace)
- Cursor movement (arrows, home/end)
- Multi-line editing
- Modified flag tracking

**API**:
```rust
EditorState::new()                    // Create empty editor
EditorState::with_text(text)          // Create with initial content
state.insert_char(c)                  // Insert character
state.insert_string(s)                // Insert string
state.delete_backward()               // Backspace
state.delete_forward()                // Delete
state.move_up/down/left/right(shift)  // Cursor movement
state.select_all()                    // Select all text
state.get_selected_text()             // Get selection
```

### 2. Theme System (`src/ui/theme.rs`)

**Purpose**: Defines colors and styling for the editor

**Themes Available**:
- **Dark** (default): Dark background with light text
- **Light**: Light background with dark text
- **Solarized Dark**: Popular solarized color scheme

**Color Palette**:
```rust
Theme {
    background: Color,           // Editor background
    foreground: Color,           // Text color
    current_line: Color,         // Current line highlight
    selection: Color,            // Selection background
    cursor: Color,               // Cursor color
    line_number: Color,          // Line number color
    line_number_bg: Color,       // Gutter background
    line_number_active: Color,   // Active line number
    status_bar_bg: Color,        // Status bar background
    status_bar_fg: Color,        // Status bar text
    border: Color,               // Border color
    // Syntax highlighting colors (future use)
    comment: Color,
    keyword: Color,
    string: Color,
    number: Color,
    function: Color,
}
```

**Font Configuration**:
```rust
FontConfig {
    family: String,         // Font family (monospace)
    size: f32,              // Font size in points (14.0)
    line_height: f32,       // Line height multiplier (1.5)
    char_width: f32,        // Character width (8.0)
}
```

### 3. Gutter View (`src/ui/gutter.rs`)

**Purpose**: Displays line numbers on the left side

**Features**:
- Dynamic line number rendering
- Highlights current line number
- Adjusts width based on line count
- Synchronized scrolling with editor

**Rendering**:
- Fixed width: 60px
- Right-aligned line numbers
- Monospace font
- Updates reactively when editor changes

### 4. Status Bar (`src/ui/statusbar.rs`)

**Purpose**: Shows editor information at the bottom

**Information Displayed**:
- **Left side**: File path and modification status
  - `filename [+]` if modified
  - `untitled` if no file

- **Right side**: Cursor and document info
  - `Ln X, Col Y` - Cursor position (1-indexed)
  - `N lines` - Total line count
  - `N selected` - Character count when selecting
  - `UTF-8` - Encoding (when no selection)

**Layout**:
- Height: 28px
- Border on top
- Space-between justify content
- Monospace font at 12pt

### 5. Editor View (`src/ui/editor_view.rs`)

**Purpose**: Main text editing area with rendering and input handling

**Rendering Features**:
- **Line-by-line rendering**: Each line is a separate view
- **Current line highlight**: Subtle background highlight
- **Selection rendering**: Highlighted background for selected text
- **Cursor rendering**: Block cursor with inverted colors
- **Character-by-character layout**: Monospace character grid

**Input Handling**:

Keyboard shortcuts:
```
Character Input:
  - Any character       → Insert at cursor
  - Ctrl+A              → Select all
  - Ctrl+C              → Copy (placeholder)
  - Ctrl+X              → Cut (placeholder)
  - Ctrl+V              → Paste (placeholder)

Navigation:
  - Arrow keys          → Move cursor
  - Shift + Arrows      → Extend selection
  - Home                → Line start
  - End                 → Line end

Editing:
  - Backspace           → Delete before cursor
  - Delete              → Delete at cursor
  - Enter               → Insert newline
  - Tab                 → Insert 4 spaces
  - Escape              → Clear selection
```

**Scrolling**:
- Vertical scroll support
- Synchronized with gutter
- Smooth scrolling

### 6. Application View (`src/ui/mod.rs`)

**Purpose**: Combines all components into complete UI

**Layout**:
```
┌─────────────────────────────────────┐
│        Editor View                  │
│  ┌────┬─────────────────────────┐   │
│  │ G  │  Text Area              │   │
│  │ u  │  (scrollable)           │   │
│  │ t  │                         │   │
│  │ t  │                         │   │
│  │ e  │                         │   │
│  │ r  │                         │   │
│  └────┴─────────────────────────┘   │
├─────────────────────────────────────┤
│        Status Bar                   │
│  filename [+]    Ln X, Col Y | ...  │
└─────────────────────────────────────┘
```

**Reactive Updates**:
- All components use RwSignal for reactivity
- Changes to editor state automatically update UI
- 60 FPS rendering capability

## Main Application (`src/main.rs`)

**Purpose**: Entry point that initializes Floem and runs the app

**Initialization**:
```rust
fn main() {
    // Create reactive state
    let editor_state = RwSignal::new(EditorState::with_text(initial_text));
    let theme = RwSignal::new(Theme::dark());
    let font_config = RwSignal::new(FontConfig::default());

    // Configure window
    let window_config = WindowConfig::default()
        .title("Lite XL - Text Editor")
        .size((1200.0, 800.0));

    // Run application
    Application::new()
        .window(move |_| app_view(editor_state, theme, font_config), Some(window_config))
        .run();
}
```

**Initial Content**:
- Welcome message with instructions
- Example Rust code
- Keyboard shortcut reference

## Dependencies

```toml
[dependencies]
floem = "0.2"  # UI framework with reactive primitives
```

## Performance Characteristics

### Rendering
- **Target**: 60 FPS (16.67ms per frame)
- **Strategy**: Reactive updates only re-render changed components
- **Optimizations**:
  - Character-level view caching by Floem
  - Only visible lines rendered (via scroll container)
  - Minimal allocations in hot paths

### Memory
- **Text Storage**: String-based line storage
- **View Tree**: Floem manages view hierarchy
- **State**: Minimal state duplication via RwSignal

### Responsiveness
- **Keystroke Latency**: < 5ms (target)
- **UI Updates**: Immediate via reactive signals
- **Scrolling**: Smooth via native Floem scroll

## Usage

### Building
```bash
cargo build --release --bin lite-xl
```

### Running
```bash
cargo run --release --bin lite-xl
```

### Features Demonstrated

1. **Text Editing**:
   - Type any character to insert
   - Use arrows to navigate
   - Backspace/Delete to remove text
   - Enter for new lines

2. **Selection**:
   - Shift + arrows to select
   - Ctrl+A to select all
   - Selection highlighted in blue

3. **Visual Feedback**:
   - Current line highlighted
   - Line numbers update
   - Status bar shows cursor position
   - Modified indicator appears

## Future Enhancements

### Planned Features
1. **Clipboard Integration**: Real copy/paste support
2. **Syntax Highlighting**: Integrate syntect for language support
3. **Multi-cursor**: Support for multiple cursors
4. **Undo/Redo**: Full undo stack implementation
5. **File I/O**: Open and save files
6. **Find/Replace**: Search functionality
7. **Configuration**: User settings and key bindings
8. **Command Palette**: Quick command access

### Performance Improvements
1. **Virtual Scrolling**: Only render visible lines
2. **Line Caching**: Cache rendered line views
3. **Syntax Cache**: Background syntax highlighting
4. **Rope Data Structure**: For large file support

## Code Quality

### Strengths
- **Separation of Concerns**: Clear module boundaries
- **Reactive Architecture**: Leverages Floem's signals
- **Type Safety**: Full Rust type system
- **Documentation**: Comprehensive inline docs

### Areas for Improvement
- **Error Handling**: Currently uses unwrap in places
- **Testing**: Needs unit tests for editor state
- **Accessibility**: Keyboard navigation only
- **Input Methods**: No IME support yet

## Contributing

When working on the UI:

1. **Editor State**: Keep editor logic separate from UI
2. **Reactivity**: Use RwSignal for state that affects UI
3. **Performance**: Profile rendering in release mode
4. **Theming**: Use theme colors, not hardcoded values
5. **Documentation**: Update this file for major changes

## License

MIT License (same as parent project)

## References

- [Floem Documentation](https://github.com/lapce/floem)
- [Floem Examples](https://github.com/lapce/floem/tree/main/examples)
- [Peniko Color API](https://docs.rs/peniko/)

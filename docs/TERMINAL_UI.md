# Terminal UI Components

This document describes the Floem-based terminal UI components for Lite XL.

## Overview

The terminal UI system consists of three main components:

1. **Terminal Canvas** (`terminal_canvas.rs`) - Core rendering and input handling
2. **Terminal Tabs** (`terminal_tabs.rs`) - Tab management for multiple terminals
3. **Terminal Panel** (`terminal_panel.rs`) - Dockable panel container

## Architecture

```
┌─────────────────────────────────────────────────────┐
│ Terminal Panel (Dockable Container)                 │
│ ┌─────────────────────────────────────────────────┐ │
│ │ Tab Bar                                         │ │
│ │ [Terminal 1] [Terminal 2] [+]           [⊟] [×]│ │
│ ├─────────────────────────────────────────────────┤ │
│ │ Terminal Canvas (Active Tab)                    │ │
│ │                                                 │ │
│ │ $ ls -la                                        │ │
│ │ total 64                                        │ │
│ │ drwxr-xr-x  5 user  staff  160                 │ │
│ │ -rw-r--r--  1 user  staff 1234 README.md       │ │
│ │ █                                               │ │
│ │                                                 │ │
│ └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

## Components

### Terminal Canvas

**File:** `src/ui/terminal_canvas.rs`

#### Data Structures

##### `TerminalCell`
Represents a single character cell in the terminal grid.

```rust
pub struct TerminalCell {
    pub ch: char,           // Character to display
    pub fg: Color,          // Foreground color
    pub bg: Color,          // Background color
    pub bold: bool,         // Bold attribute
    pub italic: bool,       // Italic attribute
    pub underline: bool,    // Underline attribute
    pub reverse: bool,      // Reverse video
}
```

**Methods:**
- `new(ch: char) -> Self` - Create cell with default styling
- `default() -> Self` - Create empty cell with space character
- `with_theme(theme: &Theme) -> Self` - Apply theme colors

##### `TerminalCursor`
Cursor position and rendering style.

```rust
pub struct TerminalCursor {
    pub row: usize,
    pub col: usize,
    pub visible: bool,
    pub style: TerminalCursorStyle,
}
```

**Cursor Styles:**
- `Block` - Full cell block cursor
- `Underline` - Bottom underline cursor
- `Bar` - Left edge vertical bar cursor

##### `TerminalGrid`
The terminal grid state containing all cells and cursor.

```rust
pub struct TerminalGrid {
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Vec<TerminalCell>>,
    pub cursor: TerminalCursor,
    pub selection: Option<(usize, usize, usize, usize)>,
    pub scroll_offset: usize,
}
```

**Methods:**
- `new(cols, rows) -> Self` - Create new grid
- `get(row, col) -> Option<&TerminalCell>` - Get cell reference
- `get_mut(row, col) -> Option<&mut TerminalCell>` - Get mutable cell
- `set(row, col, cell)` - Set cell at position
- `resize(cols, rows)` - Resize grid dimensions
- `is_selected(row, col) -> bool` - Check if position is selected

#### Views

##### `terminal_canvas_view`
Main view component for rendering terminal cells.

```rust
pub fn terminal_canvas_view(
    grid: RwSignal<TerminalGrid>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
    on_input: impl Fn(Vec<u8>) + 'static + Clone,
) -> impl View
```

**Features:**
- Cell-based rendering with font metrics
- Selection highlighting
- Cursor rendering (block, underline, bar)
- Mouse input (selection, dragging)
- Keyboard input conversion to terminal sequences
- Scrolling support

#### Input Handling

##### `key_event_to_terminal_sequence`
Converts Floem keyboard events to terminal input sequences.

```rust
pub fn key_event_to_terminal_sequence(event: &KeyEvent) -> Option<Vec<u8>>
```

**Supported Keys:**
- Character input (including Ctrl combinations)
- Enter → `\r`
- Tab → `\t`
- Backspace → `\x7f`
- Escape → `\x1b`
- Arrow keys → ANSI escape sequences
- Function keys → ANSI escape sequences

#### Utilities

##### `get_selected_text`
Extract text from current selection.

```rust
pub fn get_selected_text(grid: &TerminalGrid) -> Option<String>
```

### Terminal Tabs

**File:** `src/ui/terminal_tabs.rs`

#### Data Structures

##### `TerminalTab`
Information about a single terminal tab.

```rust
pub struct TerminalTab {
    pub id: usize,
    pub title: String,
    pub grid: TerminalGrid,
    pub active: bool,
    pub working_dir: String,
    pub is_busy: bool,
}
```

##### `TabManager`
Manages multiple terminal tabs.

```rust
pub struct TabManager {
    pub tabs: Vec<TerminalTab>,
    pub active_index: usize,
    pub next_id: usize,
}
```

**Methods:**
- `new() -> Self` - Create with one initial tab
- `add_tab(title: Option<String>)` - Add new tab
- `close_tab(index: usize)` - Close tab (keeps at least one)
- `switch_to_tab(index: usize)` - Switch active tab
- `active_tab() -> Option<&TerminalTab>` - Get active tab
- `next_tab()` - Switch to next tab (wraps around)
- `prev_tab()` - Switch to previous tab (wraps around)

#### Views

##### `terminal_tab_bar_view`
Tab bar UI component.

```rust
pub fn terminal_tab_bar_view(
    tab_manager: RwSignal<TabManager>,
    theme: RwSignal<Theme>,
) -> impl View
```

**Features:**
- Dynamic tab rendering
- Click to switch tabs
- Close button on each tab
- Add new tab button
- Busy indicator
- Hover effects

##### `tab_view`
Individual tab button.

```rust
pub fn tab_view(
    index: usize,
    tab: TerminalTab,
    is_active: bool,
    theme: RwSignal<Theme>,
    on_select: impl Fn() + 'static,
    on_close: impl Fn() + 'static,
) -> impl View
```

### Terminal Panel

**File:** `src/ui/terminal_panel.rs`

#### Data Structures

##### `DockPosition`
Panel docking position.

```rust
pub enum DockPosition {
    Bottom,  // Docked at bottom
    Left,    // Docked on left
    Right,   // Docked on right
}
```

##### `TerminalPanelState`
Complete panel state.

```rust
pub struct TerminalPanelState {
    pub visible: bool,
    pub position: DockPosition,
    pub size: f64,
    pub min_size: f64,
    pub max_size_fraction: f64,
    pub tab_manager: TabManager,
    pub is_resizing: bool,
}
```

**Methods:**
- `new() -> Self` - Create new panel state
- `toggle()` - Toggle visibility
- `show()` - Show panel
- `hide()` - Hide panel
- `set_position(position: DockPosition)` - Set dock position
- `resize(delta: f64, window_size: f64)` - Resize panel

#### Views

##### `terminal_panel_view`
Main panel view with docking and resizing.

```rust
pub fn terminal_panel_view(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View
```

**Features:**
- Dockable positioning (bottom, left, right)
- Drag-to-resize with handle
- Size constraints (min/max)
- Collapsible/expandable
- Integrated tab bar and terminal canvas
- Control buttons (position, close)

##### `create_terminal_panel`
Helper to create panel with state.

```rust
pub fn create_terminal_panel(
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> (RwSignal<TerminalPanelState>, impl View)
```

#### Keyboard Shortcuts

##### `handle_terminal_shortcuts`
Process keyboard shortcuts for terminal panel.

```rust
pub fn handle_terminal_shortcuts(
    panel_state: RwSignal<TerminalPanelState>,
    key: &str,
    ctrl: bool,
) -> bool
```

**Default Shortcuts:**
- `Ctrl+\`` - Toggle terminal panel
- `Ctrl+Shift+T` - Create new terminal tab
- `Ctrl+Shift+W` - Close current tab
- `Ctrl+Tab` - Switch to next tab

## Usage Example

### Basic Integration

```rust
use floem::{Application, reactive::RwSignal, window::WindowConfig};
use lite_xl::ui::{create_terminal_panel, Theme, FontConfig};

fn main() {
    let theme = RwSignal::new(Theme::dark());
    let font_config = RwSignal::new(FontConfig::default());

    // Create terminal panel
    let (panel_state, terminal_panel) = create_terminal_panel(theme, font_config);

    // Show panel
    panel_state.update(|state| {
        state.show();
    });

    // Create application view
    let app_view = move || {
        v_stack((
            // Your main content
            editor_view(...),

            // Terminal panel
            terminal_panel,
        ))
    };

    Application::new()
        .window(app_view, Some(WindowConfig::default()))
        .run();
}
```

### Custom Terminal Grid

```rust
use lite_xl::ui::{TerminalGrid, TerminalCell, terminal_canvas_view};

// Create grid
let mut grid = TerminalGrid::new(80, 24);

// Write text
for (col, ch) in "Hello, World!".chars().enumerate() {
    let mut cell = TerminalCell::new(ch);
    cell.fg = Color::rgb8(100, 200, 100);
    cell.bold = true;
    grid.set(0, col, cell);
}

// Create view
let grid_signal = RwSignal::new(grid);
let on_input = |bytes: Vec<u8>| {
    println!("Input: {:?}", bytes);
};

let canvas = terminal_canvas_view(
    grid_signal,
    theme,
    font_config,
    on_input,
);
```

### Tab Management

```rust
use lite_xl::ui::TabManager;

let mut manager = TabManager::new();

// Add tabs
manager.add_tab(Some("Build".to_string()));
manager.add_tab(Some("Test".to_string()));

// Switch tabs
manager.next_tab();

// Get active tab
if let Some(tab) = manager.active_tab() {
    println!("Active: {}", tab.title);
}

// Close tab
manager.close_tab(1);
```

## Theme Integration

The terminal UI components integrate seamlessly with the Lite XL theme system:

```rust
// Theme colors used:
- background: Terminal background
- foreground: Default text color
- cursor: Cursor color
- selection: Selection background
- status_bar_bg: Tab bar background
- status_bar_fg: Tab text color
- border: Panel borders and separators
- line_number: Inactive tab color
```

## Font Configuration

Terminal rendering uses the `FontConfig` for proper metrics:

```rust
pub struct FontConfig {
    pub family: String,      // Font family (use monospace)
    pub size: f32,          // Font size in points
    pub line_height: f32,   // Line height multiplier
    pub char_width: f32,    // Character width in pixels
}
```

**Important:** For terminal rendering, ensure:
- Use a monospace font family
- `char_width` matches actual rendered width
- `line_height` provides adequate spacing

## Advanced Features

### Selection and Copy

```rust
use lite_xl::ui::get_selected_text;

let grid = grid_signal.get();
if let Some(text) = get_selected_text(&grid) {
    // Copy to clipboard
    clipboard.set_text(text);
}
```

### Custom Cursor Styles

```rust
grid.update(|g| {
    g.cursor.style = TerminalCursorStyle::Bar;
    g.cursor.visible = true;
});
```

### Dynamic Resizing

```rust
// Resize terminal grid
grid.update(|g| {
    g.resize(new_cols, new_rows);
});

// Resize panel
panel_state.update(|state| {
    state.resize(delta, window_height);
});
```

### Position Cycling

```rust
// Cycle through dock positions
panel_state.update(|state| {
    state.position = match state.position {
        DockPosition::Bottom => DockPosition::Right,
        DockPosition::Right => DockPosition::Left,
        DockPosition::Left => DockPosition::Bottom,
    };
});
```

## Performance Considerations

1. **Cell Rendering**: Uses Floem's reactive system for efficient updates
2. **Font Metrics**: Calculate once and cache in `FontConfig`
3. **Selection**: Only renders selection overlay when active
4. **Scrollback**: Limit scrollback buffer size for memory efficiency
5. **Reactive Updates**: Use `RwSignal::update` for batch changes

## Future Enhancements

Potential improvements for the terminal UI:

- [ ] Scrollback buffer integration
- [ ] ANSI escape sequence parsing and rendering
- [ ] PTY (pseudo-terminal) backend integration
- [ ] Link detection and click handling
- [ ] Search in terminal output
- [ ] Copy/paste integration
- [ ] Terminal title from escape sequences
- [ ] Split terminal views
- [ ] Customizable color schemes
- [ ] Sixel graphics support
- [ ] Unicode and emoji rendering improvements

## Testing

Run the demo example:

```bash
cargo run --example terminal_ui_demo
```

Run component tests:

```bash
cargo test --lib terminal_canvas
cargo test --lib terminal_tabs
cargo test --lib terminal_panel
```

## See Also

- [Terminal Core Demo](../examples/terminal_core_demo.rs) - Core data structures
- [Theme System](../src/ui/theme.rs) - Theme configuration
- [Floem Documentation](https://docs.rs/floem/) - UI framework

## License

MIT License - Same as Lite XL project

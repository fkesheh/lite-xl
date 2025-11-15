# Terminal Plugin Integration Guide

This guide shows how to integrate the terminal plugin with the existing Lite XL editor.

## Integration Points

### 1. Main Application Layout

**File:** `/home/user/lite-xl/src/main.rs`

```rust
use floem::{
    Application,
    reactive::RwSignal,
    views::{v_stack, h_stack, container, Decorators},
};
use lite_xl::{
    editor::EditorState,
    ui::{Theme, FontConfig, app_view},
    plugins::terminal::{
        init_terminal_plugin,
        terminal_panel_view,
        TerminalConfig,
        TerminalPanelState,
    },
};

fn main() {
    // Create reactive signals
    let editor = RwSignal::new(EditorState::new());
    let theme = RwSignal::new(Theme::dark());
    let font_config = RwSignal::new(FontConfig::default());

    // NEW: Create terminal panel state
    let terminal_panel = RwSignal::new(TerminalPanelState::new());

    // NEW: Initialize terminal plugin
    let terminal_config = TerminalConfig::default();
    let terminal_handler = init_terminal_plugin(
        terminal_panel,
        terminal_config,
    );

    let app = Application::new().window(
        |_| {
            // Main layout with terminal
            app_view_with_terminal(
                editor,
                terminal_panel,
                theme,
                font_config,
            )
        },
        Some(WindowConfig::default()
            .size((1200, 800))
            .title("Lite XL")),
    );

    app.run();
}

/// Application view with integrated terminal
fn app_view_with_terminal(
    editor: RwSignal<EditorState>,
    terminal_panel: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    container(
        // Choose layout based on terminal position
        move || {
            let panel = terminal_panel.get();

            match panel.position {
                PanelPosition::Bottom => {
                    // Vertical split (editor on top, terminal on bottom)
                    v_stack((
                        editor_area(editor, theme, font_config),
                        terminal_area(terminal_panel, theme),
                    ))
                }
                PanelPosition::Left => {
                    // Horizontal split (terminal on left, editor on right)
                    h_stack((
                        terminal_area(terminal_panel, theme),
                        editor_area(editor, theme, font_config),
                    ))
                }
                PanelPosition::Right => {
                    // Horizontal split (editor on left, terminal on right)
                    h_stack((
                        editor_area(editor, theme, font_config),
                        terminal_area(terminal_panel, theme),
                    ))
                }
            }
        }
    )
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height_full()
            .background(theme_val.background)
    })
}

fn editor_area(
    editor: RwSignal<EditorState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    v_stack((
        // Main editor view
        container(
            editor_view(editor, theme, font_config)
        )
        .style(|s| s.flex_grow(1.0).width_full()),

        // Status bar
        statusbar_view(editor, theme),
    ))
    .style(|s| s.flex_grow(1.0))
}

fn terminal_area(
    terminal_panel: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        terminal_panel_view(terminal_panel, theme)
    )
    .style(move |s| {
        let panel = terminal_panel.get();

        if !panel.visible {
            return s.display(Display::None);
        }

        match panel.position {
            PanelPosition::Bottom => {
                s.width_full().height(panel.size)
            }
            PanelPosition::Left | PanelPosition::Right => {
                s.height_full().width(panel.size)
            }
        }
    })
}
```

### 2. Configuration Integration

**File:** `/home/user/lite-xl/src/config/mod.rs`

```rust
use serde::{Deserialize, Serialize};
use lite_xl::plugins::terminal::TerminalConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Editor settings (existing)
    #[serde(default)]
    pub editor: EditorConfig,

    /// UI settings (existing)
    #[serde(default)]
    pub ui: UiConfig,

    /// Keymap settings (existing)
    #[serde(default)]
    pub keymap: KeymapConfig,

    /// Language-specific settings (existing)
    #[serde(default)]
    pub languages: HashMap<String, LanguageConfig>,

    /// NEW: Terminal settings
    #[serde(default)]
    pub terminal: TerminalConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig::default(),
            ui: UiConfig::default(),
            keymap: KeymapConfig::default(),
            languages: Self::default_languages(),
            terminal: TerminalConfig::default(), // NEW
        }
    }
}
```

### 3. Command Integration

**File:** `/home/user/lite-xl/src/commands/mod.rs`

```rust
use lite_xl::plugins::terminal::TerminalCommand;

/// Extended Command enum with terminal commands
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // Existing commands
    Insert(String),
    Delete,
    Undo,
    Redo,
    // ... other existing commands

    // NEW: Terminal commands
    Terminal(TerminalCommand),
}

impl Command {
    pub async fn execute(&self, context: &mut CommandContext) -> Result<()> {
        match self {
            // Existing command execution
            Command::Insert(text) => {
                // ... existing logic
            }

            // NEW: Terminal command execution
            Command::Terminal(term_cmd) => {
                term_cmd.execute(&mut context.terminal_panel).await?;
            }

            _ => {}
        }
        Ok(())
    }
}
```

### 4. Keybinding Integration

**File:** `/home/user/lite-xl/src/commands/mod.rs` (continued)

```rust
impl KeyMap {
    pub fn default_bindings() -> Self {
        let mut keymap = KeyMap::new();

        // Existing keybindings
        keymap.bind("ctrl+c", Command::Copy);
        keymap.bind("ctrl+v", Command::Paste);
        // ... other existing bindings

        // NEW: Terminal keybindings
        keymap.bind("ctrl+`", Command::Terminal(TerminalCommand::Toggle));
        keymap.bind("ctrl+shift+t", Command::Terminal(TerminalCommand::New));
        keymap.bind("ctrl+shift+w", Command::Terminal(TerminalCommand::Close));
        keymap.bind("ctrl+tab", Command::Terminal(TerminalCommand::NextTab));
        keymap.bind("ctrl+shift+tab", Command::Terminal(TerminalCommand::PrevTab));
        keymap.bind("ctrl+1", Command::Terminal(TerminalCommand::SwitchToTab(1)));
        keymap.bind("ctrl+2", Command::Terminal(TerminalCommand::SwitchToTab(2)));
        keymap.bind("ctrl+3", Command::Terminal(TerminalCommand::SwitchToTab(3)));
        keymap.bind("ctrl+shift+k", Command::Terminal(TerminalCommand::Clear));

        keymap
    }
}
```

### 5. Theme Integration

**File:** `/home/user/lite-xl/src/ui/theme.rs`

```rust
use floem::peniko::Color as FloemColor;

#[derive(Debug, Clone)]
pub struct Theme {
    // Existing theme colors
    pub background: FloemColor,
    pub foreground: FloemColor,
    pub selection: FloemColor,
    // ... other existing colors

    // NEW: Terminal-specific colors
    pub terminal: TerminalTheme,
}

#[derive(Debug, Clone)]
pub struct TerminalTheme {
    /// Terminal background
    pub background: FloemColor,

    /// Terminal foreground (default text color)
    pub foreground: FloemColor,

    /// Terminal cursor
    pub cursor: FloemColor,

    /// Terminal selection
    pub selection: FloemColor,

    /// Tab bar background
    pub tab_bar_bg: FloemColor,

    /// Active tab background
    pub tab_active_bg: FloemColor,

    /// Inactive tab background
    pub tab_inactive_bg: FloemColor,

    /// Tab hover background
    pub tab_hover_bg: FloemColor,

    /// Border color
    pub border: FloemColor,

    /// ANSI color palette (16 colors)
    pub ansi: [FloemColor; 16],
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            // Existing theme colors
            background: rgb(30, 30, 30),
            foreground: rgb(220, 220, 220),
            // ... other existing colors

            // NEW: Terminal theme
            terminal: TerminalTheme {
                background: rgb(20, 20, 20),
                foreground: rgb(220, 220, 220),
                cursor: rgb(255, 255, 255),
                selection: rgba(70, 130, 180, 100),
                tab_bar_bg: rgb(25, 25, 25),
                tab_active_bg: rgb(40, 40, 40),
                tab_inactive_bg: rgb(30, 30, 30),
                tab_hover_bg: rgb(35, 35, 35),
                border: rgb(50, 50, 50),
                ansi: [
                    rgb(0, 0, 0),           // Black
                    rgb(205, 49, 49),       // Red
                    rgb(13, 188, 121),      // Green
                    rgb(229, 229, 16),      // Yellow
                    rgb(36, 114, 200),      // Blue
                    rgb(188, 63, 188),      // Magenta
                    rgb(17, 168, 205),      // Cyan
                    rgb(229, 229, 229),     // White
                    rgb(102, 102, 102),     // Bright Black
                    rgb(241, 76, 76),       // Bright Red
                    rgb(35, 209, 139),      // Bright Green
                    rgb(245, 245, 67),      // Bright Yellow
                    rgb(59, 142, 234),      // Bright Blue
                    rgb(214, 112, 214),     // Bright Magenta
                    rgb(41, 184, 219),      // Bright Cyan
                    rgb(229, 229, 229),     // Bright White
                ],
            },
        }
    }
}

fn rgb(r: u8, g: u8, b: u8) -> FloemColor {
    FloemColor::rgb8(r, g, b)
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> FloemColor {
    FloemColor::rgba8(r, g, b, a)
}
```

### 6. Event Integration

**File:** `/home/user/lite-xl/src/events/mod.rs`

```rust
use lite_xl::plugins::terminal::TerminalEvent;

/// Extended EditorEvent with terminal events
#[derive(Debug, Clone)]
pub enum EditorEvent {
    // Existing events
    Key(KeyEvent),
    Mouse(MouseEvent),
    Window(WindowEvent),
    // ... other existing events

    // NEW: Terminal events
    Terminal(TerminalEvent),
}

pub struct EventDispatcher {
    // Existing handlers
    editor_handler: Box<dyn EventHandler>,

    // NEW: Terminal event handler
    terminal_handler: Option<TerminalEventHandler>,
}

impl EventDispatcher {
    pub async fn dispatch(&mut self, event: EditorEvent) {
        match event {
            EditorEvent::Key(key_event) => {
                // Check if terminal is focused
                if let Some(ref mut handler) = self.terminal_handler {
                    if handler.is_focused() {
                        // Send to terminal
                        handler.handle_key(key_event).await;
                        return;
                    }
                }
                // Otherwise, send to editor
                self.editor_handler.handle_key(key_event);
            }

            EditorEvent::Terminal(term_event) => {
                if let Some(ref mut handler) = self.terminal_handler {
                    handler.handle(term_event).await;
                }
            }

            _ => {
                // Handle other events
            }
        }
    }

    pub fn set_terminal_handler(&mut self, handler: TerminalEventHandler) {
        self.terminal_handler = Some(handler);
    }
}
```

## File Structure After Integration

```
src/
├── lib.rs                          # Updated with terminal exports
├── main.rs                         # Updated with terminal integration
│
├── plugins/                        # NEW directory
│   ├── mod.rs
│   └── terminal/
│       ├── mod.rs
│       ├── backend/
│       ├── parser/
│       ├── buffer/
│       ├── state/
│       ├── ui/
│       ├── config.rs
│       ├── commands.rs
│       ├── events.rs
│       └── clipboard.rs
│
├── editor/
│   └── mod.rs                      # Existing, unchanged
│
├── ui/
│   ├── mod.rs                      # Updated to use terminal layout
│   ├── theme.rs                    # Updated with terminal theme
│   ├── editor_view.rs              # Existing, unchanged
│   ├── gutter.rs                   # Existing, unchanged
│   └── statusbar.rs                # Existing, unchanged
│
├── commands/
│   └── mod.rs                      # Updated with terminal commands
│
├── events/
│   └── mod.rs                      # Updated with terminal events
│
├── config/
│   └── mod.rs                      # Updated with terminal config
│
└── ... (other existing modules)
```

## Step-by-Step Integration

### Step 1: Add Dependencies

Update `/home/user/lite-xl/Cargo.toml`:

```toml
[dependencies]
# Existing dependencies
floem = "0.2"
tokio = { version = "1.35", features = ["full"] }
# ... other existing deps

# NEW: Terminal dependencies
vte = "0.13"
bitflags = "2.4"

[target.'cfg(unix)'.dependencies]
nix = { version = "0.27", features = ["process", "pty"] }

[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = ["Win32_System_Console"] }
```

### Step 2: Create Plugin Module

Create `/home/user/lite-xl/src/plugins/mod.rs`:

```rust
pub mod terminal;

pub use terminal::{
    TerminalConfig,
    TerminalPanelState,
    terminal_panel_view,
};
```

### Step 3: Implement Terminal Plugin

Follow the architecture in `TERMINAL_PLUGIN_ARCHITECTURE.md` to implement:

1. Backend (PTY, process, shell)
2. Parser (ANSI, colors, CSI)
3. Buffer (grid, cell, scrollback, cursor)
4. State (terminal, panel, tab)
5. UI (terminal_view, tab_bar, panel)

### Step 4: Update lib.rs

Update `/home/user/lite-xl/src/lib.rs`:

```rust
pub mod buffer;
pub mod clipboard;
pub mod commands;
pub mod document;
pub mod events;
pub mod undo;

// NEW
pub mod plugins;

// Re-export
pub use plugins::terminal;
```

### Step 5: Update Configuration

Add terminal configuration support to `/home/user/lite-xl/src/config/mod.rs`.

### Step 6: Register Commands

Add terminal commands to the command system.

### Step 7: Update Main Application

Integrate terminal panel into main application layout.

### Step 8: Test

```bash
# Run tests
cargo test plugins::terminal

# Run example
cargo run --example terminal_core_demo

# Run application
cargo run --bin lite-xl
```

## Usage After Integration

### Open Terminal

Press `` Ctrl+` `` to toggle terminal visibility.

### Create New Terminal Tab

Press `Ctrl+Shift+T` to create a new terminal tab.

### Switch Between Tabs

- `Ctrl+Tab`: Next tab
- `Ctrl+Shift+Tab`: Previous tab
- `Ctrl+1-9`: Switch to specific tab

### Change Position

- `Ctrl+Shift+B`: Move to bottom
- `Ctrl+Shift+L`: Move to left
- `Ctrl+Shift+R`: Move to right

### Resize Panel

Drag the resize handle or use:
- `Ctrl+Shift+=`: Increase size
- `Ctrl+Shift+-`: Decrease size

## Example Configuration

`~/.config/rust-editor/config.toml`:

```toml
[terminal]
shell = "/bin/zsh"
shell_args = ["-l"]
scrollback_lines = 10000
font_size = 14.0
cursor_style = "block"
default_position = "bottom"
default_size = 300.0

[terminal.colors]
foreground = "#dcdcdc"
background = "#1e1e1e"
cursor = "#ffffff"
selection = "#4682b480"

[terminal.env]
EDITOR = "lite-xl"
VISUAL = "lite-xl"
```

## Troubleshooting

### Terminal Not Showing

1. Check `terminal_panel.visible` is `true`
2. Verify `terminal_panel.terminals` has at least one terminal
3. Check layout conditional rendering

### Input Not Working

1. Ensure terminal view has focus
2. Check `handle_terminal_input()` is called
3. Verify PTY is writable

### Output Not Appearing

1. Check PTY read task is running
2. Verify ANSI parser is processing bytes
3. Check grid is being updated
4. Ensure reactive signals are triggering re-renders

### Performance Issues

1. Reduce scrollback buffer size
2. Limit terminal dimensions
3. Check for excessive re-renders
4. Profile with `cargo flamegraph`

## API Usage Examples

### Create Terminal Programmatically

```rust
use lite_xl::plugins::terminal::TerminalCommand;

// In your event handler
let cmd = TerminalCommand::New;
cmd.execute(&mut terminal_panel_state).await?;
```

### Send Command to Active Terminal

```rust
if let Some(terminal) = terminal_panel_state.active() {
    terminal.send_input(b"ls -la\n").await?;
}
```

### Handle Terminal Output

```rust
// This is handled automatically by the I/O task
// But you can subscribe to output events:

match event {
    TerminalEvent::Output { terminal_id, data } => {
        // Process output
        println!("Terminal {}: {} bytes", terminal_id, data.len());
    }
    _ => {}
}
```

## Advanced Integration

### Custom Terminal Commands

```rust
// Add custom terminal commands
impl TerminalCommand {
    pub fn RunScript(script: String) -> Self {
        // Implementation
    }
}

// Use in command palette
keymap.bind("ctrl+shift+r", Command::Terminal(
    TerminalCommand::RunScript("./build.sh".to_string())
));
```

### Terminal Links

```rust
// Detect file paths in terminal output
// and make them clickable to open in editor

impl TerminalView {
    fn detect_file_paths(&self, line: &str) -> Vec<FilePath> {
        // Parse line for file paths
        // Return clickable regions
    }

    fn handle_click(&mut self, row: usize, col: usize) {
        if let Some(path) = self.get_path_at(row, col) {
            // Open file in editor
            app_state.editor.open_file(path);
        }
    }
}
```

### Split Terminal

```rust
// Future feature: Split terminal panes
impl TerminalCommand {
    SplitHorizontal,
    SplitVertical,
}
```

## Conclusion

This integration guide demonstrates how to seamlessly add the terminal plugin to the existing Lite XL editor while maintaining clean separation of concerns and following the established architecture patterns.

Key integration points:
1. ✅ Main application layout
2. ✅ Configuration system
3. ✅ Command system
4. ✅ Keybindings
5. ✅ Theme system
6. ✅ Event handling

The terminal plugin is designed to be:
- **Modular**: Self-contained in `src/plugins/terminal/`
- **Configurable**: TOML-based configuration
- **Extensible**: Clean API for customization
- **Performant**: Reactive updates, async I/O
- **Cross-platform**: Works on Linux, macOS, Windows

---

**Integration Guide Version**: 1.0
**Last Updated**: 2025-11-15

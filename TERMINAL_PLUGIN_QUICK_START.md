# Terminal Plugin - Quick Start Guide

## Overview

This guide provides a quick introduction to using and integrating the terminal plugin.

## Basic Usage

### Opening a Terminal

```rust
// Toggle terminal panel
Ctrl + `

// Create new terminal tab
Ctrl + Shift + T

// Close current terminal
Ctrl + Shift + W
```

### Integration Example

```rust
// src/main.rs

use floem::reactive::RwSignal;
use lite_xl::plugins::terminal::{
    init_terminal_plugin,
    terminal_panel_view,
    TerminalConfig,
};

fn main() {
    // Create app state
    let mut app_state = AppState::new();

    // Initialize terminal plugin
    let terminal_config = TerminalConfig::default();
    let terminal_handler = init_terminal_plugin(&mut app_state, terminal_config);

    // Create main UI
    let app = floem::Application::new().window(
        |_| {
            v_stack((
                // Main editor view
                editor_view(app_state.editor, app_state.theme, app_state.font_config)
                    .style(|s| s.flex_grow(1.0)),

                // Terminal panel (conditionally shown)
                container(
                    terminal_panel_view(app_state.terminal_panel, app_state.theme)
                )
                .style(move |s| {
                    let panel = app_state.terminal_panel.get();
                    if panel.visible {
                        s.display(Display::Flex)
                    } else {
                        s.display(Display::None)
                    }
                }),
            ))
        },
        Some(WindowConfig::default()
            .size((1200, 800))
            .title("Lite XL")),
    );

    app.run();
}
```

## Creating a Terminal Programmatically

```rust
use lite_xl::plugins::terminal::TerminalCommand;

// Create new terminal
let cmd = TerminalCommand::New;
cmd.execute(&mut panel_state).await?;

// Switch to specific tab
let cmd = TerminalCommand::SwitchToTab(2);
cmd.execute(&mut panel_state).await?;

// Change position to side
let cmd = TerminalCommand::MoveToRight;
cmd.execute(&mut panel_state).await?;
```

## Configuration

### TOML Configuration

Create or edit `~/.config/rust-editor/config.toml`:

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

[terminal.env]
EDITOR = "lite-xl"
```

### Programmatic Configuration

```rust
use lite_xl::plugins::terminal::TerminalConfig;

let config = TerminalConfig {
    shell: Some("/bin/bash".to_string()),
    scrollback_lines: 5000,
    font_size: 16.0,
    cursor_style: CursorStyleConfig::Bar,
    ..Default::default()
};
```

## Architecture at a Glance

```
Terminal Plugin Architecture
═══════════════════════════════

┌─────────────────────────────────────────┐
│         UI Layer (Floem Views)          │
│  ┌─────────────────────────────────┐    │
│  │   terminal_panel_view()         │    │
│  │   ├── tab_bar_view()            │    │
│  │   └── terminal_view()           │    │
│  │       ├── grid_view()           │    │
│  │       └── cursor_view()         │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────┐
│      State (Reactive Signals)           │
│  • RwSignal<TerminalPanelState>        │
│    ├── Vec<TerminalState>              │
│    ├── active_terminal: usize          │
│    └── position, size, visible         │
└─────────────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────┐
│      Terminal State (per tab)           │
│  • Grid (2D array of Cells)            │
│  • Scrollback buffer                    │
│  • Cursor position & style             │
│  • PTY handle                           │
│  • Shell process                        │
└─────────────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────┐
│         Backend (PTY + Process)         │
│  • PTY creation (Unix/Windows)         │
│  • Shell spawning                       │
│  • I/O handling (async)                │
│  • ANSI parsing                         │
└─────────────────────────────────────────┘
```

## Data Flow

### Input Flow (Keyboard → Terminal)

```
User presses key
    ↓
Floem KeyDown event
    ↓
handle_terminal_input()
    ↓
Convert key to bytes/escape sequences
    ↓
terminal.send_input(bytes)
    ↓
PTY write
    ↓
Shell process receives input
```

### Output Flow (Terminal → UI)

```
Shell produces output
    ↓
PTY read (async task)
    ↓
ANSI parser (vte)
    ↓
Terminal actions (Print, MoveCursor, SetColor, etc.)
    ↓
Update Grid cells
    ↓
RwSignal update
    ↓
Floem reactive re-render
    ↓
grid_view() renders cells
```

## Key Components

### 1. Cell - Basic Terminal Unit

```rust
pub struct Cell {
    ch: char,                    // Character ('A', '中', '🚀')
    fg: Color,                   // Foreground RGB
    bg: Color,                   // Background RGB
    attrs: CellAttributes,       // Bold, Italic, etc.
}
```

### 2. Grid - 2D Array of Cells

```rust
pub struct Grid {
    cols: usize,                 // Width (e.g., 80)
    rows: usize,                 // Height (e.g., 24)
    cells: Vec<Vec<Cell>>,       // rows × cols
}
```

### 3. Scrollback - History Buffer

```rust
pub struct Scrollback {
    max_lines: usize,            // Max history (e.g., 10,000)
    lines: VecDeque<Vec<Cell>>,  // Circular buffer
    scroll_offset: usize,        // Current position
}
```

### 4. TerminalState - Single Terminal

```rust
pub struct TerminalState {
    id: TerminalId,
    title: String,
    grid: Arc<RwLock<Grid>>,
    scrollback: Arc<RwLock<Scrollback>>,
    cursor: Cursor,
    pty: Arc<RwLock<Pty>>,
    process: Arc<RwLock<Option<Process>>>,
}
```

### 5. TerminalPanelState - Multi-Terminal Manager

```rust
pub struct TerminalPanelState {
    terminals: Vec<TerminalState>,
    active_terminal: usize,
    visible: bool,
    position: PanelPosition,
    size: f64,
}
```

## Common Operations

### Send Text to Terminal

```rust
let text = "ls -la\n";
terminal.send_input(text.as_bytes()).await?;
```

### Resize Terminal

```rust
terminal.resize(120, 30).await?;  // 120 cols, 30 rows
```

### Scroll Back

```rust
scrollback.scroll_up(10);  // Scroll up 10 lines
```

### Clear Terminal

```rust
terminal.grid.write().await.clear();
```

## ANSI Escape Sequences

### Common Sequences

| Sequence | Description |
|----------|-------------|
| `\x1b[H` | Move cursor to home (0,0) |
| `\x1b[2J` | Clear entire screen |
| `\x1b[31m` | Set foreground to red |
| `\x1b[42m` | Set background to green |
| `\x1b[1m` | Bold text |
| `\x1b[0m` | Reset all attributes |
| `\x1b[?25h` | Show cursor |
| `\x1b[?25l` | Hide cursor |

### Example: Colored Output

```rust
// Bash command that produces colored output
echo -e "\x1b[31mRed text\x1b[0m"
echo -e "\x1b[1;32mBold green\x1b[0m"
echo -e "\x1b[44mBlue background\x1b[0m"
```

## Platform Differences

### Unix (Linux, macOS)

- Uses POSIX PTY (`/dev/ptmx`)
- Shell: Usually `bash`, `zsh`, or `fish`
- Environment: Inherits from parent process

### Windows

- Uses ConPTY API (Windows 10+)
- Shell: Usually PowerShell or `cmd.exe`
- Line endings: CRLF vs LF

### Cross-Platform Shell Detection

```rust
pub fn detect_default_shell() -> String {
    #[cfg(unix)]
    {
        std::env::var("SHELL")
            .unwrap_or_else(|_| "/bin/sh".to_string())
    }

    #[cfg(windows)]
    {
        std::env::var("COMSPEC")
            .unwrap_or_else(|_| "powershell.exe".to_string())
    }
}
```

## Performance Considerations

### Rendering Optimization

- **Reactive Updates**: Only re-render when grid changes
- **Partial Rendering**: Only update changed cells (future)
- **GPU Acceleration**: Use Floem's GPU backend

### Memory Management

- **Scrollback Limit**: Configure max lines (default: 10,000)
- **Cell Reuse**: Recycle cell objects
- **Arc/RwLock**: Minimize cloning

### I/O Efficiency

- **Buffered Reads**: Read 4KB chunks from PTY
- **Async Processing**: Non-blocking I/O with Tokio
- **Batch Updates**: Group grid updates

## Troubleshooting

### Terminal Not Appearing

```rust
// Check panel visibility
println!("Visible: {}", panel_state.get().visible);

// Ensure at least one terminal exists
println!("Terminals: {}", panel_state.get().terminals.len());
```

### No Output in Terminal

1. Check PTY is connected
2. Verify shell process is running
3. Check I/O task is spawned
4. Enable debug logging

### Colors Not Working

1. Verify ANSI parser is enabled
2. Check color conversion
3. Ensure theme has ANSI palette
4. Test with simple ANSI codes

### Performance Issues

1. Reduce scrollback buffer size
2. Check for excessive re-renders
3. Profile with `cargo flamegraph`
4. Consider virtualization for large grids

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::default();
        assert_eq!(cell.ch, ' ');
        assert_eq!(cell.fg, Color::WHITE);
    }

    #[test]
    fn test_grid_resize() {
        let mut grid = Grid::new(80, 24);
        grid.resize(120, 30);
        assert_eq!(grid.cols, 120);
        assert_eq!(grid.rows, 30);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_terminal_echo() {
    let mut terminal = TerminalState::new(0, 80, 24).await.unwrap();
    terminal.start(None).await.unwrap();

    // Send input
    terminal.send_input(b"echo hello\n").await.unwrap();

    // Wait for output
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Check grid contains "hello"
    let grid = terminal.grid.read().await;
    let text = grid_to_string(&grid);
    assert!(text.contains("hello"));
}
```

## API Reference

### Main API Functions

```rust
// Initialize plugin
pub fn init_terminal_plugin(
    app_state: &mut AppState,
    config: TerminalConfig,
) -> TerminalEventHandler

// Create terminal panel view
pub fn terminal_panel_view(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View

// Execute terminal command
pub async fn execute_command(
    command: TerminalCommand,
    panel: &mut TerminalPanelState,
) -> Result<()>
```

### Events

```rust
pub enum TerminalEvent {
    NewTerminal,
    CloseTerminal(TerminalId),
    SwitchTerminal(TerminalId),
    ToggleTerminal,
    Output { terminal_id: TerminalId, data: Vec<u8> },
    ProcessExited { terminal_id: TerminalId, exit_code: i32 },
}
```

## Resources

### Documentation

- Main Architecture: `TERMINAL_PLUGIN_ARCHITECTURE.md`
- API Reference: Generated with `cargo doc --open`
- Examples: `examples/terminal_demo.rs`

### External Resources

- [VT100 Escape Codes](https://vt100.net/docs/vt100-ug/chapter3.html)
- [ANSI Escape Sequences](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [Alacritty Terminal](https://github.com/alacritty/alacritty) - Reference implementation
- [VTE Parser](https://docs.rs/vte/) - ANSI parser crate

### Related Code

- Floem Framework: https://github.com/lapce/floem
- Portable PTY: https://docs.rs/portable-pty/
- Nix PTY: https://docs.rs/nix/latest/nix/pty/

---

**Quick Start Version**: 1.0
**Last Updated**: 2025-11-15

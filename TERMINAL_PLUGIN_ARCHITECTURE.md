# Terminal Plugin Architecture Design

**Version:** 1.0
**Date:** 2025-11-15
**Target Platform:** Cross-platform (Linux, macOS, Windows)
**UI Framework:** Floem 0.2

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Module Structure](#module-structure)
4. [Data Structures](#data-structures)
5. [Component Hierarchy](#component-hierarchy)
6. [Event Handling](#event-handling)
7. [State Management](#state-management)
8. [API Design](#api-design)
9. [Configuration](#configuration)
10. [Keyboard Shortcuts](#keyboard-shortcuts)
11. [Implementation Plan](#implementation-plan)
12. [Dependencies](#dependencies)

---

## Overview

The terminal plugin provides an integrated terminal emulator within the Lite XL text editor. It supports multiple terminal instances via tabs, can be docked to the bottom or side, includes full ANSI color support, scrollback buffer, and shell integration.

### Key Features

- **Multiple Terminals**: Tab-based interface for managing multiple terminal sessions
- **Flexible Docking**: Can be docked to bottom, left, or right side of the editor
- **Resizable Panel**: Drag-to-resize terminal panel height/width
- **ANSI Support**: Full ANSI color codes and escape sequences
- **Scrollback**: Configurable scrollback buffer (default: 10,000 lines)
- **Copy/Paste**: Native clipboard integration
- **Shell Integration**: Auto-detection of bash, zsh, fish, PowerShell
- **Cross-Platform**: Linux, macOS, and Windows support

---

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────┐
│                    Editor Window                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │            Main Editor View                       │  │
│  │         (existing editor_view)                    │  │
│  │                                                   │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ ┌─────────┬─────────┬─────────┐  [+] [Settings]  │  │
│  │ │ Term 1  │ Term 2  │ Term 3  │     Tab Bar       │  │
│  │ └─────────┴─────────┴─────────┘                   │  │
│  │ ┌─────────────────────────────────────────────┐   │  │
│  │ │  user@host:~/project$ ls -la               │   │  │
│  │ │  total 64                                   │   │  │
│  │ │  drwxr-xr-x  5 user  staff  160 Nov 15     │   │  │
│  │ │  -rw-r--r--  1 user  staff 1234 Nov 15     │   │  │
│  │ │  user@host:~/project$ █                    │   │  │
│  │ │                                             │   │  │
│  │ └─────────────────────────────────────────────┘   │  │
│  │              Terminal Panel                       │  │
│  └───────────────────────────────────────────────────┘  │
│                  Status Bar                             │
└─────────────────────────────────────────────────────────┘
```

### Component Layers

```
┌──────────────────────────────────────────┐
│         UI Layer (Floem)                 │
│  - terminal_view                         │
│  - tab_bar_view                          │
│  - terminal_panel_view                   │
└──────────────────────────────────────────┘
                   ↓
┌──────────────────────────────────────────┐
│        State Management Layer            │
│  - TerminalState (RwSignal)             │
│  - TerminalPanelState (RwSignal)        │
│  - Active tab tracking                   │
└──────────────────────────────────────────┘
                   ↓
┌──────────────────────────────────────────┐
│      Terminal Backend Layer              │
│  - PTY (Pseudo-terminal)                │
│  - Process spawning                      │
│  - I/O handling                          │
└──────────────────────────────────────────┘
                   ↓
┌──────────────────────────────────────────┐
│      ANSI Parser Layer                   │
│  - Escape sequence parsing              │
│  - Color conversion                      │
│  - Cursor control                        │
└──────────────────────────────────────────┘
```

---

## Module Structure

### File Organization

```
src/
├── plugins/
│   ├── mod.rs                      # Plugin system exports
│   │
│   └── terminal/
│       ├── mod.rs                  # Terminal plugin main module
│       │
│       ├── backend/
│       │   ├── mod.rs             # Backend abstraction
│       │   ├── pty.rs             # PTY management
│       │   ├── process.rs         # Process spawning
│       │   ├── shell.rs           # Shell detection & config
│       │   └── platform/
│       │       ├── mod.rs         # Platform-specific exports
│       │       ├── unix.rs        # Unix (Linux/macOS) PTY
│       │       └── windows.rs     # Windows ConPTY
│       │
│       ├── parser/
│       │   ├── mod.rs             # ANSI parser exports
│       │   ├── ansi.rs            # ANSI escape sequence parser
│       │   ├── colors.rs          # Color parsing & conversion
│       │   └── csi.rs             # CSI sequence handling
│       │
│       ├── buffer/
│       │   ├── mod.rs             # Buffer management
│       │   ├── grid.rs            # Terminal grid (rows × cols)
│       │   ├── cell.rs            # Terminal cell (char + style)
│       │   ├── scrollback.rs      # Scrollback buffer
│       │   └── cursor.rs          # Cursor state
│       │
│       ├── state/
│       │   ├── mod.rs             # State management
│       │   ├── terminal.rs        # Single terminal state
│       │   ├── panel.rs           # Terminal panel state
│       │   └── tab.rs             # Tab management
│       │
│       ├── ui/
│       │   ├── mod.rs             # UI components
│       │   ├── terminal_view.rs   # Main terminal rendering view
│       │   ├── tab_bar.rs         # Tab bar component
│       │   ├── panel.rs           # Terminal panel container
│       │   └── theme.rs           # Terminal theme colors
│       │
│       ├── config.rs              # Terminal configuration
│       ├── commands.rs            # Terminal-specific commands
│       ├── events.rs              # Terminal event handling
│       └── clipboard.rs           # Terminal clipboard integration
│
└── lib.rs                         # Update to export plugins module
```

### Module Responsibilities

#### `plugins/terminal/mod.rs`
- Public API for terminal plugin
- Plugin initialization and lifecycle
- Integration with editor

#### `backend/`
- **pty.rs**: PTY (pseudo-terminal) creation and management
- **process.rs**: Child process spawning and lifecycle
- **shell.rs**: Shell detection (bash, zsh, fish, PowerShell)
- **platform/**: OS-specific implementations

#### `parser/`
- **ansi.rs**: Parse ANSI escape sequences
- **colors.rs**: Convert ANSI colors to RGB
- **csi.rs**: Control Sequence Introducer handling

#### `buffer/`
- **grid.rs**: 2D grid of terminal cells
- **cell.rs**: Individual cell (character + foreground + background + attributes)
- **scrollback.rs**: Circular buffer for scrollback
- **cursor.rs**: Cursor position and styling

#### `state/`
- **terminal.rs**: Single terminal instance state
- **panel.rs**: Panel visibility, position, size
- **tab.rs**: Multiple terminal tabs management

#### `ui/`
- **terminal_view.rs**: Floem view for rendering terminal
- **tab_bar.rs**: Tab selection UI
- **panel.rs**: Resizable panel container
- **theme.rs**: Color schemes for terminal

---

## Data Structures

### Core Data Types

```rust
// src/plugins/terminal/buffer/cell.rs

/// A single terminal cell
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// The character in this cell (supports Unicode)
    pub ch: char,

    /// Foreground color
    pub fg: Color,

    /// Background color
    pub bg: Color,

    /// Text attributes (bold, italic, underline, etc.)
    pub attrs: CellAttributes,
}

bitflags::bitflags! {
    /// Cell attributes for text styling
    pub struct CellAttributes: u8 {
        const BOLD          = 0b0000_0001;
        const DIM           = 0b0000_0010;
        const ITALIC        = 0b0000_0100;
        const UNDERLINE     = 0b0000_1000;
        const BLINK         = 0b0001_0000;
        const REVERSE       = 0b0010_0000;
        const HIDDEN        = 0b0100_0000;
        const STRIKETHROUGH = 0b1000_0000;
    }
}

/// RGB Color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Cell {
    pub fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
            attrs: CellAttributes::empty(),
        }
    }

    pub fn reset(&mut self) {
        self.ch = ' ';
        self.fg = Color::WHITE;
        self.bg = Color::BLACK;
        self.attrs = CellAttributes::empty();
    }
}
```

```rust
// src/plugins/terminal/buffer/grid.rs

/// Terminal grid (2D array of cells)
pub struct Grid {
    /// Number of columns
    cols: usize,

    /// Number of rows (visible area)
    rows: usize,

    /// The actual cell data (rows × cols)
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(cols: usize, rows: usize) -> Self {
        let cells = vec![vec![Cell::default(); cols]; rows];
        Self { cols, rows, cells }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row).and_then(|r| r.get_mut(col))
    }

    pub fn resize(&mut self, cols: usize, rows: usize) {
        // Resize grid, preserving content where possible
        self.cols = cols;
        self.rows = rows;
        self.cells.resize(rows, vec![Cell::default(); cols]);
        for row in &mut self.cells {
            row.resize(cols, Cell::default());
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        // Move lines up, add empty lines at bottom
        for _ in 0..lines {
            self.cells.remove(0);
            self.cells.push(vec![Cell::default(); self.cols]);
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.reset();
            }
        }
    }
}
```

```rust
// src/plugins/terminal/buffer/scrollback.rs

/// Scrollback buffer (circular buffer)
pub struct Scrollback {
    /// Maximum number of lines to store
    max_lines: usize,

    /// Stored lines (oldest at index 0)
    lines: VecDeque<Vec<Cell>>,

    /// Current scroll offset (0 = viewing bottom)
    scroll_offset: usize,
}

impl Scrollback {
    pub fn new(max_lines: usize) -> Self {
        Self {
            max_lines,
            lines: VecDeque::with_capacity(max_lines),
            scroll_offset: 0,
        }
    }

    pub fn push_line(&mut self, line: Vec<Cell>) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    pub fn get_line(&self, index: usize) -> Option<&Vec<Cell>> {
        self.lines.get(index)
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = (self.scroll_offset + lines).min(self.lines.len());
    }

    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    pub fn is_at_bottom(&self) -> bool {
        self.scroll_offset == 0
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }
}
```

```rust
// src/plugins/terminal/buffer/cursor.rs

/// Cursor state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cursor {
    /// Row position (0-indexed)
    pub row: usize,

    /// Column position (0-indexed)
    pub col: usize,

    /// Cursor visibility
    pub visible: bool,

    /// Cursor style
    pub style: CursorStyle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            visible: true,
            style: CursorStyle::Block,
        }
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn move_up(&mut self, n: usize) {
        self.row = self.row.saturating_sub(n);
    }

    pub fn move_down(&mut self, n: usize, max_row: usize) {
        self.row = (self.row + n).min(max_row);
    }

    pub fn move_forward(&mut self, n: usize, max_col: usize) {
        self.col = (self.col + n).min(max_col);
    }

    pub fn move_backward(&mut self, n: usize) {
        self.col = self.col.saturating_sub(n);
    }
}
```

```rust
// src/plugins/terminal/state/terminal.rs

use std::sync::Arc;
use tokio::sync::RwLock;

/// Single terminal instance state
#[derive(Debug, Clone)]
pub struct TerminalState {
    /// Unique terminal ID
    pub id: TerminalId,

    /// Terminal title (tab label)
    pub title: String,

    /// Grid for visible area
    pub grid: Arc<RwLock<Grid>>,

    /// Scrollback buffer
    pub scrollback: Arc<RwLock<Scrollback>>,

    /// Cursor state
    pub cursor: Cursor,

    /// Terminal dimensions
    pub cols: usize,
    pub rows: usize,

    /// PTY handle
    pty: Arc<RwLock<Pty>>,

    /// Shell process
    process: Arc<RwLock<Option<Process>>>,

    /// Running state
    pub running: bool,
}

pub type TerminalId = usize;

impl TerminalState {
    pub async fn new(id: TerminalId, cols: usize, rows: usize) -> anyhow::Result<Self> {
        let grid = Arc::new(RwLock::new(Grid::new(cols, rows)));
        let scrollback = Arc::new(RwLock::new(Scrollback::new(10_000)));
        let pty = Arc::new(RwLock::new(Pty::new(cols, rows)?));

        Ok(Self {
            id,
            title: format!("Terminal {}", id),
            grid,
            scrollback,
            cursor: Cursor::new(),
            cols,
            rows,
            pty,
            process: Arc::new(RwLock::new(None)),
            running: false,
        })
    }

    /// Start the terminal (spawn shell)
    pub async fn start(&mut self, shell: Option<String>) -> anyhow::Result<()> {
        let shell_path = shell.unwrap_or_else(|| Shell::detect_default());
        let process = self.pty.write().await.spawn_shell(&shell_path)?;

        *self.process.write().await = Some(process);
        self.running = true;

        // Start I/O processing task
        self.start_io_task();

        Ok(())
    }

    /// Send input to terminal
    pub async fn send_input(&self, data: &[u8]) -> anyhow::Result<()> {
        self.pty.write().await.write(data).await
    }

    /// Resize terminal
    pub async fn resize(&mut self, cols: usize, rows: usize) -> anyhow::Result<()> {
        self.cols = cols;
        self.rows = rows;

        self.grid.write().await.resize(cols, rows);
        self.pty.write().await.resize(cols as u16, rows as u16)?;

        Ok(())
    }

    /// Process output from PTY
    async fn process_output(&mut self, data: &[u8]) {
        // Parse ANSI sequences and update grid
        let mut parser = AnsiParser::new();

        for byte in data {
            if let Some(action) = parser.parse(*byte) {
                self.apply_action(action).await;
            }
        }
    }

    async fn apply_action(&mut self, action: AnsiAction) {
        match action {
            AnsiAction::Print(ch) => {
                // Write character at cursor position
                let mut grid = self.grid.write().await;
                if let Some(cell) = grid.get_mut(self.cursor.row, self.cursor.col) {
                    cell.ch = ch;
                    // Apply current style
                }

                // Move cursor forward
                self.cursor.move_forward(1, self.cols - 1);
            }
            AnsiAction::MoveCursor { row, col } => {
                self.cursor.move_to(row, col);
            }
            AnsiAction::ClearScreen => {
                self.grid.write().await.clear();
            }
            AnsiAction::SetForeground(color) => {
                // Update current foreground color
            }
            // ... handle other ANSI actions
        }
    }

    fn start_io_task(&self) {
        // Spawn async task to read from PTY and process output
        let pty = Arc::clone(&self.pty);
        let grid = Arc::clone(&self.grid);

        tokio::spawn(async move {
            let mut buffer = [0u8; 4096];
            loop {
                match pty.write().await.read(&mut buffer).await {
                    Ok(n) if n > 0 => {
                        // Process output
                        // self.process_output(&buffer[..n]).await;
                    }
                    _ => break,
                }
            }
        });
    }
}
```

```rust
// src/plugins/terminal/state/panel.rs

/// Terminal panel state (manages all terminals)
#[derive(Debug, Clone)]
pub struct TerminalPanelState {
    /// All terminal instances
    pub terminals: Vec<TerminalState>,

    /// Active terminal index
    pub active_terminal: usize,

    /// Panel visibility
    pub visible: bool,

    /// Panel position (Bottom, Left, Right)
    pub position: PanelPosition,

    /// Panel size (height for Bottom, width for Left/Right)
    pub size: f64,

    /// Minimum panel size
    pub min_size: f64,

    /// Maximum panel size (as fraction of window)
    pub max_size_fraction: f64,

    /// Next terminal ID
    next_id: TerminalId,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelPosition {
    Bottom,
    Left,
    Right,
}

impl TerminalPanelState {
    pub fn new() -> Self {
        Self {
            terminals: Vec::new(),
            active_terminal: 0,
            visible: false,
            position: PanelPosition::Bottom,
            size: 300.0,
            min_size: 100.0,
            max_size_fraction: 0.7,
            next_id: 0,
        }
    }

    /// Create a new terminal
    pub async fn new_terminal(&mut self, cols: usize, rows: usize) -> anyhow::Result<TerminalId> {
        let id = self.next_id;
        self.next_id += 1;

        let mut terminal = TerminalState::new(id, cols, rows).await?;
        terminal.start(None).await?;

        self.terminals.push(terminal);
        self.active_terminal = self.terminals.len() - 1;
        self.visible = true;

        Ok(id)
    }

    /// Close a terminal
    pub fn close_terminal(&mut self, id: TerminalId) {
        if let Some(idx) = self.terminals.iter().position(|t| t.id == id) {
            self.terminals.remove(idx);

            if self.terminals.is_empty() {
                self.visible = false;
                self.active_terminal = 0;
            } else if self.active_terminal >= self.terminals.len() {
                self.active_terminal = self.terminals.len() - 1;
            }
        }
    }

    /// Get active terminal
    pub fn active(&self) -> Option<&TerminalState> {
        self.terminals.get(self.active_terminal)
    }

    /// Get active terminal (mutable)
    pub fn active_mut(&mut self) -> Option<&mut TerminalState> {
        self.terminals.get_mut(self.active_terminal)
    }

    /// Switch to terminal by index
    pub fn switch_to(&mut self, index: usize) {
        if index < self.terminals.len() {
            self.active_terminal = index;
        }
    }

    /// Toggle panel visibility
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    /// Set panel position
    pub fn set_position(&mut self, position: PanelPosition) {
        self.position = position;

        // Adjust size for new position
        match position {
            PanelPosition::Bottom => self.size = 300.0,
            PanelPosition::Left | PanelPosition::Right => self.size = 400.0,
        }
    }

    /// Resize panel
    pub fn set_size(&mut self, size: f64) {
        self.size = size.max(self.min_size);
    }
}
```

---

## Component Hierarchy

### Floem View Structure

```rust
// src/plugins/terminal/ui/mod.rs

use floem::{
    reactive::RwSignal,
    View,
    views::{container, h_stack, v_stack, Decorators},
};

/// Create the complete terminal panel view
pub fn terminal_panel_view(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        v_stack((
            // Tab bar at top
            tab_bar_view(panel_state, theme),

            // Active terminal view
            active_terminal_view(panel_state, theme),
        ))
    )
    .style(move |s| {
        let panel = panel_state.get();
        let theme_val = theme.get();

        match panel.position {
            PanelPosition::Bottom => {
                s.width_full()
                    .height(panel.size)
                    .background(theme_val.terminal.background)
            }
            PanelPosition::Left | PanelPosition::Right => {
                s.height_full()
                    .width(panel.size)
                    .background(theme_val.terminal.background)
            }
        }
    })
}

/// View for the active terminal
fn active_terminal_view(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        move || {
            let panel = panel_state.get();
            if let Some(terminal) = panel.active() {
                terminal_view(terminal.clone(), theme)
            } else {
                empty_terminal_view(theme)
            }
        }
    )
    .style(|s| s.flex_grow(1.0).width_full())
}
```

```rust
// src/plugins/terminal/ui/tab_bar.rs

/// Tab bar component
pub fn tab_bar_view(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View {
    h_stack((
        // Terminal tabs
        move || {
            let panel = panel_state.get();
            panel.terminals.iter().enumerate().map(|(idx, term)| {
                tab_button(
                    term.clone(),
                    idx,
                    idx == panel.active_terminal,
                    panel_state,
                    theme,
                )
            }).collect::<Vec<_>>()
        },

        // New tab button
        new_tab_button(panel_state, theme),

        // Settings button
        settings_button(panel_state, theme),
    ))
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height(32.0)
            .background(theme_val.terminal.tab_bar_bg)
            .border_bottom(1.0)
            .border_color(theme_val.terminal.border)
    })
}

fn tab_button(
    terminal: TerminalState,
    index: usize,
    is_active: bool,
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        h_stack((
            // Terminal title
            text(terminal.title.clone()),

            // Close button
            button("×")
                .on_click(move |_| {
                    panel_state.update(|p| p.close_terminal(terminal.id));
                }),
        ))
    )
    .on_click(move |_| {
        panel_state.update(|p| p.switch_to(index));
    })
    .style(move |s| {
        let theme_val = theme.get();
        let bg = if is_active {
            theme_val.terminal.tab_active_bg
        } else {
            theme_val.terminal.tab_inactive_bg
        };

        s.padding(8.0)
            .margin_right(2.0)
            .background(bg)
            .border_radius(4.0)
            .hover(|s| s.background(theme_val.terminal.tab_hover_bg))
    })
}
```

```rust
// src/plugins/terminal/ui/terminal_view.rs

use floem::{
    reactive::RwSignal,
    peniko::Color,
    View,
    views::{container, stack, text, Decorators},
};

/// Main terminal rendering view
pub fn terminal_view(
    terminal: TerminalState,
    theme: RwSignal<Theme>,
) -> impl View {
    let grid_signal = RwSignal::new(terminal.grid.clone());
    let cursor_signal = RwSignal::new(terminal.cursor);

    container(
        stack((
            // Render terminal grid
            grid_view(grid_signal, theme),

            // Render cursor
            cursor_view(cursor_signal, theme),
        ))
    )
    .on_event(EventListener::KeyDown, move |event| {
        // Handle keyboard input
        if let Event::KeyDown(key_event) = event {
            handle_terminal_input(terminal.clone(), key_event);
        }
    })
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height_full()
            .background(theme_val.terminal.background)
            .padding(8.0)
    })
}

fn grid_view(
    grid: RwSignal<Arc<RwLock<Grid>>>,
    theme: RwSignal<Theme>,
) -> impl View {
    v_stack(
        move || {
            let grid_guard = grid.get();
            let grid_lock = grid_guard.blocking_read();

            (0..grid_lock.rows).map(|row| {
                render_row(&grid_lock, row, theme)
            }).collect::<Vec<_>>()
        }
    )
    .style(|s| {
        s.font_family("monospace")
            .font_size(14.0)
            .line_height(1.4)
    })
}

fn render_row(grid: &Grid, row: usize, theme: RwSignal<Theme>) -> impl View {
    h_stack(
        (0..grid.cols).map(|col| {
            if let Some(cell) = grid.get(row, col) {
                render_cell(cell.clone())
            } else {
                render_empty_cell()
            }
        })
    )
}

fn render_cell(cell: Cell) -> impl View {
    text(cell.ch.to_string())
        .style(move |s| {
            s.color(rgb_to_floem_color(cell.fg))
                .background(rgb_to_floem_color(cell.bg))
                .apply_if(cell.attrs.contains(CellAttributes::BOLD), |s| {
                    s.font_weight(700)
                })
                .apply_if(cell.attrs.contains(CellAttributes::ITALIC), |s| {
                    s.font_style(FontStyle::Italic)
                })
                .apply_if(cell.attrs.contains(CellAttributes::UNDERLINE), |s| {
                    s.text_decoration(TextDecoration::Underline)
                })
        })
}

fn cursor_view(
    cursor: RwSignal<Cursor>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(empty())
        .style(move |s| {
            let cursor_val = cursor.get();
            let theme_val = theme.get();

            if !cursor_val.visible {
                return s.display(Display::None);
            }

            let char_width = 8.0;  // Monospace character width
            let char_height = 20.0; // Line height

            let x = cursor_val.col as f64 * char_width;
            let y = cursor_val.row as f64 * char_height;

            match cursor_val.style {
                CursorStyle::Block => {
                    s.position(Position::Absolute)
                        .inset_left(x)
                        .inset_top(y)
                        .width(char_width)
                        .height(char_height)
                        .background(theme_val.terminal.cursor)
                }
                CursorStyle::Underline => {
                    s.position(Position::Absolute)
                        .inset_left(x)
                        .inset_top(y + char_height - 2.0)
                        .width(char_width)
                        .height(2.0)
                        .background(theme_val.terminal.cursor)
                }
                CursorStyle::Bar => {
                    s.position(Position::Absolute)
                        .inset_left(x)
                        .inset_top(y)
                        .width(2.0)
                        .height(char_height)
                        .background(theme_val.terminal.cursor)
                }
            }
        })
}

fn handle_terminal_input(terminal: TerminalState, event: KeyEvent) {
    let data = match event.key {
        Key::Character(ch) => {
            // Regular character input
            ch.as_bytes().to_vec()
        }
        Key::Enter => b"\r".to_vec(),
        Key::Backspace => b"\x7f".to_vec(),
        Key::Tab => b"\t".to_vec(),
        Key::ArrowUp => b"\x1b[A".to_vec(),
        Key::ArrowDown => b"\x1b[B".to_vec(),
        Key::ArrowRight => b"\x1b[C".to_vec(),
        Key::ArrowLeft => b"\x1b[D".to_vec(),
        Key::Home => b"\x1b[H".to_vec(),
        Key::End => b"\x1b[F".to_vec(),
        Key::PageUp => b"\x1b[5~".to_vec(),
        Key::PageDown => b"\x1b[6~".to_vec(),
        _ => return,
    };

    // Apply modifiers
    let modified_data = if event.modifiers.contains(Modifiers::CTRL) {
        apply_ctrl_modifier(&data)
    } else {
        data
    };

    // Send to terminal
    tokio::spawn(async move {
        let _ = terminal.send_input(&modified_data).await;
    });
}

fn apply_ctrl_modifier(data: &[u8]) -> Vec<u8> {
    // Convert Ctrl+key combinations to control characters
    // e.g., Ctrl+C = 0x03
    match data {
        [b'c'] | [b'C'] => vec![0x03], // Ctrl+C
        [b'd'] | [b'D'] => vec![0x04], // Ctrl+D
        [b'z'] | [b'Z'] => vec![0x1a], // Ctrl+Z
        _ => data.to_vec(),
    }
}
```

---

## Event Handling

### Event Flow

```
User Input (Keyboard/Mouse)
         ↓
   Floem Event System
         ↓
  Terminal Event Handler
         ↓
┌─────────────────────────┐
│  Key Press Event        │
│  - Convert to bytes     │
│  - Apply modifiers      │
│  - Handle special keys  │
└─────────────────────────┘
         ↓
    Send to PTY
         ↓
   Shell Process
         ↓
  Output from PTY
         ↓
   ANSI Parser
         ↓
 Update Terminal Grid
         ↓
   Reactive Update
         ↓
  UI Re-renders
```

### Terminal Events

```rust
// src/plugins/terminal/events.rs

use crate::events::{EditorEvent, KeyEvent, MouseEvent};

/// Terminal-specific events
#[derive(Debug, Clone)]
pub enum TerminalEvent {
    /// New terminal requested
    NewTerminal,

    /// Close terminal
    CloseTerminal(TerminalId),

    /// Switch to terminal
    SwitchTerminal(TerminalId),

    /// Toggle terminal visibility
    ToggleTerminal,

    /// Change terminal position
    ChangePosition(PanelPosition),

    /// Resize terminal panel
    ResizePanel(f64),

    /// Terminal output received
    Output {
        terminal_id: TerminalId,
        data: Vec<u8>,
    },

    /// Terminal process exited
    ProcessExited {
        terminal_id: TerminalId,
        exit_code: i32,
    },

    /// Paste to terminal
    Paste(String),

    /// Copy from terminal
    Copy,

    /// Clear terminal
    Clear(TerminalId),

    /// Reset terminal
    Reset(TerminalId),
}

/// Terminal event handler
pub struct TerminalEventHandler {
    panel_state: RwSignal<TerminalPanelState>,
}

impl TerminalEventHandler {
    pub fn new(panel_state: RwSignal<TerminalPanelState>) -> Self {
        Self { panel_state }
    }

    pub async fn handle(&mut self, event: TerminalEvent) {
        match event {
            TerminalEvent::NewTerminal => {
                self.create_terminal().await;
            }
            TerminalEvent::CloseTerminal(id) => {
                self.panel_state.update(|p| p.close_terminal(id));
            }
            TerminalEvent::SwitchTerminal(id) => {
                self.switch_terminal(id);
            }
            TerminalEvent::ToggleTerminal => {
                self.panel_state.update(|p| p.toggle_visibility());
            }
            TerminalEvent::Output { terminal_id, data } => {
                self.process_output(terminal_id, &data).await;
            }
            TerminalEvent::ProcessExited { terminal_id, exit_code } => {
                self.handle_process_exit(terminal_id, exit_code);
            }
            TerminalEvent::Paste(text) => {
                self.paste_to_active_terminal(&text).await;
            }
            TerminalEvent::Copy => {
                self.copy_from_active_terminal();
            }
            _ => {}
        }
    }

    async fn create_terminal(&mut self) {
        let cols = 80;  // Default columns
        let rows = 24;  // Default rows

        self.panel_state.update(|p| {
            tokio::spawn(async move {
                let _ = p.new_terminal(cols, rows).await;
            });
        });
    }

    async fn process_output(&self, terminal_id: TerminalId, data: &[u8]) {
        let panel = self.panel_state.get();
        if let Some(terminal) = panel.terminals.iter().find(|t| t.id == terminal_id) {
            // terminal.process_output(data).await;
        }
    }
}
```

---

## State Management

### Reactive State Pattern

```rust
// Integration with editor

use floem::reactive::RwSignal;

pub struct AppState {
    /// Editor state (existing)
    pub editor: RwSignal<EditorState>,

    /// Terminal panel state (new)
    pub terminal_panel: RwSignal<TerminalPanelState>,

    /// Theme (existing)
    pub theme: RwSignal<Theme>,

    /// Font config (existing)
    pub font_config: RwSignal<FontConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            editor: RwSignal::new(EditorState::new()),
            terminal_panel: RwSignal::new(TerminalPanelState::new()),
            theme: RwSignal::new(Theme::dark()),
            font_config: RwSignal::new(FontConfig::default()),
        }
    }
}
```

### State Updates

```rust
// Create new terminal
app_state.terminal_panel.update(|panel| {
    tokio::spawn(async move {
        panel.new_terminal(80, 24).await.unwrap();
    });
});

// Close terminal
app_state.terminal_panel.update(|panel| {
    panel.close_terminal(terminal_id);
});

// Toggle visibility
app_state.terminal_panel.update(|panel| {
    panel.toggle_visibility();
});

// Resize panel
app_state.terminal_panel.update(|panel| {
    panel.set_size(new_size);
});
```

---

## API Design

### Public API

```rust
// src/plugins/terminal/mod.rs

pub mod backend;
pub mod parser;
pub mod buffer;
pub mod state;
pub mod ui;
pub mod config;
pub mod commands;
pub mod events;
pub mod clipboard;

// Re-exports
pub use state::{TerminalState, TerminalPanelState, PanelPosition, TerminalId};
pub use config::TerminalConfig;
pub use events::{TerminalEvent, TerminalEventHandler};
pub use ui::terminal_panel_view;

/// Initialize terminal plugin
pub fn init_terminal_plugin(
    app_state: &mut AppState,
    config: TerminalConfig,
) -> TerminalEventHandler {
    // Create terminal panel state
    let panel_state = RwSignal::new(TerminalPanelState::new());
    app_state.terminal_panel = panel_state;

    // Create event handler
    let event_handler = TerminalEventHandler::new(panel_state);

    // Register terminal commands
    register_terminal_commands(app_state);

    event_handler
}

/// Register terminal commands with the editor
fn register_terminal_commands(app_state: &mut AppState) {
    // Register commands like:
    // - terminal:new
    // - terminal:close
    // - terminal:toggle
    // - terminal:next
    // - terminal:previous
}
```

### Command API

```rust
// src/plugins/terminal/commands.rs

use crate::commands::Command;

/// Terminal-specific commands
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalCommand {
    /// Create new terminal
    New,

    /// Close current terminal
    Close,

    /// Close all terminals
    CloseAll,

    /// Toggle terminal panel
    Toggle,

    /// Show terminal panel
    Show,

    /// Hide terminal panel
    Hide,

    /// Next terminal tab
    NextTab,

    /// Previous terminal tab
    PrevTab,

    /// Switch to terminal N (1-indexed)
    SwitchToTab(usize),

    /// Move terminal panel to bottom
    MoveToBottom,

    /// Move terminal panel to left
    MoveToLeft,

    /// Move terminal panel to right
    MoveToRight,

    /// Increase panel size
    IncreaseSize,

    /// Decrease panel size
    DecreaseSize,

    /// Clear current terminal
    Clear,

    /// Reset current terminal
    Reset,

    /// Split terminal horizontally
    SplitHorizontal,

    /// Split terminal vertically
    SplitVertical,
}

impl TerminalCommand {
    pub async fn execute(&self, panel_state: &mut TerminalPanelState) -> anyhow::Result<()> {
        match self {
            TerminalCommand::New => {
                panel_state.new_terminal(80, 24).await?;
            }
            TerminalCommand::Close => {
                if let Some(term) = panel_state.active() {
                    let id = term.id;
                    panel_state.close_terminal(id);
                }
            }
            TerminalCommand::Toggle => {
                panel_state.toggle_visibility();
            }
            TerminalCommand::NextTab => {
                let next = (panel_state.active_terminal + 1) % panel_state.terminals.len();
                panel_state.switch_to(next);
            }
            TerminalCommand::PrevTab => {
                let prev = if panel_state.active_terminal == 0 {
                    panel_state.terminals.len() - 1
                } else {
                    panel_state.active_terminal - 1
                };
                panel_state.switch_to(prev);
            }
            TerminalCommand::MoveToBottom => {
                panel_state.set_position(PanelPosition::Bottom);
            }
            // ... other commands
        }
        Ok(())
    }
}
```

---

## Configuration

### Terminal Configuration

```rust
// src/plugins/terminal/config.rs

use serde::{Deserialize, Serialize};

/// Terminal plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    /// Default shell (auto-detect if None)
    #[serde(default)]
    pub shell: Option<String>,

    /// Shell arguments
    #[serde(default)]
    pub shell_args: Vec<String>,

    /// Scrollback buffer size (lines)
    #[serde(default = "default_scrollback")]
    pub scrollback_lines: usize,

    /// Terminal font family (uses editor font if None)
    #[serde(default)]
    pub font_family: Option<String>,

    /// Terminal font size
    #[serde(default = "default_font_size")]
    pub font_size: f32,

    /// Line height multiplier
    #[serde(default = "default_line_height")]
    pub line_height: f32,

    /// Cursor blink rate (ms, 0 = no blink)
    #[serde(default = "default_cursor_blink")]
    pub cursor_blink_rate: u64,

    /// Cursor style
    #[serde(default)]
    pub cursor_style: CursorStyleConfig,

    /// Default panel position
    #[serde(default)]
    pub default_position: PanelPositionConfig,

    /// Default panel size (pixels)
    #[serde(default = "default_panel_size")]
    pub default_size: f64,

    /// Enable bell
    #[serde(default = "default_true")]
    pub enable_bell: bool,

    /// Environment variables
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Color scheme
    #[serde(default)]
    pub colors: TerminalColors,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            shell: None,
            shell_args: Vec::new(),
            scrollback_lines: 10_000,
            font_family: None,
            font_size: 14.0,
            line_height: 1.4,
            cursor_blink_rate: 500,
            cursor_style: CursorStyleConfig::Block,
            default_position: PanelPositionConfig::Bottom,
            default_size: 300.0,
            enable_bell: true,
            env: HashMap::new(),
            colors: TerminalColors::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CursorStyleConfig {
    Block,
    Underline,
    Bar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelPositionConfig {
    Bottom,
    Left,
    Right,
}

/// Terminal color scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalColors {
    /// Foreground color
    #[serde(default = "default_foreground")]
    pub foreground: String,

    /// Background color
    #[serde(default = "default_background")]
    pub background: String,

    /// Cursor color
    #[serde(default = "default_cursor")]
    pub cursor: String,

    /// Selection background
    #[serde(default = "default_selection")]
    pub selection: String,

    /// ANSI colors (0-15)
    #[serde(default = "default_ansi_colors")]
    pub ansi: [String; 16],
}

impl Default for TerminalColors {
    fn default() -> Self {
        Self {
            foreground: "#dcdcdc".to_string(),
            background: "#1e1e1e".to_string(),
            cursor: "#ffffff".to_string(),
            selection: "#4682b480".to_string(),
            ansi: default_ansi_colors(),
        }
    }
}

fn default_ansi_colors() -> [String; 16] {
    [
        "#000000".to_string(), // Black
        "#cd3131".to_string(), // Red
        "#0dbc79".to_string(), // Green
        "#e5e510".to_string(), // Yellow
        "#2472c8".to_string(), // Blue
        "#bc3fbc".to_string(), // Magenta
        "#11a8cd".to_string(), // Cyan
        "#e5e5e5".to_string(), // White
        "#666666".to_string(), // Bright Black
        "#f14c4c".to_string(), // Bright Red
        "#23d18b".to_string(), // Bright Green
        "#f5f543".to_string(), // Bright Yellow
        "#3b8eea".to_string(), // Bright Blue
        "#d670d6".to_string(), // Bright Magenta
        "#29b8db".to_string(), // Bright Cyan
        "#e5e5e5".to_string(), // Bright White
    ]
}

// Default value functions
fn default_scrollback() -> usize { 10_000 }
fn default_font_size() -> f32 { 14.0 }
fn default_line_height() -> f32 { 1.4 }
fn default_cursor_blink() -> u64 { 500 }
fn default_panel_size() -> f64 { 300.0 }
fn default_true() -> bool { true }
fn default_foreground() -> String { "#dcdcdc".to_string() }
fn default_background() -> String { "#1e1e1e".to_string() }
fn default_cursor() -> String { "#ffffff".to_string() }
fn default_selection() -> String { "#4682b480".to_string() }
```

### Example TOML Configuration

```toml
# config.toml

[terminal]
# Shell to use (auto-detect if not specified)
# shell = "/bin/zsh"
shell_args = ["-l"]

# Scrollback buffer size
scrollback_lines = 10000

# Font settings (uses editor font if not specified)
# font_family = "JetBrains Mono"
font_size = 14.0
line_height = 1.4

# Cursor settings
cursor_blink_rate = 500  # milliseconds, 0 = no blink
cursor_style = "block"   # block, underline, or bar

# Panel settings
default_position = "bottom"  # bottom, left, or right
default_size = 300.0         # pixels

# Bell
enable_bell = true

# Environment variables
[terminal.env]
EDITOR = "lite-xl"
VISUAL = "lite-xl"

# Color scheme
[terminal.colors]
foreground = "#dcdcdc"
background = "#1e1e1e"
cursor = "#ffffff"
selection = "#4682b480"

# ANSI colors (0-15)
ansi = [
    "#000000", "#cd3131", "#0dbc79", "#e5e510",
    "#2472c8", "#bc3fbc", "#11a8cd", "#e5e5e5",
    "#666666", "#f14c4c", "#23d18b", "#f5f543",
    "#3b8eea", "#d670d6", "#29b8db", "#e5e5e5",
]
```

---

## Keyboard Shortcuts

### Default Keybindings

| Action | Keybinding | Description |
|--------|-----------|-------------|
| Toggle Terminal | `` Ctrl+` `` | Show/hide terminal panel |
| New Terminal | `Ctrl+Shift+T` | Create new terminal tab |
| Close Terminal | `Ctrl+Shift+W` | Close current terminal |
| Next Tab | `Ctrl+Tab` | Switch to next terminal tab |
| Previous Tab | `Ctrl+Shift+Tab` | Switch to previous terminal tab |
| Switch to Tab 1-9 | `Ctrl+1` to `Ctrl+9` | Switch to specific terminal tab |
| Copy | `Ctrl+Shift+C` | Copy selected text |
| Paste | `Ctrl+Shift+V` | Paste from clipboard |
| Clear Terminal | `Ctrl+Shift+K` | Clear current terminal |
| Reset Terminal | `Ctrl+Shift+R` | Reset current terminal |
| Move to Bottom | `Ctrl+Shift+B` | Move panel to bottom |
| Move to Left | `Ctrl+Shift+L` | Move panel to left |
| Move to Right | `Ctrl+Shift+Right` | Move panel to right |
| Increase Size | `Ctrl+Shift+=` | Increase panel size |
| Decrease Size | `Ctrl+Shift+-` | Decrease panel size |
| Scroll Up | `Shift+PageUp` | Scroll back in terminal |
| Scroll Down | `Shift+PageDown` | Scroll forward in terminal |

### Terminal Input Keybindings

| Key | Escape Sequence | Description |
|-----|----------------|-------------|
| Enter | `\r` | Carriage return |
| Backspace | `\x7f` | Delete (DEL) |
| Tab | `\t` | Tab character |
| Arrow Up | `\x1b[A` | Cursor up |
| Arrow Down | `\x1b[B` | Cursor down |
| Arrow Right | `\x1b[C` | Cursor right |
| Arrow Left | `\x1b[D` | Cursor left |
| Home | `\x1b[H` | Cursor to start |
| End | `\x1b[F` | Cursor to end |
| Page Up | `\x1b[5~` | Page up |
| Page Down | `\x1b[6~` | Page down |
| F1-F12 | `\x1bOP` - `\x1b[24~` | Function keys |
| Ctrl+C | `\x03` | Interrupt (ETX) |
| Ctrl+D | `\x04` | End of transmission |
| Ctrl+Z | `\x1a` | Suspend |

---

## Implementation Plan

### Phase 1: Core Backend (Week 1-2)

1. **PTY Implementation**
   - [ ] Create `backend/pty.rs` with platform abstraction
   - [ ] Implement Unix PTY using `libc` (Linux/macOS)
   - [ ] Implement Windows ConPTY using `winapi`
   - [ ] Add tests for PTY creation and I/O

2. **Process Management**
   - [ ] Implement `backend/process.rs` for shell spawning
   - [ ] Add `backend/shell.rs` for shell detection
   - [ ] Support bash, zsh, fish, PowerShell
   - [ ] Handle process lifecycle (spawn, kill, exit)

3. **ANSI Parser**
   - [ ] Create `parser/ansi.rs` for escape sequence parsing
   - [ ] Implement `parser/colors.rs` for color conversion
   - [ ] Add `parser/csi.rs` for CSI sequences
   - [ ] Support common ANSI codes (cursor, color, erase)

### Phase 2: Terminal Buffer (Week 3)

1. **Grid and Cell**
   - [ ] Implement `buffer/cell.rs` with attributes
   - [ ] Create `buffer/grid.rs` for 2D cell array
   - [ ] Add grid resizing and scrolling

2. **Scrollback Buffer**
   - [ ] Implement `buffer/scrollback.rs` with circular buffer
   - [ ] Support configurable scrollback size
   - [ ] Add scroll navigation

3. **Cursor Management**
   - [ ] Create `buffer/cursor.rs` for cursor state
   - [ ] Support multiple cursor styles
   - [ ] Handle cursor visibility and blinking

### Phase 3: State Management (Week 4)

1. **Terminal State**
   - [ ] Implement `state/terminal.rs` for single terminal
   - [ ] Add async I/O processing
   - [ ] Connect PTY output to ANSI parser
   - [ ] Update grid based on parsed actions

2. **Panel State**
   - [ ] Create `state/panel.rs` for multi-terminal management
   - [ ] Add tab management
   - [ ] Support panel positioning and sizing
   - [ ] Implement visibility toggle

### Phase 4: UI Components (Week 5-6)

1. **Terminal View**
   - [ ] Create `ui/terminal_view.rs` for rendering
   - [ ] Render grid with proper styling
   - [ ] Display cursor with correct style
   - [ ] Handle keyboard input

2. **Tab Bar**
   - [ ] Implement `ui/tab_bar.rs` for tab management
   - [ ] Show terminal titles
   - [ ] Support tab switching
   - [ ] Add new tab / close tab buttons

3. **Panel Container**
   - [ ] Create `ui/panel.rs` for panel container
   - [ ] Support resizable panel (drag handle)
   - [ ] Handle different panel positions
   - [ ] Integrate with main editor layout

4. **Theme Integration**
   - [ ] Add `ui/theme.rs` for terminal colors
   - [ ] Support ANSI color palette
   - [ ] Match editor theme

### Phase 5: Integration (Week 7)

1. **Configuration**
   - [ ] Add terminal config to `config/mod.rs`
   - [ ] Support TOML configuration
   - [ ] Add default values

2. **Commands**
   - [ ] Register terminal commands
   - [ ] Add command implementations
   - [ ] Integrate with editor command system

3. **Event Handling**
   - [ ] Create `events.rs` for terminal events
   - [ ] Handle terminal lifecycle events
   - [ ] Support clipboard operations

4. **Main Integration**
   - [ ] Update `main.rs` to include terminal panel
   - [ ] Add terminal to app layout
   - [ ] Wire up keyboard shortcuts

### Phase 6: Polish & Testing (Week 8)

1. **Testing**
   - [ ] Unit tests for all modules
   - [ ] Integration tests
   - [ ] Cross-platform testing (Linux, macOS, Windows)
   - [ ] Performance testing

2. **Documentation**
   - [ ] API documentation
   - [ ] User guide
   - [ ] Configuration examples
   - [ ] Troubleshooting guide

3. **Performance Optimization**
   - [ ] Optimize grid rendering
   - [ ] Reduce allocations
   - [ ] Profile and optimize hot paths

---

## Dependencies

### Required Crates

```toml
[dependencies]
# Existing dependencies
floem = "0.2"
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
thiserror = "1.0"
anyhow = "1.0"

# Terminal-specific dependencies
# PTY support
[target.'cfg(unix)'.dependencies]
libc = "0.2"
nix = { version = "0.27", features = ["process", "pty"] }

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["wincon", "winbase", "processthreadsapi"] }
windows = { version = "0.52", features = ["Win32_System_Console"] }

# ANSI parsing
vte = "0.13"  # VT emulator parser

# Optional: for better terminal emulation
[dependencies.portable-pty]
version = "0.8"
optional = true

# Bitflags for cell attributes
bitflags = "2.4"
```

### Dependency Justification

- **nix**: Unix PTY management (Linux, macOS)
- **winapi/windows**: Windows ConPTY support
- **vte**: Battle-tested VT emulator parser (used by Alacritty)
- **portable-pty**: Cross-platform PTY abstraction (optional, simpler API)
- **bitflags**: Efficient cell attribute flags

---

## Summary

This design provides a comprehensive, production-ready terminal plugin architecture for the Lite XL Rust text editor. The design:

1. **Integrates seamlessly** with existing Floem reactive architecture
2. **Supports all requirements**: Multiple tabs, docking, ANSI colors, scrollback, clipboard, shell integration, cross-platform
3. **Follows existing patterns**: Module structure, event handling, configuration, state management
4. **Is extensible**: Plugin API for future enhancements
5. **Is performant**: Reactive updates, efficient rendering, minimal allocations
6. **Is well-documented**: Clear API, comprehensive examples, implementation plan

The implementation can be completed in approximately 8 weeks with proper testing and documentation.

---

## Next Steps

1. Review this design document with the team
2. Approve dependency additions to `Cargo.toml`
3. Begin Phase 1 implementation (Core Backend)
4. Set up CI/CD for cross-platform testing
5. Create tracking issues for each phase

---

**Document Status**: Draft v1.0
**Last Updated**: 2025-11-15
**Author**: Terminal Plugin Architecture Team
**Review Status**: Pending Review

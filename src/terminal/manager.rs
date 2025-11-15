//! Terminal manager - manages multiple terminals

use crate::terminal::backend::{Backend, BackendError};
use crate::terminal::buffer::{Cell, Cursor, Grid, Scrollback};
use crate::terminal::clipboard::{SelectionMode, TerminalClipboard};
use crate::terminal::config::TerminalConfig;
use crate::terminal::parser::{AnsiParser, TerminalAction};
use std::sync::atomic::{AtomicUsize, Ordering};

static TERMINAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

/// Unique terminal identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(usize);

impl TerminalId {
    fn next() -> Self {
        Self(TERMINAL_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

/// A single terminal instance
pub struct Terminal {
    /// Unique identifier
    pub id: TerminalId,

    /// Terminal title
    pub title: String,

    /// Grid buffer
    pub grid: Grid,

    /// Scrollback buffer
    pub scrollback: Scrollback,

    /// Cursor position
    pub cursor: Cursor,

    /// ANSI parser
    parser: AnsiParser,

    /// Clipboard manager
    clipboard: TerminalClipboard,

    /// Backend handle
    backend: Option<Backend>,

    /// Whether the terminal is still running
    pub running: bool,

    /// Scroll offset (0 = at bottom, positive = scrolled up)
    scroll_offset: usize,
}

impl Terminal {
    /// Create a new terminal
    pub async fn new(config: &TerminalConfig) -> Result<Self, BackendError> {
        let grid = Grid::new(config.rows as usize, config.cols as usize);
        let scrollback = Scrollback::new(config.scrollback_lines, config.cols as usize);
        let cursor = Cursor::new();
        let parser = AnsiParser::new();
        let clipboard = TerminalClipboard::new();

        // Try to create backend, but don't fail if PTY is not available
        use crate::terminal::backend::process::{ShellConfig, ShellType};
        let shell_config = ShellConfig::with_shell(ShellType::from_path(&config.shell))
            .args(config.shell_args.clone());
        let backend = Backend::new(shell_config, config.rows, config.cols, config.scrollback_lines).ok();

        let id = TerminalId::next();
        let title = format!("Terminal {}", id.0);

        Ok(Self {
            id,
            title,
            grid,
            scrollback,
            cursor,
            parser,
            clipboard,
            backend,
            running: true,
            scroll_offset: 0,
        })
    }

    /// Process output from PTY
    pub fn process_output(&mut self, data: &[u8]) {
        let actions = self.parser.parse(data);
        for action in actions {
            self.apply_action(action);
        }
    }

    /// Apply a parsed action
    fn apply_action(&mut self, action: TerminalAction) {
        match action {
            TerminalAction::Print(ch) => {
                let cell = Cell::new(ch);
                self.grid.set(self.cursor.row, self.cursor.col, cell);
                self.cursor.forward(1, self.grid.cols() - 1);

                // Handle line wrap
                if self.cursor.col >= self.grid.cols() {
                    self.cursor.col = 0;
                    self.cursor.newline();

                    // Scroll if needed
                    if self.cursor.row >= self.grid.rows() {
                        self.scroll_up();
                    }
                }
            }
            TerminalAction::Execute(_) => {
                // Control characters are handled by specific actions
            }
            TerminalAction::CarriageReturn => {
                self.cursor.carriage_return();
            }
            TerminalAction::LineFeed => {
                self.cursor.newline();
                if self.cursor.row >= self.grid.rows() {
                    self.scroll_up();
                }
            }
            TerminalAction::Tab => {
                // Tab: move to next tab stop (every 8 columns)
                let next_tab = ((self.cursor.col / 8) + 1) * 8;
                self.cursor.col = next_tab.min(self.grid.cols() - 1);
            }
            TerminalAction::Backspace => {
                self.cursor.backward(1);
            }
            TerminalAction::CursorGoTo { line, col } => {
                self.cursor.move_to(
                    (line.saturating_sub(1)).min(self.grid.rows() - 1),
                    (col.saturating_sub(1)).min(self.grid.cols() - 1),
                );
            }
            TerminalAction::CursorUp(n) => {
                self.cursor.up(n);
            }
            TerminalAction::CursorDown(n) => {
                self.cursor.down(n);
            }
            TerminalAction::CursorForward(n) => {
                self.cursor.forward(n, self.grid.cols() - 1);
            }
            TerminalAction::CursorBackward(n) => {
                self.cursor.backward(n);
            }
            TerminalAction::ClearScreen => {
                self.grid.clear();
                self.cursor.move_to(0, 0);
            }
            TerminalAction::ClearLine => {
                self.grid.clear_row(self.cursor.row);
            }
            TerminalAction::ClearToEndOfLine => {
                if let Some(row) = self.grid.row(self.cursor.row) {
                    for col in self.cursor.col..row.len() {
                        self.grid.set(self.cursor.row, col, Cell::default());
                    }
                }
            }
            TerminalAction::ClearToBeginningOfLine => {
                for col in 0..=self.cursor.col {
                    self.grid.set(self.cursor.row, col, Cell::default());
                }
            }
            TerminalAction::SetTitle(title) => {
                self.title = title;
            }
            // Handle other terminal actions but don't implement full functionality yet
            _ => {}
        }
    }

    /// Scroll grid up by one line
    fn scroll_up(&mut self) {
        // Save top line to scrollback
        if let Some(line) = self.grid.row(0) {
            self.scrollback.push_line(line.to_vec());
        }

        // Scroll the grid
        self.grid.scroll_up();

        // Keep cursor at bottom row
        self.cursor.row = self.grid.rows() - 1;
    }

    /// Send input to terminal
    pub async fn send_input(&mut self, data: &[u8]) -> Result<(), BackendError> {
        if let Some(backend) = &mut self.backend {
            backend.write(data)?;
        }
        Ok(())
    }

    /// Clear the terminal
    pub fn clear(&mut self) {
        self.grid.clear();
        self.scrollback.clear();
        self.cursor.move_to(0, 0);
    }

    /// Resize the terminal
    pub async fn resize(&mut self, cols: u16, rows: u16) -> Result<(), BackendError> {
        self.grid.resize(cols as usize, rows as usize);
        if let Some(backend) = &mut self.backend {
            backend.resize(cols, rows)?;
        }
        Ok(())
    }

    /// Kill the terminal
    pub fn kill(&mut self) {
        self.running = false;
        self.backend = None;
    }

    /// Get scroll offset
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Set scroll offset
    pub fn set_scroll_offset(&mut self, offset: usize) {
        self.scroll_offset = offset.min(self.scrollback.len());
    }

    /// Scroll up by n lines
    pub fn scroll_up_by(&mut self, n: usize) {
        self.scroll_offset = (self.scroll_offset + n).min(self.scrollback.len());
    }

    /// Scroll down by n lines
    pub fn scroll_down_by(&mut self, n: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(n);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = self.scrollback.len();
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    /// Start selection
    pub fn start_selection(&mut self, row: usize, col: usize, mode: SelectionMode) {
        self.clipboard.start_selection(row, col, mode);
    }

    /// Update selection
    pub fn update_selection(&mut self, row: usize, col: usize) {
        self.clipboard.update_selection(row, col);
    }

    /// Finalize selection
    pub fn finalize_selection(&mut self) {
        self.clipboard.finalize_selection();
    }

    /// Copy selection to clipboard
    pub fn copy_selection(&mut self) -> String {
        self.clipboard.copy_selection(&self.grid)
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.clipboard.clear_selection();
    }

    /// Get clipboard reference
    pub fn clipboard(&self) -> &TerminalClipboard {
        &self.clipboard
    }

    /// Get mutable clipboard reference
    pub fn clipboard_mut(&mut self) -> &mut TerminalClipboard {
        &mut self.clipboard
    }
}

/// Manager for multiple terminals
pub struct TerminalManager {
    /// Configuration
    config: TerminalConfig,

    /// All terminals
    terminals: Vec<Terminal>,

    /// Index of current terminal
    current_index: usize,

    /// Whether terminal panel is visible
    visible: bool,
}

impl TerminalManager {
    /// Create a new terminal manager
    pub fn new(config: TerminalConfig) -> Self {
        Self {
            config,
            terminals: Vec::new(),
            current_index: 0,
            visible: false,
        }
    }

    /// Create a new terminal
    pub async fn create_terminal(&mut self) -> Result<TerminalId, BackendError> {
        let terminal = Terminal::new(&self.config).await?;
        let id = terminal.id;
        self.terminals.push(terminal);
        self.current_index = self.terminals.len() - 1;
        self.visible = true;
        Ok(id)
    }

    /// Get current terminal
    pub fn current(&self) -> Option<&Terminal> {
        self.terminals.get(self.current_index)
    }

    /// Get mutable current terminal
    pub fn current_mut(&mut self) -> Option<&mut Terminal> {
        self.terminals.get_mut(self.current_index)
    }

    /// Get terminal by ID
    pub fn get(&self, id: TerminalId) -> Option<&Terminal> {
        self.terminals.iter().find(|t| t.id == id)
    }

    /// Get mutable terminal by ID
    pub fn get_mut(&mut self, id: TerminalId) -> Option<&mut Terminal> {
        self.terminals.iter_mut().find(|t| t.id == id)
    }

    /// Get all terminals
    pub fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    /// Get terminal count
    pub fn terminal_count(&self) -> usize {
        self.terminals.len()
    }

    /// Close current terminal
    pub fn close_current(&mut self) {
        if !self.terminals.is_empty() {
            self.terminals.remove(self.current_index);
            if self.current_index >= self.terminals.len() && self.current_index > 0 {
                self.current_index -= 1;
            }
            if self.terminals.is_empty() {
                self.visible = false;
            }
        }
    }

    /// Close all terminals
    pub fn close_all(&mut self) {
        self.terminals.clear();
        self.current_index = 0;
        self.visible = false;
    }

    /// Switch to next terminal
    pub fn next_terminal(&mut self) {
        if !self.terminals.is_empty() {
            self.current_index = (self.current_index + 1) % self.terminals.len();
        }
    }

    /// Switch to previous terminal
    pub fn previous_terminal(&mut self) {
        if !self.terminals.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.terminals.len() - 1
            } else {
                self.current_index - 1
            };
        }
    }

    /// Switch to specific terminal by index (1-based)
    pub fn switch_to(&mut self, index: usize) {
        if index > 0 && index <= self.terminals.len() {
            self.current_index = index - 1;
        }
    }

    /// Toggle visibility
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    /// Show terminal panel
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide terminal panel
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if terminal panel is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Clear current terminal
    pub fn clear_current(&mut self) {
        if let Some(terminal) = self.current_mut() {
            terminal.clear();
        }
    }

    /// Kill current terminal process
    pub async fn kill_current(&mut self) {
        if let Some(terminal) = self.current_mut() {
            terminal.kill();
        }
    }

    /// Send input to current terminal
    pub async fn send_input_to_current(&mut self, data: &[u8]) -> Result<(), BackendError> {
        if let Some(terminal) = self.current_mut() {
            terminal.send_input(data).await?;
        }
        Ok(())
    }

    /// Get current terminal index
    pub fn current_index(&self) -> usize {
        self.current_index
    }
}

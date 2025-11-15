//! Terminal state management
//!
//! This module provides a thread-safe terminal state wrapper that combines:
//! - Grid buffer, cursor, and scrollback
//! - ANSI parser for escape sequences
//! - PTY backend for process I/O
//! - Clipboard operations for selection
//! - Async update loop for reading from PTY

use crate::terminal::backend::{Backend, BackendError};
use crate::terminal::buffer::{Cell, Cursor, Grid, Scrollback};
use crate::terminal::clipboard::{SelectionMode, TerminalClipboard};
use crate::terminal::config::TerminalConfig;
use crate::terminal::parser::{AnsiParser, TerminalAction};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

/// Terminal state update event
#[derive(Debug, Clone)]
pub enum TerminalEvent {
    /// Output received from PTY
    Output(Vec<u8>),
    /// Terminal was resized
    Resized { cols: u16, rows: u16 },
    /// Terminal process exited
    Exited,
    /// Title changed
    TitleChanged(String),
    /// Bell triggered
    Bell,
}

/// Thread-safe terminal state
pub struct TerminalState {
    /// Unique identifier
    pub id: usize,

    /// Terminal title
    title: String,

    /// Grid buffer
    grid: Grid,

    /// Scrollback buffer
    scrollback: Scrollback,

    /// Cursor position
    cursor: Cursor,

    /// ANSI parser
    parser: AnsiParser,

    /// Clipboard manager
    clipboard: TerminalClipboard,

    /// Backend handle
    backend: Option<Backend>,

    /// Configuration
    config: TerminalConfig,

    /// Whether the terminal is still running
    running: bool,

    /// Scroll offset (0 = at bottom, positive = scrolled up)
    scroll_offset: usize,
}

impl TerminalState {
    /// Create a new terminal state
    pub async fn new(id: usize, config: TerminalConfig) -> Result<Self, BackendError> {
        let grid = Grid::new(config.cols as usize, config.rows as usize);
        let scrollback = Scrollback::new(config.scrollback_lines, config.cols as usize);
        let cursor = Cursor::new();
        let parser = AnsiParser::new();
        let clipboard = TerminalClipboard::new();

        // Try to create backend
        use crate::terminal::backend::process::{ShellConfig, ShellType};
        let shell_config = ShellConfig::with_shell(ShellType::from_path(&config.shell))
            .args(config.shell_args.clone());
        let backend = Backend::new(shell_config, config.rows, config.cols, config.scrollback_lines).ok();

        let title = format!("Terminal {}", id);

        Ok(Self {
            id,
            title,
            grid,
            scrollback,
            cursor,
            parser,
            clipboard,
            backend,
            config,
            running: true,
            scroll_offset: 0,
        })
    }

    /// Get terminal title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set terminal title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Get grid reference
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Get cursor reference
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Get scrollback reference
    pub fn scrollback(&self) -> &Scrollback {
        &self.scrollback
    }

    /// Get clipboard reference
    pub fn clipboard(&self) -> &TerminalClipboard {
        &self.clipboard
    }

    /// Get mutable clipboard reference
    pub fn clipboard_mut(&mut self) -> &mut TerminalClipboard {
        &mut self.clipboard
    }

    /// Check if terminal is running
    pub fn is_running(&self) -> bool {
        self.running
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
                // Other cursor movement handled by other actions
            }
            TerminalAction::Execute(_) => {
                // Control characters are handled by specific actions
            }
            TerminalAction::CarriageReturn => {
                self.cursor.carriage_return();
            }
            TerminalAction::LineFeed => {
                // Terminal state handles line feed
            }
            TerminalAction::Tab => {
                // Tab handling
            }
            TerminalAction::Backspace => {
                // Backspace handling
            }
            TerminalAction::CursorGoTo { line, col } => {
                // Cursor positioning - note: line and col are 1-indexed
            }
            TerminalAction::CursorUp(_n) => {
                // Cursor up
            }
            TerminalAction::CursorDown(_n) => {
                // Cursor down
            }
            TerminalAction::CursorForward(_n) => {
                // Cursor forward
            }
            TerminalAction::CursorBackward(_n) => {
                // Cursor backward
            }
            TerminalAction::ClearScreen => {
                self.grid.clear();
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
            // Handle other actions
            _ => {}
        }

        // Reset scroll offset when new output is received
        if matches!(action, TerminalAction::Print(_) | TerminalAction::Execute(_)) {
            self.scroll_offset = 0;
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
        self.scroll_offset = 0;
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

    /// Get backend reference
    pub fn backend(&self) -> Option<&Backend> {
        self.backend.as_ref()
    }
}

/// Shared terminal state (thread-safe)
pub type SharedTerminalState = Arc<RwLock<TerminalState>>;

/// Terminal state with async update loop
pub struct TerminalStateHandle {
    /// Shared state
    state: SharedTerminalState,

    /// Update task handle
    update_task: Option<JoinHandle<()>>,

    /// Event sender
    event_tx: Option<mpsc::UnboundedSender<TerminalEvent>>,
}

impl TerminalStateHandle {
    /// Create a new terminal state handle
    pub async fn new(id: usize, config: TerminalConfig) -> Result<Self, BackendError> {
        let state = Arc::new(RwLock::new(TerminalState::new(id, config).await?));

        Ok(Self {
            state,
            update_task: None,
            event_tx: None,
        })
    }

    /// Start async update loop
    pub fn start_update_loop(&mut self) -> mpsc::UnboundedReceiver<TerminalEvent> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        // Note: The backend's event system should be used instead of a polling loop.
        // For now, we just create the channel without a background task.
        // TODO: Integrate with backend's event channel properly to forward events.

        self.event_tx = Some(event_tx);

        event_rx
    }

    /// Get shared state
    pub fn state(&self) -> SharedTerminalState {
        self.state.clone()
    }

    /// Stop update loop
    pub fn stop_update_loop(&mut self) {
        if let Some(task) = self.update_task.take() {
            task.abort();
        }
        self.event_tx = None;
    }
}

impl Drop for TerminalStateHandle {
    fn drop(&mut self) {
        self.stop_update_loop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_terminal_state_creation() {
        let config = TerminalConfig::default();
        let state = TerminalState::new(1, config).await.unwrap();

        assert_eq!(state.id, 1);
        assert!(state.is_running());
        assert_eq!(state.scroll_offset(), 0);
    }

    #[tokio::test]
    async fn test_terminal_state_scroll() {
        let config = TerminalConfig::default();
        let mut state = TerminalState::new(1, config).await.unwrap();

        state.scroll_up_by(5);
        assert_eq!(state.scroll_offset(), 0); // No scrollback yet

        state.scroll_to_bottom();
        assert_eq!(state.scroll_offset(), 0);
    }

    #[tokio::test]
    async fn test_terminal_state_selection() {
        let config = TerminalConfig::default();
        let mut state = TerminalState::new(1, config).await.unwrap();

        state.start_selection(0, 0, SelectionMode::Normal);
        assert!(state.clipboard().has_selection());

        state.clear_selection();
        assert!(!state.clipboard().has_selection());
    }

    #[tokio::test]
    async fn test_terminal_state_clear() {
        let config = TerminalConfig::default();
        let mut state = TerminalState::new(1, config).await.unwrap();

        state.clear();
        assert_eq!(state.cursor().row, 0);
        assert_eq!(state.cursor().col, 0);
    }

    #[tokio::test]
    async fn test_terminal_state_handle() {
        let config = TerminalConfig::default();
        let handle = TerminalStateHandle::new(1, config).await.unwrap();

        let state = handle.state();
        let state_guard = state.read().await;
        assert_eq!(state_guard.id, 1);
    }
}

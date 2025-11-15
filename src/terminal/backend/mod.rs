//! Terminal backend implementation.
//!
//! This module provides the backend infrastructure for terminal emulation,
//! including PTY management and shell process handling.

pub mod process;
pub mod pty;

pub use process::{detect_available_shells, ShellConfig, ShellType};
pub use pty::{Pty, PtyError, PtyResult};

#[cfg(feature = "pty")]
pub use pty::read_pty_async;

// Type aliases for compatibility with existing code
pub type Backend = TerminalBackend;
pub type BackendError = PtyError;

use super::buffer::TerminalBuffer;
use tokio::sync::mpsc;

/// Events emitted by the terminal backend.
#[derive(Debug, Clone)]
pub enum TerminalEvent {
    /// Data received from the PTY.
    Data(Vec<u8>),
    /// The terminal process has exited with the given exit code.
    Exit(Option<i32>),
    /// An error occurred in the backend.
    Error(String),
    /// The terminal was resized.
    Resize { rows: u16, cols: u16 },
}

/// Terminal backend that manages PTY and shell process.
///
/// This combines the PTY, shell process management, and terminal buffer
/// into a unified interface for terminal emulation.
pub struct TerminalBackend {
    /// The PTY instance.
    #[cfg(feature = "pty")]
    pty: Pty,
    /// The terminal buffer.
    buffer: TerminalBuffer,
    /// Channel for receiving terminal events.
    event_rx: mpsc::UnboundedReceiver<TerminalEvent>,
    /// Channel for sending terminal events (kept for cloning).
    event_tx: mpsc::UnboundedSender<TerminalEvent>,
    /// Current terminal size.
    size: (u16, u16), // (rows, cols)
}

impl TerminalBackend {
    /// Creates a new terminal backend with the given configuration.
    ///
    /// # Arguments
    /// * `config` - Shell configuration
    /// * `rows` - Number of rows
    /// * `cols` - Number of columns
    /// * `scrollback_lines` - Maximum scrollback lines
    #[cfg(feature = "pty")]
    pub fn new(
        config: ShellConfig,
        rows: u16,
        cols: u16,
        scrollback_lines: usize,
    ) -> PtyResult<Self> {
        let pty = Pty::spawn(
            &config.command(),
            &config.args_as_strs(),
            config.cwd_as_str(),
            rows,
            cols,
        )?;

        let buffer = TerminalBuffer::new(rows as usize, cols as usize, scrollback_lines);
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Ok(Self {
            pty,
            buffer,
            event_rx,
            event_tx,
            size: (rows, cols),
        })
    }

    #[cfg(not(feature = "pty"))]
    pub fn new(
        _config: ShellConfig,
        rows: u16,
        cols: u16,
        scrollback_lines: usize,
    ) -> PtyResult<Self> {
        let buffer = TerminalBuffer::new(rows as usize, cols as usize, scrollback_lines);
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Ok(Self {
            buffer,
            event_rx,
            event_tx,
            size: (rows, cols),
        })
    }

    /// Returns a reference to the terminal buffer.
    pub fn buffer(&self) -> &TerminalBuffer {
        &self.buffer
    }

    /// Returns a mutable reference to the terminal buffer.
    pub fn buffer_mut(&mut self) -> &mut TerminalBuffer {
        &mut self.buffer
    }

    /// Returns the current terminal size as (rows, cols).
    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    /// Writes data to the PTY.
    #[cfg(feature = "pty")]
    pub fn write(&mut self, data: &[u8]) -> PtyResult<usize> {
        self.pty.write(data)
    }

    #[cfg(not(feature = "pty"))]
    pub fn write(&mut self, _data: &[u8]) -> PtyResult<usize> {
        Err(PtyError::NotEnabled)
    }

    /// Flushes the PTY writer.
    #[cfg(feature = "pty")]
    pub fn flush(&mut self) -> PtyResult<()> {
        self.pty.flush()
    }

    #[cfg(not(feature = "pty"))]
    pub fn flush(&mut self) -> PtyResult<()> {
        Err(PtyError::NotEnabled)
    }

    /// Resizes the terminal.
    #[cfg(feature = "pty")]
    pub fn resize(&mut self, rows: u16, cols: u16) -> PtyResult<()> {
        self.pty.resize(rows, cols)?;
        self.buffer.resize(rows as usize, cols as usize);
        self.size = (rows, cols);

        // Emit resize event
        let _ = self.event_tx.send(TerminalEvent::Resize { rows, cols });

        Ok(())
    }

    #[cfg(not(feature = "pty"))]
    pub fn resize(&mut self, rows: u16, cols: u16) -> PtyResult<()> {
        self.buffer.resize(rows as usize, cols as usize);
        self.size = (rows, cols);
        let _ = self.event_tx.send(TerminalEvent::Resize { rows, cols });
        Ok(())
    }

    /// Checks if the terminal process is still alive.
    #[cfg(feature = "pty")]
    pub fn is_alive(&mut self) -> bool {
        self.pty.is_alive()
    }

    #[cfg(not(feature = "pty"))]
    pub fn is_alive(&mut self) -> bool {
        false
    }

    /// Tries to receive a terminal event without blocking.
    pub fn try_recv_event(&mut self) -> Option<TerminalEvent> {
        self.event_rx.try_recv().ok()
    }

    /// Receives a terminal event, waiting asynchronously.
    pub async fn recv_event(&mut self) -> Option<TerminalEvent> {
        self.event_rx.recv().await
    }

    /// Gets a sender for terminal events (for async PTY reading).
    pub fn event_sender(&self) -> mpsc::UnboundedSender<TerminalEvent> {
        self.event_tx.clone()
    }

    /// Starts reading from the PTY asynchronously.
    ///
    /// This spawns a background task that reads from the PTY and sends
    /// data events to the event channel.
    #[cfg(feature = "pty")]
    pub fn start_reading(&self) {
        let reader = self.pty.clone_reader();
        let event_tx = self.event_tx.clone();

        tokio::spawn(async move {
            // Create a channel for raw data
            let (data_tx, mut data_rx) = mpsc::unbounded_channel();

            // Spawn the raw PTY reader
            tokio::spawn(async move {
                read_pty_async(reader, data_tx).await;
            });

            // Forward data as events
            while let Some(data) = data_rx.recv().await {
                let _ = event_tx.send(TerminalEvent::Data(data));
            }
        });
    }

    #[cfg(not(feature = "pty"))]
    pub fn start_reading(&self) {
        // No-op when PTY is disabled
    }

    /// Waits for the terminal process to exit.
    #[cfg(feature = "pty")]
    pub fn wait(&mut self) -> PtyResult<portable_pty::ExitStatus> {
        // Note: This is synchronous. In a real async context, you'd want to
        // handle this more carefully to avoid blocking the executor.
        self.pty.wait()
    }

    #[cfg(not(feature = "pty"))]
    pub fn wait(&mut self) -> PtyResult<portable_pty::ExitStatus> {
        Err(PtyError::NotEnabled)
    }
}

#[cfg(test)]
#[cfg(feature = "pty")]
mod tests {
    use super::*;

    #[test]
    fn test_backend_new() {
        let config = ShellConfig::default();

        #[cfg(unix)]
        let backend = TerminalBackend::new(config, 24, 80, 1000);

        #[cfg(windows)]
        let backend = TerminalBackend::new(config, 24, 80, 1000);

        #[cfg(any(unix, windows))]
        assert!(backend.is_ok());
    }

    #[test]
    fn test_backend_size() {
        let config = ShellConfig::default();

        #[cfg(unix)]
        let backend = TerminalBackend::new(config, 24, 80, 1000);

        #[cfg(windows)]
        let backend = TerminalBackend::new(config, 24, 80, 1000);

        #[cfg(any(unix, windows))]
        if let Ok(backend) = backend {
            assert_eq!(backend.size(), (24, 80));
        }
    }

    #[tokio::test]
    async fn test_backend_write() {
        let config = ShellConfig::default();

        #[cfg(unix)]
        let backend = TerminalBackend::new(config, 24, 80, 1000);

        #[cfg(windows)]
        let backend = TerminalBackend::new(config, 24, 80, 1000);

        #[cfg(any(unix, windows))]
        if let Ok(mut backend) = backend {
            let result = backend.write(b"echo test\n");
            assert!(result.is_ok());
        }
    }
}

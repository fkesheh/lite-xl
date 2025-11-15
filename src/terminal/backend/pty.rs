//! PTY (pseudo-terminal) handling using portable-pty.
//!
//! This module provides cross-platform PTY support for spawning and managing
//! terminal processes on Unix and Windows (ConPTY).

#[cfg(feature = "pty")]
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize, PtySystem};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::mpsc;

/// Errors that can occur when working with PTYs.
#[derive(Debug, Error)]
pub enum PtyError {
    /// Failed to spawn PTY.
    #[error("Failed to spawn PTY: {0}")]
    Spawn(String),

    /// Failed to read from PTY.
    #[error("Failed to read from PTY: {0}")]
    Read(#[from] io::Error),

    /// Failed to write to PTY.
    #[error("Failed to write to PTY: {0}")]
    Write(String),

    /// Failed to resize PTY.
    #[error("Failed to resize PTY: {0}")]
    Resize(String),

    /// PTY process has exited.
    #[error("PTY process has exited")]
    Exited,

    /// PTY feature is not enabled.
    #[error("PTY feature is not enabled in this build")]
    NotEnabled,
}

/// Result type for PTY operations.
pub type PtyResult<T> = Result<T, PtyError>;

/// A PTY instance wrapping portable-pty functionality.
#[cfg(feature = "pty")]
pub struct Pty {
    /// The PTY master (for reading/writing).
    master: Box<dyn MasterPty + Send>,
    /// The child process.
    child: Box<dyn Child + Send>,
    /// PTY reader (wrapped in Arc<Mutex> for thread safety).
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    /// PTY writer (wrapped in Arc<Mutex> for thread safety).
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

#[cfg(feature = "pty")]
impl Pty {
    /// Spawns a new PTY with the given command and environment.
    ///
    /// # Arguments
    /// * `cmd` - The command to execute (e.g., "bash", "zsh", "powershell")
    /// * `args` - Command arguments
    /// * `cwd` - Working directory (None for current directory)
    /// * `rows` - Initial number of rows
    /// * `cols` - Initial number of columns
    pub fn spawn(
        cmd: &str,
        args: &[&str],
        cwd: Option<&str>,
        rows: u16,
        cols: u16,
    ) -> PtyResult<Self> {
        let pty_system = native_pty_system();

        // Create the PTY
        let pty_pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| PtyError::Spawn(e.to_string()))?;

        // Build the command
        let mut command = CommandBuilder::new(cmd);
        for arg in args {
            command.arg(arg);
        }
        if let Some(dir) = cwd {
            command.cwd(dir);
        }

        // Spawn the child process
        let child = pty_pair
            .slave
            .spawn_command(command)
            .map_err(|e| PtyError::Spawn(e.to_string()))?;

        let reader = pty_pair.master.try_clone_reader().map_err(|e| PtyError::Spawn(e.to_string()))?;
        let writer = pty_pair.master.take_writer().map_err(|e| PtyError::Spawn(e.to_string()))?;

        Ok(Self {
            master: pty_pair.master,
            child,
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        })
    }

    /// Reads data from the PTY into the provided buffer.
    ///
    /// Returns the number of bytes read, or 0 if EOF.
    pub fn read(&mut self, buf: &mut [u8]) -> PtyResult<usize> {
        let mut reader = self.reader.lock().unwrap();
        reader.read(buf).map_err(PtyError::Read)
    }

    /// Writes data to the PTY.
    pub fn write(&mut self, data: &[u8]) -> PtyResult<usize> {
        let mut writer = self.writer.lock().unwrap();
        writer.write(data).map_err(|e| PtyError::Write(e.to_string()))
    }

    /// Flushes the PTY writer.
    pub fn flush(&mut self) -> PtyResult<()> {
        let mut writer = self.writer.lock().unwrap();
        writer.flush().map_err(|e| PtyError::Write(e.to_string()))
    }

    /// Resizes the PTY to the new dimensions.
    pub fn resize(&self, rows: u16, cols: u16) -> PtyResult<()> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| PtyError::Resize(e.to_string()))
    }

    /// Checks if the child process is still running.
    pub fn is_alive(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_)) => false, // Process has exited
            Ok(None) => true,     // Process is still running
            Err(_) => false,      // Error checking status, assume dead
        }
    }

    /// Waits for the child process to exit and returns the exit status.
    pub fn wait(&mut self) -> PtyResult<portable_pty::ExitStatus> {
        self.child
            .wait()
            .map_err(|e| PtyError::Spawn(e.to_string()))
    }

    /// Gets a clone of the reader for use in async contexts.
    pub fn clone_reader(&self) -> Arc<Mutex<Box<dyn Read + Send>>> {
        Arc::clone(&self.reader)
    }

    /// Gets a clone of the writer for use in async contexts.
    pub fn clone_writer(&self) -> Arc<Mutex<Box<dyn Write + Send>>> {
        Arc::clone(&self.writer)
    }
}

/// Stub implementation when PTY feature is disabled.
#[cfg(not(feature = "pty"))]
pub struct Pty;

#[cfg(not(feature = "pty"))]
impl Pty {
    pub fn spawn(
        _cmd: &str,
        _args: &[&str],
        _cwd: Option<&str>,
        _rows: u16,
        _cols: u16,
    ) -> PtyResult<Self> {
        Err(PtyError::NotEnabled)
    }

    pub fn read(&mut self, _buf: &mut [u8]) -> PtyResult<usize> {
        Err(PtyError::NotEnabled)
    }

    pub fn write(&mut self, _data: &[u8]) -> PtyResult<usize> {
        Err(PtyError::NotEnabled)
    }

    pub fn flush(&mut self) -> PtyResult<()> {
        Err(PtyError::NotEnabled)
    }

    pub fn resize(&self, _rows: u16, _cols: u16) -> PtyResult<()> {
        Err(PtyError::NotEnabled)
    }

    pub fn is_alive(&mut self) -> bool {
        false
    }

    pub fn wait(&mut self) -> PtyResult<std::process::ExitStatus> {
        Err(PtyError::NotEnabled)
    }
}

/// Async PTY reader that sends output to a channel.
#[cfg(feature = "pty")]
pub async fn read_pty_async(
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    tx: mpsc::UnboundedSender<Vec<u8>>,
) {
    use tokio::task;

    task::spawn_blocking(move || {
        let mut buf = [0u8; 8192];
        loop {
            let mut reader_guard = reader.lock().unwrap();
            match reader_guard.read(&mut buf) {
                Ok(0) => {
                    // EOF reached
                    break;
                }
                Ok(n) => {
                    let data = buf[..n].to_vec();
                    if tx.send(data).is_err() {
                        // Channel closed, stop reading
                        break;
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Non-blocking read, no data available
                    drop(reader_guard);
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(_) => {
                    // Read error, stop
                    break;
                }
            }
        }
    })
    .await
    .ok();
}

#[cfg(test)]
#[cfg(feature = "pty")]
mod tests {
    use super::*;

    #[test]
    fn test_pty_spawn() {
        // This test will only work on Unix-like systems or Windows with ConPTY
        #[cfg(unix)]
        let result = Pty::spawn("echo", &["hello"], None, 24, 80);

        #[cfg(windows)]
        let result = Pty::spawn("cmd", &["/c", "echo hello"], None, 24, 80);

        #[cfg(any(unix, windows))]
        assert!(result.is_ok());
    }

    #[test]
    fn test_pty_resize() {
        #[cfg(unix)]
        let pty = Pty::spawn("cat", &[], None, 24, 80);

        #[cfg(windows)]
        let pty = Pty::spawn("cmd", &[], None, 24, 80);

        #[cfg(any(unix, windows))]
        if let Ok(pty) = pty {
            assert!(pty.resize(30, 100).is_ok());
        }
    }
}

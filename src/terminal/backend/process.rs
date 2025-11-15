//! Shell process management for terminals.
//!
//! This module provides utilities for detecting and spawning shell processes
//! across different platforms (Unix and Windows).

use std::env;
use std::path::PathBuf;

/// Represents a shell type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellType {
    /// Bash shell.
    Bash,
    /// Zsh shell.
    Zsh,
    /// Fish shell.
    Fish,
    /// Sh (POSIX shell).
    Sh,
    /// PowerShell (Windows).
    PowerShell,
    /// PowerShell Core (cross-platform).
    Pwsh,
    /// Windows Command Prompt.
    Cmd,
    /// Custom shell.
    Custom(String),
}

impl ShellType {
    /// Returns the command name for this shell.
    pub fn command(&self) -> String {
        match self {
            ShellType::Bash => "bash".to_string(),
            ShellType::Zsh => "zsh".to_string(),
            ShellType::Fish => "fish".to_string(),
            ShellType::Sh => "sh".to_string(),
            ShellType::PowerShell => "powershell".to_string(),
            ShellType::Pwsh => "pwsh".to_string(),
            ShellType::Cmd => "cmd".to_string(),
            ShellType::Custom(cmd) => cmd.clone(),
        }
    }

    /// Returns the default arguments for this shell (for interactive use).
    pub fn default_args(&self) -> Vec<String> {
        match self {
            ShellType::Bash => vec!["-i".to_string()],
            ShellType::Zsh => vec!["-i".to_string()],
            ShellType::Fish => vec!["-i".to_string()],
            ShellType::Sh => vec!["-i".to_string()],
            ShellType::PowerShell => vec!["-NoLogo".to_string()],
            ShellType::Pwsh => vec!["-NoLogo".to_string()],
            ShellType::Cmd => vec![],
            ShellType::Custom(_) => vec![],
        }
    }

    /// Detects the default shell for the current platform.
    ///
    /// On Unix, this checks the `SHELL` environment variable.
    /// On Windows, this defaults to PowerShell or cmd.exe.
    pub fn detect() -> Self {
        #[cfg(unix)]
        {
            if let Ok(shell) = env::var("SHELL") {
                return Self::from_path(&shell);
            }
            ShellType::Sh
        }

        #[cfg(windows)]
        {
            // Try to detect PowerShell Core first, then PowerShell, then fall back to cmd
            if which::which("pwsh").is_ok() {
                ShellType::Pwsh
            } else if which::which("powershell").is_ok() {
                ShellType::PowerShell
            } else {
                ShellType::Cmd
            }
        }
    }

    /// Creates a ShellType from a shell path.
    pub fn from_path(path: &str) -> Self {
        let path_lower = path.to_lowercase();

        if path_lower.contains("bash") {
            ShellType::Bash
        } else if path_lower.contains("zsh") {
            ShellType::Zsh
        } else if path_lower.contains("fish") {
            ShellType::Fish
        } else if path_lower.contains("pwsh") {
            ShellType::Pwsh
        } else if path_lower.contains("powershell") {
            ShellType::PowerShell
        } else if path_lower.contains("cmd") {
            ShellType::Cmd
        } else if path_lower.ends_with("sh") || path_lower.ends_with("sh.exe") {
            ShellType::Sh
        } else {
            ShellType::Custom(path.to_string())
        }
    }

    /// Checks if the shell is available on the system.
    #[cfg(feature = "pty")]
    pub fn is_available(&self) -> bool {
        use std::process::Command;

        let cmd = self.command();

        // Try to run the shell with --version or /? to check if it exists
        #[cfg(unix)]
        let result = Command::new(&cmd).arg("--version").output();

        #[cfg(windows)]
        let result = if cmd == "cmd" {
            Command::new(&cmd).arg("/?").output()
        } else {
            Command::new(&cmd).arg("--version").output()
        };

        result.is_ok()
    }

    #[cfg(not(feature = "pty"))]
    pub fn is_available(&self) -> bool {
        false
    }
}

/// Shell configuration for spawning terminal processes.
#[derive(Debug, Clone)]
pub struct ShellConfig {
    /// The shell type to use.
    pub shell: ShellType,
    /// Additional arguments to pass to the shell.
    pub args: Vec<String>,
    /// Working directory for the shell (None for current directory).
    pub cwd: Option<PathBuf>,
    /// Environment variables to set.
    pub env: Vec<(String, String)>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        let shell = ShellType::detect();
        Self {
            args: shell.default_args(),
            shell,
            cwd: None,
            env: Vec::new(),
        }
    }
}

impl ShellConfig {
    /// Creates a new shell configuration with the default shell.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a shell configuration with a specific shell type.
    pub fn with_shell(shell: ShellType) -> Self {
        Self {
            args: shell.default_args(),
            shell,
            cwd: None,
            env: Vec::new(),
        }
    }

    /// Sets the shell type.
    pub fn shell(mut self, shell: ShellType) -> Self {
        self.shell = shell;
        self
    }

    /// Sets the arguments for the shell.
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Adds an argument to the shell.
    pub fn arg(mut self, arg: String) -> Self {
        self.args.push(arg);
        self
    }

    /// Sets the working directory.
    pub fn cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = Some(cwd);
        self
    }

    /// Adds an environment variable.
    pub fn env(mut self, key: String, value: String) -> Self {
        self.env.push((key, value));
        self
    }

    /// Gets the command name for this configuration.
    pub fn command(&self) -> String {
        self.shell.command()
    }

    /// Gets the arguments as string slices.
    pub fn args_as_strs(&self) -> Vec<&str> {
        self.args.iter().map(|s| s.as_str()).collect()
    }

    /// Gets the working directory as a string.
    pub fn cwd_as_str(&self) -> Option<&str> {
        self.cwd.as_ref().map(|p| p.to_str()).flatten()
    }
}

/// Detects common shells available on the system.
#[cfg(feature = "pty")]
pub fn detect_available_shells() -> Vec<ShellType> {
    let mut shells = Vec::new();

    let candidates = vec![
        ShellType::Bash,
        ShellType::Zsh,
        ShellType::Fish,
        ShellType::Sh,
        ShellType::Pwsh,
        ShellType::PowerShell,
        ShellType::Cmd,
    ];

    for shell in candidates {
        if shell.is_available() {
            shells.push(shell);
        }
    }

    shells
}

#[cfg(not(feature = "pty"))]
pub fn detect_available_shells() -> Vec<ShellType> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_type_command() {
        assert_eq!(ShellType::Bash.command(), "bash");
        assert_eq!(ShellType::Zsh.command(), "zsh");
        assert_eq!(ShellType::PowerShell.command(), "powershell");
    }

    #[test]
    fn test_shell_type_from_path() {
        assert_eq!(ShellType::from_path("/bin/bash"), ShellType::Bash);
        assert_eq!(ShellType::from_path("/usr/bin/zsh"), ShellType::Zsh);
        assert_eq!(ShellType::from_path("/usr/bin/fish"), ShellType::Fish);

        #[cfg(windows)]
        {
            assert_eq!(
                ShellType::from_path("C:\\Windows\\System32\\cmd.exe"),
                ShellType::Cmd
            );
            assert_eq!(
                ShellType::from_path("C:\\Windows\\System32\\WindowsPowerShell\\powershell.exe"),
                ShellType::PowerShell
            );
        }
    }

    #[test]
    fn test_shell_type_detect() {
        // Just verify it doesn't panic
        let shell = ShellType::detect();
        assert!(!shell.command().is_empty());
    }

    #[test]
    fn test_shell_config_default() {
        let config = ShellConfig::default();
        assert!(!config.command().is_empty());
    }

    #[test]
    fn test_shell_config_builder() {
        let config = ShellConfig::new()
            .shell(ShellType::Bash)
            .arg("--login".to_string())
            .env("FOO".to_string(), "bar".to_string());

        assert_eq!(config.shell, ShellType::Bash);
        assert!(config.args.contains(&"--login".to_string()));
        assert_eq!(config.env, vec![("FOO".to_string(), "bar".to_string())]);
    }

    #[test]
    #[cfg(feature = "pty")]
    fn test_detect_available_shells() {
        let shells = detect_available_shells();
        // At least one shell should be available on any system
        assert!(!shells.is_empty());
    }
}

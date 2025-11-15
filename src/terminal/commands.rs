//! Terminal commands

use crate::terminal::manager::TerminalManager;
use std::fmt;

/// Terminal commands
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TerminalCommand {
    /// Create a new terminal
    New,

    /// Close the current terminal
    Close,

    /// Close all terminals
    CloseAll,

    /// Switch to next terminal
    Next,

    /// Switch to previous terminal
    Previous,

    /// Switch to terminal by index (1-based)
    SwitchTo(usize),

    /// Toggle terminal panel visibility
    Toggle,

    /// Show terminal panel
    Show,

    /// Hide terminal panel
    Hide,

    /// Clear current terminal
    Clear,

    /// Kill current terminal process
    Kill,

    /// Send input to current terminal
    SendInput(String),
}

impl TerminalCommand {
    /// Get command description
    pub fn description(&self) -> &'static str {
        match self {
            TerminalCommand::New => "New Terminal",
            TerminalCommand::Close => "Close Terminal",
            TerminalCommand::CloseAll => "Close All Terminals",
            TerminalCommand::Next => "Next Terminal",
            TerminalCommand::Previous => "Previous Terminal",
            TerminalCommand::SwitchTo(_) => "Switch to Terminal",
            TerminalCommand::Toggle => "Toggle Terminal Panel",
            TerminalCommand::Show => "Show Terminal Panel",
            TerminalCommand::Hide => "Hide Terminal Panel",
            TerminalCommand::Clear => "Clear Terminal",
            TerminalCommand::Kill => "Kill Terminal Process",
            TerminalCommand::SendInput(_) => "Send Input to Terminal",
        }
    }

    /// Execute command on terminal manager
    pub async fn execute(&self, manager: &mut TerminalManager) -> Result<(), String> {
        match self {
            TerminalCommand::New => {
                manager.create_terminal().await.map_err(|e| e.to_string())?;
                Ok(())
            }
            TerminalCommand::Close => {
                manager.close_current();
                Ok(())
            }
            TerminalCommand::CloseAll => {
                manager.close_all();
                Ok(())
            }
            TerminalCommand::Next => {
                manager.next_terminal();
                Ok(())
            }
            TerminalCommand::Previous => {
                manager.previous_terminal();
                Ok(())
            }
            TerminalCommand::SwitchTo(index) => {
                manager.switch_to(*index);
                Ok(())
            }
            TerminalCommand::Toggle => {
                manager.toggle_visibility();
                Ok(())
            }
            TerminalCommand::Show => {
                manager.show();
                Ok(())
            }
            TerminalCommand::Hide => {
                manager.hide();
                Ok(())
            }
            TerminalCommand::Clear => {
                manager.clear_current();
                Ok(())
            }
            TerminalCommand::Kill => {
                manager.kill_current().await;
                Ok(())
            }
            TerminalCommand::SendInput(input) => {
                manager
                    .send_input_to_current(input.as_bytes())
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }
}

impl fmt::Display for TerminalCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

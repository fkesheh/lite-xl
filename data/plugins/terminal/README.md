# Terminal Plugin for Lite XL

An integrated terminal panel for Lite XL, similar to VSCode's integrated terminal.

## Features

- **Integrated Shell**: Run bash, zsh, or other shells directly in the editor
- **Multiple Terminals**: Create and manage multiple terminal instances with tabs
- **ANSI Color Support**: Full support for colored terminal output
- **Keyboard Shortcuts**: Quick access with familiar keybindings
- **Scrollback Buffer**: Navigate through command history (up to 1000 lines)
- **Resizable Panel**: Drag to resize the terminal panel height
- **Non-blocking I/O**: Terminal runs in background without freezing the editor

## Installation

This plugin is included in the `data/plugins/terminal/` directory. It should load automatically on startup.

## Usage

### Keyboard Shortcuts

| Shortcut | Command | Description |
|----------|---------|-------------|
| `Ctrl+`` | `terminal:toggle` | Show/hide the terminal panel |
| `Ctrl+Shift+`` | `terminal:new` | Create a new terminal tab |
| `Ctrl+PageDown` | `terminal:next` | Switch to next terminal |
| `Ctrl+PageUp` | `terminal:previous` | Switch to previous terminal |

### Commands

All commands are available via the command palette (Ctrl+Shift+P):

- `terminal:toggle` - Toggle terminal visibility
- `terminal:show` - Show the terminal
- `terminal:hide` - Hide the terminal
- `terminal:new` - Create a new terminal
- `terminal:close` - Close the current terminal
- `terminal:next` - Switch to next terminal
- `terminal:previous` - Switch to previous terminal
- `terminal:restart` - Restart the current terminal
- `terminal:clear` - Clear terminal output

### Using the Terminal

1. Press `Ctrl+`` to open the terminal
2. The terminal appears at the bottom of the window
3. Type commands as you would in a normal terminal
4. Output is displayed with proper color formatting
5. Press `Ctrl+`` again to hide the terminal

### Multiple Terminals

- Click the "+" button in the tab bar to create a new terminal
- Click on terminal tabs to switch between them
- Click the "×" button on a tab to close that terminal
- Use `Ctrl+PageDown/PageUp` to cycle through terminals

## Configuration

You can customize the terminal in your user configuration:

```lua
-- In your init.lua
local config = require "core.config"

config.plugins.terminal = {
  size = 250 * SCALE,        -- Default height in pixels
  visible = false,            -- Start visible/hidden
  max_scrollback = 1000,      -- Maximum lines to keep in history
  tab_width = 150 * SCALE,    -- Width of terminal tabs
}
```

## Shell Configuration

The plugin automatically detects your default shell from the `SHELL` environment variable. On Linux/macOS, it typically uses bash or zsh. The shell is started in interactive mode (`-i` flag).

## Special Key Support

The terminal supports common control sequences:

- `Ctrl+C` - Send interrupt signal (SIGINT)
- `Ctrl+D` - Send end of transmission (exit)
- `Ctrl+Z` - Send suspend signal
- Arrow keys (up/down/left/right)
- Home, End, Page Up, Page Down
- Tab completion (if supported by your shell)

## Architecture

The plugin consists of three main components:

1. **ansi.lua** - ANSI escape sequence parser for colored output
2. **terminal.lua** - Terminal class managing shell process and I/O
3. **init.lua** - TerminalView UI component and editor integration

## Limitations

- PTY (pseudo-terminal) is not used, which may cause some programs to behave differently
- Complex TUI applications (like vim, htop) may not work properly
- Terminal emulation is basic and may not support all escape sequences

## Troubleshooting

**Terminal doesn't show**: Try `Ctrl+`` multiple times, or use command palette to run `terminal:show`

**Shell doesn't start**: Check that your `SHELL` environment variable is set correctly

**Colors look wrong**: Some programs may use 256-color or RGB color codes which are not fully supported yet

**Input not working**: Click inside the terminal area to focus it, then type

## Future Enhancements

Possible improvements for future versions:

- PTY support for better terminal emulation
- 256-color and RGB color support
- Split terminals (horizontal/vertical)
- Custom shell and argument configuration per terminal
- Task runner integration
- Terminal profiles
- Search in terminal output

## License

This plugin follows the same license as Lite XL.

# Command and Event System Documentation

This document describes the command and event system implementation for the Lite XL Rust text editor.

## Overview

The command and event system provides a comprehensive framework for handling user input and executing editor operations. It consists of three main components:

1. **Commands** - High-level editor operations
2. **Events** - Low-level input events (keyboard, mouse, window)
3. **Clipboard** - System clipboard integration

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Event Dispatcher                      │
│  - Routes events to handlers                             │
│  - Translates key events to commands via KeyMap          │
│  - Manages event queue                                   │
└───────────────────┬─────────────────────────────────────┘
                    │
         ┌──────────┼──────────┐
         │          │          │
    ┌────▼────┐ ┌──▼──────┐ ┌─▼────────┐
    │  Events │ │Commands │ │Clipboard │
    │         │ │         │ │          │
    └─────────┘ └─────────┘ └──────────┘
```

## Commands

### Command Types

Commands are organized into several categories:

#### File Operations
- `NewFile` - Create a new document
- `OpenFile` - Open file dialog
- `Save` - Save current document
- `SaveAs` - Save as new file
- `Close` - Close document
- `Quit` - Quit editor

#### Editing Operations
- `Insert(String)` - Insert text
- `Delete` - Delete selection or character
- `Cut` - Cut to clipboard
- `Copy` - Copy to clipboard
- `Paste` - Paste from clipboard
- `Undo` - Undo last change
- `Redo` - Redo last undone change
- `DuplicateLine` - Duplicate current line(s)
- `ToggleComment` - Toggle line comments
- And many more...

#### Navigation
- `MoveCursor(Movement)` - Move cursor
- `Select(Movement)` - Extend selection
- `GoToLine(usize)` - Jump to line
- `PageUp/PageDown` - Scroll by page

#### View Operations
- `ZoomIn/ZoomOut/ZoomReset` - Font size control
- `ToggleLineNumbers` - Show/hide line numbers
- `ToggleFullscreen` - Fullscreen mode

#### Multi-Cursor
- `AddCursorAbove/Below` - Add cursor above/below
- `AddCursorAtNextOccurrence` - Add cursor at next match
- `RemoveAllCursors` - Remove all but primary cursor

### Using Commands

```rust
use lite_xl_rust::commands::{Command, Movement};

// Create commands
let save = Command::Save;
let move_down = Command::MoveCursor(Movement::Down);
let insert = Command::Insert("Hello".to_string());

// Check command properties
assert!(insert.modifies_document());
assert!(insert.is_undoable());
assert_eq!(save.description(), "Save");
```

## Events

### Event Types

#### Keyboard Events
```rust
use lite_xl_rust::events::{EditorEvent, KeyEvent};
use lite_xl_rust::commands::{Key, Modifiers};

let key_event = KeyEvent::new(Key::Char('s'), Modifiers::ctrl());
let event = EditorEvent::KeyPress(key_event);
```

#### Mouse Events
```rust
use lite_xl_rust::events::{MouseEvent, MouseButton, ScreenPosition};

let mouse_event = MouseEvent::ButtonPress {
    position: ScreenPosition::new(100.0, 200.0),
    button: MouseButton::Left,
    modifiers: Modifiers::none(),
};
```

#### Window Events
```rust
use lite_xl_rust::events::WindowEvent;

let resize = WindowEvent::Resize { width: 1024, height: 768 };
let focus = WindowEvent::Focus;
```

### Event Dispatcher

The event dispatcher is the central hub for event handling:

```rust
use lite_xl_rust::events::{EventDispatcher, EditorEvent};
use lite_xl_rust::commands::KeyMap;

// Create dispatcher with default keymap
let mut dispatcher = EventDispatcher::new();

// Or with custom keymap
let keymap = KeyMap::default_keymap();
let mut dispatcher = EventDispatcher::with_keymap(keymap);

// Post events
dispatcher.post(EditorEvent::Command(Command::Save));

// Process events
dispatcher.process_all();

// Or process one at a time
while let Some(event) = dispatcher.process_next() {
    // Event was processed
}
```

### Event Handlers

Implement the `EventHandler` trait to handle events:

```rust
use lite_xl_rust::events::{EventHandler, EditorEvent};

struct MyHandler;

impl EventHandler for MyHandler {
    fn handle_event(&mut self, event: &EditorEvent) -> bool {
        match event {
            EditorEvent::Command(cmd) => {
                println!("Command: {}", cmd);
                true // Event handled
            }
            _ => false // Not handled
        }
    }
}

// Register handler
let handler = Box::new(MyHandler);
dispatcher.add_handler(handler);
```

## Keybindings

### Default Keybindings

The system includes default keybindings for common operations:

| Keybinding | Command |
|------------|---------|
| Ctrl+N | New File |
| Ctrl+O | Open File |
| Ctrl+S | Save |
| Ctrl+W | Close |
| Ctrl+Z | Undo |
| Ctrl+Y | Redo |
| Ctrl+X | Cut |
| Ctrl+C | Copy |
| Ctrl+V | Paste |
| Ctrl+A | Select All |
| Ctrl+F | Find |
| Ctrl+H | Replace |
| F3 | Find Next |
| Arrow Keys | Move Cursor |
| Shift+Arrows | Select |
| Ctrl+Arrows | Word Navigation |
| Home/End | Line Start/End |
| Ctrl+Home/End | Document Start/End |
| Page Up/Down | Scroll Page |

### Custom Keybindings

Create custom keybindings:

```rust
use lite_xl_rust::commands::{KeyMap, Key, Modifiers, Command};

let mut keymap = KeyMap::new();

// Bind Ctrl+Shift+D to Duplicate Line
keymap.bind(
    Key::Char('d'),
    Modifiers::ctrl_shift(),
    Command::DuplicateLine
);

// Bind F2 to Save
keymap.bind(
    Key::F2,
    Modifiers::none(),
    Command::Save
);

// Remove a binding
keymap.unbind(Key::Char('d'), Modifiers::ctrl_shift());

// Look up command for keybinding
if let Some(cmd) = keymap.lookup(Key::Char('s'), Modifiers::ctrl()) {
    println!("Ctrl+S -> {}", cmd);
}

// Get all keybindings for a command
let bindings = keymap.bindings_for_command(&Command::Save);
for binding in bindings {
    println!("Binding: {:?}", binding);
}
```

### Parsing Keybindings from Strings

```rust
use lite_xl_rust::events::keyboard::parse_keybinding;

// Parse keybinding string
let binding = parse_keybinding("Ctrl+Shift+S").unwrap();
let binding = parse_keybinding("Alt+F4").unwrap();
let binding = parse_keybinding("F5").unwrap();

// Format keybinding to string
use lite_xl_rust::events::keyboard::format_keybinding;
let str = format_keybinding(&binding);
```

## Clipboard Integration

### Basic Usage

```rust
use lite_xl_rust::clipboard::Clipboard;

let mut clipboard = Clipboard::new();

// Set text
clipboard.set_text("Hello, clipboard!".to_string()).unwrap();

// Get text
let text = clipboard.get_text().unwrap();
```

### Clipboard History

```rust
// Create clipboard with history
let mut clipboard = Clipboard::with_history_size(100);

// Add entries
clipboard.set_text("First".to_string()).unwrap();
clipboard.set_text("Second".to_string()).unwrap();

// Access history
let history = clipboard.get_history();
for (i, entry) in history.iter().enumerate() {
    println!("[{}] {}", i, entry);
}

// Get specific entry
let entry = clipboard.get_history_entry(0);
```

### Multi-Cursor Clipboard

For multi-cursor editing, each cursor can have its own clipboard entry:

```rust
// Set multiple clipboard entries (one per cursor)
clipboard.set_multi(vec![
    "cursor1 text".to_string(),
    "cursor2 text".to_string(),
    "cursor3 text".to_string(),
]);

// Check if multi-cursor clipboard is active
if clipboard.has_multi() {
    let entries = clipboard.get_multi();
    // Paste each entry at corresponding cursor
}

// Clear multi-cursor clipboard
clipboard.clear_multi();
```

## Implementation Files

The command and event system is implemented in the following files:

### Commands
- `/home/user/lite-xl/src/commands/mod.rs` - Main command definitions and keymap
- `/home/user/lite-xl/src/commands/editing.rs` - Editing command implementations
- `/home/user/lite-xl/src/commands/file.rs` - File operation implementations
- `/home/user/lite-xl/src/commands/navigation.rs` - Navigation command implementations

### Events
- `/home/user/lite-xl/src/events/mod.rs` - Event types and dispatcher
- `/home/user/lite-xl/src/events/keyboard.rs` - Keyboard event mapping and parsing

### Clipboard
- `/home/user/lite-xl/src/clipboard.rs` - Clipboard integration

## Example: Complete Integration

Here's a complete example showing how all pieces work together:

```rust
use lite_xl_rust::commands::{Command, KeyMap, Key, Modifiers};
use lite_xl_rust::events::{EventDispatcher, EditorEvent, EventHandler};
use lite_xl_rust::clipboard::Clipboard;

// Create components
let mut keymap = KeyMap::default_keymap();
let mut dispatcher = EventDispatcher::with_keymap(keymap);
let mut clipboard = Clipboard::new();

// Add custom handler
struct EditorHandler {
    clipboard: Clipboard,
}

impl EventHandler for EditorHandler {
    fn handle_event(&mut self, event: &EditorEvent) -> bool {
        match event {
            EditorEvent::Command(Command::Copy) => {
                // Copy selection to clipboard
                let selection = "selected text"; // Get from editor
                self.clipboard.set_text(selection.to_string()).unwrap();
                true
            }
            EditorEvent::Command(Command::Paste) => {
                // Paste from clipboard
                if let Ok(text) = self.clipboard.get_text() {
                    // Insert into editor
                }
                true
            }
            _ => false
        }
    }
}

let handler = Box::new(EditorHandler { clipboard });
dispatcher.add_handler(handler);

// Simulate user pressing Ctrl+C
let key_event = KeyEvent::new(Key::Char('c'), Modifiers::ctrl());
dispatcher.post(EditorEvent::KeyPress(key_event));

// Process events
dispatcher.process_all();
```

## Testing

Run the demo to see the system in action:

```bash
cargo run --example command_event_demo
```

Run tests:

```bash
# Test all modules
cargo test

# Test specific module
cargo test commands::
cargo test events::
cargo test clipboard::

# Run with output
cargo test -- --nocapture
```

## Extension Points

The system is designed to be extensible:

1. **Custom Commands** - Use `Command::Custom { id, args }` for plugin commands
2. **Custom Events** - Use `EditorEvent::Custom { name, data }` for plugin events
3. **Event Handlers** - Implement `EventHandler` trait for custom event processing
4. **Keybindings** - Create custom keymaps for different editing modes (vim, emacs, etc.)

## Performance Considerations

- **Event Queue**: Uses efficient MPSC channels for event queueing
- **Keybinding Lookup**: O(1) HashMap lookup for key to command translation
- **Command Execution**: Commands are lightweight enums with minimal overhead
- **Undo Support**: Commands track whether they're undoable for efficient undo/redo

## Future Enhancements

Planned improvements:

1. **Chord Keybindings** - Multi-key sequences (e.g., "Ctrl+K Ctrl+S")
2. **Context-Aware Bindings** - Different bindings per mode/context
3. **Macro Recording** - Record and playback command sequences
4. **Custom Event Filters** - Pre-process events before dispatch
5. **Async Command Execution** - Long-running commands in background
6. **System Clipboard Integration** - Platform-specific clipboard access

## Troubleshooting

### Keybinding Conflicts

Use the conflict detector to find conflicting bindings:

```rust
use lite_xl_rust::events::keyboard::ConflictDetector;

let mut detector = ConflictDetector::new();
let conflicts = detector.check_keymap(&keymap);

for conflict in conflicts {
    eprintln!("Conflict: {}", conflict);
}
```

### Event Not Handled

If an event isn't being handled:

1. Check if the keybinding is registered in the keymap
2. Verify event handlers are registered with the dispatcher
3. Ensure handlers return `true` only when they handle the event
4. Check if a higher-priority handler consumed the event

### Clipboard Issues

If clipboard operations fail:

1. The system falls back to internal clipboard automatically
2. Check platform-specific clipboard support
3. Verify clipboard permissions on the platform

## License

This implementation is part of the Lite XL project and follows the project's license terms.

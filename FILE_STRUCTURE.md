# Command and Event System - File Structure

## Implementation Files

### Core Command System

```
src/commands/
├── mod.rs              # Main command definitions and keymap (628 lines)
│   ├── Command enum (70+ variants)
│   ├── Movement enum
│   ├── KeyMap implementation
│   ├── Key and Modifiers types
│   ├── Default keybindings
│   └── Tests
│
├── editing.rs          # Editing command implementations (481 lines)
│   ├── EditContext trait
│   ├── Insert, delete, cut, copy, paste
│   ├── Line operations (duplicate, move, join, split)
│   ├── Indentation
│   ├── Comment toggling
│   ├── Text transformations
│   └── Tests
│
├── file.rs             # File operation implementations (410 lines)
│   ├── FileContext trait
│   ├── New, open, save, close operations
│   ├── File encoding detection
│   ├── Line ending management
│   ├── Error types
│   └── Tests
│
└── navigation.rs       # Navigation implementations (576 lines)
    ├── NavigationContext trait
    ├── Position type
    ├── Movement implementations
    ├── Go to line/position
    ├── Bracket matching
    └── Tests
```

### Core Event System

```
src/events/
├── mod.rs              # Event types and dispatcher (560 lines)
│   ├── EditorEvent enum
│   ├── KeyEvent, MouseEvent, WindowEvent
│   ├── EventDispatcher
│   ├── EventHandler trait
│   ├── ClickDetector
│   └── Tests
│
└── keyboard.rs         # Keyboard event mapping (595 lines)
    ├── Keybinding parser
    ├── Keybinding formatter
    ├── KeyboardLayout trait
    ├── ConflictDetector
    ├── ImeState
    ├── KeyboardRecorder
    └── Tests
```

### Clipboard Integration

```
src/
└── clipboard.rs        # Clipboard implementation (327 lines)
    ├── Clipboard manager
    ├── History support
    ├── Multi-cursor clipboard
    ├── ClipboardContext trait
    ├── ClipboardContent enum
    └── Tests
```

### Integration

```
src/
└── lib.rs              # Library root (updated)
    ├── Module declarations
    └── Public API exports
```

### Documentation

```
.
├── COMMANDS_AND_EVENTS.md      # Complete user guide
├── IMPLEMENTATION_SUMMARY.md   # Implementation summary
└── FILE_STRUCTURE.md           # This file
```

### Examples

```
examples/
└── command_event_demo.rs       # Working demo (175 lines)
    ├── Event handling demo
    ├── Keybinding examples
    ├── Clipboard usage
    └── Navigation examples
```

## File Locations (Absolute Paths)

### Implementation
- `/home/user/lite-xl/src/commands/mod.rs`
- `/home/user/lite-xl/src/commands/editing.rs`
- `/home/user/lite-xl/src/commands/file.rs`
- `/home/user/lite-xl/src/commands/navigation.rs`
- `/home/user/lite-xl/src/events/mod.rs`
- `/home/user/lite-xl/src/events/keyboard.rs`
- `/home/user/lite-xl/src/clipboard.rs`
- `/home/user/lite-xl/src/lib.rs` (updated)

### Documentation
- `/home/user/lite-xl/COMMANDS_AND_EVENTS.md`
- `/home/user/lite-xl/IMPLEMENTATION_SUMMARY.md`
- `/home/user/lite-xl/FILE_STRUCTURE.md`

### Examples
- `/home/user/lite-xl/examples/command_event_demo.rs`

## Module Dependencies

```
                    lib.rs
                      │
        ┌─────────────┼─────────────┐
        │             │             │
    commands       events      clipboard
        │
   ┌────┼────┐
   │    │    │
editing file navigation
```

## Testing

Run tests for each module:

```bash
# All tests
cargo test

# Specific modules
cargo test commands::
cargo test events::
cargo test clipboard::

# With output
cargo test -- --nocapture
```

## Usage Example

```bash
# Run the demo
cargo run --example command_event_demo

# Build the library
cargo build --lib

# Check for errors
cargo check --lib

# Generate documentation
cargo doc --open
```

## Lines of Code

| Module | Lines | Tests | Total |
|--------|-------|-------|-------|
| commands/mod.rs | 528 | 100 | 628 |
| commands/editing.rs | 398 | 83 | 481 |
| commands/file.rs | 340 | 70 | 410 |
| commands/navigation.rs | 456 | 120 | 576 |
| events/mod.rs | 480 | 80 | 560 |
| events/keyboard.rs | 495 | 100 | 595 |
| clipboard.rs | 287 | 40 | 327 |
| **Total** | **2,984** | **593** | **3,577** |

Plus documentation (~1,500 lines) and examples (~175 lines).

**Grand Total: ~5,250 lines of code and documentation**

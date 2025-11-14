//! Command and Event System Demo
//!
//! This example demonstrates how to use the command and event system
//! to create a basic text editor interaction flow.

use lite_xl_rust::commands::{Command, Key, KeyMap, Modifiers, Movement};
use lite_xl_rust::events::{
    EditorEvent, EventDispatcher, EventHandler, KeyEvent, MouseButton, MouseEvent,
    ScreenPosition, WindowEvent,
};
use lite_xl_rust::clipboard::Clipboard;

/// Simple event handler that prints events
struct LoggingHandler {
    name: String,
}

impl LoggingHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl EventHandler for LoggingHandler {
    fn handle_event(&mut self, event: &EditorEvent) -> bool {
        match event {
            EditorEvent::Command(cmd) => {
                println!("[{}] Command: {}", self.name, cmd);
            }
            EditorEvent::KeyPress(key_event) => {
                println!("[{}] Key Press: {:?}", self.name, key_event);
            }
            EditorEvent::Mouse(mouse_event) => {
                println!("[{}] Mouse Event: {:?}", self.name, mouse_event);
            }
            EditorEvent::Window(window_event) => {
                println!("[{}] Window Event: {:?}", self.name, window_event);
            }
            _ => {}
        }
        false // Don't consume events
    }
}

fn main() {
    println!("=== Lite XL Command and Event System Demo ===\n");

    // Create a keymap with default bindings
    println!("1. Creating keymap with default bindings...");
    let keymap = KeyMap::default_keymap();
    println!("   Keymap created with {} bindings\n", keymap.all_bindings().count());

    // Demonstrate some default keybindings
    println!("2. Testing default keybindings:");
    test_keybinding(&keymap, Key::Char('s'), Modifiers::ctrl(), "Save");
    test_keybinding(&keymap, Key::Char('c'), Modifiers::ctrl(), "Copy");
    test_keybinding(&keymap, Key::Char('v'), Modifiers::ctrl(), "Paste");
    test_keybinding(&keymap, Key::Char('z'), Modifiers::ctrl(), "Undo");
    test_keybinding(&keymap, Key::Char('y'), Modifiers::ctrl(), "Redo");
    println!();

    // Create an event dispatcher
    println!("3. Creating event dispatcher...");
    let mut dispatcher = EventDispatcher::with_keymap(keymap);

    // Add a logging handler
    let handler = Box::new(LoggingHandler::new("MainHandler"));
    dispatcher.add_handler(handler);
    println!("   Event dispatcher created\n");

    // Demonstrate event posting and processing
    println!("4. Posting and processing events:\n");

    // Post a key event that maps to Save command
    let key_event = KeyEvent::new(Key::Char('s'), Modifiers::ctrl());
    dispatcher.post(EditorEvent::KeyPress(key_event));

    // Post a direct command
    dispatcher.post(EditorEvent::Command(Command::Copy));

    // Post a mouse event
    let mouse_event = MouseEvent::ButtonPress {
        position: ScreenPosition::new(100.0, 200.0),
        button: MouseButton::Left,
        modifiers: Modifiers::none(),
    };
    dispatcher.post(EditorEvent::Mouse(mouse_event));

    // Post a window event
    dispatcher.post(EditorEvent::Window(WindowEvent::Resize {
        width: 1024,
        height: 768,
    }));

    // Process all events
    let count = dispatcher.process_all();
    println!("\n   Processed {} events\n", count);

    // Demonstrate navigation commands
    println!("5. Testing navigation commands:");
    demonstrate_navigation();
    println!();

    // Demonstrate clipboard
    println!("6. Testing clipboard:");
    demonstrate_clipboard();
    println!();

    // Demonstrate custom keybindings
    println!("7. Creating custom keybindings:");
    demonstrate_custom_keybindings();
    println!();

    println!("=== Demo Complete ===");
}

fn test_keybinding(keymap: &KeyMap, key: Key, modifiers: Modifiers, expected: &str) {
    if let Some(command) = keymap.lookup(key, modifiers) {
        println!("   {:?} + {:?} -> {}", key, modifiers, command);
        assert_eq!(command.description(), expected);
    } else {
        println!("   {:?} + {:?} -> Not bound", key, modifiers);
    }
}

fn demonstrate_navigation() {
    let movements = vec![
        (Movement::Left, "Move left"),
        (Movement::Right, "Move right"),
        (Movement::Up, "Move up"),
        (Movement::Down, "Move down"),
        (Movement::LineStart, "Move to line start"),
        (Movement::LineEnd, "Move to line end"),
        (Movement::WordLeft, "Move word left"),
        (Movement::WordRight, "Move word right"),
        (Movement::DocumentStart, "Move to document start"),
        (Movement::DocumentEnd, "Move to document end"),
    ];

    for (movement, description) in movements {
        let command = Command::MoveCursor(movement);
        println!("   {} -> {}", description, command);
    }
}

fn demonstrate_clipboard() {
    let mut clipboard = Clipboard::new();

    // Set some text
    clipboard.set_text("Hello, clipboard!".to_string()).unwrap();
    let text = clipboard.get_text().unwrap();
    println!("   Set clipboard: {}", text);

    // Add more to history
    clipboard.set_text("Second entry".to_string()).unwrap();
    clipboard.set_text("Third entry".to_string()).unwrap();

    println!("   Clipboard history:");
    for (i, entry) in clipboard.get_history().iter().enumerate() {
        println!("     [{}] {}", i, entry);
    }

    // Test multi-cursor clipboard
    clipboard.set_multi(vec![
        "cursor1".to_string(),
        "cursor2".to_string(),
        "cursor3".to_string(),
    ]);
    println!("   Multi-cursor entries: {} entries", clipboard.get_multi().len());
}

fn demonstrate_custom_keybindings() {
    let mut keymap = KeyMap::new();

    // Add custom keybindings
    keymap.bind(Key::Char('d'), Modifiers::ctrl_shift(), Command::DuplicateLine);
    keymap.bind(Key::F2, Modifiers::none(), Command::Save);
    keymap.bind(Key::Char('t'), Modifiers::ctrl(), Command::ShowFilePicker);

    println!("   Custom keybindings:");
    for (binding, command) in keymap.all_bindings() {
        println!("     {:?} -> {}", binding, command);
    }

    // Test looking up a custom binding
    if let Some(command) = keymap.lookup(Key::Char('d'), Modifiers::ctrl_shift()) {
        println!("   Ctrl+Shift+D -> {}", command);
    }
}

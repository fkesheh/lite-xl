# Terminal UI Integration Guide

This guide shows how to integrate the terminal UI components into the Lite XL application.

## Quick Start

### 1. Add Terminal Panel to Application

Modify your `src/main.rs` to include the terminal panel:

```rust
use floem::{
    Application,
    event::{Event, EventListener},
    keyboard::Key,
    reactive::RwSignal,
    views::{container, v_stack, Decorators},
    window::WindowConfig,
};

use lite_xl::ui::{
    app_view, create_terminal_panel, handle_terminal_shortcuts,
    FontConfig, Theme,
};

fn main() {
    // Create state
    let theme = RwSignal::new(Theme::dark());
    let font_config = RwSignal::new(FontConfig::default());
    let editor_state = RwSignal::new(EditorState::with_text(""));

    // Create terminal panel
    let (terminal_panel_state, terminal_panel) =
        create_terminal_panel(theme, font_config);

    // Build application view with terminal
    let view = container(
        v_stack((
            // Main editor
            app_view(editor_state, theme, font_config),

            // Terminal panel (docked at bottom by default)
            terminal_panel,
        ))
    )
    .on_event_stop(EventListener::KeyDown, move |event| {
        if let Event::KeyDown(key_event) = event {
            let modifiers = &key_event.modifiers;
            if let Key::Character(ch) = &key_event.key.logical_key {
                // Handle terminal shortcuts
                handle_terminal_shortcuts(
                    terminal_panel_state,
                    ch,
                    modifiers.control(),
                );
            }
        }
    })
    .style(|s| s.width_full().height_full());

    // Run application
    Application::new()
        .window(move |_| view, Some(WindowConfig::default()))
        .run();
}
```

### 2. Customize Docking Position

```rust
// Show terminal panel at bottom
terminal_panel_state.update(|state| {
    state.show();
    state.set_position(DockPosition::Bottom);
});

// Or dock on the right
terminal_panel_state.update(|state| {
    state.show();
    state.set_position(DockPosition::Right);
});
```

### 3. Add Keyboard Shortcuts

Integrate with your existing command system:

```rust
use lite_xl::ui::handle_terminal_shortcuts;

// In your key event handler
match key {
    Key::Character(ch) => {
        if handle_terminal_shortcuts(
            terminal_panel_state,
            ch,
            ctrl_pressed,
        ) {
            return; // Shortcut handled
        }
        // Continue with other key handling...
    }
    _ => {}
}
```

## Advanced Integration

### Custom Layout with Terminal Panel

Create a custom layout that includes the terminal panel:

```rust
fn custom_layout(
    editor: RwSignal<EditorState>,
    terminal_panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    let position = terminal_panel_state.get().position;

    match position {
        DockPosition::Bottom => {
            v_stack((
                // Editor on top
                editor_area(editor, theme, font_config),
                // Terminal at bottom
                terminal_panel_view(terminal_panel_state, theme, font_config),
            ))
        }
        DockPosition::Left => {
            h_stack((
                // Terminal on left
                terminal_panel_view(terminal_panel_state, theme, font_config),
                // Editor on right
                editor_area(editor, theme, font_config),
            ))
        }
        DockPosition::Right => {
            h_stack((
                // Editor on left
                editor_area(editor, theme, font_config),
                // Terminal on right
                terminal_panel_view(terminal_panel_state, theme, font_config),
            ))
        }
    }
}
```

### Connect to PTY Backend

Integrate with a PTY (pseudo-terminal) backend:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

// Create PTY backend (pseudo-code)
let pty = Arc::new(Mutex::new(create_pty()));

// Update terminal grid when PTY output arrives
let grid_signal = /* your grid signal */;
tokio::spawn(async move {
    loop {
        let output = pty.lock().await.read().await;
        grid_signal.update(|grid| {
            // Parse ANSI sequences and update grid
            process_pty_output(grid, &output);
        });
    }
});

// Send input to PTY
let on_input = move |bytes: Vec<u8>| {
    let pty = pty.clone();
    tokio::spawn(async move {
        pty.lock().await.write(&bytes).await;
    });
};
```

### Theme Switching

Update terminal colors when theme changes:

```rust
// Watch for theme changes
theme.subscribe(move |new_theme| {
    terminal_panel_state.update(|state| {
        if let Some(tab) = state.tab_manager.active_tab_mut() {
            // Update terminal colors
            for row in &mut tab.grid.cells {
                for cell in row {
                    if cell.fg == old_theme.foreground {
                        cell.fg = new_theme.foreground;
                    }
                    if cell.bg == old_theme.background {
                        cell.bg = new_theme.background;
                    }
                }
            }
        }
    });
});
```

### Save/Restore Terminal State

Persist terminal state across sessions:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TerminalConfig {
    visible: bool,
    position: DockPosition,
    size: f64,
    tabs: Vec<TabConfig>,
}

// Save state
fn save_terminal_state(state: &TerminalPanelState) -> Result<()> {
    let config = TerminalConfig {
        visible: state.visible,
        position: state.position,
        size: state.size,
        tabs: state.tab_manager.tabs.iter().map(|tab| {
            TabConfig {
                title: tab.title.clone(),
                working_dir: tab.working_dir.clone(),
            }
        }).collect(),
    };

    let config_str = toml::to_string(&config)?;
    std::fs::write("terminal.toml", config_str)?;
    Ok(())
}

// Restore state
fn restore_terminal_state() -> Result<TerminalPanelState> {
    let config_str = std::fs::read_to_string("terminal.toml")?;
    let config: TerminalConfig = toml::from_str(&config_str)?;

    let mut state = TerminalPanelState::new();
    state.visible = config.visible;
    state.position = config.position;
    state.size = config.size;

    // Restore tabs...

    Ok(state)
}
```

## Menu Integration

Add terminal menu items:

```rust
// Menu structure (conceptual)
Menu {
    "View" => [
        MenuItem {
            label: "Toggle Terminal",
            shortcut: "Ctrl+`",
            action: || terminal_panel_state.update(|s| s.toggle()),
        },
        Separator,
        SubMenu {
            label: "Terminal Position",
            items: [
                MenuItem {
                    label: "Bottom",
                    action: || terminal_panel_state.update(|s| {
                        s.set_position(DockPosition::Bottom)
                    }),
                },
                MenuItem {
                    label: "Left",
                    action: || terminal_panel_state.update(|s| {
                        s.set_position(DockPosition::Left)
                    }),
                },
                MenuItem {
                    label: "Right",
                    action: || terminal_panel_state.update(|s| {
                        s.set_position(DockPosition::Right)
                    }),
                },
            ],
        },
    ],
    "Terminal" => [
        MenuItem {
            label: "New Terminal",
            shortcut: "Ctrl+Shift+T",
            action: || terminal_panel_state.update(|s| {
                s.tab_manager.add_tab(None);
                s.show();
            }),
        },
        MenuItem {
            label: "Close Terminal",
            shortcut: "Ctrl+Shift+W",
            action: || terminal_panel_state.update(|s| {
                s.tab_manager.close_tab(s.tab_manager.active_index);
            }),
        },
        Separator,
        MenuItem {
            label: "Next Terminal",
            shortcut: "Ctrl+Tab",
            action: || terminal_panel_state.update(|s| {
                s.tab_manager.next_tab();
            }),
        },
        MenuItem {
            label: "Previous Terminal",
            shortcut: "Ctrl+Shift+Tab",
            action: || terminal_panel_state.update(|s| {
                s.tab_manager.prev_tab();
            }),
        },
    ],
}
```

## Command Palette Integration

Add terminal commands to the command palette:

```rust
commands.register("terminal:toggle", || {
    terminal_panel_state.update(|s| s.toggle());
});

commands.register("terminal:new", || {
    terminal_panel_state.update(|s| {
        s.tab_manager.add_tab(None);
        s.show();
    });
});

commands.register("terminal:close", || {
    terminal_panel_state.update(|s| {
        s.tab_manager.close_tab(s.tab_manager.active_index);
    });
});

commands.register("terminal:next", || {
    terminal_panel_state.update(|s| s.tab_manager.next_tab());
});

commands.register("terminal:previous", || {
    terminal_panel_state.update(|s| s.tab_manager.prev_tab());
});

commands.register("terminal:dock-bottom", || {
    terminal_panel_state.update(|s| s.set_position(DockPosition::Bottom));
});

commands.register("terminal:dock-left", || {
    terminal_panel_state.update(|s| s.set_position(DockPosition::Left));
});

commands.register("terminal:dock-right", || {
    terminal_panel_state.update(|s| s.set_position(DockPosition::Right));
});
```

## Status Bar Integration

Show terminal status in the status bar:

```rust
fn terminal_status(terminal_panel_state: RwSignal<TerminalPanelState>) -> impl View {
    label(move || {
        let state = terminal_panel_state.get();
        if state.visible {
            format!(
                "Terminal: {} ({} tabs)",
                match state.position {
                    DockPosition::Bottom => "Bottom",
                    DockPosition::Left => "Left",
                    DockPosition::Right => "Right",
                },
                state.tab_manager.tabs.len()
            )
        } else {
            "Terminal: Hidden".to_string()
        }
    })
}
```

## Performance Optimization

### Lazy Loading

Only create terminal views when visible:

```rust
fn terminal_panel_lazy(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    container(
        dyn_view(move || {
            if panel_state.get().visible {
                terminal_panel_view(panel_state, theme, font_config)
                    .into_any()
            } else {
                empty().into_any()
            }
        })
    )
}
```

### Virtual Scrolling

For large scrollback buffers, implement virtual scrolling:

```rust
// Only render visible rows
let visible_rows = calculate_visible_rows(scroll_offset, panel_height);
for row in visible_rows {
    render_terminal_line(row, grid, theme, font);
}
```

## Troubleshooting

### Terminal Not Showing

Check that the panel state is visible:

```rust
terminal_panel_state.update(|s| {
    println!("Visible: {}", s.visible);
    s.show();
});
```

### Input Not Working

Ensure the canvas has keyboard focus:

```rust
// The terminal canvas should have .keyboard_navigable()
terminal_canvas_view(...)
    .keyboard_navigable()
```

### Colors Look Wrong

Verify theme integration:

```rust
let cell = TerminalCell::new('A').with_theme(&theme.get());
```

### Resize Handle Not Working

Check that pointer events are properly captured:

```rust
resize_handle(...)
    .on_event_stop(EventListener::PointerDown, ...)
    .on_event_stop(EventListener::PointerMove, ...)
    .on_event_stop(EventListener::PointerUp, ...)
```

## Testing Integration

Test the terminal panel integration:

```bash
# Run the demo
cargo run --example terminal_ui_demo

# Run tests
cargo test --lib ui::terminal_panel
cargo test --lib ui::terminal_tabs
cargo test --lib ui::terminal_canvas
```

## Next Steps

1. Integrate with PTY backend for real terminal functionality
2. Add ANSI escape sequence parser
3. Implement scrollback buffer
4. Add copy/paste support
5. Implement search in terminal
6. Add split terminal views
7. Support custom color schemes

## See Also

- [Terminal UI Documentation](TERMINAL_UI.md)
- [Floem Documentation](https://docs.rs/floem/)
- [PTY Integration](https://docs.rs/portable-pty/)

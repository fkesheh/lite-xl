/// Terminal UI Components Demo
///
/// This example demonstrates the Floem-based terminal UI components
/// including the terminal canvas, tab bar, and dockable panel.
///
/// Run with: cargo run --example terminal_ui_demo

use floem::{
    Application,
    reactive::RwSignal,
    views::{container, v_stack, Decorators},
    window::WindowConfig,
};

use lite_xl::ui::{
    create_terminal_panel, terminal_canvas_view, DockPosition, FontConfig, TerminalCell,
    TerminalGrid, TerminalPanelState, Theme,
};

fn main() {
    println!("═══════════════════════════════════════════════");
    println!("  Terminal UI Components Demo");
    println!("═══════════════════════════════════════════════\n");

    // Create theme and font config
    let theme = RwSignal::new(Theme::dark());
    let font_config = RwSignal::new(FontConfig::default());

    // Demo 1: Basic Terminal Grid
    println!("Demo 1: Creating Terminal Grid");
    println!("─────────────────────────────────────────────");
    let mut grid = TerminalGrid::new(80, 24);

    // Write some text to the grid
    let text = "Hello, Terminal UI!";
    for (col, ch) in text.chars().enumerate() {
        let mut cell = TerminalCell::new(ch);
        cell = cell.with_theme(&theme.get());
        grid.set(0, col, cell);
    }

    // Add colored text
    for (col, ch) in "Colored Text".chars().enumerate() {
        let mut cell = TerminalCell::new(ch);
        cell.fg = floem::peniko::Color::rgb8(100, 200, 100);
        cell.bold = true;
        grid.set(1, col, cell);
    }

    println!("  Grid size: {}x{}", grid.cols, grid.rows);
    println!("  Cursor position: ({}, {})", grid.cursor.row, grid.cursor.col);
    println!();

    // Demo 2: Terminal Panel State
    println!("Demo 2: Terminal Panel State");
    println!("─────────────────────────────────────────────");
    let mut panel_state = TerminalPanelState::new();

    println!("  Initial state:");
    println!("    Visible: {}", panel_state.visible);
    println!("    Position: {:?}", panel_state.position);
    println!("    Size: {}", panel_state.size);

    panel_state.show();
    println!("  After show():");
    println!("    Visible: {}", panel_state.visible);

    panel_state.set_position(DockPosition::Right);
    println!("  After set_position(Right):");
    println!("    Position: {:?}", panel_state.position);
    println!("    Size: {}", panel_state.size);
    println!();

    // Demo 3: Tab Management
    println!("Demo 3: Tab Management");
    println!("─────────────────────────────────────────────");
    println!("  Initial tabs: {}", panel_state.tab_manager.tabs.len());

    panel_state.tab_manager.add_tab(Some("Build".to_string()));
    panel_state.tab_manager.add_tab(Some("Test".to_string()));

    println!("  After adding 2 tabs: {}", panel_state.tab_manager.tabs.len());
    println!("  Active tab index: {}", panel_state.tab_manager.active_index);

    if let Some(active_tab) = panel_state.tab_manager.active_tab() {
        println!("  Active tab title: {}", active_tab.title);
    }

    panel_state.tab_manager.next_tab();
    println!("  After next_tab():");
    if let Some(active_tab) = panel_state.tab_manager.active_tab() {
        println!("    Active tab title: {}", active_tab.title);
    }
    println!();

    // Demo 4: Creating UI Application
    println!("Demo 4: Creating UI Application");
    println!("─────────────────────────────────────────────");
    println!("  Starting Floem application...");
    println!("  Keyboard shortcuts:");
    println!("    Ctrl+` - Toggle terminal panel");
    println!("    Ctrl+Shift+T - New terminal tab");
    println!("    Ctrl+Shift+W - Close current tab");
    println!("    Ctrl+Tab - Switch to next tab");
    println!();

    // Create the terminal panel
    let (panel_state_signal, terminal_panel) = create_terminal_panel(theme, font_config);

    // Show the panel by default for the demo
    panel_state_signal.update(|state| {
        state.show();
        state.set_position(DockPosition::Bottom);
    });

    // Create window configuration
    let window_config = WindowConfig::default()
        .title("Terminal UI Demo - Lite XL")
        .size((1200.0, 800.0));

    // Build application view
    let app_view = move || {
        container(
            v_stack((
                // Main content area placeholder
                container(
                    floem::views::label(|| {
                        "Terminal Panel Demo\n\n\
                        The terminal panel is shown at the bottom.\n\
                        Use Ctrl+` to toggle visibility.\n\
                        Use the '+' button to add new terminal tabs.\n\
                        Click on tabs to switch between them.\n\
                        Drag the resize handle to adjust panel size."
                    })
                    .style(|s| {
                        s.font_size(16.0)
                            .padding(20.0)
                    }),
                )
                .style(|s| s.flex_grow(1.0).width_full()),

                // Terminal panel
                terminal_panel,
            ))
        )
        .style(move |s| {
            let theme_val = theme.get();
            s.width_full()
                .height_full()
                .background(theme_val.background)
        })
    };

    // Run the application
    Application::new()
        .window(app_view, Some(window_config))
        .run();

    println!("═══════════════════════════════════════════════");
    println!("  Demo Complete!");
    println!("═══════════════════════════════════════════════");
}

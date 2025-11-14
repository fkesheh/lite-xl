/// Main editor view component
///
/// Handles:
/// - Text rendering
/// - Cursor rendering
/// - Selection rendering
/// - Keyboard input
/// - Scrolling

use floem::{
    event::EventListener,
    keyboard::{Key, KeyEvent, NamedKey},
    peniko::Color,
    reactive::{RwSignal, SignalGet, SignalUpdate},
    style::{AlignItems, CursorStyle},
    View,
    views::{container, dyn_stack, h_stack, h_stack_from_iter, label, scroll, v_stack, Decorators},
};

use crate::editor::{EditorState, Position};
use super::theme::{FontConfig, Theme};
use super::gutter::gutter_view;

/// Create the main editor view with text area
pub fn editor_view(
    editor: RwSignal<EditorState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    let scroll_offset = RwSignal::new(0.0f64);

    let text_area = text_area_view(editor, theme, font_config, scroll_offset);

    h_stack((
        gutter_view(
            editor,
            theme,
            scroll_offset,
            font_config.get().line_height_px(),
            30, // visible lines (approximate)
        ),
        text_area,
    ))
    .style(|s| s.width_full().height_full())
}

/// Text area view - handles text rendering and input
fn text_area_view(
    editor: RwSignal<EditorState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
    scroll_offset: RwSignal<f64>,
) -> impl View {
    container(
        scroll(
            container(
                dyn_stack(
                    move || {
                        let editor_state = editor.get();
                        0..editor_state.line_count()
                    },
                    |line_num| *line_num,
                    move |line_num| {
                        let editor_state = editor.get();
                        let theme_val = theme.get();
                        let font = font_config.get();
                        let line_height = font.line_height_px();
                        let line_text = editor_state.lines().get(line_num).map(|s| s.as_str()).unwrap_or("");
                        let is_current = line_num == editor_state.cursor().line;
                        let has_selection = editor_state.selection().is_some();

                        render_line(
                            line_num,
                            line_text,
                            is_current,
                            has_selection,
                            editor_state.cursor(),
                            editor_state.selection(),
                            theme_val,
                            font,
                            line_height,
                        )
                    }
                )
                .style(|s| s.width_full().flex_col())
            )
            .style(move |s| {
                let theme_val = theme.get();
                s.width_full()
                    .min_height_full()
                    .background(theme_val.background)
                    .padding(8.0)
            })
        )
        .style(|s| s.width_full().height_full())
    )
    .on_event_stop(EventListener::KeyDown, move |event| {
        if let floem::event::Event::KeyDown(key_event) = event {
            handle_key_event(editor, key_event);
        }
    })
    .style(move |s| {
        s.width_full()
            .height_full()
            .cursor(CursorStyle::Text)
            .focus_visible(|s| s.outline(2.0).outline_color(Color::rgb8(100, 100, 255)))
    })
    .keyboard_navigable()
}

/// Render a single line of text
fn render_line(
    line_num: usize,
    line_text: &str,
    is_current: bool,
    has_selection: bool,
    cursor: Position,
    selection: Option<(Position, Position)>,
    theme: Theme,
    font: FontConfig,
    line_height: f32,
) -> impl View {
    let text_to_display = if line_text.is_empty() {
        " ".to_string()
    } else {
        line_text.to_string()
    };

    let has_cursor = cursor.line == line_num;
    let cursor_col = if has_cursor { cursor.col } else { 0 };

    // Determine if this line has selection
    let (sel_start, sel_end) = if let Some((start, end)) = selection {
        let (start, end) = if start.line < end.line || (start.line == end.line && start.col < end.col) {
            (start, end)
        } else {
            (end, start)
        };

        if line_num >= start.line && line_num <= end.line {
            let sel_start_col = if line_num == start.line { start.col } else { 0 };
            let sel_end_col = if line_num == end.line {
                end.col
            } else {
                line_text.len()
            };
            (Some(sel_start_col), Some(sel_end_col))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    container(
        h_stack_from_iter({
            let mut segments = Vec::new();
            let chars: Vec<char> = text_to_display.chars().collect();

            // Extract values we need from font and theme before the loop
            let font_size = font.size;
            let char_width = font.char_width_px();
            let theme_bg = theme.background;
            let theme_fg = theme.foreground;
            let theme_cursor = theme.cursor;
            let theme_selection = theme.selection;

            for (i, ch) in chars.iter().enumerate() {
                let is_cursor_pos = has_cursor && i == cursor_col;
                let is_selected = if let (Some(start), Some(end)) = (sel_start, sel_end) {
                    i >= start && i < end
                } else {
                    false
                };

                let char_str = ch.to_string();
                let bg_color = if is_selected {
                    Some(theme_selection)
                } else if is_cursor_pos {
                    Some(theme_cursor)
                } else {
                    None
                };

                let fg_color = if is_cursor_pos {
                    theme_bg // Invert for cursor
                } else {
                    theme_fg
                };

                segments.push(
                    container(
                        label(move || char_str.clone())
                            .style(move |s| {
                                s.font_size(font_size as f64)
                                    .color(fg_color)
                                    .font_family("monospace".to_string())
                                    .line_height(1.0)
                            })
                    )
                    .style(move |s| {
                        let mut style = s.height(line_height as f64)
                            .min_width(char_width as f64)
                            .align_items(AlignItems::Center);

                        if let Some(bg) = bg_color {
                            style = style.background(bg);
                        }

                        style
                    })
                );
            }

            // Add cursor at end of line if needed
            if has_cursor && cursor_col >= text_to_display.len() {
                segments.push(
                    container(
                        label(|| " ")
                            .style(move |s| {
                                s.font_size(font_size as f64)
                                    .color(theme_bg)
                                    .font_family("monospace".to_string())
                                    .line_height(1.0)
                            })
                    )
                    .style(move |s| {
                        s.height(line_height as f64)
                            .min_width(char_width as f64)
                            .background(theme_cursor)
                            .align_items(AlignItems::Center)
                    })
                );
            }

            segments
        })
    )
    .style(move |s| {
        let mut style = s.width_full()
            .height(line_height as f64)
            .min_height(line_height as f64)
            .align_items(AlignItems::Center);

        if is_current && !has_selection {
            style = style.background(theme.current_line);
        }

        style
    })
}

/// Handle keyboard events
fn handle_key_event(editor: RwSignal<EditorState>, event: &KeyEvent) {
    let key = &event.key.logical_key;
    let modifiers = &event.modifiers;

    let shift = modifiers.shift();
    let ctrl = modifiers.control();
    let alt = modifiers.alt();

    editor.update(|state| {
        match key {
            // Character input
            Key::Character(ch) => {
                if ctrl {
                    // Handle Ctrl+key combinations
                    match ch.as_str() {
                        "a" => state.select_all(),
                        "c" => {
                            // Copy (simplified - would need clipboard integration)
                            if let Some(text) = state.get_selected_text() {
                                println!("Copy: {}", text);
                            }
                        }
                        "x" => {
                            // Cut (simplified - would need clipboard integration)
                            if let Some(text) = state.get_selected_text() {
                                println!("Cut: {}", text);
                                state.delete_backward();
                            }
                        }
                        "v" => {
                            // Paste (simplified - would need clipboard integration)
                            println!("Paste");
                        }
                        _ => {}
                    }
                } else if !alt {
                    // Regular character input
                    for c in ch.chars() {
                        state.insert_char(c);
                    }
                }
            }

            // Navigation keys
            Key::Named(named) => match named {
                NamedKey::ArrowUp => state.move_up(shift),
                NamedKey::ArrowDown => state.move_down(shift),
                NamedKey::ArrowLeft => state.move_left(shift),
                NamedKey::ArrowRight => state.move_right(shift),
                NamedKey::Home => state.move_line_start(shift),
                NamedKey::End => state.move_line_end(shift),
                NamedKey::Backspace => state.delete_backward(),
                NamedKey::Delete => state.delete_forward(),
                NamedKey::Enter => state.insert_newline(),
                NamedKey::Tab => {
                    if !shift {
                        state.insert_string("    ");
                    }
                }
                NamedKey::Escape => {
                    // Clear selection
                    state.move_left(false);
                }
                _ => {}
            },

            _ => {}
        }
    });
}

/// Status bar component
///
/// Displays editor information at the bottom:
/// - Cursor position (line:column)
/// - Total lines
/// - File status (modified/saved)
/// - File type

use floem::{
    reactive::{RwSignal, SignalGet},
    style::{AlignItems, JustifyContent},
    View,
    views::{container, h_stack, label, Decorators},
};

use crate::editor::EditorState;
use super::theme::Theme;

/// Create a status bar view
pub fn statusbar_view(
    editor: RwSignal<EditorState>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        h_stack((
            // Left side - file status
            container(
                label(move || {
                    let state = editor.get();
                    let modified = if state.is_modified() { " [+]" } else { "" };
                    let path = state.file_path().unwrap_or("untitled");
                    format!("{}{}", path, modified)
                })
                .style(move |s| {
                    let theme_val = theme.get();
                    s.font_size(12.0)
                        .color(theme_val.status_bar_fg)
                        .font_family("monospace".to_string())
                })
            )
            .style(|s| s.padding(4.0)),

            // Spacer
            container(label(|| ""))
                .style(|s| s.flex_grow(1.0)),

            // Right side - cursor position and line count
            container(
                h_stack((
                    label(move || {
                        let state = editor.get();
                        let cursor = state.cursor();
                        format!("Ln {}, Col {}", cursor.line + 1, cursor.col + 1)
                    })
                    .style(move |s| {
                        let theme_val = theme.get();
                        s.font_size(12.0)
                            .color(theme_val.status_bar_fg)
                            .font_family("monospace".to_string())
                    }),

                    container(
                        label(|| " | ")
                    )
                    .style(move |s| {
                        let theme_val = theme.get();
                        s.color(theme_val.status_bar_fg)
                            .padding_left(8.0)
                            .padding_right(8.0)
                    }),

                    label(move || {
                        let state = editor.get();
                        format!("{} lines", state.line_count())
                    })
                    .style(move |s| {
                        let theme_val = theme.get();
                        s.font_size(12.0)
                            .color(theme_val.status_bar_fg)
                            .font_family("monospace".to_string())
                    }),

                    container(
                        label(|| " | ")
                    )
                    .style(move |s| {
                        let theme_val = theme.get();
                        s.color(theme_val.status_bar_fg)
                            .padding_left(8.0)
                            .padding_right(8.0)
                    }),

                    label(move || {
                        let state = editor.get();
                        if let Some((start, end)) = state.selection() {
                            let (start, end) = if start.line < end.line || (start.line == end.line && start.col < end.col) {
                                (start, end)
                            } else {
                                (end, start)
                            };

                            let char_count = if start.line == end.line {
                                end.col - start.col
                            } else {
                                // Simplified calculation for multi-line
                                let lines = state.lines();
                                let mut count = lines[start.line].len() - start.col + 1; // +1 for newline
                                for i in start.line + 1..end.line {
                                    count += lines[i].len() + 1; // +1 for newline
                                }
                                count += end.col;
                                count
                            };

                            format!("{} selected", char_count)
                        } else {
                            "UTF-8".to_string()
                        }
                    })
                    .style(move |s| {
                        let theme_val = theme.get();
                        s.font_size(12.0)
                            .color(theme_val.status_bar_fg)
                            .font_family("monospace".to_string())
                    }),
                ))
            )
            .style(|s| s.padding(4.0)),
        ))
    )
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height(28.0)
            .background(theme_val.status_bar_bg)
            .border_top(1.0)
            .border_color(theme_val.border)
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::SpaceBetween)
            .flex_shrink(0.0)
    })
}

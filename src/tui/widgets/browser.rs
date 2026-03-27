use super::shared::{clean_path, file_icon, format_size, key_badge, key_desc};
use crate::tui::{
    app::{App, BrowserFocus, StatusKind},
    theme,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub(crate) fn draw(frame: &mut Frame, app: &mut App) {
    let [header, path_bar_area, body, footer] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .areas(frame.area());

    // ── Compact header ──────────────────────────────────────────────
    let header_line = Line::from(vec![
        Span::styled(
            format!(" AMADiag v{} ", env!("CARGO_PKG_VERSION")),
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "Select an AMA troubleshooter bundle to analyze",
            Style::default().fg(Color::White),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(header_line).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("AMADiag"),
        ),
        header,
    );

    // ── Integrated path input bar ───────────────────────────────────
    let path_focused = app.browser_focus() == BrowserFocus::PathBar;
    let border_style = if path_focused {
        theme::focused_border()
    } else {
        theme::unfocused_border()
    };

    let path_text = if app.input_path().is_empty() && !path_focused {
        Line::from(Span::styled(
            " Type or paste a path to a bundle (.zip, .tgz, folder)…",
            Style::default().fg(Color::DarkGray),
        ))
    } else if app.input_path().is_empty() {
        Line::from(Span::styled(
            " Start typing a path…",
            Style::default().fg(Color::DarkGray),
        ))
    } else {
        Line::from(Span::raw(format!(" {}", app.input_path())))
    };

    let mut path_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(if path_focused {
            Span::styled(
                " Path (editing) ",
                Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(" Path ", Style::default().fg(Color::DarkGray))
        });

    // Show validation hint/error inline in the path bar title
    if let Some(hint) = app.input_hint() {
        path_block = path_block.title_bottom(Line::from(Span::styled(
            format!(" ✓ {hint} "),
            Style::default().fg(theme::SUCCESS),
        )));
    }
    if let Some(error) = app.input_error() {
        path_block = path_block.title_bottom(Line::from(Span::styled(
            format!(" ✗ {error} "),
            Style::default().fg(theme::ERROR),
        )));
    }

    frame.render_widget(
        Paragraph::new(path_text)
            .block(path_block)
            .wrap(Wrap { trim: false }),
        path_bar_area,
    );

    // Show cursor when path bar is focused
    if path_focused {
        let cursor_offset = app.input_path().len() as u16 + 1; // +1 for leading space
        let cursor_x =
            (path_bar_area.x + 1 + cursor_offset).min(path_bar_area.right().saturating_sub(2));
        let cursor_y = path_bar_area.y + 1;
        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    }

    // ── File list ───────────────────────────────────────────────────
    let binding = app.browser_path().display().to_string();
    let full_path = clean_path(&binding);
    let hidden_indicator = if app.show_hidden() {
        " [hidden: shown]"
    } else {
        ""
    };

    let entry_count = app.browser_entries().len();
    let has_bundles = app.browser_entries().iter().any(|entry| entry.is_bundle);

    let list_focused = app.browser_focus() == BrowserFocus::FileList;
    let list_border_style = if list_focused {
        theme::focused_border()
    } else {
        theme::unfocused_border()
    };

    if entry_count == 0 {
        frame.render_widget(
            Paragraph::new("  (empty directory)")
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .border_style(list_border_style)
                        .title(format!(" 📂 {full_path}{hidden_indicator} ")),
                )
                .style(Style::default().fg(Color::DarkGray)),
            body,
        );
    } else {
        let items: Vec<ListItem> = app
            .browser_entries()
            .iter()
            .map(|entry| {
                let (icon, style) = if entry.name == ".." {
                    (
                        "↩ ",
                        Style::default()
                            .fg(theme::DIR_COLOR)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if entry.is_dir {
                    (
                        "📁 ",
                        Style::default()
                            .fg(theme::DIR_COLOR)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if entry.is_bundle {
                    (
                        "🔍 ",
                        Style::default()
                            .fg(theme::BUNDLE_COLOR)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    (
                        file_icon(&entry.name),
                        Style::default().fg(theme::FILE_COLOR),
                    )
                };

                let mut spans = vec![
                    Span::styled(icon, style),
                    Span::styled(entry.name.clone(), style),
                ];

                if let Some(count) = entry.child_count {
                    spans.push(Span::styled(
                        format!("  ({count} items)"),
                        Style::default().fg(Color::DarkGray),
                    ));
                } else if let Some(size) = entry.size {
                    spans.push(Span::styled(
                        format!("  ({})", format_size(size)),
                        Style::default().fg(Color::DarkGray),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let title = format!(" 📂 {full_path}{hidden_indicator}  ({entry_count} items) ");
        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(list_border_style)
            .title(title);

        if !has_bundles {
            block = block.title_bottom(Line::from(Span::styled(
                " 💡 No AMA bundles (.zip, .tgz) found in this directory ",
                Style::default().fg(theme::WARNING),
            )));
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(theme::highlight())
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, body, app.browser_state());
    }

    // ── Footer ──────────────────────────────────────────────────────
    let footer_line = match app.status() {
        Some(status) => Line::from(Span::styled(
            status.text.clone(),
            match status.kind {
                StatusKind::Info => Style::default().fg(Color::Green),
                StatusKind::Error => Style::default().fg(Color::Red),
            },
        )),
        None => {
            if path_focused {
                Line::from(vec![
                    key_badge("Enter"),
                    key_desc("Analyze  "),
                    key_badge("Tab"),
                    key_desc("File list  "),
                    key_badge("Esc"),
                    key_desc("Quit  "),
                    Span::styled(
                        "  Paste supported",
                        Style::default().fg(Color::DarkGray),
                    ),
                ])
            } else {
                Line::from(vec![
                    key_badge("Enter"),
                    key_desc("Select  "),
                    key_badge("Bksp"),
                    key_desc("Parent  "),
                    key_badge("/"),
                    key_desc("Type path  "),
                    key_badge("Tab"),
                    key_desc("Path bar  "),
                    key_badge("h"),
                    key_desc(if app.show_hidden() {
                        "Hide hidden  "
                    } else {
                        "Show hidden  "
                    }),
                    key_badge("Esc"),
                    key_desc("Quit"),
                ])
            }
        }
    };
    frame.render_widget(
        Paragraph::new(footer_line).block(Block::bordered().border_type(BorderType::Rounded)),
        footer,
    );
}

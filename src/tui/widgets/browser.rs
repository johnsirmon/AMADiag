use super::shared::{clean_path, file_icon, format_size, key_badge, key_desc};
use crate::tui::{
    app::{App, StatusKind},
    theme,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

pub(crate) fn draw(frame: &mut Frame, app: &mut App) {
    let [header, body, footer] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .areas(frame.area());

    let [title_area, path_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Min(20)])
        .areas(header);

    let big_title = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::Cyan))
        .lines(vec![Line::from("AMADiag")])
        .build();
    frame.render_widget(
        big_title,
        Block::bordered()
            .border_type(BorderType::Rounded)
            .inner(title_area),
    );
    frame.render_widget(
        Block::bordered().border_type(BorderType::Rounded),
        title_area,
    );

    let binding = app.browser_path().display().to_string();
    let full_path = clean_path(&binding);
    let folder_name = app
        .browser_path()
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| full_path.to_string());
    let hidden_indicator = if app.show_hidden() {
        " [hidden: shown]"
    } else {
        ""
    };
    let path_block = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                format!("v{}", env!("CARGO_PKG_VERSION")),
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw("  "),
            Span::styled(
                "Select an AMA troubleshooter bundle to analyze",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(" 📂 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                &folder_name,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(hidden_indicator, Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(Span::styled(
            format!("    {full_path}"),
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(Block::bordered().border_type(BorderType::Rounded));
    frame.render_widget(path_block, path_area);

    let entry_count = app.browser_entries().len();
    let has_bundles = app.browser_entries().iter().any(|entry| entry.is_bundle);

    if entry_count == 0 {
        frame.render_widget(
            Paragraph::new("  (empty directory)")
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("Files"),
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

        let title = format!("Files ({entry_count} items)");
        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
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

    let footer_line = match app.status() {
        Some(status) => Line::from(Span::styled(
            status.text.clone(),
            match status.kind {
                StatusKind::Info => Style::default().fg(Color::Green),
                StatusKind::Error => Style::default().fg(Color::Red),
            },
        )),
        None => Line::from(vec![
            key_badge("Enter"),
            key_desc("Select  "),
            key_badge("Bksp"),
            key_desc("Parent  "),
            key_badge("t"),
            key_desc("Type path  "),
            key_badge("h"),
            key_desc(if app.show_hidden() {
                "Hide hidden  "
            } else {
                "Show hidden  "
            }),
            key_badge("Esc"),
            key_desc("Quit"),
        ]),
    };
    frame.render_widget(
        Paragraph::new(footer_line).block(Block::bordered().border_type(BorderType::Rounded)),
        footer,
    );
}

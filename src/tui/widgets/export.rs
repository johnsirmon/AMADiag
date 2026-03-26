use super::shared::{centered_rect, key_badge, key_desc};
use crate::{
    reporters::OutputFormat,
    tui::{app::App, theme},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Paragraph, Wrap},
    Frame,
};

pub(crate) fn draw(frame: &mut Frame, app: &mut App) {
    let popup = centered_rect(70, 50, frame.area());
    frame.render_widget(Clear, popup);

    let [title_area, format_area, path_area, hint_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .areas(popup);

    frame.render_widget(
        Paragraph::new(Span::styled(
            " Generate Report",
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        ))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Export"),
        ),
        title_area,
    );

    let md_selected = matches!(app.export_format(), OutputFormat::Markdown);
    let json_selected = matches!(app.export_format(), OutputFormat::Json);
    let md_style = if md_selected {
        Style::default()
            .fg(theme::SUCCESS)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::MUTED)
    };
    let json_style = if json_selected {
        Style::default()
            .fg(theme::SUCCESS)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::MUTED)
    };

    let format_line = Line::from(vec![
        Span::raw("  Format: "),
        Span::styled(
            if md_selected {
                "(*)  Markdown"
            } else {
                "( )  Markdown"
            },
            md_style,
        ),
        Span::raw("    "),
        Span::styled(
            if json_selected {
                "(*)  JSON"
            } else {
                "( )  JSON"
            },
            json_style,
        ),
    ]);
    frame.render_widget(
        Paragraph::new(format_line).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Format [Tab to toggle]"),
        ),
        format_area,
    );

    let path_text = if app.export_path().is_empty() {
        Text::from(Line::from(Span::styled(
            "Enter output path...",
            Style::default().fg(theme::MUTED),
        )))
    } else {
        Text::from(app.export_path().to_string())
    };
    frame.render_widget(
        Paragraph::new(path_text)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Output Path (editable)")
                    .border_style(theme::focused_border()),
            )
            .wrap(Wrap { trim: false }),
        path_area,
    );

    let cursor_offset = app.export_path().len() as u16;
    let cursor_x = (path_area.x + 1 + cursor_offset).min(path_area.right().saturating_sub(2));
    let cursor_y = path_area.y + 1;
    frame.set_cursor_position(Position::new(cursor_x, cursor_y));

    let default_dir = app
        .last_path()
        .and_then(|path| path.parent())
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| ".".to_string());
    let mut hint_lines = vec![
        Line::from(Span::styled(
            format!("  Default directory: {default_dir}"),
            Style::default().fg(theme::MUTED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Report will be written when you press Enter.",
            Style::default().fg(theme::MUTED),
        )),
    ];
    if app.export_overwrite_pending() {
        hint_lines.insert(
            0,
            Line::from(Span::styled(
                "  ⚠ File already exists!",
                Style::default()
                    .fg(theme::WARNING)
                    .add_modifier(Modifier::BOLD),
            )),
        );
    }
    frame.render_widget(
        Paragraph::new(Text::from(hint_lines)).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Info"),
        ),
        hint_area,
    );

    let footer_line = if app.export_overwrite_pending() {
        Line::from(vec![
            Span::styled(
                " ⚠ File exists! ",
                Style::default()
                    .fg(theme::WARNING)
                    .add_modifier(Modifier::BOLD),
            ),
            key_badge("Enter"),
            key_desc("Overwrite  "),
            key_badge("Esc"),
            key_desc("Cancel"),
        ])
    } else {
        Line::from(vec![
            key_badge("Enter"),
            key_desc("Generate  "),
            key_badge("Tab"),
            key_desc("Toggle format  "),
            key_badge("Esc"),
            key_desc("Cancel"),
        ])
    };
    frame.render_widget(
        Paragraph::new(footer_line).block(Block::bordered().border_type(BorderType::Rounded)),
        footer_area,
    );
}

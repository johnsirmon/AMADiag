use super::shared::{centered_rect, clean_path};
use crate::tui::{app::App, theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Paragraph, Wrap},
    Frame,
};

pub(crate) fn draw_path_input(frame: &mut Frame, app: &mut App) {
    let [header, body, footer] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(2),
        ])
        .areas(frame.area());

    frame.render_widget(
        Paragraph::new(format!("AMADiag v{}", env!("CARGO_PKG_VERSION")))
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("AMADiag TUI"),
            )
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        header,
    );

    let popup = centered_rect(80, 40, body);
    frame.render_widget(Clear, popup);

    let [input_area, detail_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(4)])
        .areas(popup);

    let input_text = if app.input_path().is_empty() {
        Text::from(Line::from(Span::styled(
            r"Example: C:\AMA-Diag-Logs or C:\temp\bundle.zip",
            Style::default().fg(Color::DarkGray),
        )))
    } else {
        Text::from(app.input_path().to_string())
    };

    let input = Paragraph::new(input_text)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Input path (.zip, .tgz, .tar.gz, or extracted folder)")
                .border_style(theme::focused_border()),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(input, input_area);

    let cursor_offset = app.input_path().len() as u16;
    let cursor_x = (input_area.x + 1 + cursor_offset).min(input_area.right().saturating_sub(2));
    let cursor_y = input_area.y + 1;
    frame.set_cursor_position(Position::new(cursor_x, cursor_y));

    let mut lines = vec![
        Line::from("Type or paste a path, then press Enter to start analysis."),
        Line::from("Press Ctrl+T to switch back to the file browser."),
        Line::from(""),
    ];
    if let Some(hint) = app.input_hint() {
        lines.push(Line::from(Span::styled(
            hint,
            Style::default().fg(Color::Green),
        )));
    }
    if let Some(error) = app.input_error() {
        lines.push(Line::from(Span::styled(
            error,
            Style::default().fg(Color::Red),
        )));
    }

    let details = Paragraph::new(Text::from(lines))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Validation"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(details, detail_area);

    frame.render_widget(
        Paragraph::new("Enter: analyze  Ctrl+T: file browser  Esc: quit  Paste supported")
            .style(Style::default().fg(Color::DarkGray)),
        footer,
    );
}

pub(crate) fn draw_analyzing(frame: &mut Frame, app: &mut App) {
    let popup = centered_rect(60, 20, frame.area());
    frame.render_widget(Clear, popup);

    let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let current = spinner[app.tick() % spinner.len()];
    let text = Text::from(vec![
        Line::from(Span::styled(
            format!("{current} Analyzing bundle"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(clean_path(app.input_path()).to_string()),
        Line::from(""),
        Line::from("The analyzer is running on a worker thread so the UI stays responsive."),
        Line::from("Press q to quit."),
    ]);

    let paragraph = Paragraph::new(text)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Analyzing"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, popup);
}

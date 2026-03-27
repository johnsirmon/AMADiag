use super::shared::{centered_rect, clean_path};
use crate::tui::app::App;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Paragraph, Wrap},
    Frame,
};

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

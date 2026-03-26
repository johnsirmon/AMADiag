use crate::model::diagnostic::Severity as UiSeverity;
use crate::tui::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

pub(super) fn group_severity_badge(severity: UiSeverity) -> &'static str {
    match severity {
        UiSeverity::Critical => "CRIT",
        UiSeverity::High => "HIGH",
        UiSeverity::Medium => "WARN",
        UiSeverity::Low => "LOW",
        UiSeverity::Info => "INFO",
    }
}

pub(super) fn group_severity_style(severity: UiSeverity) -> Style {
    match severity {
        UiSeverity::Critical | UiSeverity::High => Style::default().fg(theme::SEVERITY_CRITICAL),
        UiSeverity::Medium => Style::default().fg(theme::SEVERITY_WARNING),
        UiSeverity::Low => Style::default().fg(theme::TEXT_DIM),
        UiSeverity::Info => Style::default().fg(theme::SEVERITY_INFO),
    }
}

pub(super) fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [vertical] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area)[1..2]
    else {
        return area;
    };

    let [horizontal] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical)[1..2]
    else {
        return area;
    };

    horizontal
}

pub(super) fn clean_path(s: &str) -> &str {
    s.strip_prefix(r"\\?\").unwrap_or(s)
}

pub(super) fn file_icon(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.ends_with(".zip") || lower.ends_with(".tgz") || lower.ends_with(".tar.gz") {
        "📦 "
    } else if lower.ends_with(".yaml")
        || lower.ends_with(".yml")
        || lower.ends_with(".json")
        || lower.ends_with(".xml")
    {
        "📄 "
    } else if lower.ends_with(".log") || lower.ends_with(".txt") {
        "📝 "
    } else if lower.ends_with(".csv") {
        "📊 "
    } else {
        "   "
    }
}

pub(super) fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

pub(super) fn key_badge(label: &str) -> Span<'_> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .bg(theme::MUTED)
            .fg(theme::TEXT)
            .add_modifier(Modifier::BOLD),
    )
}

pub(super) fn key_desc(desc: &str) -> Span<'_> {
    Span::styled(desc, Style::default().fg(theme::TEXT_DIM))
}

pub(super) fn severity_pill(level: u8, label: &str, filter: u8, color: Color) -> Span<'static> {
    let active = filter >= level;
    let dot = if active { "\u{25cf}" } else { "\u{25cb}" };
    Span::styled(
        format!(" {level}{dot}{label} "),
        if active {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::MUTED)
        },
    )
}

pub(super) fn render_vertical_scrollbar(
    frame: &mut Frame,
    area: Rect,
    content_length: usize,
    scroll_offset: u16,
) {
    let viewport = area.height.saturating_sub(2) as usize;
    if content_length <= viewport {
        return;
    }

    let mut scrollbar_state = ScrollbarState::new(content_length).position(scroll_offset as usize);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("\u{2191}"))
            .end_symbol(Some("\u{2193}")),
        area,
        &mut scrollbar_state,
    );
}

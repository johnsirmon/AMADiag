use super::shared::{
    group_severity_badge, group_severity_style, key_badge, key_desc, render_vertical_scrollbar,
    severity_pill,
};
use crate::tui::{
    app::{App, Focus, StatusKind},
    theme,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Cell, List, ListItem, Paragraph, Row, Sparkline, Table, Wrap},
    Frame,
};

pub(crate) fn draw(frame: &mut Frame, app: &mut App) {
    let [summary_area, body_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .areas(frame.area());

    let [upper_area, evidence_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
        .areas(body_area);

    let [navigator_area, center_area, detail_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(46),
            Constraint::Percentage(34),
        ])
        .areas(upper_area);

    let [timeline_area, findings_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(8)])
        .areas(center_area);

    draw_summary(frame, app, summary_area);
    draw_navigator(frame, app, navigator_area);
    draw_timeline(frame, app, timeline_area);
    draw_findings(frame, app, findings_area);
    draw_details(frame, app, detail_area);
    draw_evidence(frame, app, evidence_area);
    draw_footer(frame, app, footer_area);
}

fn draw_summary(frame: &mut Frame, app: &App, area: Rect) {
    let Some(report) = app.report() else {
        return;
    };

    let env = &report.environment;
    let lines = vec![
        Line::from(vec![
            Span::styled("Host: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                env.hostname
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string()),
                Style::default().fg(theme::ACCENT),
            ),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Platform: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(
                env.platform
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
            ),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("AMA: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                env.ama_version
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string()),
                Style::default().fg(theme::ACCENT),
            ),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("OS: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(env.os.clone().unwrap_or_else(|| "Unknown".to_string())),
        ]),
        Line::from(vec![
            Span::styled("Files: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}  ", report.files_analyzed)),
            Span::styled("│  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Grouped findings: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(app.grouped_finding_count().to_string()),
        ]),
        Line::from(vec![
            Span::styled(
                "Active span: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(app.analyzed_span_label()),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Window: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.current_time_filter_label()),
        ]),
        Line::from(vec![
            Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}  ", app.selected_category_label())),
            Span::styled("│  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Stale logs skipped: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(app.stale_log_files_skipped().to_string()),
        ]),
    ];

    let title = format!(
        " {} — AMADiag v{} ",
        app.bundle_name(),
        env!("CARGO_PKG_VERSION")
    );

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Span::styled(
                        title,
                        Style::default()
                            .fg(theme::ACCENT)
                            .add_modifier(Modifier::BOLD),
                    )),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn draw_navigator(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .navigator_items()
        .iter()
        .map(|(label, _)| ListItem::new(label.clone()))
        .collect();

    let title = if app.focus() == Focus::Navigator {
        "Categories [focus]"
    } else {
        "Categories"
    };

    frame.render_stateful_widget(
        List::new(items)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(title),
            )
            .highlight_style(theme::highlight())
            .highlight_symbol("▶ "),
        area,
        app.navigator_state(),
    );
}

fn draw_timeline(frame: &mut Frame, app: &App, area: Rect) {
    let data = app.timeline_points();
    let max = data.iter().copied().max().unwrap_or(1);
    let event_count = app.timeline_event_count();

    let title = if let Some((start, end)) = app.timeline_range_label() {
        format!("Timeline — {start} to {end} ({event_count} events)")
    } else {
        format!("Timeline ({})", app.current_time_filter_label())
    };

    // Split area: sparkline + 1-line time axis
    let [spark_area, axis_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .areas(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title.clone())
                .inner(area),
        );

    // Render the bordered container
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title),
        area,
    );

    // Render sparkline inside the container
    frame.render_widget(
        Sparkline::default()
            .style(Style::default().fg(theme::ACCENT))
            .max(max)
            .data(data),
        spark_area,
    );

    // Render time axis labels
    if let Some((start, end)) = app.timeline_range_label() {
        let axis_width = axis_area.width as usize;
        let end_len = end.len();
        let padding = axis_width.saturating_sub(start.len() + end_len);
        let axis_text = format!("{start}{:>width$}", end, width = padding + end_len);
        frame.render_widget(
            Paragraph::new(Span::styled(
                axis_text,
                Style::default().fg(Color::DarkGray),
            )),
            axis_area,
        );
    }
}

fn draw_findings(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.grouped_finding_count() == 0 {
        frame.render_widget(
            Paragraph::new("No grouped findings match the current filters.")
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("Findings"),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
        return;
    }

    let header = Row::new(vec!["Sev", "Category", "Title", "Last seen", "Count"])
        .style(
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .grouped_findings()
        .iter()
        .map(|group| {
            Row::new(vec![
                Cell::from(group_severity_badge(group.severity)),
                Cell::from(group.category.to_string()),
                Cell::from(group.title.clone()),
                Cell::from(
                    group
                        .last_seen
                        .map(|ts| ts.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|| "Unknown".to_string()),
                ),
                Cell::from(group.event_count().to_string()),
            ])
            .style(group_severity_style(group.severity))
        })
        .collect();

    let focus_label = if app.focus() == Focus::Findings {
        " [focus]"
    } else {
        ""
    };
    let title = format!(
        "Grouped findings{focus_label} {}/{}",
        app.selected_index().map(|index| index + 1).unwrap_or(0),
        app.grouped_finding_count()
    );

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(14),
            Constraint::Percentage(48),
            Constraint::Length(18),
            Constraint::Length(7),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title),
    )
    .column_spacing(1)
    .row_highlight_style(theme::highlight())
    .highlight_symbol("▶ ");

    frame.render_stateful_widget(table, area, app.findings_state());
}

fn draw_details(frame: &mut Frame, app: &App, area: Rect) {
    let border_style = if app.focus() == Focus::Details {
        theme::focused_border()
    } else {
        theme::unfocused_border()
    };

    let text = if let Some(group) = app.selected_group() {
        let mut lines = vec![
            Line::from(Span::styled(
                format!("{} {}", group_severity_badge(group.severity), group.title),
                group_severity_style(group.severity).add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("Category: {}", group.category)),
            Line::from(format!("Status: {}", group.status)),
            Line::from(format!("Events: {}", group.event_count())),
            Line::from(format!(
                "Window: {} -> {}",
                group
                    .first_seen
                    .map(|ts| ts.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
                group
                    .last_seen
                    .map(|ts| ts.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Summary",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(group.summary.clone()),
            Line::from(""),
            Line::from(Span::styled(
                "Likely causes",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ];

        if group.likely_causes.is_empty() {
            lines.push(Line::from("No likely causes inferred."));
        } else {
            for cause in &group.likely_causes {
                lines.push(Line::from(format!("  • {cause}")));
            }
        }

        lines.extend([
            Line::from(""),
            Line::from(Span::styled(
                "Suggested next actions",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]);

        if group.suggested_actions.is_empty() {
            lines.push(Line::from("No suggested actions captured."));
        } else {
            for action in &group.suggested_actions {
                lines.push(Line::from(format!("  • {action}")));
            }
        }

        lines.extend([
            Line::from(""),
            Line::from(Span::styled(
                format!("Evidence previews ({})", group.evidence.len()),
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]);

        if group.evidence.is_empty() {
            lines.push(Line::from("No direct evidence lines captured."));
        } else {
            for evidence in &group.evidence {
                lines.push(Line::from(format!(
                    "  • {}:{}  {}",
                    evidence.file_id, evidence.line_range.start, evidence.preview
                )));
            }
        }

        if !group.doc_links.is_empty() {
            lines.extend([
                Line::from(""),
                Line::from(Span::styled(
                    "Documentation",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
            ]);
            for link in &group.doc_links {
                lines.push(Line::from(link.clone()));
            }
        }

        Text::from(lines)
    } else {
        Text::from("Run an analysis to see grouped finding details here.")
    };

    let content_length = text.lines.len();
    let details = Paragraph::new(text)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title("Details"),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.detail_scroll(), 0));
    frame.render_widget(details, area);
    render_vertical_scrollbar(frame, area, content_length, app.detail_scroll());
}

fn draw_evidence(frame: &mut Frame, app: &App, area: Rect) {
    let border_style = if app.focus() == Focus::Evidence {
        theme::focused_border()
    } else {
        theme::unfocused_border()
    };

    let lines: Vec<Line> = app
        .evidence_lines()
        .iter()
        .map(|line| Line::from(line.clone()))
        .collect();
    let text = if lines.is_empty() {
        Text::from("No evidence context available.")
    } else {
        Text::from(lines)
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title("Evidence context"),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.evidence_scroll(), 0));
    frame.render_widget(paragraph, area);
    render_vertical_scrollbar(
        frame,
        area,
        app.evidence_lines().len(),
        app.evidence_scroll(),
    );
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let content = match app.status() {
        Some(status) => Line::from(Span::styled(
            status.text.clone(),
            match status.kind {
                StatusKind::Info => Style::default().fg(theme::SUCCESS),
                StatusKind::Error => Style::default().fg(theme::ERROR),
            },
        )),
        None => {
            let filter = app.severity_filter();
            Line::from(vec![
                key_badge("Tab"),
                key_desc("Pane  "),
                key_badge("M"),
                key_desc("MD  "),
                key_badge("J"),
                key_desc("JSON  "),
                key_badge("T"),
                key_desc("Time  "),
                key_badge("R"),
                key_desc("Rerun  "),
                key_badge("N"),
                key_desc("New  "),
                severity_pill(1, "C", filter, theme::SEVERITY_CRITICAL),
                severity_pill(2, "W", filter, theme::SEVERITY_WARNING),
                severity_pill(3, "I", filter, theme::SEVERITY_INFO),
                Span::raw(" "),
                key_badge("Q"),
                key_desc("Quit"),
            ])
        }
    };

    frame.render_widget(
        Paragraph::new(content).block(Block::bordered().border_type(BorderType::Rounded)),
        area,
    );
}

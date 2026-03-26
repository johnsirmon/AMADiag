use super::app::{App, Focus, Screen, StatusKind};
use super::theme;
use crate::model::diagnostic::Severity as UiSeverity;
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Cell, Clear, List, ListItem, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Sparkline, Table, Wrap,
    },
    Frame,
};
use tui_big_text::{BigText, PixelSize};

pub fn draw(frame: &mut Frame, app: &mut App) {
    match app.screen() {
        Screen::FileBrowser => draw_file_browser(frame, app),
        Screen::PathInput => draw_path_input(frame, app),
        Screen::Analyzing => draw_analyzing(frame, app),
        Screen::Dashboard => draw_dashboard(frame, app),
        Screen::Export => draw_export(frame, app),
    }
}

// ── File Browser ──────────────────────────────────────────────────────

fn draw_file_browser(frame: &mut Frame, app: &mut App) {
    let [header, body, footer] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .areas(frame.area());

    // Header: big text title with version + path breadcrumb merged
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

    // File list with separator between dirs and files
    let entry_count = app.browser_entries().len();
    let has_bundles = app.browser_entries().iter().any(|e| e.is_bundle);

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
        let mut items: Vec<ListItem> = Vec::new();

        for (_i, entry) in app.browser_entries().iter().enumerate() {
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

            // Append size or item count
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

            items.push(ListItem::new(Line::from(spans)));
        }

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

    // Footer with pill-badge keybindings
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

fn draw_path_input(frame: &mut Frame, app: &mut App) {
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

    // Show cursor at end of input text
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

fn draw_analyzing(frame: &mut Frame, app: &mut App) {
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

fn draw_dashboard(frame: &mut Frame, app: &mut App) {
    let [summary_area, body_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
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
        .constraints([Constraint::Length(5), Constraint::Min(8)])
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
            Span::styled(
                format!("AMADiag v{}  ", env!("CARGO_PKG_VERSION")),
                Style::default()
                    .fg(theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("Bundle: {}  ", clean_path(&report.bundle_path))),
            Span::raw(format!("Files: {}  ", report.files_analyzed)),
            Span::raw(format!("Legacy findings: {}  ", report.findings.len())),
            Span::raw(format!("Grouped findings: {}", app.grouped_finding_count())),
        ]),
        Line::from(vec![
            Span::raw(format!(
                "Platform: {}  ",
                env.platform
                    .map(|platform| platform.to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            )),
            Span::raw(format!(
                "OS: {}  ",
                env.os.clone().unwrap_or_else(|| "Unknown".to_string())
            )),
            Span::raw(format!(
                "AMA: {}  ",
                env.ama_version
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
            )),
            Span::raw(format!(
                "Host: {}",
                env.hostname
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
            )),
        ]),
        Line::from(vec![
            Span::styled("Window: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.current_time_filter_label()),
            Span::raw("  "),
            Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.selected_category_label()),
            Span::raw("  "),
            Span::styled("Visible groups: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.grouped_finding_count().to_string()),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Ops cockpit"),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn draw_navigator(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .navigator_items()
        .into_iter()
        .map(|(label, _)| ListItem::new(label))
        .collect();

    let title = if app.focus() == Focus::Navigator {
        "Categories [focus]"
    } else {
        "Categories"
    };

    frame.render_stateful_widget(
        List::new(items)
            .block(Block::bordered().border_type(BorderType::Rounded).title(title))
            .highlight_style(theme::highlight())
            .highlight_symbol("▶ "),
        area,
        app.navigator_state(),
    );
}

fn draw_timeline(frame: &mut Frame, app: &App, area: Rect) {
    let data = app.timeline_points();
    let max = data.iter().copied().max().unwrap_or(1);
    let title = format!("Timeline ({})", app.current_time_filter_label());
    frame.render_widget(
        Sparkline::default()
            .block(Block::bordered().border_type(BorderType::Rounded).title(title))
            .style(Style::default().fg(theme::ACCENT))
            .max(max)
            .data(data),
        area,
    );
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
        .style(Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD))
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
                    group.last_seen
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
    .block(Block::bordered().border_type(BorderType::Rounded).title(title))
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
                lines.push(Line::from(format!("  \u{2022} {cause}")));
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
                lines.push(Line::from(format!("  \u{2022} {action}")));
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
                    "  \u{2022} {}:{}  {}",
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

    // Scrollbar
    let viewport = area.height.saturating_sub(2) as usize;
    if content_length > viewport {
        let mut scrollbar_state =
            ScrollbarState::new(content_length).position(app.detail_scroll() as usize);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("\u{2191}"))
                .end_symbol(Some("\u{2193}")),
            area,
            &mut scrollbar_state,
        );
    }
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

    let viewport = area.height.saturating_sub(2) as usize;
    if app.evidence_lines().len() > viewport {
        let mut scrollbar_state =
            ScrollbarState::new(app.evidence_lines().len()).position(app.evidence_scroll() as usize);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("\u{2191}"))
                .end_symbol(Some("\u{2193}")),
            area,
            &mut scrollbar_state,
        );
    }
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let content = match app.status() {
        Some(status) => Line::from(Span::styled(
            status.text.clone(),
            match status.kind {
                StatusKind::Info => Style::default().fg(Color::Green),
                StatusKind::Error => Style::default().fg(Color::Red),
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

    let paragraph =
        Paragraph::new(content).block(Block::bordered().border_type(BorderType::Rounded));
    frame.render_widget(paragraph, area);
}

// ── Export Screen ─────────────────────────────────────────────────────

fn draw_export(frame: &mut Frame, app: &mut App) {
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

    // Title
    frame.render_widget(
        Paragraph::new(Span::styled(
            " Generate Report",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Export"),
        ),
        title_area,
    );

    // Format selector
    let md_style = if matches!(
        app.export_format(),
        crate::reporters::OutputFormat::Markdown
    ) {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let json_style = if matches!(app.export_format(), crate::reporters::OutputFormat::Json) {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let format_line = Line::from(vec![
        Span::raw("  Format: "),
        Span::styled(
            if matches!(
                app.export_format(),
                crate::reporters::OutputFormat::Markdown
            ) {
                "(*)  Markdown"
            } else {
                "( )  Markdown"
            },
            md_style,
        ),
        Span::raw("    "),
        Span::styled(
            if matches!(app.export_format(), crate::reporters::OutputFormat::Json) {
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

    // Path field
    let path_text = if app.export_path().is_empty() {
        Text::from(Line::from(Span::styled(
            "Enter output path...",
            Style::default().fg(Color::DarkGray),
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

    // Show cursor at end of export path
    let cursor_offset = app.export_path().len() as u16;
    let cursor_x = (path_area.x + 1 + cursor_offset).min(path_area.right().saturating_sub(2));
    let cursor_y = path_area.y + 1;
    frame.set_cursor_position(Position::new(cursor_x, cursor_y));

    // Hint
    let default_dir = app
        .last_path()
        .and_then(|p| p.parent())
        .map(|p| p.display().to_string())
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
                "  \u{26a0} File already exists!",
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

    // Footer
    let footer_line = if app.export_overwrite_pending() {
        Line::from(vec![
            Span::styled(
                " \u{26a0} File exists! ",
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

fn group_severity_badge(severity: UiSeverity) -> &'static str {
    match severity {
        UiSeverity::Critical => "CRIT",
        UiSeverity::High => "HIGH",
        UiSeverity::Medium => "WARN",
        UiSeverity::Low => "LOW",
        UiSeverity::Info => "INFO",
    }
}

fn group_severity_style(severity: UiSeverity) -> Style {
    match severity {
        UiSeverity::Critical => Style::default().fg(theme::SEVERITY_CRITICAL),
        UiSeverity::High => Style::default().fg(theme::SEVERITY_CRITICAL),
        UiSeverity::Medium => Style::default().fg(theme::SEVERITY_WARNING),
        UiSeverity::Low => Style::default().fg(theme::TEXT_DIM),
        UiSeverity::Info => Style::default().fg(theme::SEVERITY_INFO),
    }
}

fn severity_totals(report: &crate::analyzers::finding::DiagnosticReport) -> String {
    format!(
        "Critical: {}  Warning: {}  Info: {}",
        report.finding_count_by_severity(crate::analyzers::finding::Severity::Critical),
        report.finding_count_by_severity(crate::analyzers::finding::Severity::Warning),
        report.finding_count_by_severity(crate::analyzers::finding::Severity::Info),
    )
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
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

/// Strip the Windows extended-length path prefix `\\?\` from display strings.
fn clean_path(s: &str) -> &str {
    s.strip_prefix(r"\\?\").unwrap_or(s)
}

/// Return a file-type icon based on extension.
fn file_icon(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.ends_with(".zip") || lower.ends_with(".tgz") || lower.ends_with(".tar.gz") {
        "📦 "
    } else if lower.ends_with(".yaml") || lower.ends_with(".yml") {
        "📄 "
    } else if lower.ends_with(".json") {
        "📄 "
    } else if lower.ends_with(".xml") {
        "📄 "
    } else if lower.ends_with(".log") || lower.ends_with(".txt") {
        "📝 "
    } else if lower.ends_with(".csv") {
        "📊 "
    } else {
        "   "
    }
}

/// Format a byte count into a human-readable string.
fn format_size(bytes: u64) -> String {
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

/// Render a key label as a pill badge (dark bg, white text).
fn key_badge(label: &str) -> Span<'_> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .bg(theme::MUTED)
            .fg(theme::TEXT)
            .add_modifier(Modifier::BOLD),
    )
}

/// Render a key description in muted text.
fn key_desc(desc: &str) -> Span<'_> {
    Span::styled(desc, Style::default().fg(theme::TEXT_DIM))
}

/// Render a severity filter pill with active/inactive state.
fn severity_pill(level: u8, label: &str, filter: u8, color: Color) -> Span<'static> {
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

use super::app::{App, Focus, Screen, StatusKind};
use super::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Wrap,
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
    let path_str = clean_path(&binding);
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
        Line::from(""),
        Line::from(vec![
            Span::styled(" 📂 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                path_str,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(hidden_indicator, Style::default().fg(Color::DarkGray)),
        ]),
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
    let [summary_area, main_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .areas(frame.area());

    draw_summary(frame, app, summary_area);

    let [findings_area, detail_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .areas(main_area);

    draw_findings(frame, app, findings_area);
    draw_details(frame, app, detail_area);
    draw_footer(frame, app, footer_area);
}

fn draw_summary(frame: &mut Frame, app: &App, area: Rect) {
    let Some(report) = app.report() else {
        return;
    };

    let [left, middle, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .areas(area);

    let bundle_summary = Paragraph::new(Text::from(vec![
        Line::from(format!("Bundle: {}", clean_path(&report.bundle_path))),
        Line::from(format!("Files analyzed: {}", report.files_analyzed)),
        Line::from(format!("Findings: {}", report.findings.len())),
    ]))
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(format!("Summary — AMADiag v{}", env!("CARGO_PKG_VERSION"))),
    )
    .wrap(Wrap { trim: false });
    frame.render_widget(bundle_summary, left);

    let env = &report.environment;
    let environment = Paragraph::new(Text::from(vec![
        Line::from(format!(
            "Platform: {}",
            env.platform
                .map(|platform| platform.to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        )),
        Line::from(format!(
            "OS: {}",
            env.os.clone().unwrap_or_else(|| "Unknown".to_string())
        )),
        Line::from(format!(
            "AMA: {}",
            env.ama_version
                .clone()
                .unwrap_or_else(|| "Unknown".to_string())
        )),
        Line::from(format!(
            "Host: {}",
            env.hostname
                .clone()
                .unwrap_or_else(|| "Unknown".to_string())
        )),
    ]))
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Environment"),
    )
    .wrap(Wrap { trim: false });
    frame.render_widget(environment, middle);

    let root_cause = app.primary_finding();
    let root_cause_lines = if let Some(finding) = root_cause {
        vec![
            Line::from(Span::styled(
                format!("{} {}", severity_badge(finding.severity), finding.name),
                severity_style(finding.severity).add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("Rule: {}", finding.rule_id)),
            Line::from(format!("Category: {}", finding.category)),
            Line::from(""),
            Line::from(severity_totals(report)),
        ]
    } else {
        vec![
            Line::from("No findings detected."),
            Line::from(""),
            Line::from(severity_totals(report)),
        ]
    };

    let focus = Paragraph::new(Text::from(root_cause_lines))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Root cause focus"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(focus, right);
}

fn draw_findings(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.report().is_none() {
        return;
    }

    let filtered_count = app.filtered_finding_count();

    if filtered_count == 0 {
        let msg = if app.severity_filter() < 3 {
            "No findings match the current severity filter. Press 3 to show all."
        } else {
            "No issues detected. The AMA configuration appears healthy."
        };
        frame.render_widget(
            Paragraph::new(msg)
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

    // Build items from filtered findings (immutable borrow scoped here)
    let items: Vec<ListItem> = app
        .filtered_findings()
        .iter()
        .map(|finding| {
            let line = Line::from(vec![
                Span::styled(
                    format!("{} ", severity_badge(finding.severity)),
                    severity_style(finding.severity),
                ),
                Span::raw(format!("[{}] {}", finding.rule_id, finding.name)),
            ]);
            ListItem::new(line)
        })
        .collect();

    // Build title with severity counts (separate borrow scope)
    let (crit_count, warn_count, info_count) = {
        let report = app.report().unwrap();
        (
            report.finding_count_by_severity(crate::analyzers::finding::Severity::Critical),
            report.finding_count_by_severity(crate::analyzers::finding::Severity::Warning),
            report.finding_count_by_severity(crate::analyzers::finding::Severity::Info),
        )
    };

    let filter_label = match app.severity_filter() {
        1 => " [Crit only]",
        2 => " [Crit+Warn]",
        _ => "",
    };

    let focus_label = if app.focus() == Focus::Findings {
        " [focus]"
    } else {
        ""
    };

    let selected_display = app.selected_index().map(|i| i + 1).unwrap_or(0);
    let title = format!(
        "Findings{focus_label} {selected_display}/{filtered_count} ({crit_count}C {warn_count}W {info_count}I){filter_label}",
    );

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, app.findings_state());
}

fn draw_details(frame: &mut Frame, app: &App, area: Rect) {
    let border_style = if app.focus() == Focus::Details {
        theme::focused_border()
    } else {
        theme::unfocused_border()
    };

    let text = if let Some(finding) = app
        .selected_filtered_finding()
        .or_else(|| app.primary_finding())
    {
        let mut lines = vec![
            Line::from(Span::styled(
                format!("{} {}", severity_badge(finding.severity), finding.name),
                severity_style(finding.severity).add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("Rule: {}", finding.rule_id)),
            Line::from(format!("Severity: {}", finding.severity)),
            Line::from(format!("Category: {}", finding.category)),
            Line::from(""),
            Line::from(Span::styled(
                "Description",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(finding.description.clone()),
            Line::from(""),
            Line::from(Span::styled(
                format!("Evidence ({})", finding.evidence.len()),
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ];

        // Evidence lines (placed right after header)
        if finding.evidence.is_empty() {
            lines.push(Line::from(
                "No direct evidence lines captured for this finding.",
            ));
        } else {
            for evidence in &finding.evidence {
                lines.push(Line::from(format!("  \u{2022} {evidence}")));
            }
        }

        lines.extend([
            Line::from(""),
            Line::from(Span::styled(
                "Remediation",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(finding.remediation.clone()),
            Line::from(""),
            Line::from(Span::styled(
                "Documentation",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(
                finding
                    .doc_link
                    .clone()
                    .unwrap_or_else(|| "No documentation link supplied.".to_string()),
            ),
        ]);

        Text::from(lines)
    } else {
        Text::from("Run an analysis to see detailed findings here.")
    };

    let content_length = text.lines.len();
    let details = Paragraph::new(text)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title("Finding details"),
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
                key_badge("M"),
                key_desc("MD  "),
                key_badge("J"),
                key_desc("JSON  "),
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
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .wrap(Wrap { trim: false }),
        path_area,
    );

    // Hint
    let default_dir = app
        .last_path()
        .and_then(|p| p.parent())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| ".".to_string());
    let hint_lines = vec![
        Line::from(Span::styled(
            format!("  Default directory: {default_dir}"),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Report will be written when you press Enter.",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    frame.render_widget(
        Paragraph::new(Text::from(hint_lines)).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Info"),
        ),
        hint_area,
    );

    // Footer
    let footer_line = Line::from(vec![
        key_badge("Enter"),
        key_desc("Generate  "),
        key_badge("Tab"),
        key_desc("Toggle format  "),
        key_badge("Esc"),
        key_desc("Cancel"),
    ]);
    frame.render_widget(
        Paragraph::new(footer_line).block(Block::bordered().border_type(BorderType::Rounded)),
        footer_area,
    );
}

fn severity_badge(severity: crate::analyzers::finding::Severity) -> &'static str {
    match severity {
        crate::analyzers::finding::Severity::Critical => "CRIT",
        crate::analyzers::finding::Severity::Warning => "WARN",
        crate::analyzers::finding::Severity::Info => "INFO",
    }
}

fn severity_style(severity: crate::analyzers::finding::Severity) -> Style {
    match severity {
        crate::analyzers::finding::Severity::Critical => Style::default().fg(Color::Red),
        crate::analyzers::finding::Severity::Warning => Style::default().fg(Color::Yellow),
        crate::analyzers::finding::Severity::Info => Style::default().fg(Color::Blue),
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
            .bg(Color::DarkGray)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )
}

/// Render a key description in muted text.
fn key_desc(desc: &str) -> Span<'_> {
    Span::styled(desc, Style::default().fg(Color::Gray))
}

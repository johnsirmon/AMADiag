use super::app::{App, Focus, Screen, StatusKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

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
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .areas(frame.area());

    // Header
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                " AMADiag ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  Select an AMA troubleshooter bundle to analyze"),
        ]))
        .block(Block::default().borders(Borders::ALL)),
        header,
    );

    // Body: breadcrumb + file list
    let [breadcrumb_area, list_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(4)])
        .areas(body);

    // Breadcrumb
    let path_str = app.browser_path().display().to_string();
    let breadcrumb = Paragraph::new(Line::from(vec![
        Span::styled(" Location: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(&path_str, Style::default().fg(Color::White)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Current Directory"),
    );
    frame.render_widget(breadcrumb, breadcrumb_area);

    // File list
    let entries = app.browser_entries();
    if entries.is_empty() {
        frame.render_widget(
            Paragraph::new("  (empty directory)")
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .style(Style::default().fg(Color::DarkGray)),
            list_area,
        );
    } else {
        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| {
                let (marker, style) = if entry.is_dir {
                    (
                        "DIR  ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if entry.is_bundle {
                    (
                        "AMA  ",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ("     ", Style::default().fg(Color::DarkGray))
                };
                let line = Line::from(vec![
                    Span::styled(marker, style),
                    Span::styled(&entry.name, style),
                ]);
                ListItem::new(line)
            })
            .collect();

        let title = format!("Files ({} items)", entries.len());
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, list_area, app.browser_state());
    }

    // Footer
    let footer_line = match app.status() {
        Some(status) => Line::from(Span::styled(
            status.text.clone(),
            match status.kind {
                StatusKind::Info => Style::default().fg(Color::Green),
                StatusKind::Error => Style::default().fg(Color::Red),
            },
        )),
        None => Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(Color::Cyan)),
            Span::raw("Select  "),
            Span::styled("[Backspace] ", Style::default().fg(Color::Cyan)),
            Span::raw("Parent  "),
            Span::styled("[t] ", Style::default().fg(Color::Cyan)),
            Span::raw("Type path  "),
            Span::styled("[Esc/q] ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit"),
        ]),
    };
    frame.render_widget(
        Paragraph::new(footer_line).block(Block::default().borders(Borders::ALL)),
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
        Paragraph::new("AMADiag Interactive")
            .block(Block::default().borders(Borders::ALL).title("AMADiag TUI"))
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
            Block::default()
                .borders(Borders::ALL)
                .title("Input path (.zip, .tgz, .tar.gz, or extracted folder)"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(input, input_area);

    let mut lines = vec![
        Line::from("Type or paste a path, then press Enter to start analysis."),
        Line::from("This first version is path-driven on purpose to keep the integration small."),
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
        .block(Block::default().borders(Borders::ALL).title("Validation"))
        .wrap(Wrap { trim: false });
    frame.render_widget(details, detail_area);

    frame.render_widget(
        Paragraph::new("Enter: analyze  Esc/q: quit  Paste supported")
            .style(Style::default().fg(Color::DarkGray)),
        footer,
    );
}

fn draw_analyzing(frame: &mut Frame, app: &mut App) {
    let popup = centered_rect(60, 20, frame.area());
    frame.render_widget(Clear, popup);

    let spinner = ["|", "/", "-", "\\"];
    let current = spinner[app.tick() % spinner.len()];
    let text = Text::from(vec![
        Line::from(Span::styled(
            format!("{current} Analyzing bundle"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(app.input_path()),
        Line::from(""),
        Line::from("The analyzer is running on a worker thread so the UI stays responsive."),
        Line::from("Press q to quit."),
    ]);

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Analyzing"))
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
        Line::from(format!("Bundle: {}", report.bundle_path)),
        Line::from(format!("Files analyzed: {}", report.files_analyzed)),
        Line::from(format!("Findings: {}", report.findings.len())),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Summary"))
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
    .block(Block::default().borders(Borders::ALL).title("Environment"))
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
            Block::default()
                .borders(Borders::ALL)
                .title("Root cause focus"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(focus, right);
}

fn draw_findings(frame: &mut Frame, app: &mut App, area: Rect) {
    let Some(report) = app.report() else {
        return;
    };

    if report.findings.is_empty() {
        frame.render_widget(
            Paragraph::new("No issues detected. The AMA configuration appears healthy.")
                .block(Block::default().borders(Borders::ALL).title("Findings"))
                .wrap(Wrap { trim: true }),
            area,
        );
        return;
    }

    let items: Vec<ListItem> = report
        .findings
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

    let title = if app.focus() == Focus::Findings {
        format!(
            "Findings [focus] {}/{}",
            app.selected_index().map(|i| i + 1).unwrap_or(0),
            app.finding_count()
        )
    } else {
        format!(
            "Findings {}/{}",
            app.selected_index().map(|i| i + 1).unwrap_or(0),
            app.finding_count()
        )
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, app.findings_state());
}

fn draw_details(frame: &mut Frame, app: &App, area: Rect) {
    let title = if app.focus() == Focus::Details {
        format!("Finding details [focus]  scroll {}", app.detail_scroll())
    } else {
        format!("Finding details  scroll {}", app.detail_scroll())
    };

    let text = if let Some(finding) = app.selected_finding().or_else(|| app.primary_finding()) {
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
        ];

        if finding.evidence.is_empty() {
            lines.push(Line::from(
                "No direct evidence lines captured for this finding.",
            ));
        } else {
            for evidence in &finding.evidence {
                lines.push(Line::from(format!("- {evidence}")));
            }
        }

        Text::from(lines)
    } else {
        Text::from("Run an analysis to see detailed findings here.")
    };

    let details = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false })
        .scroll((app.detail_scroll(), 0));
    frame.render_widget(details, area);
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let status_line = match app.status() {
        Some(status) => Span::styled(
            status.text.clone(),
            match status.kind {
                StatusKind::Info => Style::default().fg(Color::Green),
                StatusKind::Error => Style::default().fg(Color::Red),
            },
        ),
        None => Span::styled(
            "Up/Down: select  PgUp/PgDn: scroll  Home/End: jump  Left/Right/Tab: switch pane  m: export .md  j: export .json  n/Esc: change path  r: rerun  q: quit",
            Style::default().fg(Color::DarkGray),
        ),
    };

    let paragraph = Paragraph::new(Line::from(status_line))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    frame.render_widget(paragraph, area);
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

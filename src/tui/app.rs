use crate::analyzers::finding::{DiagnosticReport, Finding, Severity};
use crate::input;
use crate::reporters::OutputFormat;
use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    PathInput,
    Analyzing,
    Dashboard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Findings,
    Details,
}

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    Submit,
    Backspace,
    InputChar(char),
    Paste(String),
    Next,
    Previous,
    PageDown,
    PageUp,
    FocusNext,
    EditPath,
    Retry,
    ExportMarkdown,
    ExportJson,
}

pub enum ActionResult {
    None,
    Quit,
    Analyze(PathBuf),
    Export(OutputFormat),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    Info,
    Error,
}

#[derive(Debug, Clone)]
pub struct StatusMessage {
    pub kind: StatusKind,
    pub text: String,
    expires_at: Option<Instant>,
}

impl StatusMessage {
    fn new(kind: StatusKind, text: impl Into<String>, ttl: Option<Duration>) -> Self {
        Self {
            kind,
            text: text.into(),
            expires_at: ttl.map(|ttl| Instant::now() + ttl),
        }
    }

    fn is_expired(&self) -> bool {
        self.expires_at
            .is_some_and(|expires_at| Instant::now() >= expires_at)
    }
}

pub struct App {
    screen: Screen,
    focus: Focus,
    input_path: String,
    input_hint: Option<String>,
    input_error: Option<String>,
    report: Option<DiagnosticReport>,
    findings_state: ListState,
    detail_scroll: u16,
    status: Option<StatusMessage>,
    pending_analysis: Option<PathBuf>,
    last_path: Option<PathBuf>,
    tick: usize,
}

impl App {
    pub fn new(initial_path: Option<PathBuf>) -> Self {
        let input_path = initial_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_default();
        let pending_analysis = initial_path.clone();
        let screen = if pending_analysis.is_some() {
            Screen::Analyzing
        } else {
            Screen::PathInput
        };

        let mut app = Self {
            screen,
            focus: Focus::Findings,
            input_path,
            input_hint: None,
            input_error: None,
            report: None,
            findings_state: ListState::default(),
            detail_scroll: 0,
            status: None,
            pending_analysis,
            last_path: initial_path,
            tick: 0,
        };
        app.findings_state.select(None);
        app.refresh_input_validation();
        app
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn focus(&self) -> Focus {
        self.focus
    }

    pub fn input_path(&self) -> &str {
        &self.input_path
    }

    pub fn input_hint(&self) -> Option<&str> {
        self.input_hint.as_deref()
    }

    pub fn input_error(&self) -> Option<&str> {
        self.input_error.as_deref()
    }

    pub fn report(&self) -> Option<&DiagnosticReport> {
        self.report.as_ref()
    }

    pub fn findings_state(&mut self) -> &mut ListState {
        &mut self.findings_state
    }

    pub fn selected_finding(&self) -> Option<&Finding> {
        let report = self.report.as_ref()?;
        let index = self.findings_state.selected()?;
        report.findings.get(index)
    }

    pub fn primary_finding(&self) -> Option<&Finding> {
        self.report
            .as_ref()?
            .findings
            .iter()
            .max_by_key(|finding| severity_rank(finding.severity))
    }

    pub fn detail_scroll(&self) -> u16 {
        self.detail_scroll
    }

    pub fn status(&self) -> Option<&StatusMessage> {
        self.status.as_ref()
    }

    pub fn tick(&self) -> usize {
        self.tick
    }

    pub fn take_pending_analysis(&mut self) -> Option<PathBuf> {
        self.pending_analysis.take()
    }

    pub fn handle_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::Quit => ActionResult::Quit,
            Action::Submit => self.submit_path(),
            Action::Backspace => {
                if self.screen == Screen::PathInput {
                    self.input_path.pop();
                    self.refresh_input_validation();
                }
                ActionResult::None
            }
            Action::InputChar(ch) => {
                if self.screen == Screen::PathInput {
                    self.input_path.push(ch);
                    self.refresh_input_validation();
                }
                ActionResult::None
            }
            Action::Paste(text) => {
                if self.screen == Screen::PathInput {
                    self.input_path.push_str(&text);
                    self.refresh_input_validation();
                }
                ActionResult::None
            }
            Action::Next => {
                if self.screen == Screen::Dashboard {
                    self.move_next();
                }
                ActionResult::None
            }
            Action::Previous => {
                if self.screen == Screen::Dashboard {
                    self.move_previous();
                }
                ActionResult::None
            }
            Action::PageDown => {
                if self.screen == Screen::Dashboard {
                    self.scroll_details(8);
                }
                ActionResult::None
            }
            Action::PageUp => {
                if self.screen == Screen::Dashboard {
                    self.detail_scroll = self.detail_scroll.saturating_sub(8);
                }
                ActionResult::None
            }
            Action::FocusNext => {
                if self.screen == Screen::Dashboard {
                    self.focus = match self.focus {
                        Focus::Findings => Focus::Details,
                        Focus::Details => Focus::Findings,
                    };
                }
                ActionResult::None
            }
            Action::EditPath => {
                self.screen = Screen::PathInput;
                self.detail_scroll = 0;
                ActionResult::None
            }
            Action::Retry => {
                if let Some(path) = self.last_path.clone() {
                    self.start_analysis(path)
                } else {
                    ActionResult::None
                }
            }
            Action::ExportMarkdown => {
                if self.screen == Screen::Dashboard && self.report.is_some() {
                    ActionResult::Export(OutputFormat::Markdown)
                } else {
                    ActionResult::None
                }
            }
            Action::ExportJson => {
                if self.screen == Screen::Dashboard && self.report.is_some() {
                    ActionResult::Export(OutputFormat::Json)
                } else {
                    ActionResult::None
                }
            }
        }
    }

    pub fn finish_analysis(&mut self, result: std::result::Result<DiagnosticReport, String>) {
        match result {
            Ok(report) => {
                self.report = Some(report);
                self.screen = Screen::Dashboard;
                self.focus = Focus::Findings;
                self.detail_scroll = 0;
                self.findings_state.select(
                    self.report
                        .as_ref()
                        .and_then(|report| (!report.findings.is_empty()).then_some(0)),
                );

                let finding_count = self
                    .report
                    .as_ref()
                    .map(|report| report.findings.len())
                    .unwrap_or_default();
                self.set_info_status(format!("Analysis complete: {finding_count} finding(s)"));
            }
            Err(err) => {
                self.screen = Screen::PathInput;
                self.set_error_status(format!("Analysis failed: {err}"));
            }
        }
    }

    pub fn set_info_status(&mut self, text: impl Into<String>) {
        self.status = Some(StatusMessage::new(
            StatusKind::Info,
            text,
            Some(Duration::from_secs(5)),
        ));
    }

    pub fn set_error_status(&mut self, text: impl Into<String>) {
        self.status = Some(StatusMessage::new(StatusKind::Error, text, None));
    }

    pub fn on_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
        if self.status.as_ref().is_some_and(StatusMessage::is_expired) {
            self.status = None;
        }
    }

    fn submit_path(&mut self) -> ActionResult {
        if self.screen != Screen::PathInput {
            return ActionResult::None;
        }

        let trimmed = self.input_path.trim();
        if trimmed.is_empty() {
            self.input_error = Some("Enter a path to a bundle or extracted folder.".to_string());
            return ActionResult::None;
        }

        let path = PathBuf::from(trimmed);
        match input::detect_format(&path) {
            Ok(_) => self.start_analysis(path),
            Err(err) => {
                self.input_error = Some(err.to_string());
                ActionResult::None
            }
        }
    }

    fn start_analysis(&mut self, path: PathBuf) -> ActionResult {
        self.last_path = Some(path.clone());
        self.pending_analysis = None;
        self.screen = Screen::Analyzing;
        self.input_error = None;
        self.status = None;
        self.detail_scroll = 0;
        ActionResult::Analyze(path)
    }

    fn refresh_input_validation(&mut self) {
        let trimmed = self.input_path.trim();
        if trimmed.is_empty() {
            self.input_hint = None;
            self.input_error = None;
            return;
        }

        match input::detect_format(&PathBuf::from(trimmed)) {
            Ok(format) => {
                self.input_hint = Some(format!("Detected input: {format}"));
                self.input_error = None;
            }
            Err(err) => {
                self.input_hint = None;
                self.input_error = Some(err.to_string());
            }
        }
    }

    fn move_next(&mut self) {
        if self.focus == Focus::Details {
            self.scroll_details(1);
            return;
        }

        let Some(report) = self.report.as_ref() else {
            return;
        };
        if report.findings.is_empty() {
            return;
        }

        let current = self.findings_state.selected().unwrap_or(0);
        let next = usize::min(current + 1, report.findings.len() - 1);
        self.findings_state.select(Some(next));
        self.detail_scroll = 0;
    }

    fn move_previous(&mut self) {
        if self.focus == Focus::Details {
            self.detail_scroll = self.detail_scroll.saturating_sub(1);
            return;
        }

        let current = self.findings_state.selected().unwrap_or(0);
        let previous = current.saturating_sub(1);
        self.findings_state.select(Some(previous));
        self.detail_scroll = 0;
    }

    fn scroll_details(&mut self, amount: u16) {
        self.detail_scroll = self.detail_scroll.saturating_add(amount);
    }
}

fn severity_rank(severity: Severity) -> usize {
    match severity {
        Severity::Critical => 3,
        Severity::Warning => 2,
        Severity::Info => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzers::finding::{Category, EnvironmentInfo, Severity};

    #[test]
    fn primary_finding_prefers_highest_severity() {
        let mut app = App::new(None);
        app.report = Some(DiagnosticReport {
            environment: EnvironmentInfo::default(),
            findings: vec![
                Finding {
                    rule_id: "INFO-1".to_string(),
                    name: "Info finding".to_string(),
                    severity: Severity::Info,
                    category: Category::Connectivity,
                    description: String::new(),
                    evidence: Vec::new(),
                    remediation: String::new(),
                    doc_link: None,
                },
                Finding {
                    rule_id: "CRIT-1".to_string(),
                    name: "Critical finding".to_string(),
                    severity: Severity::Critical,
                    category: Category::Identity,
                    description: String::new(),
                    evidence: Vec::new(),
                    remediation: String::new(),
                    doc_link: None,
                },
            ],
            files_analyzed: 0,
            bundle_path: "bundle.zip".to_string(),
        });

        assert_eq!(
            app.primary_finding()
                .map(|finding| finding.rule_id.as_str()),
            Some("CRIT-1")
        );
    }
}

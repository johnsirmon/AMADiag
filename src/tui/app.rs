use crate::analyzers::finding::{DiagnosticReport, Finding, Severity};
use crate::input;
use crate::reporters::OutputFormat;
use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    FileBrowser,
    PathInput,
    Analyzing,
    Dashboard,
    Export,
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
    FocusPrevious,
    Next,
    Previous,
    PageDown,
    PageUp,
    Home,
    End,
    FocusNext,
    EditPath,
    Retry,
    ExportMarkdown,
    ExportJson,
    ToggleView,
    BrowserParent,
    SeverityFilter(u8),
    ExportConfirm,
    ExportCancel,
    ExportToggleFormat,
    ToggleHidden,
}

pub enum ActionResult {
    None,
    Quit,
    Analyze(PathBuf),
    ShowExport(OutputFormat),
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

#[derive(Debug, Clone)]
pub struct BrowserEntry {
    pub name: String,
    pub is_dir: bool,
    pub is_bundle: bool,
    pub size: Option<u64>,
    pub child_count: Option<usize>,
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
    // File browser state
    browser_path: PathBuf,
    browser_entries: Vec<BrowserEntry>,
    browser_state: ListState,
    // Severity filter: 1=Critical only, 2=Critical+Warning, 3=all
    severity_filter: u8,
    // Export screen state
    export_format: OutputFormat,
    export_path: String,
    // Hidden files toggle
    show_hidden: bool,
    // Number of directories (for separator rendering)
    dir_count: usize,
    // Export overwrite confirmation pending
    export_overwrite_pending: bool,
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
            Screen::FileBrowser
        };

        let browser_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

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
            browser_path: browser_path.clone(),
            browser_entries: Vec::new(),
            browser_state: ListState::default(),
            severity_filter: 3,
            export_format: OutputFormat::Markdown,
            export_path: String::new(),
            show_hidden: false,
            dir_count: 0,
            export_overwrite_pending: false,
        };
        app.findings_state.select(None);
        app.refresh_input_validation();
        if app.screen == Screen::FileBrowser {
            app.load_browser_dir(&browser_path);
        }
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

    pub fn selected_index(&self) -> Option<usize> {
        self.findings_state.selected()
    }

    #[allow(dead_code)]
    pub fn finding_count(&self) -> usize {
        self.report
            .as_ref()
            .map(|report| report.findings.len())
            .unwrap_or_default()
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

    pub fn browser_path(&self) -> &PathBuf {
        &self.browser_path
    }

    pub fn browser_entries(&self) -> &[BrowserEntry] {
        &self.browser_entries
    }

    pub fn browser_state(&mut self) -> &mut ListState {
        &mut self.browser_state
    }

    pub fn severity_filter(&self) -> u8 {
        self.severity_filter
    }

    pub fn export_format(&self) -> OutputFormat {
        self.export_format
    }

    pub fn export_path(&self) -> &str {
        &self.export_path
    }

    pub fn last_path(&self) -> Option<&PathBuf> {
        self.last_path.as_ref()
    }

    pub fn show_hidden(&self) -> bool {
        self.show_hidden
    }

    #[allow(dead_code)]
    pub fn dir_count(&self) -> usize {
        self.dir_count
    }

    pub fn export_overwrite_pending(&self) -> bool {
        self.export_overwrite_pending
    }

    /// Returns findings filtered by the current severity filter.
    pub fn filtered_findings(&self) -> Vec<&Finding> {
        let Some(report) = self.report.as_ref() else {
            return Vec::new();
        };
        report
            .findings
            .iter()
            .filter(|f| match self.severity_filter {
                1 => f.severity == Severity::Critical,
                2 => f.severity == Severity::Critical || f.severity == Severity::Warning,
                _ => true,
            })
            .collect()
    }

    pub fn filtered_finding_count(&self) -> usize {
        self.filtered_findings().len()
    }

    pub fn selected_filtered_finding(&self) -> Option<&Finding> {
        let filtered = self.filtered_findings();
        let index = self.findings_state.selected()?;
        filtered.get(index).copied()
    }

    pub fn handle_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::Quit => ActionResult::Quit,
            Action::Submit => match self.screen {
                Screen::FileBrowser => self.browser_select(),
                Screen::PathInput => self.submit_path(),
                Screen::Export => self.confirm_export(),
                _ => ActionResult::None,
            },
            Action::Backspace => match self.screen {
                Screen::PathInput => {
                    self.input_path.pop();
                    self.refresh_input_validation();
                    ActionResult::None
                }
                Screen::FileBrowser => {
                    self.browser_parent();
                    ActionResult::None
                }
                Screen::Export => {
                    self.export_path.pop();
                    self.export_overwrite_pending = false;
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
            Action::InputChar(ch) => match self.screen {
                Screen::PathInput => {
                    self.input_path.push(ch);
                    self.refresh_input_validation();
                    ActionResult::None
                }
                Screen::Export => {
                    self.export_path.push(ch);
                    self.export_overwrite_pending = false;
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
            Action::Paste(text) => match self.screen {
                Screen::PathInput => {
                    self.input_path.push_str(&text);
                    self.refresh_input_validation();
                    ActionResult::None
                }
                Screen::Export => {
                    self.export_path.push_str(&text);
                    self.export_overwrite_pending = false;
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
            Action::FocusPrevious => {
                if self.screen == Screen::Dashboard {
                    self.focus = match self.focus {
                        Focus::Findings => Focus::Details,
                        Focus::Details => Focus::Findings,
                    };
                }
                ActionResult::None
            }
            Action::Next => match self.screen {
                Screen::Dashboard => {
                    self.move_next();
                    ActionResult::None
                }
                Screen::FileBrowser => {
                    self.browser_move_next();
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
            Action::Previous => match self.screen {
                Screen::Dashboard => {
                    self.move_previous();
                    ActionResult::None
                }
                Screen::FileBrowser => {
                    self.browser_move_previous();
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
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
            Action::Home => match self.screen {
                Screen::Dashboard => {
                    self.move_home();
                    ActionResult::None
                }
                Screen::FileBrowser => {
                    self.browser_state.select(Some(0));
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
            Action::End => match self.screen {
                Screen::Dashboard => {
                    self.move_end();
                    ActionResult::None
                }
                Screen::FileBrowser => {
                    let count = self.browser_entries.len();
                    if count > 0 {
                        self.browser_state.select(Some(count - 1));
                    }
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
            Action::FocusNext => {
                if self.screen == Screen::Dashboard {
                    self.focus = match self.focus {
                        Focus::Findings => Focus::Details,
                        Focus::Details => Focus::Findings,
                    };
                }
                ActionResult::None
            }
            Action::EditPath => match self.screen {
                Screen::PathInput => ActionResult::Quit,
                Screen::FileBrowser => ActionResult::Quit,
                Screen::Analyzing => ActionResult::None,
                Screen::Export => {
                    self.screen = Screen::Dashboard;
                    ActionResult::None
                }
                Screen::Dashboard => {
                    self.screen = Screen::FileBrowser;
                    self.detail_scroll = 0;
                    let path = self.browser_path.clone();
                    self.load_browser_dir(&path);
                    ActionResult::None
                }
            },
            Action::Retry => {
                if let Some(path) = self.last_path.clone() {
                    self.start_analysis(path)
                } else {
                    ActionResult::None
                }
            }
            Action::ExportMarkdown => {
                if self.screen == Screen::Dashboard && self.report.is_some() {
                    self.enter_export_screen(OutputFormat::Markdown);
                    ActionResult::None
                } else {
                    ActionResult::None
                }
            }
            Action::ExportJson => {
                if self.screen == Screen::Dashboard && self.report.is_some() {
                    self.enter_export_screen(OutputFormat::Json);
                    ActionResult::None
                } else {
                    ActionResult::None
                }
            }
            Action::ToggleView => match self.screen {
                Screen::FileBrowser => {
                    self.screen = Screen::PathInput;
                    ActionResult::None
                }
                Screen::PathInput => {
                    self.screen = Screen::FileBrowser;
                    let path = self.browser_path.clone();
                    self.load_browser_dir(&path);
                    ActionResult::None
                }
                _ => ActionResult::None,
            },
            Action::BrowserParent => {
                if self.screen == Screen::FileBrowser {
                    self.browser_parent();
                }
                ActionResult::None
            }
            Action::SeverityFilter(level) => {
                if self.screen == Screen::Dashboard {
                    self.severity_filter = level.clamp(1, 3);
                    // Reset selection to stay in bounds
                    let count = self.filtered_finding_count();
                    if count > 0 {
                        let selected = self.findings_state.selected().unwrap_or(0).min(count - 1);
                        self.findings_state.select(Some(selected));
                    } else {
                        self.findings_state.select(None);
                    }
                    self.detail_scroll = 0;
                }
                ActionResult::None
            }
            Action::ExportConfirm => {
                if self.screen == Screen::Export {
                    self.confirm_export()
                } else {
                    ActionResult::None
                }
            }
            Action::ExportCancel => {
                if self.screen == Screen::Export {
                    self.export_overwrite_pending = false;
                    self.screen = Screen::Dashboard;
                }
                ActionResult::None
            }
            Action::ExportToggleFormat => {
                if self.screen == Screen::Export {
                    self.export_format = match self.export_format {
                        OutputFormat::Markdown => OutputFormat::Json,
                        OutputFormat::Json => OutputFormat::Markdown,
                    };
                    // Update extension in export path
                    self.refresh_export_path_extension();
                }
                ActionResult::None
            }
            Action::ToggleHidden => {
                if self.screen == Screen::FileBrowser {
                    self.show_hidden = !self.show_hidden;
                    let path = self.browser_path.clone();
                    self.load_browser_dir(&path);
                }
                ActionResult::None
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
                self.severity_filter = 3;
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
                self.screen = Screen::FileBrowser;
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

    pub fn return_to_dashboard(&mut self) {
        self.screen = Screen::Dashboard;
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
        self.input_path = path.display().to_string();
        self.pending_analysis = None;
        self.screen = Screen::Analyzing;
        self.input_error = None;
        self.status = None;
        self.detail_scroll = 0;
        ActionResult::Analyze(path)
    }

    // --- File browser methods ---

    pub fn load_browser_dir(&mut self, path: &std::path::Path) {
        let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        self.browser_path = canonical;
        self.browser_entries.clear();

        let entries = match std::fs::read_dir(&self.browser_path) {
            Ok(rd) => rd,
            Err(_) => {
                self.set_error_status(format!(
                    "Cannot read directory: {}",
                    self.browser_path.display()
                ));
                return;
            }
        };

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            // Filter hidden files (dotfiles) unless show_hidden is enabled
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }

            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let is_bundle = Self::is_bundle_entry(&entry);

            let (size, child_count) = if is_dir {
                let count = entry.path().read_dir().map(|rd| rd.count()).ok();
                (None, count)
            } else {
                let sz = entry.metadata().map(|m| m.len()).ok();
                (sz, None)
            };

            let entry = BrowserEntry {
                name,
                is_dir,
                is_bundle,
                size,
                child_count,
            };

            if is_dir {
                dirs.push(entry);
            } else {
                files.push(entry);
            }
        }

        dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        // Add parent directory entry
        if let Some(parent) = self.browser_path.parent() {
            if parent != self.browser_path.as_path() {
                dirs.insert(
                    0,
                    BrowserEntry {
                        name: "..".to_string(),
                        is_dir: true,
                        is_bundle: false,
                        size: None,
                        child_count: None,
                    },
                );
            }
        }

        self.dir_count = dirs.len();
        self.browser_entries.extend(dirs);
        self.browser_entries.extend(files);

        if !self.browser_entries.is_empty() {
            self.browser_state.select(Some(0));
        } else {
            self.browser_state.select(None);
        }
    }

    fn is_bundle_entry(entry: &std::fs::DirEntry) -> bool {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name.ends_with(".tgz") || name.ends_with(".tar.gz") || name.ends_with(".zip") {
            return true;
        }
        false
    }

    fn browser_select(&mut self) -> ActionResult {
        let Some(index) = self.browser_state.selected() else {
            return ActionResult::None;
        };
        let Some(entry) = self.browser_entries.get(index) else {
            return ActionResult::None;
        };

        let full_path = self.browser_path.join(&entry.name);

        // Handle parent directory entry
        if entry.name == ".." {
            self.browser_parent();
            return ActionResult::None;
        }

        if entry.is_dir {
            // Check if it's a valid bundle directory
            if input::detect_format(&full_path).is_ok() {
                self.input_path = full_path.display().to_string();
                return self.start_analysis(full_path);
            }
            // Otherwise navigate into it
            let path = full_path.clone();
            self.load_browser_dir(&path);
            ActionResult::None
        } else if entry.is_bundle {
            self.input_path = full_path.display().to_string();
            self.start_analysis(full_path)
        } else {
            self.set_error_status("Not a recognized bundle format (.tgz, .zip, or directory)");
            ActionResult::None
        }
    }

    fn browser_parent(&mut self) {
        if let Some(parent) = self.browser_path.parent().map(|p| p.to_path_buf()) {
            self.load_browser_dir(&parent);
        }
    }

    fn browser_move_next(&mut self) {
        if self.browser_entries.is_empty() {
            return;
        }
        let current = self.browser_state.selected().unwrap_or(0);
        let next = usize::min(current + 1, self.browser_entries.len() - 1);
        self.browser_state.select(Some(next));
    }

    fn browser_move_previous(&mut self) {
        let current = self.browser_state.selected().unwrap_or(0);
        self.browser_state.select(Some(current.saturating_sub(1)));
    }

    // --- Export screen methods ---

    fn enter_export_screen(&mut self, format: OutputFormat) {
        self.export_format = format;
        // Derive default export path from the original bundle path
        let source = self
            .last_path
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new("."));
        let parent = source.parent().unwrap_or_else(|| std::path::Path::new("."));

        // Check if parent is writable; fall back to CWD
        let export_dir = if parent.exists() && is_dir_writable(parent) {
            parent.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        };

        let stem = source
            .file_stem()
            .and_then(|v| v.to_str())
            .filter(|v| !v.is_empty())
            .unwrap_or("amadiag-report");

        let ext = match format {
            OutputFormat::Markdown => "md",
            OutputFormat::Json => "json",
        };

        self.export_path = export_dir
            .join(format!("{stem}.amadiag.{ext}"))
            .display()
            .to_string();
        self.export_overwrite_pending = false;
        self.screen = Screen::Export;
    }

    fn confirm_export(&mut self) -> ActionResult {
        if self.export_path.trim().is_empty() {
            self.set_error_status("Export path cannot be empty.");
            return ActionResult::None;
        }
        let path = PathBuf::from(self.export_path.trim());
        if path.exists() && !self.export_overwrite_pending {
            self.export_overwrite_pending = true;
            return ActionResult::None;
        }
        self.export_overwrite_pending = false;
        ActionResult::ShowExport(self.export_format)
    }

    fn refresh_export_path_extension(&mut self) {
        let new_ext = match self.export_format {
            OutputFormat::Markdown => ".md",
            OutputFormat::Json => ".json",
        };
        // Replace the trailing extension
        if let Some(pos) = self.export_path.rfind(".amadiag.") {
            self.export_path.truncate(pos);
            self.export_path.push_str(&format!(".amadiag{new_ext}"));
        }
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

        let count = self.filtered_finding_count();
        if count == 0 {
            return;
        }

        let current = self.findings_state.selected().unwrap_or(0);
        let next = usize::min(current + 1, count - 1);
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

    fn move_home(&mut self) {
        if self.focus == Focus::Details {
            self.detail_scroll = 0;
            return;
        }

        if self.filtered_finding_count() > 0 {
            self.findings_state.select(Some(0));
            self.detail_scroll = 0;
        }
    }

    fn move_end(&mut self) {
        if self.focus == Focus::Details {
            self.detail_scroll = self.detail_max_scroll();
            return;
        }

        let count = self.filtered_finding_count();
        if count > 0 {
            self.findings_state.select(Some(count - 1));
            self.detail_scroll = 0;
        }
    }

    fn detail_max_scroll(&self) -> u16 {
        let Some(finding) = self
            .selected_filtered_finding()
            .or_else(|| self.primary_finding())
        else {
            return 0;
        };
        15u16.saturating_add(finding.evidence.len() as u16)
    }

    fn scroll_details(&mut self, amount: u16) {
        let max = self.detail_max_scroll();
        self.detail_scroll = self.detail_scroll.saturating_add(amount).min(max);
    }
}

fn severity_rank(severity: Severity) -> usize {
    match severity {
        Severity::Critical => 3,
        Severity::Warning => 2,
        Severity::Info => 1,
    }
}

fn is_dir_writable(path: &std::path::Path) -> bool {
    let test_file = path.join(".amadiag_write_test");
    match std::fs::write(&test_file, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(&test_file);
            true
        }
        Err(_) => false,
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

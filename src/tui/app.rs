use crate::analyzers::finding::DiagnosticReport;
use crate::detect::TuiAnalysis;
use crate::input;
use crate::model::diagnostic::{Category as UiCategory, FindingGroup, Severity as UiSeverity};
use crate::reporters::OutputFormat;
use ratatui::widgets::{ListState, TableState};
use std::collections::BTreeSet;
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
    Navigator,
    Findings,
    Details,
    Evidence,
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
    CycleTimeFilter,
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
    analysis: Option<TuiAnalysis>,
    findings_state: TableState,
    navigator_state: ListState,
    detail_scroll: u16,
    evidence_scroll: u16,
    evidence_lines: Vec<String>,
    status: Option<StatusMessage>,
    pending_analysis: Option<PathBuf>,
    last_path: Option<PathBuf>,
    tick: usize,
    browser_path: PathBuf,
    browser_entries: Vec<BrowserEntry>,
    browser_state: ListState,
    severity_filter: u8,
    export_format: OutputFormat,
    export_path: String,
    show_hidden: bool,
    dir_count: usize,
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
            analysis: None,
            findings_state: TableState::default(),
            navigator_state: ListState::default(),
            detail_scroll: 0,
            evidence_scroll: 0,
            evidence_lines: Vec::new(),
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
        app.navigator_state.select(Some(0));
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

    pub fn findings_state(&mut self) -> &mut TableState {
        &mut self.findings_state
    }

    pub fn navigator_state(&mut self) -> &mut ListState {
        &mut self.navigator_state
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.findings_state.selected()
    }

    pub fn detail_scroll(&self) -> u16 {
        self.detail_scroll
    }

    pub fn evidence_scroll(&self) -> u16 {
        self.evidence_scroll
    }

    pub fn evidence_lines(&self) -> &[String] {
        &self.evidence_lines
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

    pub fn export_overwrite_pending(&self) -> bool {
        self.export_overwrite_pending
    }

    #[cfg(test)]
    pub fn primary_finding(&self) -> Option<&crate::analyzers::finding::Finding> {
        self.report
            .as_ref()?
            .findings
            .iter()
            .max_by_key(|finding| severity_rank(finding.severity))
    }

    pub fn navigator_items(&self) -> Vec<(String, Option<UiCategory>)> {
        let Some(analysis) = self.analysis.as_ref() else {
            return vec![("All categories".to_string(), None)];
        };

        let mut categories = BTreeSet::new();
        for event in &analysis.events {
            categories.insert(event.category);
        }

        let mut items = vec![("All categories".to_string(), None)];
        items.extend(
            categories
                .into_iter()
                .map(|category| (category.to_string(), Some(category))),
        );
        items
    }

    pub fn selected_category_label(&self) -> String {
        let items = self.navigator_items();
        let selected = self.navigator_state.selected().unwrap_or(0);
        items
            .get(selected)
            .map(|(label, _)| label.clone())
            .unwrap_or_else(|| "All categories".to_string())
    }

    pub fn selected_group(&self) -> Option<&FindingGroup> {
        let analysis = self.analysis.as_ref()?;
        let selected = self.findings_state.selected()?;
        analysis.grouped_findings.get(selected)
    }

    pub fn grouped_findings(&self) -> &[FindingGroup] {
        self.analysis
            .as_ref()
            .map(|analysis| analysis.grouped_findings.as_slice())
            .unwrap_or(&[])
    }

    pub fn grouped_finding_count(&self) -> usize {
        self.grouped_findings().len()
    }

    pub fn timeline_points(&self) -> Vec<u64> {
        self.analysis
            .as_ref()
            .map(|analysis| {
                analysis
                    .event_store
                    .timeline(&analysis.filter)
                    .into_iter()
                    .map(|bucket| u64::try_from(bucket.warning_or_higher).unwrap_or(u64::MAX))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn current_time_filter_label(&self) -> String {
        self.analysis
            .as_ref()
            .map(|analysis| analysis.filter.time_filter.label())
            .unwrap_or_else(|| "All".to_string())
    }

    pub fn finish_analysis(&mut self, result: std::result::Result<TuiAnalysis, String>) {
        match result {
            Ok(analysis) => {
                let report = analysis.report.clone();
                let finding_count = analysis.grouped_findings.len();
                self.analysis = Some(analysis);
                self.report = Some(report);
                self.screen = Screen::Dashboard;
                self.focus = Focus::Findings;
                self.detail_scroll = 0;
                self.evidence_scroll = 0;
                self.severity_filter = 3;
                self.navigator_state.select(Some(0));
                self.findings_state
                    .select((self.grouped_finding_count() > 0).then_some(0));
                self.refresh_evidence();
                self.set_info_status(format!(
                    "Analysis complete: {finding_count} grouped finding(s)"
                ));
            }
            Err(err) => {
                self.screen = Screen::FileBrowser;
                self.analysis = None;
                self.report = None;
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
                        Focus::Navigator => Focus::Evidence,
                        Focus::Findings => Focus::Navigator,
                        Focus::Details => Focus::Findings,
                        Focus::Evidence => Focus::Details,
                    };
                }
                ActionResult::None
            }
            Action::FocusNext => {
                if self.screen == Screen::Dashboard {
                    self.focus = match self.focus {
                        Focus::Navigator => Focus::Findings,
                        Focus::Findings => Focus::Details,
                        Focus::Details => Focus::Evidence,
                        Focus::Evidence => Focus::Navigator,
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
                    self.scroll_active_panel(8);
                }
                ActionResult::None
            }
            Action::PageUp => {
                if self.screen == Screen::Dashboard {
                    self.scroll_active_panel(-8);
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
            Action::EditPath => match self.screen {
                Screen::PathInput | Screen::FileBrowser => ActionResult::Quit,
                Screen::Analyzing => ActionResult::None,
                Screen::Export => {
                    self.screen = Screen::Dashboard;
                    ActionResult::None
                }
                Screen::Dashboard => {
                    self.screen = Screen::FileBrowser;
                    self.detail_scroll = 0;
                    self.evidence_scroll = 0;
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
                }
                ActionResult::None
            }
            Action::ExportJson => {
                if self.screen == Screen::Dashboard && self.report.is_some() {
                    self.enter_export_screen(OutputFormat::Json);
                }
                ActionResult::None
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
                    if let Some(analysis) = self.analysis.as_mut() {
                        analysis.filter.min_severity = match self.severity_filter {
                            1 => UiSeverity::Critical,
                            2 => UiSeverity::Medium,
                            _ => UiSeverity::Info,
                        };
                        analysis.refresh_groups();
                    }
                    self.sync_dashboard_state();
                }
                ActionResult::None
            }
            Action::CycleTimeFilter => {
                if self.screen == Screen::Dashboard {
                    if let Some(analysis) = self.analysis.as_mut() {
                        analysis.filter.cycle_time_filter();
                        analysis.refresh_groups();
                    }
                    self.sync_dashboard_state();
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
        self.evidence_scroll = 0;
        ActionResult::Analyze(path)
    }

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
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }

            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let is_bundle = Self::is_bundle_entry(&entry);
            let (size, child_count) = if is_dir {
                let count = entry.path().read_dir().map(|rd| rd.count()).ok();
                (None, count)
            } else {
                let size = entry.metadata().map(|metadata| metadata.len()).ok();
                (size, None)
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

        dirs.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
        files.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));

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
        name.ends_with(".tgz") || name.ends_with(".tar.gz") || name.ends_with(".zip")
    }

    fn browser_select(&mut self) -> ActionResult {
        let Some(index) = self.browser_state.selected() else {
            return ActionResult::None;
        };
        let Some(entry) = self.browser_entries.get(index) else {
            return ActionResult::None;
        };

        let full_path = self.browser_path.join(&entry.name);
        if entry.name == ".." {
            self.browser_parent();
            return ActionResult::None;
        }

        if entry.is_dir {
            if input::detect_format(&full_path).is_ok() {
                self.input_path = full_path.display().to_string();
                return self.start_analysis(full_path);
            }
            self.load_browser_dir(&full_path);
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
        if let Some(parent) = self.browser_path.parent().map(|path| path.to_path_buf()) {
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

    fn enter_export_screen(&mut self, format: OutputFormat) {
        self.export_format = format;
        let source = self
            .last_path
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new("."));
        let parent = source.parent().unwrap_or_else(|| std::path::Path::new("."));

        let export_dir = if parent.exists() && is_dir_writable(parent) {
            parent.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        };

        let stem = source
            .file_stem()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .unwrap_or("amadiag-report");
        let extension = match format {
            OutputFormat::Markdown => "md",
            OutputFormat::Json => "json",
        };

        self.export_path = export_dir
            .join(format!("{stem}.amadiag.{extension}"))
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
        match self.focus {
            Focus::Navigator => {
                let count = self.navigator_items().len();
                if count == 0 {
                    return;
                }
                let current = self.navigator_state.selected().unwrap_or(0);
                self.navigator_state
                    .select(Some(usize::min(current + 1, count - 1)));
                self.apply_category_filter();
            }
            Focus::Findings => {
                let count = self.grouped_finding_count();
                if count == 0 {
                    return;
                }
                let current = self.findings_state.selected().unwrap_or(0);
                self.findings_state
                    .select(Some(usize::min(current + 1, count - 1)));
                self.detail_scroll = 0;
                self.evidence_scroll = 0;
                self.refresh_evidence();
            }
            Focus::Details => self.scroll_active_panel(1),
            Focus::Evidence => self.scroll_active_panel(1),
        }
    }

    fn move_previous(&mut self) {
        match self.focus {
            Focus::Navigator => {
                let current = self.navigator_state.selected().unwrap_or(0);
                self.navigator_state.select(Some(current.saturating_sub(1)));
                self.apply_category_filter();
            }
            Focus::Findings => {
                let current = self.findings_state.selected().unwrap_or(0);
                self.findings_state.select(Some(current.saturating_sub(1)));
                self.detail_scroll = 0;
                self.evidence_scroll = 0;
                self.refresh_evidence();
            }
            Focus::Details => self.scroll_active_panel(-1),
            Focus::Evidence => self.scroll_active_panel(-1),
        }
    }

    fn move_home(&mut self) {
        match self.focus {
            Focus::Navigator => {
                self.navigator_state.select(Some(0));
                self.apply_category_filter();
            }
            Focus::Findings => {
                if self.grouped_finding_count() > 0 {
                    self.findings_state.select(Some(0));
                    self.refresh_evidence();
                }
            }
            Focus::Details => self.detail_scroll = 0,
            Focus::Evidence => self.evidence_scroll = 0,
        }
    }

    fn move_end(&mut self) {
        match self.focus {
            Focus::Navigator => {
                let count = self.navigator_items().len();
                if count > 0 {
                    self.navigator_state.select(Some(count - 1));
                    self.apply_category_filter();
                }
            }
            Focus::Findings => {
                let count = self.grouped_finding_count();
                if count > 0 {
                    self.findings_state.select(Some(count - 1));
                    self.refresh_evidence();
                }
            }
            Focus::Details => self.detail_scroll = self.detail_max_scroll(),
            Focus::Evidence => self.evidence_scroll = self.evidence_max_scroll(),
        }
    }

    fn apply_category_filter(&mut self) {
        let selected = self.navigator_state.selected().unwrap_or(0);
        let category = self
            .navigator_items()
            .get(selected)
            .and_then(|(_, category)| *category);

        if let Some(analysis) = self.analysis.as_mut() {
            analysis.filter.categories = category.into_iter().collect::<BTreeSet<_>>();
            analysis.refresh_groups();
        }

        self.sync_dashboard_state();
    }

    fn sync_dashboard_state(&mut self) {
        let count = self.grouped_finding_count();
        if count > 0 {
            let selected = self.findings_state.selected().unwrap_or(0).min(count - 1);
            self.findings_state.select(Some(selected));
        } else {
            self.findings_state.select(None);
        }
        self.detail_scroll = 0;
        self.evidence_scroll = 0;
        self.refresh_evidence();
    }

    fn refresh_evidence(&mut self) {
        let Some(group) = self.selected_group() else {
            self.evidence_lines = vec!["No evidence available.".to_string()];
            return;
        };

        let Some(evidence) = group.evidence.first() else {
            self.evidence_lines = vec!["No direct evidence lines captured.".to_string()];
            return;
        };

        if evidence.file_id == "legacy" {
            self.evidence_lines = group
                .evidence
                .iter()
                .map(|item| item.preview.clone())
                .collect();
            return;
        }

        self.evidence_lines = self
            .analysis
            .as_ref()
            .and_then(|analysis| {
                analysis
                    .event_store
                    .load_evidence_context(evidence, 2, 2)
                    .ok()
            })
            .unwrap_or_else(|| vec![evidence.preview.clone()]);
    }

    fn detail_max_scroll(&self) -> u16 {
        let Some(group) = self.selected_group() else {
            return 0;
        };
        16u16
            .saturating_add(group.evidence.len() as u16)
            .saturating_add(group.likely_causes.len() as u16)
            .saturating_add(group.suggested_actions.len() as u16)
            .saturating_add(group.doc_links.len() as u16)
    }

    fn evidence_max_scroll(&self) -> u16 {
        self.evidence_lines
            .len()
            .saturating_sub(1)
            .try_into()
            .unwrap_or(u16::MAX)
    }

    fn scroll_active_panel(&mut self, delta: i16) {
        match self.focus {
            Focus::Details => {
                if delta.is_negative() {
                    self.detail_scroll = self.detail_scroll.saturating_sub(delta.unsigned_abs());
                } else {
                    self.detail_scroll = self
                        .detail_scroll
                        .saturating_add(delta as u16)
                        .min(self.detail_max_scroll());
                }
            }
            Focus::Evidence => {
                if delta.is_negative() {
                    self.evidence_scroll =
                        self.evidence_scroll.saturating_sub(delta.unsigned_abs());
                } else {
                    self.evidence_scroll = self
                        .evidence_scroll
                        .saturating_add(delta as u16)
                        .min(self.evidence_max_scroll());
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
fn severity_rank(severity: crate::analyzers::finding::Severity) -> usize {
    match severity {
        crate::analyzers::finding::Severity::Critical => 3,
        crate::analyzers::finding::Severity::Warning => 2,
        crate::analyzers::finding::Severity::Info => 1,
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
    use crate::analyzers::finding::{Category, EnvironmentInfo, Finding, Severity};

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

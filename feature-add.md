# AMADiag Feature Enhancement: "Ops Cockpit" TUI and 3-Pass Architecture

## 1. Objective
Upgrade the existing `AMADiag` application from a basic terminal list view to a layered "ops cockpit" built with `ratatui` [1]. Implement a 3-pass log processing architecture to handle massively large Azure Monitor Agent (AMA) diagnostic bundles efficiently via streaming, time-filtering, and lazy loading [2, 3].

## 2. Core Architectural Upgrades

### 2.1. 3-Pass Log Architecture
To prevent the app from crashing on large bundles and to remove stale historical noise, implement the following pipeline [2, 3]:
*   **Pass 1 (Streaming Parse & Index):** Read log files line-by-line using `BufRead` [3]. Extract timestamps, normalize them to UTC, and build a sparse in-memory time index (`BTreeMap<i64, Vec<usize>>` mapping UNIX minutes to event indices) [2, 4]. Only keep compact previews in memory [5].
*   **Pass 2 (Filtered Extraction):** Use the time index to extract only events that fall within the user's selected time window (e.g., last 15m, 1h, 24h) [2, 5].
*   **Pass 3 (Issue Classification & Grouping):** Run detection rules on the filtered events. Group repetitive raw events into a single actionable finding (e.g., aggregate 80 endpoint failures over 10 minutes into a single "Connectivity failures to ingestion/control endpoint" finding) [6, 7]. Show raw evidence only on demand [8].

### 2.2. Unified Normalized Data Model
Normalize the check-oriented Windows output and scenario-oriented Linux output into a single internal model before it reaches the UI [9-11].

**Implement the following Rust structs:**
```rust
use chrono::{DateTime, Utc};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsKind { Windows, Linux }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity { Info, Low, Medium, High, Critical }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status { Pass, Warn, Fail, Unknown }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Install, Service, Heartbeat, Connectivity, Dcr, Mdsd,
    FluentBit, Syslog, CustomLogs, CpuMemory, ArcExtension,
    IdentityAuth, ProxyTls, Config, Other
}

#[derive(Debug, Clone)]
pub struct EvidenceRef {
    pub file_id: String,
    pub line_range: Range<usize>,
    pub preview: String,
}

#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    pub ts: Option<DateTime<Utc>>,
    pub os: OsKind,
    pub category: Category,
    pub severity: Severity,
    pub status: Status,
    pub title: String,
    pub summary: String,
    pub details: Option<String>,
    pub evidence: Vec<EvidenceRef>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FindingGroup {
    pub key: String,
    pub category: Category,
    pub severity: Severity,
    pub status: Status,
    pub title: String,
    pub summary: String,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub events: Vec<usize>,
    pub likely_causes: Vec<String>,
    pub suggested_actions: Vec<String>,
}
[Sources: 53, 54, 55]
3. "Ops Cockpit" TUI Layout
Build a 4-pane dashboard using ratatui with the following layout
:
Top Bar: Displays host metadata (OS, AMA version), active time filters, and selected time window
.
Left Pane (Issue Navigator): A List widget showing categories
. Preload differently based on OS:
Windows: Config, ArcExtension, Connectivity, Heartbeat, IdentityAuth, ProxyTls
.
Linux: Install, Heartbeat, Connectivity, Syslog, CustomLogs, CpuMemory, FluentBit, Mdsd
.
Center Pane (Overview & Timeline):
A status Table (sortable by severity/time, filterable by category) compressing findings into actionable rows
.
A compact Sparkline or BarChart showing failures/warnings per 5-minute bucket
.
Right Pane (Details): A Paragraph widget showing why the check flagged, likely root causes, and recommended next actions
.
Bottom Pane (Evidence Context): A Scrollbar-enabled grep-like view displaying only the raw log lines relevant to the selected finding, plus a few lines of surrounding context
.
3.1. State Management
Implement an AppState to manage tabs and filters
:
use ratatui::widgets::TableState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab { Overview, Findings, RawLogs, Metadata }

pub struct AppState {
    pub active_tab: ActiveTab,
    pub filter: FilterState,
    pub store: EventStore,
    pub findings: Vec<FindingGroup>,
    pub selected_finding: usize,
    pub selected_raw_line: usize,
    pub findings_table_state: TableState,
    pub status_message: String,
}
[Source: 59]
4. Implementation Phases
Phase 1 (Data & Core UI): Implement the streaming parser, DiagnosticEvent normalization, FindingGroup logic, and the central findings Table and Paragraph detail panes
. Implement the TimeFilter engine to allow filtering by LastMinutes, LastHours, or Custom
.
Phase 2 (Visuals & Context): Add the Sparkline timeline for failures over time and the bottom raw evidence context pane with lazy loading
.
Phase 3 (Advanced Features): Implement background parsing with progress updates over channels, saved filter presets, and JSON/Markdown export hooks for the currently selected finding
.
5. Directory Structure Adjustments
Refactor the project to support the new TUI structure
:
src/app.rs (State management)
src/ui.rs (Main layout rendering)
src/events.rs (Channel and event loop)
src/parser/ (Split into windows.rs, linux.rs, normalize.rs)
src/model/ (diagnostic.rs, filter.rs, finding.rs)
src/store/ (event_store.rs, grouping.rs, timeline.rs)
src/widgets/ (overview.rs, findings_table.rs, detail.rs, raw_logs.rs)

Phase 5.5
add simple functionality to identify date range of the file bundle(s).  Next provide a way to limit to the most recent files in bundle so we are getting a bunc of old stale files.  This functionality will have the ability to ignore logs, files older than 6 months.  Provide user ability to export a bundle that captures specific date ranges easily

Phase 6.  Implement updated AMA for windows parsing logic to include feedback on how to actually parse data "C:\source\AMADiag\private\ReviewingAMATroubleshooterOutput.md"

Phase 7.  add usability recommendations for reviewing AMA bundles based on evidence from this exa search here are results "C:\source\AMADiag\private\exarecommendation.md"

phase 8. add updates to parsing logic from details in C:\source\AMADiag\private\mdmdata.md


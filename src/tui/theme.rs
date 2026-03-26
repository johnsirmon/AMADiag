use ratatui::style::{Color, Modifier, Style};

// ── Brand / Accent ──────────────────────────────────────────────────
pub const ACCENT: Color = Color::Cyan;
pub const SUCCESS: Color = Color::Green;
pub const ERROR: Color = Color::Red;
pub const WARNING: Color = Color::Yellow;
pub const MUTED: Color = Color::DarkGray;
pub const TEXT: Color = Color::White;
pub const TEXT_DIM: Color = Color::Gray;

// ── Severity ────────────────────────────────────────────────────────
pub const SEVERITY_CRITICAL: Color = Color::Red;
pub const SEVERITY_WARNING: Color = Color::Yellow;
pub const SEVERITY_INFO: Color = Color::Blue;

// ── UI Chrome ───────────────────────────────────────────────────────
pub const BORDER_FOCUSED: Color = Color::Cyan;
pub const BORDER_UNFOCUSED: Color = Color::DarkGray;
pub const HIGHLIGHT_BG: Color = Color::Blue;
pub const HIGHLIGHT_FG: Color = Color::White;
pub const BUNDLE_COLOR: Color = Color::Green;
pub const DIR_COLOR: Color = Color::Cyan;
pub const FILE_COLOR: Color = Color::DarkGray;

// ── Prebuilt styles ─────────────────────────────────────────────────

pub fn focused_border() -> Style {
    Style::default().fg(BORDER_FOCUSED)
}

pub fn unfocused_border() -> Style {
    Style::default().fg(BORDER_UNFOCUSED)
}

pub fn highlight() -> Style {
    Style::default()
        .bg(HIGHLIGHT_BG)
        .fg(HIGHLIGHT_FG)
        .add_modifier(Modifier::BOLD)
}

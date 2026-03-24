pub mod json;
pub mod markdown;

use clap::ValueEnum;

/// Output format for diagnostic reports.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Markdown,
    Json,
}

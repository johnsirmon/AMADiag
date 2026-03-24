use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

/// AMADiag — Azure Monitor Agent Diagnostic Analyzer
///
/// Analyzes AMA troubleshooter output bundles to identify common failures,
/// environment sizing issues, and actionable remediation steps.
#[derive(Parser)]
#[command(name = "amadiag", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze an AMA troubleshooter output bundle
    Analyze {
        /// Path to troubleshooter bundle (.tgz, .zip, or directory)
        path: PathBuf,

        /// Output format
        #[arg(short, long, default_value = "markdown")]
        format: amadiag::reporters::OutputFormat,

        /// Write report to file instead of stdout
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Validate a bundle without full analysis
    Validate {
        /// Path to troubleshooter bundle
        path: PathBuf,
    },

    /// Manage detection rules
    Rules {
        #[command(subcommand)]
        action: RulesAction,
    },
}

#[derive(Subcommand)]
enum RulesAction {
    /// List all available detection rules
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    match cli.command {
        Commands::Analyze {
            path,
            format,
            output,
        } => {
            let report = amadiag::detect::analyze_bundle(&path)?;

            let rendered = match format {
                amadiag::reporters::OutputFormat::Markdown => {
                    amadiag::reporters::markdown::render(&report)
                }
                amadiag::reporters::OutputFormat::Json => {
                    amadiag::reporters::json::render(&report)?
                }
            };

            if let Some(output_path) = output {
                std::fs::write(&output_path, &rendered)?;
                eprintln!("Report written to {}", output_path.display());
            } else {
                println!("{rendered}");
            }

            // Exit code 1 if critical findings
            if report.has_critical_findings() {
                std::process::exit(1);
            }
        }

        Commands::Validate { path } => {
            let validation = amadiag::input::validate_bundle(&path)?;
            println!("{validation}");
        }

        Commands::Rules {
            action: RulesAction::List,
        } => {
            let rules = amadiag::analyzers::rules::load_builtin_rules()?;
            amadiag::analyzers::rules::print_rules_table(&rules);
        }
    }

    Ok(())
}

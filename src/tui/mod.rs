mod app;
mod event;
mod export;
mod theme;
mod ui;

use crate::detect::TuiAnalysis;
use anyhow::Result;
use crossterm::{
    cursor::Show,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::Duration;

use self::app::{ActionResult, App};

pub fn run(path: Option<PathBuf>) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let _guard = TerminalGuard;
    let mut app = App::new(path);
    let mut analysis_rx = app.take_pending_analysis().map(spawn_analysis);

    loop {
        app.on_tick();
        receive_analysis_result(&mut app, &mut analysis_rx);

        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if let Some(action) = event::next_action(Duration::from_millis(100), app.screen())? {
            match app.handle_action(action) {
                ActionResult::None => {}
                ActionResult::Quit => break,
                ActionResult::Analyze(path) => {
                    analysis_rx = Some(spawn_analysis(path));
                }
                ActionResult::ShowExport(format) => {
                    let export_path = app.export_path().to_string();
                    if let Some(report) = app.report() {
                        match export::export_report_to(report, format, &export_path) {
                            Ok(path) => {
                                app.set_info_status(format!("Report saved to {}", path.display()));
                                // Return to dashboard after successful export
                                app.return_to_dashboard();
                            }
                            Err(err) => app.set_error_status(format!("Export failed: {err}")),
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn receive_analysis_result(
    app: &mut App,
    analysis_rx: &mut Option<Receiver<std::result::Result<TuiAnalysis, String>>>,
) {
    let Some(rx) = analysis_rx.as_ref() else {
        return;
    };

    match rx.try_recv() {
        Ok(result) => {
            *analysis_rx = None;
            app.finish_analysis(result);
        }
        Err(TryRecvError::Empty) => {}
        Err(TryRecvError::Disconnected) => {
            *analysis_rx = None;
            app.finish_analysis(Err("Analysis worker disconnected unexpectedly".to_string()));
        }
    }
}

fn spawn_analysis(path: PathBuf) -> Receiver<std::result::Result<TuiAnalysis, String>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = crate::detect::analyze_bundle_for_tui(&path).map_err(|err| err.to_string());
        let _ = tx.send(result);
    });
    rx
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, Show)?;
    Ok(())
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = restore_terminal();
    }
}

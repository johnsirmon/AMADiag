use super::app::{Action, Screen};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub fn next_action(timeout: Duration, screen: Screen) -> Result<Option<Action>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    let action = match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match screen {
            Screen::FileBrowser => match key.code {
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::Quit)
                }
                KeyCode::Esc => Some(Action::EditPath),
                KeyCode::Enter => Some(Action::Submit),
                KeyCode::Backspace => Some(Action::BrowserParent),
                KeyCode::Up => Some(Action::Previous),
                KeyCode::Down => Some(Action::Next),
                KeyCode::Home => Some(Action::Home),
                KeyCode::End => Some(Action::End),
                KeyCode::Char('t') => Some(Action::ToggleView),
                _ => None,
            },
            Screen::PathInput => match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::Quit)
                }
                KeyCode::Esc => Some(Action::EditPath),
                KeyCode::Enter => Some(Action::Submit),
                KeyCode::Backspace => Some(Action::Backspace),
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::ToggleView)
                }
                KeyCode::Char(ch) => Some(Action::InputChar(ch)),
                _ => None,
            },
            Screen::Analyzing => match key.code {
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::Quit)
                }
                _ => None,
            },
            Screen::Dashboard => match key.code {
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::Quit)
                }
                KeyCode::Esc | KeyCode::Char('n') => Some(Action::EditPath),
                KeyCode::Tab => Some(Action::FocusNext),
                KeyCode::BackTab => Some(Action::FocusPrevious),
                KeyCode::Left => Some(Action::FocusPrevious),
                KeyCode::Right => Some(Action::FocusNext),
                KeyCode::Up => Some(Action::Previous),
                KeyCode::Down => Some(Action::Next),
                KeyCode::PageUp => Some(Action::PageUp),
                KeyCode::PageDown => Some(Action::PageDown),
                KeyCode::Home => Some(Action::Home),
                KeyCode::End => Some(Action::End),
                KeyCode::Char('r') => Some(Action::Retry),
                KeyCode::Char('m') => Some(Action::ExportMarkdown),
                KeyCode::Char('j') => Some(Action::ExportJson),
                KeyCode::Char('1') => Some(Action::SeverityFilter(1)),
                KeyCode::Char('2') => Some(Action::SeverityFilter(2)),
                KeyCode::Char('3') => Some(Action::SeverityFilter(3)),
                _ => None,
            },
            Screen::Export => match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::Quit)
                }
                KeyCode::Esc => Some(Action::ExportCancel),
                KeyCode::Enter => Some(Action::ExportConfirm),
                KeyCode::Backspace => Some(Action::Backspace),
                KeyCode::Tab => Some(Action::ExportToggleFormat),
                KeyCode::Char(ch) => Some(Action::InputChar(ch)),
                _ => None,
            },
        },
        Event::Paste(text) => Some(Action::Paste(text)),
        _ => None,
    };

    Ok(action)
}

use super::app::Action;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub fn next_action(timeout: Duration) -> Result<Option<Action>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    let action = match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Action::Quit)
            }
            KeyCode::Enter => Some(Action::Submit),
            KeyCode::Backspace => Some(Action::Backspace),
            KeyCode::Tab => Some(Action::FocusNext),
            KeyCode::Up => Some(Action::Previous),
            KeyCode::Down => Some(Action::Next),
            KeyCode::PageUp => Some(Action::PageUp),
            KeyCode::PageDown => Some(Action::PageDown),
            KeyCode::Char('n') => Some(Action::EditPath),
            KeyCode::Char('r') => Some(Action::Retry),
            KeyCode::Char('m') => Some(Action::ExportMarkdown),
            KeyCode::Char('j') => Some(Action::ExportJson),
            KeyCode::Char(ch) => Some(Action::InputChar(ch)),
            _ => None,
        },
        Event::Paste(text) => Some(Action::Paste(text)),
        _ => None,
    };

    Ok(action)
}

use super::app::{App, Screen};
use super::widgets;
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &mut App) {
    match app.screen() {
        Screen::FileBrowser => widgets::browser::draw(frame, app),
        Screen::Analyzing => widgets::input::draw_analyzing(frame, app),
        Screen::Dashboard => widgets::dashboard::draw(frame, app),
        Screen::Export => widgets::export::draw(frame, app),
    }
}

use crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::start_sequencer;
use crate::tui::entities::{App, Menu};

pub fn draw(frame: &mut Frame) {}

pub fn handle_key(app: &mut App, key: KeyEvent) {

}

pub fn move_to(app: &mut App) {
    app.current_menu = Menu::Sequencer;
    start_sequencer(app.sequencer.clone(), app.sequencer_on.clone())
}
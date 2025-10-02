use crate::start_sequencer;
use crate::tui::entities::{App, Menu};
use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io;
use std::io::Error;

pub fn draw(frame: &mut Frame) {}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<(), io::Error> {
    Ok(())
}

pub fn move_to(app: &mut App) -> Result<(), io::Error> {
    match start_sequencer(app.sequencer.clone(), app.sequencer_on.clone()) {
        Ok(_) => {
            app.current_menu = Menu::Sequencer;
            Ok(())
        }
        Err(err) => {
            Err(err)
        }
    }
}

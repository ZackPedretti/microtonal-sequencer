use crate::tui::entities::App;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io;

pub fn draw(frame: &mut Frame) {}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<(), io::Error> {
    Ok(())
}

pub fn move_to(app: &mut App) -> Result<(), io::Error> {
    Ok(())
}

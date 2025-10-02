use crate::sequencer::Sequencer;
use crossterm::event::{poll, read, Event, KeyEvent};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::{CrosstermBackend};
use ratatui::{Frame, Terminal};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use crate::tui::entities::{App, Menu};
use crate::tui::menus::link_controller_menu;
use crate::tui::menus::main_menu;
use crate::tui::menus::sequencer_menu;

mod menus;
mod entities;

pub fn run_tui(sequencer: Arc<Mutex<Sequencer>>, on: Arc<AtomicBool>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(sequencer.clone(), on.clone());

    purge_events()?;    

    while app.tui_on.load(Ordering::SeqCst) {
        terminal.draw(|frame| {
            draw_ui(frame, &app);
        })?;

        if poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                handle_key(&mut app, key_event);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn purge_events() -> Result<(), io::Error> {
    while poll(std::time::Duration::from_millis(0))? {
        let _ = read()?;
    };
    Ok(())
}

fn exit(app: &mut App) {
    app.tui_on.store(false, Ordering::SeqCst);
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match &app.current_menu {
        Menu::Main { .. } => {main_menu::handle_key(app, key)},
        Menu::Sequencer => {sequencer_menu::handle_key(app, key)}
        Menu::LinkController => {link_controller_menu::handle_key(app, key)}
    }
}

fn draw_ui(frame: &mut Frame, app: &App) {
    match &app.current_menu {
        Menu::Main { selected_menu } => main_menu::draw(frame, selected_menu.as_index()),
        Menu::Sequencer => sequencer_menu::draw(frame),
        Menu::LinkController => link_controller_menu::draw(frame),
    }
}
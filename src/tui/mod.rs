use crate::sequencer::Sequencer;
use crate::tui::entities::{App, Menu};
use crate::tui::menus::sequencer_menu;
use crate::tui::menus::{error_screen, link_controller_menu};
use crate::tui::menus::{main_menu, settings_menu};
use crossterm::event::{poll, read, Event, KeyEvent};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use std::io;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use crate::tui::error_handling::MidiSequencerTUIResult;

mod entities;
mod menus;
mod error_handling;

pub fn run_tui(sequencer: Arc<Mutex<Sequencer>>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(sequencer.clone());

    purge_events()?;

    while app.tui_on.load(Ordering::SeqCst) {
        terminal.draw(|frame| {
            draw_ui(frame, &mut app);
        })?;

        if poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                handle_key(&mut app, key_event).unwrap_or_display_err(&mut app);
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
    }
    Ok(())
}

fn exit(app: &mut App) {
    app.tui_on.store(false, Ordering::SeqCst);
}

fn handle_key(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    match app.error {
        None => match &app.current_menu {
            Menu::Main { .. } => main_menu::handle_key(app, key_event),
            Menu::Sequencer { .. } => sequencer_menu::handle_key(app, key_event),
            Menu::LinkController => link_controller_menu::handle_key(app, key_event),
            Menu::Settings => settings_menu::handle_key(app, key_event),
        },
        Some(_) => Ok(error_screen::handle_key(app, key_event)),
    }
}

fn draw_ui(frame: &mut Frame, app: &mut App) {
    match &app.error {
        None => match &app.current_menu {
            Menu::Main { .. } => main_menu::draw(frame, app),
            Menu::Sequencer { .. } => sequencer_menu::draw(frame, app),
            Menu::LinkController => link_controller_menu::draw(frame),
            Menu::Settings => settings_menu::draw(frame),
        },
        Some(err) => {
            error_screen::draw(frame, err);
        }
    }
}

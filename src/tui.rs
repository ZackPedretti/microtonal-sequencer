use std::io;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::{Frame, Terminal};
use crate::sequencer::Sequencer;
use crate::{start_sequencer, stop_sequencer};

enum Menu {
    Main,
    Sequencer,
    LinkController,
}

struct App {
    current_menu: Menu,
}

pub fn run_tui(sequencer: Arc<Mutex<Sequencer>>, on: Arc<AtomicBool>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = App { current_menu: Menu::Main };
    
    loop {
        terminal.draw(|frame| {
            draw_ui(frame, &app);
        })?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn draw_ui<B: Backend>(frame: &mut Frame<B>, app: &App) {
    
}
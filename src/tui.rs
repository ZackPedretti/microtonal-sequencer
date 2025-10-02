use crate::sequencer::Sequencer;
use crate::{start_sequencer, stop_sequencer};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget};
use ratatui::{Frame, Terminal};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

enum Menu {
    Main { selected_menu: MainMenuItem },
    Sequencer,
    LinkController,
}

enum MainMenuItem {
    StartSequencer,
    LinkController,
    Exit,
}

impl MainMenuItem {
    fn as_index(&self) -> usize {
        match self {
            MainMenuItem::StartSequencer => 0,
            MainMenuItem::LinkController => 1,
            MainMenuItem::Exit => 2,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => MainMenuItem::StartSequencer,
            1 => MainMenuItem::LinkController,
            2 => MainMenuItem::Exit,
            _ => MainMenuItem::StartSequencer, // fallback
        }
    }

    fn length() -> usize {
        MainMenuItem::Exit.as_index() + 1
    }
}

struct App {
    tui_on: AtomicBool,
    current_menu: Menu,
    sequencer: Arc<Mutex<Sequencer>>,
    sequencer_on: Arc<AtomicBool>,
}

pub fn run_tui(sequencer: Arc<Mutex<Sequencer>>, on: Arc<AtomicBool>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        tui_on: AtomicBool::new(true),
        current_menu: Menu::Main {
            selected_menu: MainMenuItem::StartSequencer,
        },
        sequencer: sequencer.clone(),
        sequencer_on: on.clone(),
    };

    while poll(std::time::Duration::from_millis(0))? {
        let _ = read()?;
    }

    while app.tui_on.load(Ordering::SeqCst) {
        terminal.draw(|frame| {
            draw_ui(frame, &app);
        })?;

        if poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Enter => handle_enter_key(&mut app),
                        KeyCode::Up => handle_up_key(&mut app),
                        KeyCode::Down => handle_down_key(&mut app),
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn handle_enter_key(app: &mut App) {
    println!("Enter key");
    match &app.current_menu {
        Menu::Main { selected_menu } => match selected_menu {
            MainMenuItem::StartSequencer => start_sequencer_menu_action(app),
            MainMenuItem::LinkController => {}
            MainMenuItem::Exit => exit(app),
        },
        Menu::Sequencer => {}
        Menu::LinkController => {}
    }
}

fn exit(app: &mut App) {
    app.tui_on.store(false, Ordering::SeqCst);
}

fn handle_down_key(app: &mut App) {
    match &app.current_menu {
        Menu::Main { selected_menu } => {
            app.current_menu = Menu::Main {
                selected_menu: MainMenuItem::from_index(
                    (selected_menu.as_index() + 1) % MainMenuItem::length(),
                ),
            };
        }
        Menu::Sequencer => {}
        Menu::LinkController => {}
    }
}

fn handle_up_key(app: &mut App) {
    match &app.current_menu {
        Menu::Main { selected_menu } => {
            app.current_menu = Menu::Main {
                selected_menu: MainMenuItem::from_index(
                    (selected_menu.as_index() + MainMenuItem::length() - 1) % MainMenuItem::length(),
                ),
            };
        }
        Menu::Sequencer => {}
        Menu::LinkController => {}
    }
}

fn start_sequencer_menu_action(app: &mut App) {
    app.current_menu = Menu::Sequencer;
    start_sequencer(app.sequencer.clone(), app.sequencer_on.clone())
}

fn draw_ui(frame: &mut Frame, app: &App) {
    match &app.current_menu {
        Menu::Main { selected_menu } => draw_main_menu(frame, selected_menu.as_index()),
        Menu::Sequencer => draw_sequencer_menu(frame),
        Menu::LinkController => draw_link_controller_menu(frame),
    }
}

fn draw_main_menu(frame: &mut Frame, menu_index: usize) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    let mut menus = vec![
        ListItem::new("Start sequencer"),
        ListItem::new("Link controller"),
    ];

    menus.push(ListItem::new("Exit"));

    let list = List::new(menus)
        .block(
            Block::bordered()
                .title(" Microtonal Sequencer ".to_span().into_centered_line())
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::LightBlue))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::LightBlue))
        .highlight_symbol("➤ ");

    let mut state = ListState::default();
    state.select(Some(menu_index));

    frame.render_stateful_widget(list, outer_layout[0], &mut state);
}

fn draw_sequencer_menu(frame: &mut Frame) {}

fn draw_link_controller_menu(frame: &mut Frame) {}

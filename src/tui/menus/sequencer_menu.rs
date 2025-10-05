use crate::init_sequencer;
use crate::tui::entities::{App, Menu, SequencerMenuItem, SubMenuItem};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, List, ListItem, ListState};
use ratatui::Frame;
use std::io;
use std::sync::atomic::Ordering;

pub fn draw(frame: &mut Frame, app: &App) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    let mut menus = vec![
        ListItem::new(match &app.sequencer_on.load(Ordering::SeqCst) {
            true => "On",
            false => "Off",
        }),
        ListItem::new("Scale"),
        ListItem::new("Sequence playlist"),
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

    let mut menu_state = ListState::default();

    let selected_index = get_selected(app);

    menu_state.select(selected_index.0);

    frame.render_stateful_widget(list, outer_layout[0], &mut menu_state);
}

fn get_selected(app: &App) -> (Option<usize>, Option<usize>) {
    match &app.current_menu {
        Menu::Sequencer {
            selected_menu,
            selected_note,
        } => match selected_menu {
            Some(menu) => (Some(menu.as_index()), None),
            None => (None, *selected_note),
        },
        _ => (None, None),
    }
}

pub fn handle_key(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    match get_selected(app) {
        (Some(..), None) => handle_key_submenu(app, key_event),
        (None, Some(..)) => handle_key_notes(app, key_event),
        (_, _) => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn handle_key_submenu(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    if key_event.kind == KeyEventKind::Press {
        return match key_event.code {
            KeyCode::Enter => on_enter_submenu(app),
            KeyCode::Up => on_up_submenu(app),
            KeyCode::Down => on_down_submenu(app),
            KeyCode::Char('q') => { 
                app.tui_on.store(false, Ordering::SeqCst);
                return Ok(());
            }
            _ => Ok(()),
        };
    }
    Ok(())
}

fn on_enter_submenu(app: &mut App) -> Result<(), io::Error> {
    match &app.current_menu {
        Menu::Sequencer { selected_menu, .. } => match selected_menu {
            None => Err(io::Error::new(io::ErrorKind::Other, "Invalid menu")),
            Some(menu) => match menu {
                SequencerMenuItem::OnOff => handle_on_off(app),
                SequencerMenuItem::Scale => handle_scale_menu(app),
                SequencerMenuItem::Playlist => handle_playlist_menu(app),
                SequencerMenuItem::Exit => {handle_exit(app)}
            },
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid menu")),
    }
}

fn on_up_submenu(app: &mut App) -> Result<(), io::Error> {
    Ok(())
}

fn on_down_submenu(app: &mut App) -> Result<(), io::Error> {
    Ok(())
}

fn handle_on_off(app: &mut App) -> Result<(), io::Error> {
    app.sequencer_on.store(!app.sequencer_on.load(Ordering::SeqCst), Ordering::SeqCst);
    if app.sequencer_on.load(Ordering::SeqCst) {
        return start_sequencer(app);
    }
    Ok(())
}

fn handle_scale_menu(app: &mut App) -> Result<(), io::Error> {
    Ok(())
}

fn handle_playlist_menu(app: &mut App) -> Result<(), io::Error> {
    Ok(())
}

fn handle_exit(app: &mut App) -> Result<(), io::Error> {
    Ok(())
}

fn handle_key_notes(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    if key_event.kind == KeyEventKind::Press {}
    Ok(())
}

fn start_sequencer(app: &mut App) -> Result<(), io::Error> {
    init_sequencer(app.sequencer.clone(), app.sequencer_on.clone())
}


pub fn move_to(app: &mut App) -> Result<(), io::Error> {
    match start_sequencer(app) {
        Ok(_) => {
            app.current_menu = Menu::Sequencer {
                selected_menu: Some(SequencerMenuItem::from_index(0)),
                selected_note: None,
            };
            Ok(())
        }
        Err(err) => Err(err),
    }
}


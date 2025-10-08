use std::io;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, List, ListItem, ListState};
use crate::tui::{exit, App};
use crate::tui::entities::{MainMenuItem, Menu, MenuItemList};
use crate::tui::menus::{link_controller_menu, sequencer_menu, settings_menu};

pub fn draw(frame: &mut Frame, app: &App) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    let mut menus = vec![
        ListItem::new("Start sequencer"),
        ListItem::new("Link controller"),
        ListItem::new("Settings")
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
    
    let menu_index = match &app.current_menu {
        Menu::Main { selected_menu } => { Some(selected_menu.as_index()) }
        _ => { None }
    };

    let mut state = ListState::default();
    state.select(menu_index);

    frame.render_stateful_widget(list, outer_layout[0], &mut state);
}

pub fn handle_key(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    if key_event.kind == KeyEventKind::Press {
        return match key_event.code {
            KeyCode::Enter => on_enter(app),
            KeyCode::Up => Ok(on_up(app)),
            KeyCode::Down => Ok(on_down(app)),
            _ => {Ok(())}
        }
    }
    Ok(())
}

fn on_enter(app: &mut App) -> Result<(), io::Error> {
    match &app.current_menu {
        Menu::Main { selected_menu } => match selected_menu {
            MainMenuItem::StartSequencer => sequencer_menu::move_to(app),
            MainMenuItem::LinkController => link_controller_menu::move_to(app),
            MainMenuItem::Settings => settings_menu::move_to(app),
            MainMenuItem::Exit => Ok(exit(app)),
        },
        _ => {Ok(())}
    }
}
fn on_up(app: &mut App) {
    match &app.current_menu {
        Menu::Main { selected_menu } => {
            app.current_menu = Menu::Main {
                selected_menu: MainMenuItem::from_index(
                    (selected_menu.as_index() + MainMenuItem::length() - 1) % MainMenuItem::length(),
                ),
            };
        }
        _ => {}
    }
}
fn on_down(app: &mut App) {
    match &app.current_menu {
        Menu::Main { selected_menu } => {
            app.current_menu = Menu::Main {
                selected_menu: MainMenuItem::from_index(
                    (selected_menu.as_index() + 1) % MainMenuItem::length(),
                ),
            };
        }
        _ => {}
    }
}
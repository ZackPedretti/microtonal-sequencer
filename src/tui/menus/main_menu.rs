use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, List, ListItem, ListState};
use crate::tui::{exit, App};
use crate::tui::entities::{MainMenuItem, Menu};
use crate::tui::menus::{link_controller_menu, sequencer_menu};

pub fn draw(frame: &mut Frame, menu_index: usize) {
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

pub fn handle_key(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Enter => on_enter(app),
            KeyCode::Up => on_up(app),
            KeyCode::Down => on_down(app),
            _ => {}
        }
    }
}

fn on_enter(app: &mut App) {
    match &app.current_menu {
        Menu::Main { selected_menu } => match selected_menu {
            MainMenuItem::StartSequencer => sequencer_menu::move_to(app),
            MainMenuItem::LinkController => {link_controller_menu::move_to(app)}
            MainMenuItem::Exit => exit(app),
        },
        _ => {}
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
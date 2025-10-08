use crate::init_sequencer;
use crate::tui::entities::{App, MainMenuItem, Menu, SequencerMenuItem, MenuItemList};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Style, Stylize, Text};
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use ratatui::Frame;
use std::io;
use std::sync::atomic::Ordering;

pub fn draw(frame: &mut Frame, app: &App) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(frame.area());

    let inner_upper_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(outer_layout[0]);

    let mut menus = vec![
        ListItem::new(match &app.sequencer_on.load(Ordering::SeqCst) {
            true => "ON",
            false => "OFF",
        }),
        ListItem::new(app.sequencer.lock().unwrap().current_scale_name()),
        ListItem::new("Save sequence"),
        ListItem::new("Load sequence"),
    ];

    menus.push(ListItem::new("Exit"));

    let list = List::new(menus)
        .block(
            Block::bordered()
                .style(Style::default().fg(Color::White))
                .title(" Options ".to_span().into_centered_line()),
        )
        .style(Style::default().fg(Color::LightBlue))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::LightBlue));

    let mut menu_state = ListState::default();

    let selected_index = get_selected(app);

    menu_state.select(selected_index.0);

    frame.render_stateful_widget(list, inner_upper_layout[0], &mut menu_state);

    let current_sequence_name = app.sequencer.lock().unwrap().current_sequence_name();
    let sequence_block_title = format!(" {} ", current_sequence_name);

    let sequence_block = Block::bordered()
        .style(Style::default().fg(Color::White))
        .title(sequence_block_title.to_span().into_centered_line());

    let sequence_area = sequence_block.inner(inner_upper_layout[1]);
    frame.render_widget(sequence_block, inner_upper_layout[1]);

    let note_height = 5;
    let note_width = note_height * 2;

    let mut y = sequence_area.top();
    let mut x =
        sequence_area.left() + (((note_width as f32 * 1.2) - note_width as f32) / 2f32) as u16;
    let base_x = x;

    let sequencer = app.sequencer.lock().unwrap();
    let notes = sequencer.current_sequence().notes;
    let current_note_index = sequencer.current_note_index;
    drop(sequencer);

    for (i, note) in notes.iter().enumerate() {
        let rect = Rect::new(x, y, note_width, note_height);

        let border_color = if i == current_note_index {
            Color::Red
        } else {
            Color::Gray
        };

        let mut note_block = Block::bordered().border_style(Style::default().fg(border_color));

        if let Some(s) = selected_index.1 {
            if i == s {
                note_block = Block::bordered()
                    .border_style(Style::default().fg(border_color))
                    .bg(Color::LightBlue);
            }
        }

        let text = Text::from(vec![
            Line::from(note.get_common_name()).centered(),
            Line::from(note.velocity.to_string()).centered(),
            Line::from(note.duration.duration.to_string()).centered(),
        ]);

        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        frame.render_widget(note_block.clone(), rect);
        let inner_rect = note_block.inner(rect);
        frame.render_widget(paragraph, inner_rect);

        x += (note_width as f32 * 1.2) as u16;
        if (x + note_width) > sequence_area.right() {
            x = base_x;
            y += note_height;
        }
    }

    let playlist_block = Block::bordered()
        .style(Style::default().fg(Color::White))
        .title(" Playlist ".to_span().into_centered_line());

    frame.render_widget(playlist_block, outer_layout[1]);
}

fn get_selected(app: &App) -> (Option<usize>, Option<usize>, Option<usize>) {
    match &app.current_menu {
        Menu::Sequencer {
            selected_menu,
            selected_note,
            selected_sequence,
        } => match selected_menu {
            Some(menu) => (Some(menu.as_index()), None, None),
            None => (None, *selected_note, *selected_sequence),
        },
        _ => (Some(0), None, None),
    }
}

pub fn handle_key(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    match get_selected(app) {
        (Some(..), None, None) => handle_key_submenu(app, key_event),
        (None, Some(..), None) => handle_key_notes(app, key_event),
        (None, None, Some(..)) => handle_key_playlist(app, key_event),
        (_, _, _) => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn handle_key_submenu(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    if key_event.kind == KeyEventKind::Press {
        return match key_event.code {
            KeyCode::Enter => on_enter_submenu(app),
            KeyCode::Up => on_up_submenu(app),
            KeyCode::Down => on_down_submenu(app),
            KeyCode::Right => on_right_submenu(app),
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
                SequencerMenuItem::Exit => handle_exit(app),
                _ => Ok(()),
            },
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid menu")),
    }
}

fn on_up_submenu(app: &mut App) -> Result<(), io::Error> {
    match &app.current_menu {
        Menu::Sequencer { selected_menu, .. } => match selected_menu {
            Some(menu) => {
                if menu.as_index() == 0 {
                    return Ok(());
                }
                app.current_menu = Menu::Sequencer {
                    selected_menu: Some(SequencerMenuItem::from_index(
                        (menu.as_index() + SequencerMenuItem::length() - 1)
                            % SequencerMenuItem::length(),
                    )),
                    selected_note: None,
                    selected_sequence: None,
                };
                Ok(())
            }
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid menu")),
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid menu")),
    }
}

fn on_down_submenu(app: &mut App) -> Result<(), io::Error> {
    match &app.current_menu {
        Menu::Sequencer { selected_menu, .. } => match selected_menu {
            Some(menu) => {
                if menu.as_index() == SequencerMenuItem::length() - 1 {
                    app.current_menu = Menu::Sequencer {
                        selected_menu: None,
                        selected_note: None,
                        selected_sequence: Some(0),
                    };
                    return Ok(());
                }
                app.current_menu = Menu::Sequencer {
                    selected_menu: Some(SequencerMenuItem::from_index(
                        (menu.as_index() + SequencerMenuItem::length() + 1)
                            % SequencerMenuItem::length(),
                    )),
                    selected_note: None,
                    selected_sequence: None,
                };
                Ok(())
            }
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid menu")),
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid menu")),
    }
}

fn on_right_submenu(app: &mut App) -> Result<(), io::Error> {
    app.current_menu = Menu::Sequencer {
        selected_menu: None,
        selected_note: Some(0),
        selected_sequence: None,
    };
    Ok(())
}

fn handle_on_off(app: &mut App) -> Result<(), io::Error> {
    app.sequencer_on
        .store(!app.sequencer_on.load(Ordering::SeqCst), Ordering::SeqCst);
    if app.sequencer_on.load(Ordering::SeqCst) {
        return start_sequencer(app);
    }
    Ok(())
}

fn handle_scale_menu(app: &mut App) -> Result<(), io::Error> {
    Ok(())
}

fn handle_exit(app: &mut App) -> Result<(), io::Error> {
    app.sequencer_on.store(false, Ordering::SeqCst);
    app.current_menu = Menu::Main {
        selected_menu: MainMenuItem::from_index(0),
    };
    Ok(())
}

fn handle_key_notes(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    if key_event.kind == KeyEventKind::Press {}
    Ok(())
}

fn handle_key_playlist(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
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
                selected_sequence: None,
            };
            Ok(())
        }
        Err(err) => Err(err),
    }
}

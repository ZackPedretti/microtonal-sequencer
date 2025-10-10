use crate::init_sequencer;
use crate::note::Note;
use crate::tui::entities::{
    App, MainMenuItem, Menu, MenuItemList, SequencerMenuItem, SequencerMenuSelectedItem,
};
use crate::tui::error_handling::MidiSequencerTUIResult;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Style, Text};
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;
use std::io;
use std::sync::atomic::Ordering;

pub fn draw(frame: &mut Frame, app: &mut App) {
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

    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::SubMenuItem { item } => menu_state.select(Some(item.as_index())),
        _ => menu_state.select(None),
    }

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

    let spacing = 2;

    let mut y = sequence_area.top();
    let mut x = sequence_area.left() + spacing;
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
            Color::White
        };

        let selected = if let SequencerMenuSelectedItem::Note { item } = get_selected(app)
            .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
        {
            Some(item)
        } else {
            None
        };

        let bg_color = match selected {
            None => Color::Reset,
            Some(selected_i) => {
                if selected_i == i {
                    if app.sequencer.lock().unwrap().started {
                        Color::LightBlue
                    } else if app.held_keys.contains(&KeyCode::Char('v')) {
                        Color::Green
                    } else if app.held_keys.contains(&KeyCode::Char('b')) {
                        Color::Magenta
                    } else if app.held_keys.contains(&KeyCode::Char('n')) {
                        Color::Cyan
                    } else {
                        Color::LightBlue
                    }
                } else {
                    Color::Reset
                }
            }
        };

        let note_block = Block::bordered()
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(bg_color));

        let text = Text::from(vec![
            Line::from(note.get_common_name()).centered(),
            Line::from(note.duration.duration.to_string()).centered(),
            Line::from(note.velocity.to_string()).centered(),
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

    let sequencer = app.sequencer.lock().unwrap();
    let sequences = sequencer.sequences.clone();
    let current_sequence_index = sequencer.current_sequence_index;

    drop(sequencer);

    let rect_width = 20;
    let playlist_area = playlist_block.inner(outer_layout[1]);
    frame.render_widget(playlist_block, outer_layout[1]);
    let rect_height = playlist_area.height;
    let mut x = playlist_area.left() + spacing;

    for (i, sequence) in sequences.iter().enumerate() {
        let rect = Rect::new(x, playlist_area.top(), rect_width, rect_height);

        let border_color = if i == current_sequence_index {
            Color::Red
        } else {
            Color::White
        };

        let sequence_block = Block::bordered()
            .border_style(Style::default().fg(border_color))
            .style(Style::default().fg(Color::White).bg(Color::Reset));

        let text = Text::from(vec![
            Line::from(sequence.name.clone()).centered(),
            Line::from(format!("X{}", sequence.repeat + 1)).centered(),
        ]);

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(sequence_block.clone(), rect);
        let inner_rect = sequence_block.inner(rect);
        frame.render_widget(paragraph, inner_rect);

        x += rect_width + spacing;
    }
}

fn get_selected(app: &App) -> Result<SequencerMenuSelectedItem, io::Error> {
    match &app.current_menu {
        Menu::Sequencer { selected_menu } => Ok(selected_menu.clone()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

pub fn handle_key(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::SubMenuItem { .. } => handle_key_submenu(app, key_event),
        SequencerMenuSelectedItem::Note { .. } => handle_key_notes(app, key_event),
        SequencerMenuSelectedItem::PlaylistItem { .. } => handle_key_playlist(app, key_event),
        SequencerMenuSelectedItem::Scale { .. } => Ok(()),
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
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::SubMenuItem { item } => match item {
            SequencerMenuItem::OnOff => handle_on_off(app),
            SequencerMenuItem::Scale => handle_scale_menu(app),
            SequencerMenuItem::Exit => handle_exit(app),
            _ => Ok(()),
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn on_up_submenu(app: &mut App) -> Result<(), io::Error> {
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::SubMenuItem { item } => {
            if item.as_index() > 0 {
                app.current_menu = Menu::Sequencer {
                    selected_menu: SequencerMenuSelectedItem::SubMenuItem {
                        item: SequencerMenuItem::from_index(item.as_index() - 1),
                    },
                }
            }
            Ok(())
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn on_down_submenu(app: &mut App) -> Result<(), io::Error> {
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::SubMenuItem { item } => {
            if item.as_index() < SequencerMenuItem::length() - 1 {
                app.current_menu = Menu::Sequencer {
                    selected_menu: SequencerMenuSelectedItem::SubMenuItem {
                        item: SequencerMenuItem::from_index(item.as_index() + 1),
                    },
                }
            }
            Ok(())
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn on_right_submenu(app: &mut App) -> Result<(), io::Error> {
    app.current_menu = Menu::Sequencer {
        selected_menu: SequencerMenuSelectedItem::Note { item: 0 },
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
    todo!()
}

fn handle_exit(app: &mut App) -> Result<(), io::Error> {
    app.sequencer_on.store(false, Ordering::SeqCst);
    app.current_menu = Menu::Main {
        selected_menu: MainMenuItem::from_index(0),
    };
    Ok(())
}

fn handle_key_notes(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    if key_event.kind == KeyEventKind::Press {
        return match key_event.code {
            KeyCode::Left => on_left_notes(app),
            KeyCode::Right => on_right_notes(app),
            KeyCode::Up => on_up_notes(app),
            KeyCode::Down => on_down_notes(app),
            _ => Ok(()),
        };
    }
    Ok(())
}

fn on_left_notes(app: &mut App) -> Result<(), io::Error> {
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::Note { item } => {
            if item > 0 {
                app.current_menu = Menu::Sequencer {
                    selected_menu: SequencerMenuSelectedItem::Note { item: item - 1 },
                };
                return Ok(());
            }
            app.current_menu = Menu::Sequencer {
                selected_menu: SequencerMenuSelectedItem::default(),
            };
            Ok(())
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn on_right_notes(app: &mut App) -> Result<(), io::Error> {
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::Note { item } => {
            if item < app.sequencer.lock().unwrap().current_sequence_length() - 1 {
                app.current_menu = Menu::Sequencer {
                    selected_menu: SequencerMenuSelectedItem::Note { item: item + 1 },
                };
            }
            Ok(())
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn on_up_notes(app: &mut App) -> Result<(), io::Error> {
    if app.held_keys.is_empty() || app.sequencer.lock().unwrap().started {
        return Ok(());
    }
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::Note { item } => {
            let mut sequencer = app.sequencer.lock().unwrap();
            let current_sequence_i = sequencer.current_sequence_index;
            let sequence = &mut sequencer.sequences[current_sequence_i];
            let note = &mut sequence.notes[item];
            if app.held_keys.contains(&KeyCode::Char('n')) {
                increment_tonality_of_note(note);
            }
            Ok(())
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

fn on_down_notes(app: &mut App) -> Result<(), io::Error> {
    if app.held_keys.is_empty() || app.sequencer.lock().unwrap().started {
        return Ok(());
    }
    match get_selected(app)
        .unwrap_or_default_val_and_display_err(app, SequencerMenuSelectedItem::default())
    {
        SequencerMenuSelectedItem::Note { item } => {
            let mut sequencer = app.sequencer.lock().unwrap();
            let current_sequence_i = sequencer.current_sequence_index;
            let sequence = &mut sequencer.sequences[current_sequence_i];
            let note = &mut sequence.notes[item];
            if app.held_keys.contains(&KeyCode::Char('n')) {
                decrement_tonality_of_note(note);
            }
            Ok(())
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "Out of bounds")),
    }
}

// TODO: Increase by octave when holding shift
fn increment_tonality_of_note(note: &mut Note) {
    let scale_len = note.scale.steps.len();
    note.note_index = (note.note_index + 1) % scale_len;
    if note.note_index == 0 {
        note.octave += 1;
    }
}

fn decrement_tonality_of_note(note: &mut Note) {
    let scale_len = note.scale.steps.len();
    note.note_index = (note.note_index + scale_len - 1) % scale_len;
    if note.note_index == scale_len - 1 {
        note.octave -= 1;
    }
}

fn handle_key_playlist(app: &mut App, key_event: KeyEvent) -> Result<(), io::Error> {
    if key_event.kind == KeyEventKind::Press {}
    todo!();
}

fn start_sequencer(app: &mut App) -> Result<(), io::Error> {
    init_sequencer(app.sequencer.clone(), app.sequencer_on.clone())
}

pub fn move_to(app: &mut App) -> Result<(), io::Error> {
    match start_sequencer(app) {
        Ok(_) => {
            app.current_menu = Menu::Sequencer {
                selected_menu: SequencerMenuSelectedItem::default(),
            };
            Ok(())
        }
        Err(err) => Err(err),
    }
}

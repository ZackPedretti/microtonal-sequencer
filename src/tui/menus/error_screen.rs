use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use std::io;
use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::text::ToSpan;
use crate::tui::entities::App;

pub fn draw(frame: &mut Frame, err: &io::Error) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());
    

    let text = format!("Error: {}\n\nPress any key to continue", err);

    let paragraph = Paragraph::new(text)
        .block(Block::default()
                   .borders(Borders::ALL)
                   .title(" Error ".to_span().into_centered_line())
                   .border_style(Style::default().fg(Color::Red)))
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, outer_layout[0]);
}

pub fn handle_key(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        app.error = None;
    }
}
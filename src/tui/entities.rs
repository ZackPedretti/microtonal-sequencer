use std::io;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use crate::sequencer::Sequencer;

pub enum Menu {
    Main { selected_menu: MainMenuItem },
    Sequencer,
    LinkController,
}

pub enum MainMenuItem {
    StartSequencer,
    LinkController,
    Exit,
}

impl MainMenuItem {
    pub(crate) fn as_index(&self) -> usize {
        match self {
            MainMenuItem::StartSequencer => 0,
            MainMenuItem::LinkController => 1,
            MainMenuItem::Exit => 2,
        }
    }

    pub(crate) fn from_index(index: usize) -> Self {
        match index {
            0 => MainMenuItem::StartSequencer,
            1 => MainMenuItem::LinkController,
            2 => MainMenuItem::Exit,
            _ => MainMenuItem::StartSequencer, // fallback
        }
    }

    pub(crate) fn length() -> usize {
        MainMenuItem::Exit.as_index() + 1
    }
}

pub struct App {
    pub(crate) tui_on: AtomicBool,
    pub(crate) current_menu: Menu,
    pub(crate) sequencer: Arc<Mutex<Sequencer>>,
    pub(crate) sequencer_on: Arc<AtomicBool>,
    pub error: Option<io::Error>,
}

impl App {
    pub fn new(sequencer: Arc<Mutex<Sequencer>>, sequencer_on: Arc<AtomicBool>) -> Self {
        App {
            tui_on: AtomicBool::new(true),
            current_menu: Menu::Main {
                selected_menu: MainMenuItem::StartSequencer,
            },
            sequencer,
            sequencer_on,
            error: None,
        }
    }
}
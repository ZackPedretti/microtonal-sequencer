use std::io;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use crate::sequencer::Sequencer;

pub enum Menu {
    Main { selected_menu: MainMenuItem },
    Sequencer { selected_menu: Option<SequencerMenuItem>, selected_note: Option<usize>, selected_sequence: Option<usize> } ,
    LinkController,
    Settings
}

pub trait MenuItemList {
    fn as_index(&self) -> usize;
    fn from_index(index: usize) -> Self;
    fn length() -> usize;
}

pub enum MainMenuItem {
    StartSequencer,
    LinkController,
    Settings,
    Exit,
}

pub enum SequencerMenuItem {
    OnOff,
    Scale,
    Save,
    Load,
    Exit,
}

impl MenuItemList for MainMenuItem {
    fn as_index(&self) -> usize {
        match self {
            MainMenuItem::StartSequencer => 0,
            MainMenuItem::LinkController => 1,
            MainMenuItem::Settings => 2,
            MainMenuItem::Exit => 3,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => MainMenuItem::StartSequencer,
            1 => MainMenuItem::LinkController,
            2 => MainMenuItem::Settings,
            3 => MainMenuItem::Exit,
            _ => MainMenuItem::StartSequencer, // fallback
        }
    }

    fn length() -> usize {
        MainMenuItem::Exit.as_index() + 1
    }
}

impl MenuItemList for SequencerMenuItem {
    fn as_index(&self) -> usize {
        match self {
            SequencerMenuItem::OnOff => 0,
            SequencerMenuItem::Scale => 1,
            SequencerMenuItem::Save => 2,
            SequencerMenuItem::Load => 3,
            SequencerMenuItem::Exit => 4,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => SequencerMenuItem::OnOff,
            1 => SequencerMenuItem::Scale,
            2 => SequencerMenuItem::Save,
            3 => SequencerMenuItem::Load,
            4 => SequencerMenuItem::Exit,
            _ => SequencerMenuItem::OnOff, // fallback
        }
    }

    fn length() -> usize {
        SequencerMenuItem::Exit.as_index() + 1
    }
}

pub struct App {
    pub(crate) tui_on: AtomicBool,
    pub sequencer_on: Arc<AtomicBool>,
    pub(crate) current_menu: Menu,
    pub(crate) sequencer: Arc<Mutex<Sequencer>>,
    pub error: Option<io::Error>,
}

impl App {
    pub fn new(sequencer: Arc<Mutex<Sequencer>>) -> Self {
        App {
            tui_on: AtomicBool::new(true),
            sequencer_on: Arc::new(AtomicBool::new(false)),
            current_menu: Menu::Main {
                selected_menu: MainMenuItem::StartSequencer,
            },
            sequencer,
            error: None,
        }
    }
}
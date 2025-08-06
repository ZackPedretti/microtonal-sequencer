use std::sync::{Arc, Mutex};
use crate::note::{Note, Scale };

pub(crate) struct Sequencer {
    scale: Arc<Mutex<Scale>>,
    notes: Arc<Mutex<Vec<Note>>>,
    current_note_index: usize,
    previous_note_index: Option<usize>,
}

impl Sequencer {
    pub(crate) fn new() -> Self {
        let scale = Arc::new(Mutex::new(Scale::new_12_tet()));
        let notes = Arc::new(Mutex::new(vec![Note::new(scale.clone()); 8]));
        Self {
            scale,
            notes,
            current_note_index: 0,
            previous_note_index: None,
        }
    }

    pub(crate) fn current_note(&self) -> Note {
        (self.notes.lock().unwrap()[self.current_note_index]).clone()
    }

    pub(crate) fn previous_note(&self) -> Option<Note> {
        match self.previous_note_index {
            None => None,
            Some(i) => Some(self.notes.lock().unwrap()[i].clone()),
        }
    }

    pub(crate) fn next(&mut self) {
        self.previous_note_index = Some(self.current_note_index);
        let notes = self.notes.lock().unwrap();
        self.current_note_index = (self.current_note_index + 1) % notes.len();
    }

    pub(crate) fn reset(&mut self) {
        self.current_note_index = 0;
        self.previous_note_index = None;
    }
}
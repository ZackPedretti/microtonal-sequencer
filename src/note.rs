use std::fmt;
use std::sync::{Arc};

pub(crate) struct Scale {
    pub(crate) name: String,
    pub(crate) steps: Vec<f64>,
    pub(crate) note_names: Vec<String>,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Scale {{ name: {}, steps: {:?}, note_names: {:?} }}",
            self.name, self.steps, self.note_names
        )
    }
}

#[derive(Clone)]
pub(crate) struct Note {
    pub(crate) scale: Arc<Scale>,
    pub(crate) octave: u8,
    pub(crate) duration: NoteDuration,
    pub(crate) note_index: u8,
    pub(crate) velocity: u8,
    pub(crate) panning: u8,
}

impl Note {

    pub fn get_midi_number(&self) -> f64 {
        let scale_note = match self.scale.steps.get(self.note_index as usize) {
            None => self.scale.steps[self.scale.steps.len() - 1],
            Some(n) => *n,
        };
        scale_note + (self.octave * 12) as f64
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Note(scale: {}, octave: {}, duration: {}, note: {}, velocity: {}, panning: {})",
            self.scale.name,
            self.octave,
            self.duration.duration,
            self.scale.note_names[self.note_index as usize],
            self.velocity,
            self.panning,
        )
    }
}

#[derive(Clone)]
pub(crate) struct NoteDuration {
    pub(crate) duration: f64,
}

impl NoteDuration {
    pub fn get_tick_length(&self) -> u8 {
        (self.duration * 24f64).round() as u8
    }
}

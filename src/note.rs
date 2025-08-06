use std::sync::{Arc, Mutex};

pub(crate) struct Scale {
    name: &'static str,
    steps: Vec<f64>,
}

impl Scale {
    pub fn new_12_tet() -> Self {
        Scale {
            name: "12-TET",
            steps: (0..12).map(|x| x as f64).collect()
        }
    }
    pub fn new_24_tet() -> Self {
        Scale {
            name: "24-TET",
            steps: (0..24).map(|x| x as f64 / 2f64).collect()
        }
    }
}

#[derive(Clone)]
pub(crate) struct Note {
    scale: Arc<Mutex<Scale>>,
    octave: u8,
    pub(crate) duration: NoteDuration,
    note_index: usize,
    velocity: u8,
    panning: u8,
}

impl Note {
    pub(crate) fn new(scale: Arc<Mutex<Scale>>) -> Self {
        const BASE_OCTAVE: u8 = 5;
        const BASE_VELOCITY: u8 = 100;
        const PANNING_CENTER: u8 = 64;
        Note {
            scale,
            octave: BASE_OCTAVE,
            duration: NoteDuration { duration: 0.5 },
            note_index: 0,
            velocity: BASE_VELOCITY,
            panning: PANNING_CENTER,
        }
    }

    pub fn get_midi_number(&self) -> f64 {
        let scale = self.scale.lock().unwrap();
        let scale_note = match scale.steps.get(self.note_index) {
            None => {scale.steps[scale.steps.len() - 1]},
            Some(n) => { *n }
        };
        scale_note + (self.octave * 12) as f64
    }
}

#[derive(Clone)]
pub(crate) struct NoteDuration {
    duration: f64,
}

impl NoteDuration {
    pub fn get_tick_length(&self) -> u8 {
        (self.duration * 24f64).round() as u8
    }
}
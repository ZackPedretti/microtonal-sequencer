use std::fmt;
use std::sync::{Arc, Mutex};

pub(crate) struct Scale {
    pub(crate) name: String,
    pub(crate) steps: Vec<f64>,
    pub(crate) note_names: Vec<String>,
}

impl Scale {
    pub fn new_12_tet() -> Self {
        Scale {
            name: "12-TET".parse().unwrap(),
            steps: (0..12).map(|x| x as f64).collect(),
            note_names: vec![
                "C".parse().unwrap(),
                "C# / Db".parse().unwrap(),
                "D".parse().unwrap(),
                "D# / Eb".parse().unwrap(),
                "E".parse().unwrap(),
                "F".parse().unwrap(),
                "F# / Gb".parse().unwrap(),
                "G".parse().unwrap(),
                "G# / Ab".parse().unwrap(),
                "A".parse().unwrap(),
                "A# / Bb".parse().unwrap(),
                "B".parse().unwrap(),
            ],
        }
    }
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
    pub(crate) scale: Arc<Mutex<Scale>>,
    pub(crate) octave: u8,
    pub(crate) duration: NoteDuration,
    pub(crate) note_index: usize,
    pub(crate) velocity: u8,
    pub(crate) panning: u8,
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
            None => scale.steps[scale.steps.len() - 1],
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
            self.scale.lock().unwrap().name,
            self.octave,
            self.duration.duration,
            self.scale.lock().unwrap().note_names[self.note_index],
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

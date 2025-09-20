use std::fmt;
use std::sync::{Arc, Mutex};
use crate::note::{Note, NoteDuration, Scale};

/// A sequence of notes that plays using a specified scale.
///
/// This struct manages the playback of a note sequence using the provided `Scale`
/// and a vector of `Note`s. The sequence can be of any length.
///
/// - `current_note_index` tracks the currently playing note.
/// - `previous_note_index` helps identify and stop the previously played note.
/// - `repeat` controls how many times the sequence will repeat:
///     - `0` means it plays once.
///     - `n` means it plays `n + 1` times.
pub(crate) struct Sequence {
    scale: Arc<Mutex<Scale>>,
    notes: Arc<Mutex<Vec<Note>>>,
    current_note_index: usize,
    previous_note_index: Option<usize>,
    repeat: u8,
}

impl Sequence {
    pub(crate) fn new() -> Self {
        let scale = Arc::new(Mutex::new(Scale::new_12_tet()));
        let notes = Arc::new(Mutex::new(vec![Note::new(scale.clone()); 8]));
        Self {
            scale,
            notes,
            current_note_index: 0,
            previous_note_index: None,
            repeat: 0,
        }
    }

    pub fn new_test() -> Self {
        let scale = Arc::new(Mutex::new(Scale::new_12_tet()));
        let notes = Arc::new(Mutex::new(vec![
            Note{
                scale: scale.clone(),
                octave: 5,
                duration: NoteDuration{duration: 0.5},
                note_index: 0,
                velocity: 100,
                panning: 64
            },
            Note{
                scale: scale.clone(),
                octave: 5,
                duration: NoteDuration{duration: 0.5},
                note_index: 2,
                velocity: 100,
                panning: 64
            },
            Note{
                scale: scale.clone(),
                octave: 5,
                duration: NoteDuration{duration: 0.5},
                note_index: 4,
                velocity: 100,
                panning: 64
            },
            Note{
                scale: scale.clone(),
                octave: 5,
                duration: NoteDuration{duration: 0.5},
                note_index: 5,
                velocity: 100,
                panning: 64
            },
            Note{
                scale: scale.clone(),
                octave: 5,
                duration: NoteDuration{duration: 0.5},
                note_index: 7,
                velocity: 100,
                panning: 64
            },
            Note{
                scale: scale.clone(),
                octave: 5,
                duration: NoteDuration{duration: 0.5},
                note_index: 9,
                velocity: 100,
                panning: 64
            },
            Note{
                scale: scale.clone(),
                octave: 5,
                duration: NoteDuration{duration: 0.5},
                note_index: 11,
                velocity: 100,
                panning: 64
            },
            Note{
                scale: scale.clone(),
                octave: 6,
                duration: NoteDuration{duration: 0.5},
                note_index: 0,
                velocity: 100,
                panning: 64
            }
        ]));
        Self {
            scale,
            notes,
            current_note_index: 0,
            previous_note_index: None,
            repeat: 0,
        }
    }

    pub(crate) fn current_note(&self) -> Note {
        self.notes.lock().unwrap()[self.current_note_index].clone()
    }

    pub(crate) fn previous_note(&self) -> Option<Note> {
        match self.previous_note_index {
            None => None,
            Some(i) => Some(self.notes.lock().unwrap()[i].clone()),
        }
    }

    pub fn is_on_last_note(&self) -> bool {
        self.current_note_index == (self.notes.lock().unwrap().len() - 1)
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

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = self.scale.lock().map_err(|_| fmt::Error)?;
        let notes = self.notes.lock().map_err(|_| fmt::Error)?;

        let notes_str = notes
            .iter()
            .map(|note| note.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "Sequence {{ scale: {}, notes: [{}], repeat: {} }}",
            *scale,
            notes_str,
            self.repeat
        )
    }
}


pub(crate) struct Sequencer {
    sequences: Vec<Sequence>,
    current_sequence_index: usize,
    times_repeated: u8,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            sequences: vec![Sequence::new_test()],
            current_sequence_index: 0,
            times_repeated: 0,
        }
    }

    pub fn new_from_sequences(sequences: Vec<Sequence>) -> Self {
        Self {
            sequences,
            current_sequence_index: 0,
            times_repeated: 0,
        }
    }

    pub fn reset(&mut self) {
        let current_sequence = &mut self.sequences[self.current_sequence_index];
        current_sequence.reset();
        self.current_sequence_index = 0;
        self.times_repeated = 0;
    }

    pub fn next_note(&mut self) {

        // Optimization: no need to check if we need to change sequence if the sequencer has
        // only one sequence
        if self.sequences.len() == 1 {
            self.sequences[0].next();
            return;
        }

        let mut current_sequence = &mut self.sequences[self.current_sequence_index];
        if !current_sequence.is_on_last_note() {
            current_sequence.next();
            return;
        }
        self.times_repeated += 1;
        if self.times_repeated <= current_sequence.repeat {
            return;
        }
        current_sequence.reset();
        self.times_repeated = 0;
        self.current_sequence_index = (self.current_sequence_index + 1) % self.sequences.len();
    }

    pub fn current_note(&self) -> Note {
        (&self.sequences[self.current_sequence_index]).current_note()
    }

    pub fn previous_note(&self) -> Option<Note> {
        (&self.sequences[self.current_sequence_index]).previous_note()
    }
}
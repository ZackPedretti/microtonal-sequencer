use std::fmt;
use std::sync::{Arc};
use crate::note::{Note, Scale};

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
#[derive(Clone)]
pub(crate) struct Sequence {
    name: String,
    scale: Arc<Scale>,
    pub notes: Vec<Note>,
    repeat: usize,
}

impl Sequence {
    pub(crate) fn new(name: String, scale: Arc<Scale>, notes: Vec<Note>) -> Self {
        Self {
            name,
            scale,
            notes,
            repeat: 0,
        }
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let notes_str = self.notes
            .iter()
            .map(|note| note.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "Sequence {{ scale: {}, notes: [{}], repeat: {} }}",
            *self.scale,
            notes_str,
            self.repeat
        )
    }
}


pub(crate) struct Sequencer {
    sequences: Vec<Sequence>,
    current_sequence_index: usize,
    times_repeated: usize,
    pub(crate) current_note_index: usize,
    previous_note: Option<Note>
}

impl Sequencer {

    pub fn new(sequences: Vec<Sequence>) -> Self {
        Self {
            sequences,
            current_sequence_index: 0,
            times_repeated: 0,
            current_note_index: 0,
            previous_note: None
        }
    }

    pub fn reset(&mut self) {
        self.current_sequence_index = 0;
        self.current_note_index = 0;
        self.times_repeated = 0;
        self.previous_note = None;
    }

    pub fn next_note(&mut self) {

        self.previous_note = Some(self.current_note());

        // Optimization: no need to check if we need to change sequence if the sequencer has
        // only one sequence
        if self.sequences.len() == 1 {
            self.current_note_index = (self.current_note_index + 1) % self.sequences[0].notes.len();            
            return;
        }

        let current_sequence = &mut self.sequences[self.current_sequence_index];

        if self.current_note_index == current_sequence.notes.len() - 1 {
            self.times_repeated += 1;
        }
        if self.times_repeated > current_sequence.repeat {
            self.times_repeated = 0;
            self.current_sequence_index = (self.current_sequence_index + 1) % self.sequences.len();
            self.current_note_index = 0;
        }
        else {
            self.current_note_index = (self.current_note_index + 1) % current_sequence.notes.len();
        }
    }

    pub fn current_note(&self) -> Note {
        self.sequences[self.current_sequence_index].notes[self.current_note_index].clone()
    }

    pub fn previous_note(&self) -> Option<Note> {
        self.previous_note.clone()
    }
    
    pub fn current_sequence(&self) -> Sequence {
        self.sequences[self.current_sequence_index].clone()
    }
    
    pub fn current_sequence_name(&self) -> String {
        self.sequences[self.current_sequence_index].name.clone()
    }
    
    pub fn current_scale_name(&self) -> String {
        self.sequences[self.current_sequence_index].scale.name.clone()
    }

    pub fn current_sequence_index_and_repetition(&self) -> (usize, usize) {
        (self.current_sequence_index, self.times_repeated)
    }

    pub fn has_changed_sequence_or_repetition(&self, sr_tuple: (usize, usize)) -> bool {
        sr_tuple.0 != self.current_sequence_index || sr_tuple.1 != self.times_repeated
    }
}
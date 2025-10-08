use crate::note::{Note, NoteDuration, Scale};
use crate::sequencer::{Sequence, Sequencer};
use serde::de::Error;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc};

#[derive(Debug, Deserialize)]
pub struct JsonScale {
    pub name: String,
    pub steps: Vec<f64>,
    pub note_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct JsonScaleFile {
    pub scales: Vec<JsonScale>,
}

#[derive(Debug, Deserialize)]
pub struct JsonSequenceFile {
    pub sequences: Vec<JsonSequence>,
}

#[derive(Debug, Deserialize)]
pub struct JsonNote {
    pub note_index: usize,
    octave: u8,
    duration: f64,
    velocity: u8,
}

#[derive(Debug, Deserialize)]
pub struct JsonSequence {
    pub name: String,
    pub scale: String,
    pub repeat: u8,
    pub notes: Vec<JsonNote>,
}

fn read_scale_file(path: &str) -> serde_json::Result<JsonScaleFile> {
    let file = File::open(path).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}

fn build_scale_from_json_scale(json_scale: JsonScale) -> Scale {
    Scale {
        name: json_scale.name.clone(),
        steps: json_scale.steps.clone(),
        note_names: json_scale.note_names.clone(),
    }
}

pub fn get_scales_from_json_file(path: &str) -> serde_json::Result<Vec<Scale>> {
    let file = match read_scale_file(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    Ok(file
        .scales
        .into_iter()
        .map(build_scale_from_json_scale)
        .collect())
}

fn read_sequence_file(path: &str) -> serde_json::Result<JsonSequenceFile> {
    let file = File::open(path).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}

fn build_note_from_json_note(json_note: JsonNote, scale: Arc<Scale>) -> Note {
    Note {
        scale,
        octave: json_note.octave,
        duration: NoteDuration {
            duration: json_note.duration,
        },
        note_index: json_note.note_index,
        velocity: json_note.velocity,
    }
}

fn build_sequence_from_json_sequence(
    json_sequence: JsonSequence,
    scale: Arc<Scale>,
) -> serde_json::Result<Sequence> {
    let notes = json_sequence
        .notes
        .into_iter()
        .map(|n| build_note_from_json_note(n, scale.clone()))
        .collect();

    Ok(Sequence::new(json_sequence.name, scale.clone(), notes))
}

fn get_arc_scale_hashmap_from_json_sequences(
    sequences: &Vec<JsonSequence>,
    scales: Vec<Scale>,
) -> HashMap<String, Arc<Scale>> {
    let mut scales_hashmap: HashMap<String, Arc<Scale>> = HashMap::new();
    let scales: Vec<Arc<Scale>> = scales.into_iter().map(Arc::new).collect();

    for seq in sequences {
        scales_hashmap
            .entry(seq.scale.clone())
            .or_insert_with(|| Arc::clone(scales.iter().find(|s| s.name == seq.scale).unwrap()));
    }

    scales_hashmap
}

pub fn get_sequences_from_json_file(
    path: &str,
    scales: Vec<Scale>,
) -> serde_json::Result<Vec<Sequence>> {
    let file = match read_sequence_file(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let scales = get_arc_scale_hashmap_from_json_sequences(&file.sequences, scales);
    
    let mut sequences = vec![];
    for seq in file.sequences {
        let scale = scales[&seq.scale].clone();
        sequences.push(build_sequence_from_json_sequence(seq, scale)?);
    }

    Ok(sequences)
}

pub fn get_sequencer_from_json(scale_path: &str, sequence_path: &str) -> Sequencer {
    let scales = get_scales_from_json_file(scale_path).unwrap();
    let sequences = get_sequences_from_json_file(sequence_path, scales).unwrap();
    Sequencer::new(sequences)
}

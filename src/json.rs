use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde_json;
use crate::note::Scale;

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
    pub note_index: u8,
    octave: u8,
    duration: f64,
    velocity: u8,
    panning: u8,
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
    let file = match(read_scale_file(path)) {
        Ok(f) => {f}
        Err(e) => {return Err(e)}
    };
    
    Ok(file.scales
        .into_iter()
        .map(build_scale_from_json_scale)
        .collect())
}
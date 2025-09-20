use std::fs::File;
use std::io::BufReader;
use serde::de::Error;
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
pub struct ScaleFile {
    pub scales: Vec<JsonScale>,
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

fn build_scale_file_from_json_file(path: &str) -> serde_json::Result<ScaleFile> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(serde_json::Error::custom(e)),
    };
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(f) => Ok(f),
        Err(e) => Err(serde_json::Error::custom(e)),
    }
}

fn build_json_scale_from_json_file(path: &str) -> serde_json::Result<Vec<JsonScale>> {
    match build_scale_file_from_json_file(path) {
        Ok(f) => {Ok(f.scales)},
        Err(e) => {Err(serde_json::Error::custom(e))}
    }
}

pub fn get_scales_from_json_file(path: &str) -> Vec<Scale> {
    let json_scales = match build_json_scale_from_json_file(path) {
        Ok(s) => {s}
        Err(e) => {panic!("{}", e)}
    };
    json_scales.iter().map(|s| Scale::from_json_scale(s)).collect()
}
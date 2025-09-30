extern crate core;

mod sequencer;
mod clock;
mod note;
mod midi;
mod json;
mod tui;

use crate::midi::{create_input_connection, create_output_connection};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use midir::{MidiInputConnection, MidiOutputConnection};
use crate::json::{get_sequencer_from_json};
use crate::tui::run_tui;

const SCALE_PATH: &str = "data\\scales.json";
const SEQUENCE_PATH: &str = "data\\sequences.json";

fn start_sequencer(sequencer: Arc<Mutex<sequencer::Sequencer>>, on: Arc<AtomicBool>) {
    let output_conn = Arc::new(Mutex::new(create_output_connection()));
    let input_conn = create_input_connection(sequencer, output_conn);
    while on.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn stop_sequencer(on: Arc<AtomicBool>) {
    on.store(false, Ordering::SeqCst);
}

fn main() {
    let sequencer = Arc::new(Mutex::new(get_sequencer_from_json(SCALE_PATH, SEQUENCE_PATH)));
    let on = Arc::new(AtomicBool::new(true));

    run_tui(sequencer.clone(), on.clone());
}
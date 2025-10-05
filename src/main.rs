extern crate core;

mod clock;
mod json;
mod midi;
mod note;
mod sequencer;
mod tui;

use crate::json::get_sequencer_from_json;
use crate::midi::{create_input_connection, create_output_connection, stop_sequencer};
use crate::tui::run_tui;
use midir::{MidiInputConnection, MidiOutputConnection};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SCALE_PATH: &str = "data\\scales.json";
const SEQUENCE_PATH: &str = "data\\sequences.json";

fn init_sequencer(
    sequencer: Arc<Mutex<sequencer::Sequencer>>,
    on: Arc<AtomicBool>,
) -> Result<(), std::io::Error> {
    on.store(true, Ordering::SeqCst);
    let output_conn = Arc::new(Mutex::new(create_output_connection()?));
    let input_conn = create_input_connection(sequencer.clone(), output_conn.clone())?;
    std::thread::spawn(move || {
        start_main_loop(input_conn, on, output_conn.clone(), sequencer.clone());
    });
    Ok(())
}

fn start_main_loop(_input_conn: MidiInputConnection<()>, on: Arc<AtomicBool>, output_conn: Arc<Mutex<MidiOutputConnection>>, sequencer: Arc<Mutex<sequencer::Sequencer>>) {
    while on.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(50));
    }
    stop_sequencer(&mut output_conn.lock().unwrap(), sequencer.clone())
}

fn main() {
    let sequencer = Arc::new(Mutex::new(get_sequencer_from_json(
        SCALE_PATH,
        SEQUENCE_PATH,
    )));
    _ = run_tui(sequencer.clone());
}

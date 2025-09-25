extern crate core;

mod sequencer;
mod clock;
mod note;
mod midi;
mod json;

use crate::midi::{create_input_connection, create_output_connection};
use std::sync::{Arc, Mutex};
use crate::json::{get_sequencer_from_json};

fn main() {
    let sequencer = Arc::new(Mutex::new(get_sequencer_from_json("data\\scales.json", "data\\sequences.json")));
    let output_conn = Arc::new(Mutex::new(create_output_connection()));
    let input_conn = create_input_connection(sequencer, output_conn);
    loop {}
}
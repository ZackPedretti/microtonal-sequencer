extern crate core;

mod sequencer;
mod clock;
mod note;
mod midi;
mod json;

use crate::midi::{create_input_connection, create_output_connection};
use crate::sequencer::Sequencer;
use std::sync::{Arc, Mutex};
use crate::json::get_scales_from_json_file;

fn main() {
    let scales = get_scales_from_json_file("data\\scales.json");
    let _ = scales.iter().for_each(|s| println!("{}", s));
    let sequencer = Arc::new(Mutex::new(Sequencer::new()));
    let output_conn = Arc::new(Mutex::new(create_output_connection()));
    let input_conn = create_input_connection(sequencer, output_conn);
    loop {}
}
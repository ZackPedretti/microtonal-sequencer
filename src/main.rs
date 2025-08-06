mod sequencer;
mod clock;
mod note;
mod midi;

use crate::midi::{create_input_connection, create_output_connection};
use crate::sequencer::Sequencer;
use std::sync::{Arc, Mutex};

fn main() {
    let sequencer = Arc::new(Mutex::new(Sequencer::new()));
    let output_conn = Arc::new(Mutex::new(create_output_connection()));
    let _ = create_input_connection(sequencer, output_conn);
    loop {}
}
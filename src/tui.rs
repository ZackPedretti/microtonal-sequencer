use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use crate::sequencer::Sequencer;
use crate::{start_sequencer, stop_sequencer};

pub fn run_tui(sequencer: Arc<Mutex<Sequencer>>, on: Arc<AtomicBool>) {
    
}

use std::sync::{Arc, Mutex};
use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use crate::sequencer::Sequencer;
use crate::clock::Clock;
use crate::note::Note;

fn midi_input_handler(
    seq: Arc<Mutex<Sequencer>>,
    output_conn: Arc<Mutex<MidiOutputConnection>>,
) -> impl FnMut(u64, &[u8], &mut ()) + Send + 'static {
    let mut clock = Clock::new();

    move |_stamp: u64, message: &[u8], _: &mut ()| {
        match message[0] {
            0xF8 => {
                println!("Clock tick");
                let mut seq = seq.lock().unwrap();
                let current_note = seq.current_note();
                if clock.has_time_passed_note(current_note.duration.get_tick_length()) {
                    let mut conn = output_conn.lock().unwrap();

                    if let Some(note) = seq.previous_note() {
                        send_note(&mut conn, note, false);
                    }
                    send_note(&mut conn, current_note, true);
                    seq.next();
                }
                clock.next();
            },
            0xFA => {
                println!("Start");
                clock.reset_tick();
                let mut seq = seq.lock().unwrap();
                seq.reset();
            },
            0xFC => println!("Stop"),
            _ => {}
        }
    }
}

pub(crate) fn create_input_connection(
    seq: Arc<Mutex<Sequencer>>,
    output_conn: Arc<Mutex<MidiOutputConnection>>,
) -> MidiInputConnection<()> {
    let mut midi_in = MidiInput::new("Rust MIDI Input").unwrap();
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let port = in_ports
        .iter()
        .find(|p| midi_in.port_name(p).unwrap().contains("SequencerInput"))
        .expect("loopMIDI port not found");

    let handler = midi_input_handler(seq, output_conn);
    midi_in.connect(port, "midir-read-input", handler, ()).unwrap()
}


fn send_note(conn: &mut MidiOutputConnection, note: Note, on: bool) {
    let status = if on { 0x90 } else { 0x80 }; // note_on / note_off
    let note_pitch = note.get_midi_number();
    let macro_pitch = note_pitch.round() as u8;
    let msg = [status, macro_pitch, 100];
    conn.send(&msg).unwrap();
}

pub(crate) fn create_output_connection() -> MidiOutputConnection {
    let midi_out = MidiOutput::new("Rust Sequencer").unwrap();

    let out_ports = midi_out.ports();
    let port = out_ports
        .iter()
        .find(|p| midi_out.port_name(p).unwrap().contains("SequencerOutput"))
        .expect("loopMIDI port not found");

    midi_out.connect(port, "RustSeq").unwrap()
}
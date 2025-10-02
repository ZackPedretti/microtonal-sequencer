use std::io;
use crate::clock::Clock;
use crate::note::Note;
use crate::sequencer::Sequencer;
use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::sync::{Arc, Mutex};

fn midi_input_handler(
    seq: Arc<Mutex<Sequencer>>,
    output_conn: Arc<Mutex<MidiOutputConnection>>,
) -> impl FnMut(u64, &[u8], &mut ()) + Send + 'static {
    let mut clock = Clock::new();

    move |_stamp: u64, message: &[u8], _: &mut ()| {
        match message[0] {
            0xF8 => {
                let mut seq = seq.lock().unwrap();
                let current_note = seq.current_note();
                if clock.has_time_passed_note(current_note.duration.get_tick_length()) {
                    let mut conn = output_conn.lock().unwrap();

                    if let Some(note) = seq.previous_note() {
                        send_note(&mut conn, note, false);
                    }
                    send_note(&mut conn, current_note, true);
                    seq.next_note();
                }
                clock.next();
            },
            0xFA => {
                clock.reset_tick();
                let mut seq = seq.lock().unwrap();
                seq.reset();
            },
            _ => {}
        }
    }
}

pub(crate) fn create_input_connection(
    seq: Arc<Mutex<Sequencer>>,
    output_conn: Arc<Mutex<MidiOutputConnection>>,
) -> Result<MidiInputConnection<()>, io::Error> {
    let mut midi_in = MidiInput::new("Rust MIDI Input").unwrap();
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let input_port_name = "SequencerInput";
    let port = match in_ports
        .iter()
        .find(|p| midi_in.port_name(p).unwrap().contains(input_port_name)) {
        Some(p) => p,
        None => { return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No input port named '{}' found. Make sure to have a MIDI input port named '{}' using loopMIDI or other tools.", input_port_name, input_port_name),
        ))}
    };

    let handler = midi_input_handler(seq, output_conn);
    Ok(midi_in.connect(port, "midir-read-input", handler, ()).unwrap())
}

fn send_pitch_bend(conn: &mut MidiOutputConnection, bend: i16, channel: u8) {
    let value = (bend + 8192).clamp(0, 16383) as u16;
    let lsb = (value & 0x7F) as u8;       // lower 7 bits
    let msb = ((value >> 7) & 0x7F) as u8; // upper 7 bits

    let status = 0xE0 | (channel & 0x0F);
    let message = [status, lsb, msb];
    conn.send(&message).unwrap();
}

fn pitch_bend_calculation(shift: f64, range: i16) -> i16 {
    ((shift / range as f64) * (8192f64)) as i16
}

fn send_note(conn: &mut MidiOutputConnection, note: Note, on: bool) {
    let channel = 0;
    let status = if on { 0x90 | channel } else { 0x80 | channel };
    let note_pitch = note.get_midi_number();
    let note_number = note_pitch.floor() as u8;
    let velocity = 100;

    let shift = note_pitch - note_pitch.floor();
    if on {
        let bend = pitch_bend_calculation(shift, 2);
        send_pitch_bend(conn, bend, channel);
    }

    let msg = [status, note_number, velocity];
    conn.send(&msg).unwrap();
}
pub(crate) fn create_output_connection() -> Result<MidiOutputConnection, io::Error> {
    let midi_out = MidiOutput::new("Rust Sequencer").unwrap();

    let out_ports = midi_out.ports();
    let output_port_name = "SequencerOutput";
    let port = match out_ports
        .iter()
        .find(|p| midi_out.port_name(p).unwrap().contains(output_port_name)) {
        Some(p) => p,
        None => { return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No input port named '{}' found. Make sure to have a MIDI input port named '{}' using loopMIDI or other tools.", output_port_name, output_port_name),
        ))}
    };

    Ok(midi_out.connect(port, "RustSeq").unwrap())
}
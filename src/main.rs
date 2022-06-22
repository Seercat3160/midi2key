// Code from github.com/seercat3160/midi2key, under the MIT license
// Adapted from https://github.com/Boddlnagg/midir/blob/e5980808e60570639ff4bb20bc983b08da9193c6/examples/test_read_input.rs, also under the MIT license

use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};
use midly::{live::LiveEvent, MidiMessage};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_, message, _| {
            on_midi(message);
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

fn on_midi(event: &[u8]) {
    let event = LiveEvent::parse(event).unwrap();
    match event {
        LiveEvent::Midi {
            channel: _,
            message,
        } => match message {
            MidiMessage::NoteOn { key, vel } => {
                note_start(key.as_int(), vel.as_int());
            }
            MidiMessage::NoteOff { key, vel: _ } => {
                note_end(key.as_int());
            }
            _ => {}
        },
        _ => {}
    }
}

fn note_start(key: u8, vel: u8) {
    println!("hit note {} with vel {}", key, vel);
}

fn note_end(key: u8) {
    println!("released note {}", key);
}

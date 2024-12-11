//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

extern crate midir;
use std::process::Command;
use crate::types::*;
use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, MidiOutput, MidiOutputPort, Ignore};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::fs::File;
use std::sync::Arc;    

#[derive(Copy, Clone, Debug)]
pub struct Events;

impl Events {


    pub fn read_midi(f: impl FnMut(u8, u8) + Send + Sync + 'static) {
        match Events::midi_in(f) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err)
        }
    }

    fn midi_in(mut f: impl FnMut(u8, u8) + Send + Sync + 'static) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        
        // Midi Input
        let mut midi_in = MidiInput::new("AudioTheorem_In")?;
        let midi_out = MidiOutput::new("AudioTheorem_Out")?;
        midi_in.ignore(Ignore::None);

        // Synthesizer
        let mut free_pats = File::open("/usr/share/soundfonts/freepats-general-midi.sf2").unwrap();
        let sound_font = Arc::new(SoundFont::new(&mut free_pats).unwrap());
        let settings = SynthesizerSettings::new(44100);
        let mut synthesizer: Synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
        let sample_count = (3 * settings.sample_rate) as usize;
        
        let in_ports = midi_in.ports();
        let out_ports = midi_out.ports();
        let in_port = match in_ports.len() {
            _ => {
                //println!("\x1B[2J\x1B[1;1H");
                println!("=====================");
                println!("!!! Audio Theorem !!!");
                println!("=====================\n");
                println!("\nAvailable Input Devices:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("\t{}: {}", i, midi_in.port_name(p).unwrap());
                }
                print!("\nSelect an Input Device: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                in_ports.get(input.trim().parse::<usize>()?)
                        .ok_or("invalid input port selected")?
            }
        };

        print!("\x1B[2J\x1B[1;1H");
        println!("=====================");
        println!("!!! Audio Theorem !!!");
        println!("=====================\n");

        let out_port: &MidiOutputPort = match out_ports.len() {
            _ => {
                println!("\nAvailable Output Devices:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("\t{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("\nSelect an Output Device: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                out_ports.get(input.trim().parse::<usize>()?)
                         .ok_or("invalid output port selected")?
            }
        };
        
        print!("\x1B[2J\x1B[1;1H");
        println!("!!! Audio Theorem !!!");
        println!("=====================\n");
        println!("Connected to Input: {}.\nSending to Output: {}.\n", midi_in.port_name(in_port)?, midi_out.port_name(out_port)?);
        println!("Press [enter] to Exit.\n");

        // let mut conn_out = midi_out.connect(out_port, "audiotheorem")?;
        let a_ = midi_in.connect(in_port, "readin", move |stamp, message, _| { 
            let velocity: u8 = message[2];
            let index: u8 = message[1];

            // Main Audio Processing Loop
                // process audio as Sequence<Tone>
                f(index, velocity); 

            

            // play audio
            // if velocity > 0 {
            //     let _ = conn_out.send(&[0x90, index, velocity]);
            // } else {
            //     let _ = conn_out.send(&[0x80, index, velocity]);
            // }

            // Audio Synthesizer
            // synthesizer.note_on(0, index.into(), velocity.into());

            // let mut left = vec![0.0; sample_count];
            // let mut right = vec![0.0; sample_count];

            // synthesizer.render(&mut left[..], &mut right[..]);

        }, ())?;
        
        input.clear();
        stdin().read_line(&mut input)?; // waiting for exit 

        Ok(())
    }
}
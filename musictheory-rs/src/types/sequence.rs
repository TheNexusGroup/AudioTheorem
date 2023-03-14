//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::scale::Scale;
use crate::types::tone::Tone;

#[derive(Clone, Debug)]
pub struct Sequence {
    size: u8,
    tones: Vec<Tone>,

}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence { size: 0, tones: Vec::new() }
    }

    fn add_tone(&mut self, index: u8) {
        self.size += 1;
        self.tones.push(Tone::from_index(index));
        self.tones.sort_by_key(|t| t.to_index());
    }

    fn delete_tone(&mut self, index: u8) {
        self.tones.retain(|&t| t.to_index() != index);
        self.size = self.tones.len() as u8;
    }

    pub fn process_input(&mut self, index: u8, velocity: u8) {
        if velocity > 0 {
            self.add_tone(index);
        } else {
            self.delete_tone(index);
        }
        self.print_state();
    }

    pub fn print_state(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("!!! Audio Theorem !!!");
        println!("=====================\n");
        println!("{:#?}", *self);
    }
}
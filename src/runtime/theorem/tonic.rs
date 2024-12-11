// ALL OF THESE LINES ARE MINE BIOTCH - 2024, Ancillary, Inc.
use std::collections::HashSet;

use super::{chord::Chord, Subsequence};
use crate::types::{Interval, Matrix, Note, Octave, Pitch, PitchClass, Scale, Tone};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Tonic {
    pub note: Option<Note>,
    pub tone: Option<Tone>,        // This can give us pitch, and we could use frequency or cents
    pub index: u8,         // 0-143 - This needs to be incorporated with the wavetable
    pub velocity: u8,      // We will end up having to normalize this to a float 0.0 - 1.0
    pub harmony: u8,       // This is the type of note instance - 0 = Played, 1 = Harmonious, ?upper_bounds? = Nonce
    // This is likely where we will add a [f32; 10] for the 100 cents of a given pitchclass if we implement that level of resolution
}

impl Tonic {
    // Gives us the standard C Natural Notes (It should have part Sharp and part Flat if it were correct).. but music theory is flawed.. so we have to make it up as we go along
    pub fn new(index: u8, velocity: u8, harmony: u8) -> Tonic {
        let tone = Tone::from_iv(index, velocity);
        let note = tone.note(); 

        Tonic { 
            note: Some(note), 
            tone: Some(tone), 
            index, 
            velocity, 
            harmony 
        }
    }

    pub fn octave(&self) -> Option<Octave> { Some(self.tone?.octave()) }
    pub fn pitch_class(&self) -> Option<PitchClass> { Some(self.tone?.pitch_class()) }
    pub fn note(&self) -> Option<Note> { if self.note.is_some() { self.note } else if self.tone.is_some() { Some(self.tone?.note()) } else { None } }
    pub fn velocity(&self) -> u8 { self.velocity }
    pub fn pitch(&self) -> Option<Pitch> { Some(self.tone?.pitch()) }
}

impl IntoIterator for Tonic {
    type Item = Tonic;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

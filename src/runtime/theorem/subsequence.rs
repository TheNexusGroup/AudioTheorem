// Disposition is a Music Theory concept that is used to determine the color of the note (later might add a fractional degree or mixing of colors as a vector) << -- we are incorporating this through pitchgroups and scales
// The intent and purpose of this system, alongside the sequence Data, is to interface the pitchgroups and scales with the audio and graphics systems to help correlate
// the color of the notes with the sound and the key of the music. This is a proprietary system that is being developed for the Ancillary via Big Stick Studios, and is not to be used without permission.

// Copyright (c) 2024 by Richard I Christopher, Big Stick Studio - All Rights Reserved, Proprietary License - The NEXUS Project

/*  Proposed Questions 
    - can we use audio structure as an indicator for an AI system to learn to analyze speech patterns, emotion, phonetics, and intent?
    - Is there a correlation to audio and emotion?
    - Can we prove that life comes from Oxygon and Hydrogen when met with Carbon and Nitrogen?
    - Will I make my first million before I'm 35?
*/

use std::collections::HashSet;
use super::{Chord, PitchGroupKernel, Tonic};
use crate::types::{Tone, Interval, Scale, Note};

pub struct Subsequence {
    pub tones: HashSet<Tonic>,          // These are initially the tones being played, and we add the tones from the pitchgroupkernel across the entire bounds
    pub speculative: HashSet<Tonic>,    // These are the tones that are being inferred from the pitchgroupkernel and analysis of the tones - This needs to be moved up to the entire sequence level
    pub chords: HashSet<Chord>,             // We need to split this further into n_inversions and n_shapes
    pub scales: HashSet<Scale>,             // We can use these to determine "gravity"
    pub kernel: PitchGroupKernel, 
    pub upper_bound: u8,                // This is the upper bound of the dynamic range for a set of keys + 7 but we may need to make this part of a filter (proprietary NEXUS)
    pub lower_bound: u8,                // This is the lower bound of the dynamic range for a set of keys - 7
}

impl Subsequence {
    pub fn new() -> Subsequence { 
        Subsequence { 
            tones: HashSet::new(), 
            speculative: HashSet::new(),
            chords: HashSet::new(), 
            scales: HashSet::new(), 
            kernel: PitchGroupKernel::new(HashSet::new()),
            upper_bound: 144,       // These need to be swapped for a filter type
            lower_bound: 0
        } 
    }

    // We need logic for the following:
    // add note
    // remove note
    // update kernel

    pub fn upper_bound(&self) -> u8 { self.tones.iter().map(|t| t.index).max().unwrap_or(144)}
    pub fn lower_bound(&self) -> u8 { self.tones.iter().map(|t| t.index).min().unwrap_or(0)}

    // We calculate +7 and -7 from the current upper and lower bounds of the tones or 
    // max of 143 and min of 0
    pub fn calculate_bounds(&mut self) 
        {
            if self.tones.is_empty() {
                self.upper_bound = 144;
                self.lower_bound = 0;
                return;
            }
            
            let current_upper = self.upper_bound();
            let current_lower = self.lower_bound();
            
            self.upper_bound = current_upper.saturating_add(7).clamp(0, 144);
            self.lower_bound = current_lower.saturating_sub(7).clamp(0, 144);
        }

    pub fn within_bounds(&self, index: u8) -> bool 
        {
            self.upper_bound >= index && self.lower_bound <= index
        }

    // Returns the gap between the upper and lower bounds (should be less than 28 for a scale and 12 (or 24) for a chord) - and smaller for tetrachordal voicings
    pub fn limits(&self) -> u8 { self.upper_bound() - self.lower_bound() }

    // This only works for notes that we actually play.. this does not work for speculative notes (which we calculate from the kernel)
    pub fn play_note(&mut self, index: u8, velocity: u8)
        {
            // We need to check if the note is within the bounds of the sequence
            if self.tones.len() == 0 
                { 
                    self.tones.insert(Tonic::new(index, velocity, 0));
                    self.calculate_bounds();
                }

            // This is the actual 'play' logic
            self.calculate_bounds();
            self.tones.insert(Tonic::new(index, velocity, 0));

            // This is the speculative - theoretical logic
            self.sync();
            // We need to update the kernel, but unfortunately we don't have a good update method to preserve caching or space-time complexity
        }

    fn cloned(&self) -> Subsequence
        {
            Subsequence { 
                tones: self.tones.clone(), 
                speculative: self.speculative.clone(),
                chords: self.chords.clone(), 
                scales: self.scales.clone(), 
                kernel: self.kernel.clone(),
                upper_bound: self.upper_bound,
                lower_bound: self.lower_bound
            }
        }

    pub fn sync(&mut self)
        {
            // We could probably improve this by updating the kernel with the new tones
            self.kernel = PitchGroupKernel::new(self.tones.clone());
            let speculative = self.kernel.normalize(self.tones.clone());

            // We go ahead and exit if this sync was a 'reset'
            if self.tones.len() == 0
                { 
                    self.lower_bound = 0;
                    self.upper_bound = 144;
                    self.speculative = speculative;
                    return; 
                }

            // We need to iterate from a lower bounds to an upper bounds and add the speculative tones
            self.calculate_bounds();
            let mut prev_vel = self.tones.iter().map(|t| t.velocity).sum::<u8>() / self.tones.len() as u8;

            for i in self.lower_bound..self.upper_bound
                {
                    // If we find a speculative tone that is in the bounds
                    let mut t_tonic = speculative.iter().find(|t| t.index == i % 12).unwrap().clone();
                            
                    // We need to make sure it's not one of the tones we're already playing (at the correct index)
                    if !self.tones.iter().any(|t| t.index == i)
                        { 
                            t_tonic.index = i;
                            t_tonic.velocity = (prev_vel + t_tonic.velocity) / 2;
                            self.speculative.insert(t_tonic.clone()); 
                            continue;
                        }

                    let p_tonic = self.tones.iter().find(|t| t.index == i).unwrap();
                    prev_vel = (prev_vel + p_tonic.velocity) / 2;
                    self.speculative.insert(p_tonic.clone()); 
                }
        }

    pub fn calculate_harmonies(&mut self) 
        {
            
        }
}
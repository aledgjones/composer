pub mod barline;
pub mod clef;
pub mod key_signature;
pub mod time_signature;
pub mod tone;

use clef::Clef;
use key_signature::KeySignature;
use serde::{Deserialize, Serialize};
use time_signature::TimeSignature;
use tone::Tone;

use crate::components::misc::Tick;

use self::barline::Barline;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entry {
    Barline(Barline),
    Clef(Clef),
    KeySignature(KeySignature),
    TimeSignature(TimeSignature),
    Tone(Tone),
}

impl Entry {
    /// Get the entries key without having to manually unwrap the entry
    pub fn key(&self) -> String {
        match self {
            Entry::Barline(barline) => barline.key.clone(),
            Entry::Clef(clef) => clef.key.clone(),
            Entry::KeySignature(key_signature) => key_signature.key.clone(),
            Entry::TimeSignature(time_signature) => time_signature.key.clone(),
            Entry::Tone(tone) => tone.key.clone(),
        }
    }

    /// Get the entries tick without having to manually unwrap the entry
    pub fn tick(&self) -> Tick {
        match self {
            Entry::Barline(barline) => barline.tick,
            Entry::Clef(clef) => clef.tick,
            Entry::KeySignature(key_signature) => key_signature.tick,
            Entry::TimeSignature(time_signature) => time_signature.tick,
            Entry::Tone(tone) => tone.tick,
        }
    }

    /// Updates the entries tick value
    ///
    /// This shouldn't be called directly. It will most likely be called via the
    /// Track interface.
    pub fn set_tick(&mut self, tick: Tick) {
        match self {
            Entry::Barline(barline) => barline.tick = tick,
            Entry::Clef(clef) => clef.tick = tick,
            Entry::KeySignature(key_signature) => key_signature.tick = tick,
            Entry::TimeSignature(time_signature) => time_signature.tick = tick,
            Entry::Tone(tone) => tone.tick = tick,
        }
    }
}

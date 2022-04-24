use crate::components::articulation::Articulation;
use crate::components::duration::Duration;
use crate::components::pitch::{Accidental, Pitch};
use crate::components::velocity::Velocity;
use crate::entries::Entry;
use crate::utils::shortid;
use crate::Engine;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// These represent the audiable tones of the music.
/// They are never directly drawn in the score.
#[derive(Debug, Clone, Serialize)]
pub struct Tone {
    pub key: String,
    pub tick: u32,
    pub duration: Duration,
    pub pitch: Pitch,
    pub velocity: Velocity,
    pub articulation: Articulation,
}

impl Tone {
    pub fn new(
        key: String,
        tick: u32,
        duration: Duration,
        pitch: Pitch,
        velocity: Velocity,
        articulation: Articulation,
    ) -> Self {
        Self {
            key,
            tick,
            duration,
            pitch,
            velocity,
            articulation,
        }
    }
}

#[wasm_bindgen]
impl Engine {
    /// Create a tone
    pub fn create_tone(
        &mut self,
        track_key: &str,
        tick: u32,
        duration: u32,
        pitch: u8,
        velocity: u8,
        articulation: Articulation,
    ) -> String {
        // we want to be able to return this at the end
        let key = shortid();
        let track = self.score.tracks.get_mut(track_key).unwrap();

        // we are now done with the entry, insert it back in
        track.insert(Entry::Tone(Tone::new(
            key.clone(),
            tick,
            Duration::new(duration),
            Pitch::new(pitch, Accidental::default(pitch)),
            Velocity::new(velocity),
            articulation,
        )));

        self.emit();

        key
    }

    /// update tone pitch
    pub fn set_tone_pitch(&mut self, track_key: &str, entry_key: &str, pitch: u8) {
        let track = self.score.tracks.get_mut(track_key).unwrap();
        let entry = track.entries.by_key.get_mut(entry_key).unwrap();
        if let Entry::Tone(tone) = entry {
            tone.pitch = Pitch::new(pitch, Accidental::default(pitch));
        }

        self.emit();
    }

    /// update tone duration
    pub fn set_tone_duration(&mut self, track_key: &str, entry_key: &str, duration: u32) {
        let track = self.score.tracks.get_mut(track_key).unwrap();
        let entry = track.entries.by_key.get_mut(entry_key).unwrap();
        if let Entry::Tone(tone) = entry {
            tone.duration = Duration::new(duration);
        }

        self.emit();
    }

    /// move the tone
    pub fn shift_tone(&mut self, track_key: &str, entry_key: &str, new_tick: u32) {
        let track = self.score.tracks.get_mut(track_key).unwrap();
        track.shift(entry_key, new_tick);
        self.emit();
    }

    /// Remove the tone
    pub fn remove_tone(&mut self, track_key: &str, entry_key: &str) {
        let track = self.score.tracks.get_mut(track_key).unwrap();
        track.remove(entry_key);
        self.emit();
    }

    /// Slice a tone
    pub fn slice_tone(&mut self, track_key: &str, entry_key: &str, slice_at: u32) {
        let track = self.score.tracks.get_mut(track_key).unwrap();

        let entry = track.entries.by_key.get_mut(entry_key).unwrap();

        if let Entry::Tone(tone) = entry {
            let diff = tone.duration.int - (slice_at - tone.tick);
            let pitch = tone.pitch.clone();
            let velocity = tone.velocity.clone();
            let articulation = tone.articulation.clone();
            tone.duration.int = slice_at - tone.tick;
            track.insert(Entry::Tone(Tone::new(
                shortid(),
                slice_at,
                Duration::new(diff),
                pitch,
                velocity,
                articulation,
            )));
        }

        self.emit();
    }

    pub fn get_tones(&self, track_key: &str) -> JsValue {
        let track = self.score.tracks.get(track_key).unwrap();
        let mut output: Vec<&Tone> = Vec::new();
        for entry in track.entries.by_key.values() {
            if let Entry::Tone(tone) = entry {
                output.push(tone);
            }
        }
        JsValue::from_serde(&output).unwrap()
    }
}

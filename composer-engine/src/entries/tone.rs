use crate::components::articulation::Articulation;
use crate::components::misc::{Tick, Ticks};
use crate::components::pitch::{Accidental, Pitch};
use crate::components::velocity::Velocity;
use crate::entries::Entry;
use crate::score::tracks::Track;
use crate::utils::shortid;
use crate::Engine;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// These represent the audiable tones of the music.
/// They are never directly drawn in the score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tone {
    pub key: String,
    pub tick: Tick,
    pub duration: Ticks,
    pub pitch: Pitch,
    pub velocity: Velocity,
    pub articulation: Articulation,
}

impl Tone {
    pub fn new(
        key: String,
        tick: Tick,
        duration: Ticks,
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

    pub fn tester(key: &str) -> Self {
        Self::new(
            key.to_string(),
            0,
            0,
            Pitch::from_int(60),
            Velocity::new(100),
            Articulation::None,
        )
    }
}

#[wasm_bindgen]
impl Engine {
    /// Create a tone
    #[allow(clippy::too_many_arguments)]
    pub fn create_tone(
        &mut self,
        track_key: &str,
        tick: Tick,
        duration: Ticks,
        pitch: u8,
        accidental: Option<Accidental>,
        velocity: u8,
        articulation: Articulation,
    ) -> String {
        // we want to be able to return this at the end
        let key = shortid();
        let track = self.score.tracks.get_mut(track_key).unwrap();

        let pitch = match accidental {
            Some(accidental) => Pitch::new(pitch, accidental),
            None => Pitch::from_int(pitch),
        };

        // we are now done with the entry, insert it back in
        track.insert(Entry::Tone(Tone::new(
            key.clone(),
            tick,
            duration,
            pitch,
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
            tone.pitch = Pitch::from_int(pitch);
        }

        self.emit();
    }

    /// update tone duration
    pub fn set_tone_duration(&mut self, track_key: &str, entry_key: &str, duration: Ticks) {
        let track = self.score.tracks.get_mut(track_key).unwrap();
        let entry = track.entries.by_key.get_mut(entry_key).unwrap();
        if let Entry::Tone(tone) = entry {
            tone.duration = duration;
        }

        self.emit();
    }

    /// move the tone
    pub fn shift_tone(&mut self, track_key: &str, entry_key: &str, new_tick: Tick) {
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
    pub fn slice_tone(&mut self, track_key: &str, entry_key: &str, slice_at: Tick) {
        let track = self.score.tracks.get_mut(track_key).unwrap();

        let entry = track.entries.by_key.get_mut(entry_key).unwrap();

        if let Entry::Tone(tone) = entry {
            let diff = tone.duration - (slice_at - tone.tick);
            let pitch = tone.pitch.clone();
            let velocity = tone.velocity.clone();
            let articulation = tone.articulation.clone();
            tone.duration = slice_at - tone.tick;
            track.insert(Entry::Tone(Tone::new(
                shortid(),
                slice_at,
                diff,
                pitch,
                velocity,
                articulation,
            )));
        }

        self.emit();
    }

    pub fn get_tones(&self, track_key: &str) -> JsValue {
        let mut output: Vec<&Tone> = Vec::new();

        let track = self.score.tracks.get(track_key).unwrap();
        for entry in track.entries.by_key.values() {
            if let Entry::Tone(tone) = entry {
                output.push(tone);
            }
        }

        serde_wasm_bindgen::to_value(&output).unwrap()
    }

    pub fn get_all_tones(&self, flow_key: &str, instrument_key: &str) -> JsValue {
        let mut output: Vec<&Tone> = Vec::new();

        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        let instruemnt = self.score.instruments.get(instrument_key).unwrap();
        for stave_key in &instruemnt.staves {
            let stave = flow.staves.get(stave_key).unwrap();
            for track_key in &stave.tracks {
                let track = self.score.tracks.get(track_key).unwrap();
                for entry in track.entries.by_key.values() {
                    if let Entry::Tone(tone) = entry {
                        output.push(tone);
                    }
                }
            }
        }

        output.sort_unstable_by_key(|a| a.tick);

        serde_wasm_bindgen::to_value(&output).unwrap()
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_tones_at_tick(&self, tick: &Tick) -> Vec<&Tone> {
        let mut output = Vec::new();

        let entry_keys = match self.entries.by_tick.get(tick) {
            Some(entries) => entries,
            None => return output,
        };

        for key in entry_keys {
            if let Some(Entry::Tone(tone)) = self.entries.by_key.get(key) {
                output.push(tone);
            }
        }

        output
    }
}

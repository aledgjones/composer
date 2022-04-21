use crate::components::duration::NoteDuration;
use crate::components::measurements::{BoundingBox, PaddingSpaces};
use crate::entries::Entry;
use crate::score::tracks::Track;
use crate::utils::shortid;
use crate::Engine;
use wasm_bindgen::prelude::*;

enum TimeSignatureType {
    Simple,
    Compound,
    Complex,
    Open,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum TimeSignatureDrawType {
    Hidden,          // always hidden
    Normal,          // open time sig as 'X'
    CommonTime,      // 'C'
    SplitCommonTime, // '¢'
}

#[derive(Debug, Clone)]
pub struct TimeSignature {
    pub key: String,
    pub tick: u32,
    pub beats: u8,
    pub beat_type: NoteDuration,
    pub draw_type: TimeSignatureDrawType,
    pub groupings: Vec<u8>,
}

impl TimeSignature {
    pub fn new(
        tick: u32,
        beats: u8,
        beat_type: NoteDuration,
        draw_type: TimeSignatureDrawType,
        groupings: Option<Vec<u8>>,
    ) -> Self {
        Self {
            key: shortid(),
            tick,
            beats,
            beat_type,
            groupings: match groupings {
                Some(groupings) => groupings,
                None => TimeSignature::groupings(beats),
            },
            draw_type,
        }
    }

    /// Return the time signature type Open, Compound, Simple or Complex.
    fn kind(&self) -> TimeSignatureType {
        TimeSignature::kind_from_beats(self.beats)
    }

    /// Return the time signature type Open, Compound, Simple or Complex.
    fn kind_from_beats(beats: u8) -> TimeSignatureType {
        if beats == 0 {
            TimeSignatureType::Open
        } else if beats > 3 && beats % 3 == 0 {
            TimeSignatureType::Compound
        } else if beats == 1 || beats == 2 || beats == 3 || beats == 4 {
            TimeSignatureType::Simple
        } else {
            TimeSignatureType::Complex
        }
    }

    fn groupings(beats: u8) -> Vec<u8> {
        if beats > 0 && beats <= 3 {
            vec![1; beats as usize]
        } else {
            match TimeSignature::kind_from_beats(beats) {
                TimeSignatureType::Simple => vec![2; (beats as usize) / 2],
                TimeSignatureType::Compound => vec![3; (beats as usize) / 3],
                TimeSignatureType::Complex => {
                    let mut out: Vec<u8> = Vec::new();
                    let mut remaining = beats;
                    while remaining > 4 {
                        out.push(3);
                        remaining = remaining - 3;
                    }
                    out.push(remaining);
                    out
                }
                TimeSignatureType::Open => vec![],
            }
        }
    }

    /// Get the number of ticks per the time signatures bar
    pub fn ticks_per_bar(&self, subdivisions: u8) -> u32 {
        (self.ticks_per_beat(subdivisions) * self.beats) as u32
    }

    /// Get the number of ticks per the time signatures beat type
    pub fn ticks_per_beat(&self, subdivisions: u8) -> u8 {
        self.beat_type.to_ticks(subdivisions)
    }

    pub fn distance_from_barline(&self, tick: u32, subdivisions: u8) -> u32 {
        match self.kind() {
            TimeSignatureType::Open => tick - self.tick,
            _ => (tick - self.tick) % self.ticks_per_bar(subdivisions),
        }
    }

    // Returns true if the tick is on a beat
    pub fn is_on_beat(&self, tick: u32, subdivisions: u8) -> bool {
        self.is_on_beat_type(tick, subdivisions, &self.beat_type)
    }

    /// Return true if a tick is on an arbitrary beat type
    pub fn is_on_beat_type(&self, tick: u32, subdivisions: u8, beat_type: &NoteDuration) -> bool {
        let ticks_per_beat = beat_type.to_ticks(subdivisions) as u32;
        ((tick - self.tick) % ticks_per_beat) == 0
    }

    // Returns true is the tick is on a beat group boundry
    pub fn is_on_grouping_boundry(&self, tick: u32, subdivisions: u8) -> bool {
        match self.kind() {
            TimeSignatureType::Open => false,
            _ => {
                let ticks_per_beat = self.ticks_per_beat(subdivisions);
                let bar_length = self.ticks_per_bar(subdivisions);
                let distance_from_first_beat = (tick - self.tick) % bar_length;

                if distance_from_first_beat == 0 {
                    return true;
                }

                let mut offset: u32 = 0;
                for group in &self.groupings {
                    offset += (group * ticks_per_beat) as u32;
                    if distance_from_first_beat == offset {
                        return true;
                    }
                }

                false
            }
        }
    }

    pub fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: 0.75,
            height: 4.0,
            padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_time_signature(
        &mut self,
        flow_key: &str,
        tick: u32,
        beats: u8,
        beat_type: NoteDuration,
        draw_type: TimeSignatureDrawType,
        groupings: Option<Vec<u8>>,
    ) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        let master = self.score.tracks.get_mut(&flow.master).unwrap();

        // remove old time signative if defined
        if let Some(old) = master.get_time_signature_at_tick(tick) {
            master.remove(&old.key);
        };

        // insert the new time signature
        let new = TimeSignature::new(tick, beats, beat_type, draw_type, groupings);
        let ticks_per_bar = new.ticks_per_bar(flow.subdivisions) as f32;
        let entry = Entry::TimeSignature(new);
        master.insert(entry);

        // calculate diff
        let next_tick = match master.get_time_signature_after_tick(tick, flow.length) {
            Some(entry) => entry.tick as f32,
            None => flow.length as f32,
        };

        let complete_bars_until_next = ((next_tick - tick as f32) / ticks_per_bar).ceil();
        let overflow = (tick as f32 + complete_bars_until_next * ticks_per_bar - next_tick) as u32;

        if overflow > 0 {
            flow.length += overflow;

            for _ in [tick..flow.length] {
                if let Some(old) = master.get_time_signature_at_tick(tick) {
                    master.shift(&old.key, old.tick + overflow);
                };
            }
        }
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_time_signature_at_tick(&self, tick: u32) -> Option<TimeSignature> {
        let entry_keys = match self.entries.by_tick.get(&tick) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys.iter() {
            if let Some(Entry::TimeSignature(time_signature)) = self.entries.by_key.get(key) {
                return Some(time_signature.clone());
            }
        }

        None
    }

    /// Returns the next time signature entry *after* a given tick if it exists
    pub fn get_time_signature_after_tick(&self, tick: u32, length: u32) -> Option<TimeSignature> {
        for i in tick + 1..length {
            if let Some(time_signature) = self.get_time_signature_at_tick(i) {
                return Some(time_signature);
            };
        }

        None
    }
}

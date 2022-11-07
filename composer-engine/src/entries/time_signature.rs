use crate::components::duration::NoteDuration;
use crate::components::measurements::{BoundingBox, PaddingSpaces};
use crate::components::misc::{Tick, Ticks};
use crate::entries::Entry;
use crate::score::tracks::Track;
use crate::utils::shortid;
use crate::Engine;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub enum TimeSignatureType {
    Simple,
    Compound,
    Complex,
    Open,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeSignatureDrawType {
    Hidden,          // always hidden
    Regular,         // 4/4 etc
    CommonTime,      // C
    SplitCommonTime, // ¢
    Open,            // X
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSignature {
    pub key: String,
    pub tick: Tick,
    pub beats: u8,
    pub beat_type: NoteDuration,
    pub draw_type: TimeSignatureDrawType,
    pub groupings: Vec<u8>,
}

impl TimeSignature {
    pub fn new(
        tick: Tick,
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
                None => TimeSignature::default_groupings(beats),
            },
            draw_type,
        }
    }

    /// create a default open time signature
    pub fn default() -> Self {
        Self::new(
            0,
            0,
            NoteDuration::Quarter,
            TimeSignatureDrawType::Hidden,
            None,
        )
    }

    /// Return the time signature type Open, Compound, Simple or Complex.
    pub fn kind(&self) -> TimeSignatureType {
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

    fn default_groupings(beats: u8) -> Vec<u8> {
        if beats > 0 && beats <= 3 {
            vec![beats]
        } else {
            match TimeSignature::kind_from_beats(beats) {
                TimeSignatureType::Simple => vec![2; (beats as usize) / 2],
                TimeSignatureType::Compound => vec![3; (beats as usize) / 3],
                TimeSignatureType::Complex => {
                    let mut out: Vec<u8> = Vec::new();
                    let mut remaining = beats;
                    while remaining > 4 {
                        out.push(3);
                        remaining -= 3;
                    }
                    out.push(remaining);
                    out
                }
                TimeSignatureType::Open => vec![2, 2],
            }
        }
    }

    /// Get the number of ticks per the time signatures bar
    pub fn ticks_per_bar(&self, subdivisions: Ticks) -> Ticks {
        self.ticks_per_beat(subdivisions) * self.beats as u32
    }

    /// Get the number of ticks per the time signatures beat type
    pub fn ticks_per_beat(&self, subdivisions: Ticks) -> Ticks {
        self.beat_type.to_ticks(subdivisions)
    }

    /// Returns how far away the tick is from the nearest barline
    pub fn distance_from_barline(&self, tick: Tick, subdivisions: Ticks) -> Ticks {
        match self.kind() {
            TimeSignatureType::Open => tick - self.tick,
            _ => (tick - self.tick) % self.ticks_per_bar(subdivisions),
        }
    }

    // Returns true if the tick is on a beat
    pub fn is_on_beat(&self, tick: Tick, subdivisions: Ticks) -> bool {
        self.is_on_beat_type(tick, &self.beat_type, subdivisions)
    }

    /// Return true if a tick is on an arbitrary beat type
    pub fn is_on_beat_type(
        &self,
        tick: Tick,
        beat_type: &NoteDuration,
        subdivisions: Ticks,
    ) -> bool {
        let ticks_per_beat = beat_type.to_ticks(subdivisions);
        ((tick - self.tick) % ticks_per_beat) == 0
    }

    pub fn is_on_first_beat(&self, tick: Tick, subdivisions: Ticks) -> bool {
        self.distance_from_barline(tick, subdivisions) == 0
    }

    pub fn get_tick_at_beat(&self, start: Tick, beat: u8, subdivisions: Ticks) -> Tick {
        start + (((beat - 1) as Tick) * self.ticks_per_beat(subdivisions))
    }

    // Returns true is the tick is on a beat group boundry
    pub fn is_on_grouping_boundry(&self, tick: Tick, subdivisions: Ticks) -> bool {
        match self.kind() {
            TimeSignatureType::Open => false,
            _ => {
                let start = tick - self.distance_from_barline(tick, subdivisions);
                for boundry in self.groupings_to_ticks(start, subdivisions) {
                    if boundry == tick {
                        return true;
                    } else {
                        continue;
                    }
                }
                false
            }
        }
    }

    pub fn groupings_to_ticks(&self, start: Tick, subdivisions: Ticks) -> Vec<Tick> {
        let mut output = vec![start];

        let mut acc = start;
        for group in &self.groupings {
            acc += *group as u32 * self.ticks_per_beat(subdivisions);
            output.push(acc);
        }

        output
    }

    pub fn metrics(&self, subdivisions: Ticks) -> BoundingBox {
        if self.draw_type == TimeSignatureDrawType::Hidden {
            return BoundingBox {
                width: 0.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 0.0, 0.0, 0.0),
            };
        }

        let is_wide = self.beats > 9
            || self.beat_type.to_ticks(subdivisions) < NoteDuration::Eighth.to_ticks(subdivisions);
        if is_wide {
            BoundingBox {
                width: 3.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 2.0, 0.0, 0.0),
            }
        } else {
            BoundingBox {
                width: 1.7,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 2.0, 0.0, 0.0),
            }
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_time_signature(
        &mut self,
        flow_key: &str,
        tick: Tick,
        beats: u8,
        beat_type: NoteDuration,
        draw_type: TimeSignatureDrawType,
        groupings: Option<Vec<u8>>,
    ) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        let master = self.score.tracks.get_mut(&flow.master).unwrap();

        // remove old time signative if defined
        if let Some(time_signature) = master.get_time_signature_at_tick(&tick) {
            let key = time_signature.key.clone();
            master.remove(&key);
        };

        // insert the new time signature
        let new = TimeSignature::new(tick, beats, beat_type, draw_type, groupings);
        let ticks_per_bar = new.ticks_per_bar(flow.subdivisions);
        master.insert(Entry::TimeSignature(new));

        // calculate diff
        let next_tick = match master.get_time_signature_after_tick(tick, flow.length) {
            Some(entry) => entry.tick,
            None => flow.length,
        };

        let overflow = (next_tick - tick) % ticks_per_bar;
        let fill = ticks_per_bar - overflow;

        if fill > 0 {
            flow.length += fill;
            for tick in tick + 1..flow.length {
                if let Some(time_signature) = master.get_time_signature_at_tick(&tick) {
                    let key = time_signature.key.clone();
                    let tick = time_signature.tick + fill;
                    master.shift(&key, tick);
                };
            }
        }

        self.emit();
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_time_signature_at_tick(&self, at: &Tick) -> Option<&TimeSignature> {
        let entry_keys = match self.entries.by_tick.get(at) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys {
            if let Some(Entry::TimeSignature(time_signature)) = self.entries.by_key.get(key) {
                return Some(time_signature);
            }
        }

        None
    }

    /// Returns the next time signature entry *after* a given tick if it exists
    pub fn get_time_signature_after_tick(
        &self,
        tick: Tick,
        length: Ticks,
    ) -> Option<&TimeSignature> {
        for i in tick + 1..length {
            if let Some(time_signature) = self.get_time_signature_at_tick(&i) {
                return Some(time_signature);
            };
        }

        None
    }
}

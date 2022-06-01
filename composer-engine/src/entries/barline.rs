use super::Entry;
use crate::components::measurements::BoundingBox;
use crate::components::measurements::PaddingSpaces;
use crate::components::misc::Tick;
use crate::score::tracks::Track;
use crate::utils::shortid;
use crate::Engine;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BarlineDrawType {
    Single,
    Double,
    EndRepeat,
    EndStartRepeat,
    StartRepeat,
    Final,
}

impl BarlineDrawType {
    pub fn metrics(&self) -> BoundingBox {
        match self {
            BarlineDrawType::Single => BoundingBox {
                width: 0.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.5, 0.0, 0.0),
            },
            BarlineDrawType::Double => BoundingBox {
                width: 0.5,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineDrawType::EndRepeat => BoundingBox {
                width: 2.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineDrawType::EndStartRepeat => BoundingBox {
                width: 3.5,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineDrawType::StartRepeat => BoundingBox {
                width: 2.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineDrawType::Final => BoundingBox {
                width: 1.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 0.0, 0.0, 0.0),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barline {
    pub key: String,
    pub tick: Tick,
    pub barline_type: BarlineDrawType,
}

impl Barline {
    pub fn new(tick: Tick, barline_type: BarlineDrawType) -> Self {
        Self {
            key: shortid(),
            tick,
            barline_type,
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_barline(&mut self, flow_key: &str, tick: Tick, draw_type: BarlineDrawType) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        let master = self.score.tracks.get_mut(&flow.master).unwrap();

        // remove old time signative if defined
        if let Some(old) = master.get_barline_at_tick(&tick) {
            let key = old.key.clone();
            master.remove(&key);
        };

        // insert the new time signature
        let new = Barline::new(tick, draw_type);
        master.insert(Entry::Barline(new));

        self.emit();
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_barline_at_tick(&self, at: &Tick) -> Option<&Barline> {
        let entry_keys = match self.entries.by_tick.get(at) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys {
            if let Some(Entry::Barline(barline)) = self.entries.by_key.get(key) {
                return Some(barline);
            }
        }

        None
    }
}

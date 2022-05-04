use super::Entry;
use crate::components::measurements::BoundingBox;
use crate::components::measurements::PaddingSpaces;
use crate::components::misc::Tick;
use crate::score::tracks::Track;
use crate::utils::shortid;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum BarlineType {
    Double,
    EndRepeat,
    EndStartRepeat,
    Final,
    Normal,
    StartRepeat,
}

#[derive(Debug, Clone)]
pub struct Barline {
    pub key: String,
    pub tick: Tick,
    pub barline_type: BarlineType,
}

impl Barline {
    pub fn new(tick: Tick, barline_type: BarlineType) -> Self {
        Self {
            key: shortid(),
            tick,
            barline_type,
        }
    }

    pub fn metrics(&self) -> BoundingBox {
        match self.barline_type {
            BarlineType::Double => BoundingBox {
                width: 0.5,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineType::EndRepeat => BoundingBox {
                width: 2.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineType::EndStartRepeat => BoundingBox {
                width: 2.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineType::Final => BoundingBox {
                width: 1.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineType::Normal => BoundingBox {
                width: 0.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
            BarlineType::StartRepeat => BoundingBox {
                width: 2.0,
                height: 4.0,
                padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
            },
        }
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_barline_at_tick(&self, at: &Tick) -> Option<Barline> {
        let entry_keys = match self.entries.by_tick.get(at) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys {
            if let Some(Entry::Barline(barline)) = self.entries.by_key.get(key) {
                return Some(barline.clone());
            }
        }

        None
    }
}

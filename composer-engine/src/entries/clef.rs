use crate::components::measurements::{BoundingBox, PaddingSpaces};
use crate::components::misc::Tick;
use crate::components::pitch::Pitch;
use crate::score::tracks::Track;
use crate::utils::shortid;
use wasm_bindgen::prelude::*;

use super::Entry;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ClefDrawType {
    Hidden,
    G,
    F,
    C,
    Percussion,
}

#[derive(Debug, Clone)]
pub struct Clef {
    pub key: String,
    pub tick: Tick,
    pub draw_as: ClefDrawType,
    pub pitch: Pitch, // the pitch that the clef sits on
    pub offset: i8,   // visual offset from middle stave line
}

impl Clef {
    pub fn new(tick: Tick, pitch: u8, offset: i8, draw_as: ClefDrawType) -> Self {
        Self {
            key: shortid(),
            tick,
            draw_as,
            pitch: Pitch::from_int(pitch),
            offset,
        }
    }

    fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: 2.8,
            height: 4.0,
            padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
        }
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_clef_at_tick(&self, tick: &Tick) -> Option<Clef> {
        let entry_keys = match self.entries.by_tick.get(tick) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys.iter() {
            if let Some(Entry::Clef(clef)) = self.entries.by_key.get(key) {
                return Some(clef.clone());
            }
        }

        None
    }
}

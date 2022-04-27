use crate::components::measurements::{BoundingBox, PaddingSpaces};
use crate::components::misc::Tick;
use crate::entries::Entry;
use crate::score::tracks::Track;
use crate::utils::shortid;
use crate::Engine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum KeySignatureMode {
    Major,
    Minor,
}

#[derive(Debug, Clone)]
pub struct KeySignature {
    pub key: String,
    pub tick: Tick,
    pub mode: KeySignatureMode,
    pub offset: i8,
}

impl KeySignature {
    pub fn new(tick: Tick, mode: KeySignatureMode, offset: i8) -> Self {
        Self {
            key: shortid(),
            tick,
            mode,
            offset,
        }
    }

    pub fn metrics(&self) -> BoundingBox {
        let width = self.offset.abs() as f32;

        let mut right_padding = 0.0;
        if width > 0.0 {
            right_padding = 1.0;
        };

        BoundingBox {
            width,
            height: 4.0,
            padding: PaddingSpaces::new(0.0, right_padding, 0.0, 0.0),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_key_signature(
        &mut self,
        flow_key: &str,
        tick: Tick,
        mode: KeySignatureMode,
        offset: i8,
    ) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        let master = self.score.tracks.get_mut(&flow.master).unwrap();

        // remove old key signative if defined
        if let Some(old) = master.get_time_signature_at_tick(&tick) {
            master.remove(&old.key);
        };

        // insert the new key signature
        master.insert(Entry::KeySignature(KeySignature::new(tick, mode, offset)));
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_key_signature_at_tick(&self, tick: &Tick) -> Option<KeySignature> {
        let entry_keys = match self.entries.by_tick.get(tick) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys.iter() {
            if let Some(Entry::KeySignature(key_signature)) = self.entries.by_key.get(key) {
                return Some(key_signature.clone());
            }
        }

        None
    }
}

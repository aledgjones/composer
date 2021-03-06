use super::clef::{Clef, ClefDrawType};
use crate::components::measurements::{BoundingBox, PaddingSpaces};
use crate::components::misc::Tick;
use crate::components::pitch::{Accidental, Pitch};
use crate::entries::Entry;
use crate::score::tracks::Track;
use crate::utils::shortid;
use crate::Engine;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

const SHARPS: [Pitch; 7] = [
    Pitch {
        int: 66,
        accidental: Accidental::Sharp,
    },
    Pitch {
        int: 61,
        accidental: Accidental::Sharp,
    },
    Pitch {
        int: 68,
        accidental: Accidental::Sharp,
    },
    Pitch {
        int: 63,
        accidental: Accidental::Sharp,
    },
    Pitch {
        int: 70,
        accidental: Accidental::Sharp,
    },
    Pitch {
        int: 65,
        accidental: Accidental::Sharp,
    },
    Pitch {
        int: 72,
        accidental: Accidental::Sharp,
    },
];

const FLATS: [Pitch; 7] = [
    Pitch {
        int: 70,
        accidental: Accidental::Flat,
    },
    Pitch {
        int: 63,
        accidental: Accidental::Flat,
    },
    Pitch {
        int: 68,
        accidental: Accidental::Flat,
    },
    Pitch {
        int: 61,
        accidental: Accidental::Flat,
    },
    Pitch {
        int: 66,
        accidental: Accidental::Flat,
    },
    Pitch {
        int: 71,
        accidental: Accidental::Flat,
    },
    Pitch {
        int: 64,
        accidental: Accidental::Flat,
    },
];

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeySignatureMode {
    Major,
    Minor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn accidentals(&self) -> FxHashSet<&Pitch> {
        let mut output = FxHashSet::default();

        if self.offset > 0 {
            for i in 0..self.offset.abs() {
                output.insert(&SHARPS[i as usize]);
            }
        }

        if self.offset < 0 {
            for i in 0..self.offset.abs() {
                output.insert(&FLATS[i as usize]);
            }
        }

        output
    }

    pub fn metrics(&self) -> BoundingBox {
        let width = self.offset.abs() as f32;
        let right_padding = if width > 0.0 { 1.0 } else { 0.0 };

        BoundingBox {
            width,
            height: 4.0,
            padding: PaddingSpaces::new(0.0, right_padding, 0.0, 0.0),
        }
    }

    pub fn pattern(&self, clef: &Clef) -> Option<[i8; 7]> {
        match clef.draw_as {
            ClefDrawType::C => match clef.offset {
                -2 => match self.offset.cmp(&0) {
                    Ordering::Greater => Some([2, -2, 1, -3, 0, -4, -1]),
                    Ordering::Less => Some([-1, -4, 0, -3, 1, -2, 2]),
                    Ordering::Equal => None,
                },
                0 => match self.offset.cmp(&0) {
                    Ordering::Greater => Some([-3, 0, -4, -1, 2, -2, 1]),
                    Ordering::Less => Some([1, -2, 2, -1, 3, 0, 4]),
                    Ordering::Equal => None,
                },
                _ => None,
            },
            ClefDrawType::F => match clef.offset {
                -2 => match self.offset.cmp(&0) {
                    Ordering::Greater => Some([-2, 1, -3, 0, 3, -1, 2]),
                    Ordering::Less => Some([2, -1, 3, 0, 4, 1, 5]),
                    Ordering::Equal => None,
                },
                _ => None,
            },
            ClefDrawType::G => match clef.offset {
                2 => match self.offset.cmp(&0) {
                    Ordering::Greater => Some([-4, -1, -5, -2, 1, -3, 0]),
                    Ordering::Less => Some([0, -3, 1, -2, 2, -1, 3]),
                    Ordering::Equal => None,
                },
                _ => None,
            },
            ClefDrawType::Hidden => None,
            ClefDrawType::Percussion => None,
        }
    }

    pub fn glyph(&self) -> String {
        let accidental = match self.offset.cmp(&0) {
            Ordering::Greater => Accidental::Sharp,
            Ordering::Less => Accidental::Flat,
            Ordering::Equal => Accidental::Natural,
        };
        accidental.to_glyph()
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
        if let Some(key_signature) = master.get_key_signature_at_tick(&tick) {
            let key = key_signature.key.clone();
            master.remove(&key);
        };

        // insert the new key signature
        master.insert(Entry::KeySignature(KeySignature::new(tick, mode, offset)));

        self.emit();
    }
}

impl Track {
    /// Returns the time signature entry at a given tick if it exists
    pub fn get_key_signature_at_tick(&self, at: &Tick) -> Option<&KeySignature> {
        let entry_keys = match self.entries.by_tick.get(at) {
            Some(entries) => entries,
            None => return None,
        };

        for key in entry_keys {
            if let Some(Entry::KeySignature(key_signature)) = self.entries.by_key.get(key) {
                return Some(key_signature);
            }
        }

        None
    }

    pub fn get_key_signature_before_tick(&self, at: Tick) -> Option<&KeySignature> {
        for tick in (0..at).rev() {
            match self.get_key_signature_at_tick(&tick) {
                Some(key_signature) => return Some(key_signature),
                None => continue,
            }
        }

        None
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::components::pitch::Accidental;
//     use crate::components::pitch::Pitch;
//     use crate::entries::key_signature::KeySignature;
//     use crate::entries::key_signature::KeySignatureMode;
//     use std::collections::HashSet;

//     #[test]
//     fn accidentals_test_1() {
//         let key = KeySignature::new(0, KeySignatureMode::Major, 0);
//         assert_eq!(key.accidentals(), HashSet::new());
//     }

//     #[test]
//     fn accidentals_test_2() {
//         let key = KeySignature::new(0, KeySignatureMode::Major, 2);
//         assert_eq!(
//             key.accidentals(),
//             hashset! {
//                 &Pitch {
//                     int: 66,
//                     accidental: Accidental::Sharp,
//                 },
//                 &Pitch {
//                     int: 61,
//                     accidental: Accidental::Sharp,
//                 }
//             }
//         );
//     }

//     #[test]
//     fn accidentals_test_3() {
//         let key = KeySignature::new(0, KeySignatureMode::Major, -2);
//         assert_eq!(
//             key.accidentals(),
//             hashset! {
//                 &Pitch {
//                     int: 70,
//                     accidental: Accidental::Flat,
//                 },
//                 &Pitch {
//                     int: 63,
//                     accidental: Accidental::Flat,
//                 }
//             }
//         );
//     }
// }

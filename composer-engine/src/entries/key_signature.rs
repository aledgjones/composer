use crate::components::measurements::{BoundingBox, PaddingSpaces};
use crate::components::misc::Tick;
use crate::components::pitch::{Accidental, Pitch};
use crate::entries::Entry;
use crate::score::tracks::Track;
use crate::utils::shortid;
use crate::Engine;
use std::collections::HashSet;
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

    pub fn accidentals(&self) -> HashSet<&Pitch> {
        let mut output = HashSet::new();

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

#[cfg(test)]
mod tests {
    use crate::components::pitch::Accidental;
    use crate::components::pitch::Pitch;
    use crate::entries::key_signature::KeySignature;
    use crate::entries::key_signature::KeySignatureMode;
    use std::collections::HashSet;

    #[test]
    fn accidentals_test_1() {
        let key = KeySignature::new(0, KeySignatureMode::Major, 0);
        assert_eq!(key.accidentals(), HashSet::new());
    }

    #[test]
    fn accidentals_test_2() {
        let key = KeySignature::new(0, KeySignatureMode::Major, 2);
        assert_eq!(
            key.accidentals(),
            hashset! {
                &Pitch {
                    int: 66,
                    accidental: Accidental::Sharp,
                },
                &Pitch {
                    int: 61,
                    accidental: Accidental::Sharp,
                }
            }
        );
    }

    #[test]
    fn accidentals_test_3() {
        let key = KeySignature::new(0, KeySignatureMode::Major, -2);
        assert_eq!(
            key.accidentals(),
            hashset! {
                &Pitch {
                    int: 70,
                    accidental: Accidental::Flat,
                },
                &Pitch {
                    int: 63,
                    accidental: Accidental::Flat,
                }
            }
        );
    }
}

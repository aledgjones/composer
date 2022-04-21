pub mod defs;
pub mod utils;

use super::config::AutoCountStyle;
use crate::score::players::PlayerType;
use crate::utils::shortid;
use crate::Engine;
use defs::{get_def, InstrumentType};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Instrument {
    pub key: String,
    pub id: String,
    pub instrument_type: InstrumentType,
    pub long_name: String,
    pub short_name: String,
    pub staves: Vec<String>,
    pub count: Option<u8>,
    pub volume: u8, // 0-127
    pub solo: bool,
    pub mute: bool,
}

impl Instrument {
    pub fn family(&self) -> &str {
        let mut parts = self.id.split('.');
        match parts.next() {
            None => "unknown",
            Some(part) => part,
        }
    }
}

#[wasm_bindgen]
impl Engine {
    /// Create an instrument
    pub fn create_instrument(&mut self, id: &str) -> String {
        let key = shortid();

        let def = get_def(id).unwrap();
        let staves = def
            .staves
            .iter()
            .map(|_| shortid())
            .collect::<Vec<String>>();

        let instrument = Instrument {
            key: key.clone(),
            id: String::from(id),
            instrument_type: def.instrument_type.clone(),
            long_name: String::from(def.long_name),
            short_name: String::from(def.short_name),
            staves,
            count: None,
            volume: 80,
            solo: false,
            mute: false,
        };

        self.score
            .instruments
            .insert(instrument.key.clone(), instrument);

        self.emit();

        key
    }

    pub fn remove_instrument(&mut self, instrument_key: &str) {
        self.score.instruments.remove(instrument_key);

        self.emit();
    }

    pub fn get_instrument_name(&self, player_key: &str, instrument_key: &str) -> String {
        let player = self.score.players.by_key.get(player_key).unwrap();
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        let count_type = match player.player_type {
            PlayerType::Solo => &self.score.config.auto_count.solo,
            PlayerType::Section => &self.score.config.auto_count.section,
        };

        match instrument.count {
            Some(count) => {
                let styled_count = match count_type {
                    AutoCountStyle::Roman => roman::to(count as i32).unwrap(),
                    AutoCountStyle::Arabic => count.to_string(),
                };
                format!("{} {}", instrument.long_name, styled_count)
            }
            None => instrument.long_name.clone(),
        }
    }

    pub fn calculate_counts(&mut self) {
        let mut instruments_solo: HashMap<String, Vec<String>> = HashMap::new();
        let mut instruments_section: HashMap<String, Vec<String>> = HashMap::new();

        // collect keys of each instruments with exactly the same name and player type
        for player_key in &self.score.players.order {
            let player = self.score.players.by_key.get(player_key).unwrap();
            for instrument_key in &player.instruments {
                let instrument = self.score.instruments.get_mut(instrument_key).unwrap();

                // None out any current counts
                instrument.count = None;

                match player.player_type {
                    PlayerType::Solo => {
                        let entry = instruments_solo
                            .entry(instrument.long_name.clone())
                            .or_insert(vec![]);
                        entry.push(instrument.key.clone());
                    }
                    PlayerType::Section => {
                        let entry = instruments_section
                            .entry(instrument.long_name.clone())
                            .or_insert(vec![]);
                        entry.push(instrument.key.clone());
                    }
                }
            }
        }

        for (_, instrument_keys) in instruments_solo {
            if instrument_keys.len() > 1 {
                for (i, instrument_key) in instrument_keys.iter().enumerate() {
                    if let Some(instrument) = self.score.instruments.get_mut(instrument_key) {
                        instrument.count = Some(i as u8 + 1)
                    };
                }
            }
        }

        for (_, instrument_keys) in instruments_section {
            if instrument_keys.len() > 1 {
                for (i, instrument_key) in instrument_keys.iter().enumerate() {
                    if let Some(instrument) = self.score.instruments.get_mut(instrument_key) {
                        instrument.count = Some(i as u8 + 1)
                    };
                }
            }
        }
    }
}

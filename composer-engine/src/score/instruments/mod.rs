pub mod defs;
pub mod utils;

use super::config::AutoCountStyle;
use crate::utils::shortid;
use crate::Engine;
use crate::{components::misc::ALPHABET_LOWERCASE, score::players::PlayerType};
use defs::{get_def, InstrumentType};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    pub key: String,
    pub id: String,
    pub instrument_type: InstrumentType,
    pub long_name: String,
    pub short_name: String,
    pub staves: Vec<String>,
    pub count: Option<String>,
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

    pub fn name(&self) -> String {
        match &self.count {
            Some(count) => {
                format!("{} {}", &self.long_name, count)
            }
            None => self.long_name.clone(),
        }
    }
}

impl Engine {
    pub fn assign_count(&mut self, entries: FxHashMap<String, Vec<(String, String)>>) {
        for (_, keys) in entries {
            if keys.len() > 1 {
                for (i, (player_key, instrument_key)) in keys.iter().enumerate() {
                    let player = self.score.players.by_key.get(player_key).unwrap();
                    let instrument = self.score.instruments.get_mut(instrument_key).unwrap();
                    let count = (i + 1) as i32;

                    let count_type = match player.player_type {
                        PlayerType::Solo => &self.score.config.auto_count.solo,
                        PlayerType::Section => &self.score.config.auto_count.section,
                    };

                    let styled_count = match count_type {
                        AutoCountStyle::Roman => roman::to(count).unwrap(),
                        AutoCountStyle::Arabic => count.to_string(),
                    };

                    instrument.count = Some(styled_count);
                }
            }
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

    pub fn get_instrument_name(&self, instrument_key: &str) -> String {
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        instrument.name()
    }

    pub fn get_instrument_id(&self, instrument_key: &str) -> String {
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        instrument.id.clone()
    }

    pub fn get_instrument_volume(&self, instrument_key: &str) -> u8 {
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        instrument.volume
    }

    pub fn set_instrument_volume(&mut self, instrument_key: &str, value: u8) {
        let instrument = self.score.instruments.get_mut(instrument_key).unwrap();
        instrument.volume = value;

        self.emit();
    }

    pub fn get_instrument_solo(&self, instrument_key: &str) -> bool {
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        instrument.solo
    }

    pub fn toggle_instrument_solo(&mut self, instrument_key: &str) {
        let instrument = self.score.instruments.get_mut(instrument_key).unwrap();
        instrument.solo = !instrument.solo;

        self.emit();
    }

    pub fn get_instrument_mute(&self, instrument_key: &str) -> bool {
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        instrument.mute
    }

    pub fn toggle_instrument_mute(&mut self, instrument_key: &str) {
        let instrument = self.score.instruments.get_mut(instrument_key).unwrap();
        instrument.mute = !instrument.mute;

        self.emit();
    }

    pub fn get_instrument_staves(&self, instrument_key: &str) -> JsValue {
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        JsValue::from_serde(&instrument.staves).unwrap()
    }

    pub fn get_instrument_tracks(&self, flow_key: &str, instrument_key: &str) -> JsValue {
        let mut output: Vec<(&String, String)> = Vec::new();
        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        let instrument = self.score.instruments.get(instrument_key).unwrap();
        let multi_stave = instrument.staves.len() > 1;

        for (i, stave_key) in instrument.staves.iter().enumerate() {
            let stave = flow.staves.get(stave_key).unwrap();
            for (ii, track_key) in stave.tracks.iter().enumerate() {
                if multi_stave {
                    output.push((
                        track_key,
                        format!("Staff ({}), Voice {}", ALPHABET_LOWERCASE[i], ii + 1),
                    ));
                } else {
                    output.push((track_key, format!("Voice {}", ii + 1)));
                }
            }
        }

        JsValue::from_serde(&output).unwrap()
    }

    pub fn calculate_counts(&mut self) {
        let mut instruments_solo: FxHashMap<String, Vec<(String, String)>> = FxHashMap::default();
        let mut instruments_section: FxHashMap<String, Vec<(String, String)>> =
            FxHashMap::default();

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
                        entry.push((player_key.clone(), instrument.key.clone()));
                    }
                    PlayerType::Section => {
                        let entry = instruments_section
                            .entry(instrument.long_name.clone())
                            .or_insert(vec![]);
                        entry.push((player_key.clone(), instrument.key.clone()));
                    }
                }
            }
        }

        self.assign_count(instruments_solo);
        self.assign_count(instruments_section);
    }
}

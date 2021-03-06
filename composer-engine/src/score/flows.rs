use super::instruments::defs::get_def;
use super::instruments::Instrument;
use super::stave::Stave;
use super::tracks::Track;
use crate::components::duration::NoteDuration;
use crate::components::misc::{Tick, Ticks};
use crate::entries::clef::Clef;
use crate::entries::key_signature::{KeySignature, KeySignatureMode};
use crate::entries::time_signature::{TimeSignature, TimeSignatureDrawType};
use crate::entries::Entry;
use crate::utils::shortid;
use crate::Engine;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct Flows {
    pub order: Vec<String>,
    pub by_key: FxHashMap<String, Flow>,
}

impl Flows {
    pub fn new() -> Flows {
        Flows {
            order: Vec::new(),
            by_key: FxHashMap::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flow {
    pub key: String,
    pub title: String,
    pub players: FxHashSet<String>, // purely for inclusion lookup -- order comes from score.players.order
    pub length: Ticks,              // number of subdivision ticks in the flow
    pub subdivisions: Ticks,

    pub master: String,
    pub staves: FxHashMap<String, Stave>,
}

impl Flow {
    pub fn new(master: &Track) -> Flow {
        Flow {
            key: shortid(),
            title: String::from(""),
            players: FxHashSet::default(),
            length: 48 * 4 * 4, // 4 * 4/4
            subdivisions: 48,

            master: master.key.clone(),
            staves: FxHashMap::default(),
        }
    }
}

type TickListItem = (f32, f32, bool, bool, bool, bool);

#[derive(Serialize)]
struct TickList {
    list: Vec<TickListItem>,
    width: f32,
}

impl Engine {
    pub fn get_flow_instruments(
        &self,
        flow_key: &str,
    ) -> (&Flow, Vec<&Instrument>, Vec<&Stave>, Vec<&Track>) {
        let mut instruments: Vec<&Instrument> = Vec::new();
        let mut staves: Vec<&Stave> = Vec::new();
        let mut tracks: Vec<&Track> = Vec::new();

        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        for player_key in &self.score.players.order {
            if flow.players.contains(player_key) {
                let player = self.score.players.by_key.get(player_key).unwrap();
                for instrument_key in &player.instruments {
                    let instrument = self.score.instruments.get(instrument_key).unwrap();
                    instruments.push(instrument);

                    for stave_key in &instrument.staves {
                        let stave = flow.staves.get(stave_key).unwrap();
                        staves.push(stave);

                        for track_key in &stave.tracks {
                            let track = self.score.tracks.get(track_key).unwrap();
                            tracks.push(track);
                        }
                    }
                }
            }
        }
        (flow, instruments, staves, tracks)
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_flow(&mut self) -> String {
        let mut master = Track::new();
        let flow = Flow::new(&master);

        master.insert(Entry::TimeSignature(TimeSignature::new(
            0,
            4,
            NoteDuration::Quarter,
            TimeSignatureDrawType::Regular,
            None,
        )));

        master.insert(Entry::KeySignature(KeySignature::new(
            0,
            KeySignatureMode::Major,
            -3,
        )));

        self.score.tracks.insert(master.key.clone(), master);

        let flow_key = flow.key.clone();

        self.score.flows.order.push(flow.key.clone());
        self.score.flows.by_key.insert(flow.key.clone(), flow);

        // add all the player keys into the new flow
        let players = self.score.players.order.clone();
        for player_key in players {
            self.assign_player_to_flow(&flow_key, &player_key);
        }

        self.emit();

        flow_key
    }

    pub fn remove_flow(&mut self, flow_key: &str) {
        self.score.flows.order.retain(|e| e != flow_key);

        let flow = self.score.flows.by_key.remove(flow_key).unwrap();
        self.score.tracks.remove(&flow.master);

        for player_key in &flow.players {
            let player = self.score.players.by_key.get(player_key).unwrap();
            for instrument_key in &player.instruments {
                let instrument = self.score.instruments.get(instrument_key).unwrap();
                for stave_key in &instrument.staves {
                    let stave = flow.staves.get(stave_key).unwrap();
                    self.score.tracks.remove(&stave.master);
                    for track_key in &stave.tracks {
                        self.score.tracks.remove(track_key);
                    }
                }
            }
        }

        self.emit();
    }

    pub fn reorder_flow(&mut self, old_index: usize, new_index: usize) {
        let removed = self.score.flows.order.remove(old_index);
        self.score.flows.order.insert(new_index, removed);

        self.emit();
    }

    pub fn rename_flow(&mut self, flow_key: &str, name: &str) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        flow.title = String::from(name);

        self.emit();
    }

    pub fn get_flow_length(&mut self, flow_key: &str) -> u32 {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        flow.length
    }

    pub fn set_flow_length(&mut self, flow_key: &str, length: Ticks) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        flow.length = length;

        self.emit();
    }

    /**
     * Assign a player to a flow
     */
    pub fn assign_player_to_flow(&mut self, flow_key: &str, player_key: &str) {
        // add the player_key to the flow
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        flow.players.insert(String::from(player_key));

        // add staves and tracks to this flow
        let player = self.score.players.by_key.get(player_key).unwrap();
        let instruments = player.instruments.clone();
        for instrument_key in instruments {
            self.assign_instrument_to_flow(flow_key, &instrument_key)
        }

        self.emit();
    }

    /**
     * Assign instrument to flow
     */
    pub fn assign_instrument_to_flow(&mut self, flow_key: &str, instrument_key: &str) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();

        let instrument = self.score.instruments.get(instrument_key).unwrap();
        let def = get_def(&instrument.id).unwrap();
        for (i, stave_key) in instrument.staves.iter().enumerate() {
            let stave_def = def.staves.get(i).unwrap();

            let mut master = Track::new();
            let clef = Entry::Clef(Clef::new(
                0,
                stave_def.clef.pitch.int,
                stave_def.clef.offset,
                stave_def.clef.draw_as.clone(),
            ));
            master.insert(clef);

            let track = Track::new();

            let mut stave = Stave::new(stave_key.clone(), &def.staves[i], &master);
            stave.tracks.push(track.key.clone());

            self.score.tracks.insert(track.key.clone(), track);
            self.score.tracks.insert(master.key.clone(), master);

            flow.staves.insert(stave.key.clone(), stave);
        }
    }

    pub fn unassign_player_from_flow(&mut self, flow_key: &str, player_key: &str) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();
        flow.players.remove(player_key);

        let player = self.score.players.by_key.get(player_key).unwrap();
        let instruments = player.instruments.clone();
        for instrument_key in instruments {
            self.unassign_instrument_from_flow(flow_key, &instrument_key);
        }

        self.emit();
    }

    pub fn unassign_instrument_from_flow(&mut self, flow_key: &str, instrument_key: &str) {
        let flow = self.score.flows.by_key.get_mut(flow_key).unwrap();

        let instrument = self.score.instruments.get(instrument_key).unwrap();
        for stave_key in &instrument.staves {
            if let Some(stave) = flow.staves.get(stave_key) {
                self.score.tracks.remove(&stave.master);
                for track_key in &stave.tracks {
                    self.score.tracks.remove(track_key);
                }
                flow.staves.remove(stave_key);
            };
        }
    }

    #[wasm_bindgen(getter)]
    pub fn flows(&self) -> JsValue {
        JsValue::from_serde(&self.score.flows.order).unwrap()
    }

    pub fn get_flow_subdivisions(&self, flow_key: &str) -> u32 {
        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        flow.subdivisions
    }

    pub fn get_flow_title(&self, flow_key: &str) -> String {
        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        flow.title.clone()
    }

    pub fn flow_contains_player(&self, flow_key: &str, player_key: &str) -> bool {
        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        flow.players.contains(player_key)
    }

    pub fn get_flow_ticks(&self, flow_key: &str) -> JsValue {
        const QUARTER_WIDTH: f32 = 72.0;

        let mut output = TickList {
            list: Vec::new(),
            width: 0.0,
        };

        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        let master = self.score.tracks.get(&flow.master).unwrap();

        let tick_width = QUARTER_WIDTH / flow.subdivisions as f32;

        let mut time_signature = &TimeSignature::default();

        for tick in 0..flow.length {
            if let Some(entry) = master.get_time_signature_at_tick(&tick) {
                time_signature = entry;
            }

            let first = time_signature.is_on_first_beat(tick, flow.subdivisions);
            let beat = time_signature.is_on_beat(tick, flow.subdivisions);
            let sub = time_signature.is_on_beat_type(
                tick,
                &time_signature.beat_type.half(),
                flow.subdivisions,
            );
            let boundry = time_signature.is_on_grouping_boundry(tick, flow.subdivisions);

            output
                .list
                .push((output.width, tick_width, first, beat, sub, boundry));

            output.width += tick_width;
        }

        JsValue::from_serde(&output).unwrap()
    }

    pub fn get_timestamp(&self, flow_key: &str, at: Tick) -> String {
        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        let master = self.score.tracks.get(&flow.master).unwrap();

        let mut time_signature = &TimeSignature::default();
        let mut bar: u32 = 0;

        for tick in 0..flow.length {
            if let Some(entry) = master.get_time_signature_at_tick(&tick) {
                time_signature = entry;
            }

            let distance = time_signature.distance_from_barline(tick, flow.subdivisions) as f32;

            if distance == 0.0 {
                bar += 1;
            };

            if at == tick {
                let ticks_per_beat = time_signature.beat_type.to_ticks(flow.subdivisions) as f32;
                let beats = (distance / ticks_per_beat).floor() + 1.0;
                let half_beats = (distance % ticks_per_beat) / (ticks_per_beat / 2.0);

                return format!("{}:{:.0}:{:.3}", bar, beats.floor(), half_beats);
            };
        }

        String::from("1:1:0.000")
    }
}

use super::instruments::defs::get_def;
use super::instruments::Instrument;
use super::players::Player;
use super::stave::Stave;
use super::tracks::Track;
use crate::components::duration::NoteDuration;
use crate::entries::clef::Clef;
use crate::entries::time_signature::{TimeSignature, TimeSignatureDrawType};
use crate::entries::Entry;
use crate::utils::shortid;
use crate::Engine;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct Flows {
    pub order: Vec<String>,
    pub by_key: HashMap<String, Flow>,
}

impl Flows {
    pub fn new() -> Flows {
        Flows {
            order: Vec::new(),
            by_key: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Flow {
    pub key: String,
    pub title: String,
    pub players: HashSet<String>, // purely for inclusion lookup -- order comes from score.players.order
    pub length: u32,              // number of subdivision ticks in the flow

    pub master: String,
    pub staves: HashMap<String, Stave>,
}

impl Flow {
    pub fn new(master: &Track) -> Flow {
        Flow {
            key: shortid(),
            title: String::from(""),
            players: HashSet::new(),
            length: 16 * 4, // 4 * 4/4

            master: master.key.clone(),
            staves: HashMap::new(),
        }
    }
}

#[derive(Serialize)]
struct Tick {
    x: f32,
    width: f32,
    first: bool,
    beat: bool,
    sub_beat: bool,
    boundry: bool,
}

#[derive(Serialize)]
struct TickList {
    list: Vec<Tick>,
    width: f32,
}

impl Engine {
    pub fn get_flow_players(
        &self,
        flow_key: &str,
    ) -> (&Flow, Vec<&Player>, Vec<&Instrument>, Vec<&Stave>) {
        let mut players: Vec<&Player> = Vec::new();
        let mut instruments: Vec<&Instrument> = Vec::new();
        let mut staves: Vec<&Stave> = Vec::new();

        let flow = self.score.flows.by_key.get(flow_key).unwrap();
        for player_key in &self.score.players.order {
            if flow.players.contains(player_key) {
                let player = self.score.players.by_key.get(player_key).unwrap();
                players.push(player);
                for instrument_key in &player.instruments {
                    let instrument = self.score.instruments.get(instrument_key).unwrap();
                    instruments.push(instrument);

                    for stave_key in &instrument.staves {
                        let stave = flow.staves.get(stave_key).unwrap();
                        staves.push(stave);
                    }
                }
            }
        }
        (flow, players, instruments, staves)
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
            TimeSignatureDrawType::Normal,
            None,
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

    pub fn set_flow_length(&mut self, flow_key: &str, length: u32) {
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
            let stave = flow.staves.get(stave_key).unwrap();
            self.score.tracks.remove(&stave.master);
            for track_key in &stave.tracks {
                self.score.tracks.remove(track_key);
            }
            flow.staves.remove(stave_key);
        }
    }

    #[wasm_bindgen(getter)]
    pub fn flows(&self) -> JsValue {
        JsValue::from_serde(&self.score.flows.order).unwrap()
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

        let mut time_signature: TimeSignature = TimeSignature::new(
            0,
            4,
            NoteDuration::Quarter,
            TimeSignatureDrawType::Hidden,
            None,
        );

        for tick in 0..flow.length {
            if let Some(entry) = master.get_time_signature_at_tick(tick) {
                time_signature = entry;
            }

            let tick_width = QUARTER_WIDTH / time_signature.subdivisions as f32;

            output.list.push(Tick {
                x: output.width,
                width: tick_width,
                first: time_signature.is_on_first_beat(tick),
                beat: time_signature.is_on_beat(tick),
                sub_beat: time_signature.is_on_beat_type(tick, &time_signature.beat_type.half()),
                boundry: time_signature.is_on_grouping_boundry(tick),
            });

            output.width += tick_width;
        }

        JsValue::from_serde(&output).unwrap()
    }
}

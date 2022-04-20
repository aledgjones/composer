use crate::utils::shortid;
use crate::Engine;
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub enum PlayerType {
    Solo,
    Section,
}

#[derive(Debug)]
pub struct Player {
    pub key: String,
    pub player_type: PlayerType,
    pub instruments: Vec<String>,
}

impl Player {
    pub fn new(key: String, player_type: PlayerType) -> Self {
        Player {
            key,
            player_type,
            instruments: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Players {
    pub order: Vec<String>,
    pub by_key: HashMap<String, Player>,
}

impl Players {
    pub fn new() -> Self {
        Players {
            order: Vec::new(),
            by_key: HashMap::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_player(&mut self, player_type: PlayerType) -> String {
        let key = shortid();
        let player = Player::new(key.clone(), player_type);

        self.score.players.order.push(player.key.clone());
        self.score.players.by_key.insert(player.key.clone(), player);

        let flows = self.score.flows.order.clone();
        for flow_key in flows {
            self.assign_player_to_flow(&flow_key, &key.clone());
        }

        self.emit();

        key
    }

    pub fn remove_player(&mut self, player_key: &str) {
        let flows = self.score.flows.order.clone();
        for flow_key in flows {
            self.unassign_player_from_flow(&flow_key, player_key);
        }

        let player = self.score.players.by_key.get(player_key).unwrap();
        let instruments = player.instruments.clone();
        for instrument_key in instruments {
            self.unassign_instrument_from_player(player_key, &instrument_key);
            self.remove_instrument(&instrument_key);
        }

        self.score.players.order.retain(|item| item != player_key);
        self.score.players.by_key.remove(player_key);

        self.calculate_counts();
        self.emit();
    }

    pub fn reorder_players(&mut self, from: usize, to: usize) {
        let mover = self.score.players.order.remove(from);
        self.score.players.order.insert(to, mover);

        self.calculate_counts();
        self.emit();
    }

    pub fn assign_instrument_to_player(&mut self, player_key: &str, instrument_key: &str) {
        let player = self.score.players.by_key.get_mut(player_key).unwrap();
        player.instruments.push(String::from(instrument_key));

        let flows = self.score.flows.order.clone();
        for flow_key in flows {
            let flow = self.score.flows.by_key.get(&flow_key).unwrap();
            if flow.players.contains(player_key) {
                self.assign_instrument_to_flow(&flow_key, instrument_key);
            }
        }

        self.calculate_counts();
        self.emit();
    }

    pub fn unassign_instrument_from_player(&mut self, player_key: &str, instrument_key: &str) {
        let player = self.score.players.by_key.get_mut(player_key).unwrap();
        player.instruments.retain(|e| e != instrument_key);

        let flows = self.score.flows.order.clone();
        for flow_key in flows {
            let flow = self.score.flows.by_key.get(&flow_key).unwrap();
            if flow.players.contains(player_key) {
                self.unassign_instrument_from_flow(&flow_key, instrument_key);
            }
        }

        self.calculate_counts();
        self.emit();
    }

    pub fn reorder_player_instruments(&mut self, player_key: &str, from: usize, to: usize) {
        let player = self.score.players.by_key.get_mut(player_key).unwrap();

        let mover = player.instruments.remove(from);
        player.instruments.insert(to, mover);

        self.calculate_counts();
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn players(&self) -> JsValue {
        JsValue::from_serde(&self.score.players.order).unwrap()
    }

    pub fn get_player_type(&self, player_key: &str) -> PlayerType {
        let player = self.score.players.by_key.get(player_key).unwrap();
        player.player_type.clone()
    }

    pub fn get_player_name(&self, player_key: &str) -> String {
        let mut out = String::new();

        let player = self.score.players.by_key.get(player_key).unwrap();
        let length = player.instruments.len();

        if length == 0 {
            return String::from("Empty-handed Player");
        }

        for (i, instrument_key) in player.instruments.iter().enumerate() {
            let name = self.get_instrument_name(player_key, instrument_key);

            if i == 0 {
                out = name;
            } else if i == length - 1 {
                out = format!("{} & {}", out, name);
            } else {
                out = format!("{}, {}", out, name);
            }
        }

        out
    }

    pub fn get_player_instruments(&self, player_key: &str) -> JsValue {
        let player = self.score.players.by_key.get(player_key).unwrap();
        JsValue::from_serde(&player.instruments).unwrap()
    }
}

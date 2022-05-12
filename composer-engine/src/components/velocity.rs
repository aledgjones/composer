use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity {
    int: u8,
}

impl Velocity {
    pub fn new(int: u8) -> Self {
        Self { int }
    }
}

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Articulation {
    None,
    Staccato,
    Staccatissimo,
    Tenuto,
    StaccatoTenuto,
}

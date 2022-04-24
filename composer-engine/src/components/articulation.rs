use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize)]
pub enum Articulation {
    None,
    Staccato,
    Staccatissimo,
    Tenuto,
    StaccatoTenuto,
}

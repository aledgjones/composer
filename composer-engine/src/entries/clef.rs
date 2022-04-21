use crate::components::measurements::{BoundingBox, PaddingSpaces};
use crate::components::pitch::{Accidental, Pitch};
use crate::utils::shortid;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ClefDrawType {
    Hidden,
    G,
    F,
    C,
    Percussion,
}

#[derive(Debug, Clone)]
pub struct Clef {
    pub key: String,
    pub tick: u32,
    pub draw_as: ClefDrawType,
    pub pitch: Pitch, // the pitch that the clef sits on
    pub offset: i8,   // visual offset from middle stave line
}

impl Clef {
    pub fn new(tick: u32, pitch: u8, offset: i8, draw_as: ClefDrawType) -> Self {
        Self {
            key: shortid(),
            tick,
            draw_as,
            pitch: Pitch::new(pitch, Accidental::Natural),
            offset,
        }
    }

    fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: 2.8,
            height: 4.0,
            padding: PaddingSpaces::new(0.0, 1.0, 0.0, 0.0),
        }
    }
}

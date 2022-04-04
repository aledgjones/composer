use crate::components::measurements::{BoundingBox, Padding, Spaces};
use crate::components::pitch::{Accidental, Pitch};
use crate::utils::shortid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum ClefDrawType {
    Hidden,
    G,
    F,
    C,
    Percussion,
}

pub struct Clef {
    pub key: String,
    pub tick: u32,
    pub draw_as: ClefDrawType,
    pub pitch: Pitch, // the pitch that the clef sits on
    pub offset: i8,   // visual offset from middle stave line
}

impl Clef {
    pub fn new(tick: u32, pitch: u8, offset: i8, draw_as: ClefDrawType) -> Clef {
        Clef {
            key: shortid(),
            tick,
            draw_as,
            pitch: Pitch::new(pitch, Accidental::Natural),
            offset,
        }
    }

    fn metrics(&self) -> BoundingBox {
        BoundingBox {
            width: Spaces(2.8),
            height: Spaces(4.0),
            padding: Padding(Spaces(0.0), Spaces(1.0), Spaces(0.0), Spaces(0.0)),
        }
    }
}

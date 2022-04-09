use crate::components::measurements::{BoundingBox, Padding};
use crate::components::pitch::{Accidental, Pitch};
use crate::components::units::Unit;
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

#[derive(Debug)]
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
            width: Unit::Space(2.8),
            height: Unit::Space(4.0),
            padding: Padding(
                Unit::Space(0.0),
                Unit::Space(1.0),
                Unit::Space(0.0),
                Unit::Space(0.0),
            ),
        }
    }
}

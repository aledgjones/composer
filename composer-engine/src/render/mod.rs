mod line;

use self::line::Line;
use crate::components::measurements::Point;
use crate::components::units::{Converter, Unit};
use crate::Engine;
use js_sys::Function;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Instruction {
    Line(Line),
}

#[wasm_bindgen]
impl Engine {
    pub fn render(&self, flow_key: &str, px_per_mm: usize, setup: Function, render: Function) {
        let converter = Converter::new(px_per_mm as f32, 2.0);

        let _ = setup.call2(
            &JsValue::NULL,
            &JsValue::from(200.0 as f32),
            &JsValue::from(500.0 as f32),
        );

        let _ = render.call1(
            &JsValue::NULL,
            &JsValue::from_serde(&Instruction::Line(Line {
                color: String::from("#000"),
                width: 1.0,
                points: vec![Point(100.0, 50.0), Point(300.0, 75.0)],
            }))
            .unwrap(),
        );
    }
}

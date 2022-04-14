pub mod line;

use crate::components::units::Converter;
use crate::score::engrave::LayoutType;
use crate::Engine;
use js_sys::Function;
use line::Line;
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
        let instructions: Vec<Instruction> = Vec::new();

        let engrave = self
            .score
            .engrave
            .get_engrave_by_type(LayoutType::Score)
            .unwrap();
        let converter = Converter::new(px_per_mm as f32, engrave.space);

        let padding_top = converter.to_px(&engrave.frame_padding.0);
        let padding_bottom = converter.to_px(&engrave.frame_padding.2);
        let padding_left = converter.to_px(&engrave.frame_padding.3);
        let padding_right = converter.to_px(&engrave.frame_padding.1);
        let instrument_name_gap = converter.to_px(&engrave.instrument_name.padding.1);

        let x = padding_left + instrument_name_gap;
        let y = padding_top;

        let width = 100.0;

        let _ = setup.call2(
            &JsValue::NULL,
            &JsValue::from(y + 0.0 + padding_bottom),
            &JsValue::from(x + width + padding_right),
        );

        for instruction in instructions {
            let _ = render.call1(&JsValue::NULL, &JsValue::from_serde(&instruction).unwrap());
        }
    }
}

impl Engine {}

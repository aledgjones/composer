mod draw_staves;
mod get_flow_players;
mod measure_instrument_names;
mod measure_vertical_spacing;

use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::components::units::Unit;
use crate::score::engrave::LayoutType;
use crate::Engine;
use js_sys::Function;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Line {
    pub color: String,
    pub width: f32,
    pub points: Vec<Point>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Instruction {
    Line(Line),
}

#[wasm_bindgen]
impl Engine {
    pub fn render(
        &self,
        flow_key: &str,
        px_per_mm: usize,
        setup: &Function,
        render: &Function,
        measure: &Function,
    ) {
        let mut instructions: Vec<Instruction> = Vec::new();

        let engrave = self
            .score
            .engrave
            .get_engrave_by_type(LayoutType::Score)
            .unwrap();

        let converter = Converter::new(px_per_mm as f32, engrave.space.to_f32());

        let padding_top = converter.to_spaces(&engrave.frame_padding.0);
        let padding_bottom = converter.to_spaces(&engrave.frame_padding.2);
        let padding_left = converter.to_spaces(&engrave.frame_padding.3);
        let padding_right = converter.to_spaces(&engrave.frame_padding.1);

        let instrument_name_gap = engrave.instrument_name.padding.1;

        let (flow, players, instruments, staves) = self.get_flow_players(flow_key);

        let vertical_spacing = self.measure_vertical_spacing(&instruments, &flow.staves, &engrave);
        let instrument_name_width =
            self.measure_instrument_names(&players, engrave, &converter, measure);

        let x = padding_left + instrument_name_width + instrument_name_gap;
        let y = padding_top;

        let content_width = Unit::Space(20.0);

        let width = x + content_width + padding_right;
        let height = y + vertical_spacing.height + padding_bottom;

        self.draw_staves(
            &staves,
            x,
            y,
            content_width,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );

        let _ = setup.call2(
            &JsValue::NULL,
            &JsValue::from(converter.to_px(&height).to_f32()),
            &JsValue::from(converter.to_px(&width).to_f32()),
        );

        for instruction in instructions {
            let _ = render.call1(&JsValue::NULL, &JsValue::from_serde(&instruction).unwrap());
        }
    }
}

mod draw_names;
mod draw_staves;
mod get_flow_players;
mod measure_instrument_names;
mod measure_text;
mod measure_vertical_spacing;

use crate::components::measurements::Point;
use crate::components::units::{Converter, Space};
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
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub value: String,
    pub color: String,
    pub font: String,
    pub size: f32,
    pub justify: String,
    pub align: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Instruction {
    Line(Line),
    Text(Text),
}

impl Instruction {
    fn to_jsvalue(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
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

        let converter = Converter::new(px_per_mm as f32, engrave.space);

        let padding_top: Space = converter.mm_to_spaces(&engrave.frame_padding.top);
        let padding_bottom: Space = converter.mm_to_spaces(&engrave.frame_padding.bottom);
        let padding_left: Space = converter.mm_to_spaces(&engrave.frame_padding.left);
        let padding_right: Space = converter.mm_to_spaces(&engrave.frame_padding.right);

        let instrument_name_gap: Space = engrave.instrument_name.padding.right;

        let (flow, players, instruments, staves) = self.get_flow_players(flow_key);

        let vertical_spacing = self.measure_vertical_spacing(&instruments, &flow.staves, engrave);
        let instrument_name_width =
            self.measure_instrument_names(&players, engrave, &converter, measure);

        let content_width: Space = 20.0;

        let width: Space = padding_left
            + instrument_name_width
            + instrument_name_gap
            + content_width
            + padding_right;
        let height: Space = padding_top + vertical_spacing.height + padding_bottom;

        self.draw_staves(
            &staves,
            &(padding_left + instrument_name_width + instrument_name_gap),
            &padding_top,
            &content_width,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        self.draw_names(
            &players,
            &(padding_left + instrument_name_width),
            &padding_top,
            &vertical_spacing,
            engrave,
            &converter,
            &mut instructions,
        );

        let _ = setup.call2(
            &JsValue::NULL,
            &JsValue::from_f64(converter.spaces_to_px(&height) as f64),
            &JsValue::from_f64(converter.spaces_to_px(&width) as f64),
        );

        for instruction in instructions {
            let _ = render.call1(&JsValue::NULL, &instruction.to_jsvalue());
        }
    }
}

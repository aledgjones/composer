mod draw_names;
mod draw_staves;
mod get_flow_players;
mod measure_instrument_names;
mod measure_text;
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

        let converter = Converter::new(px_per_mm as f32, engrave.space.as_f32());

        let padding_top = converter.to_spaces(&engrave.frame_padding.0);
        let padding_bottom = converter.to_spaces(&engrave.frame_padding.2);
        let padding_left = converter.to_spaces(&engrave.frame_padding.3);
        let padding_right = converter.to_spaces(&engrave.frame_padding.1);

        let instrument_name_gap = engrave.instrument_name.padding.1;

        let (flow, players, instruments, staves) = self.get_flow_players(flow_key);

        let vertical_spacing = self.measure_vertical_spacing(&instruments, &flow.staves, engrave);
        let instrument_name_width =
            self.measure_instrument_names(&players, engrave, &converter, measure);

        let content_width = Unit::Space(20.0);

        let width = &padding_left
            + &instrument_name_width
            + &instrument_name_gap
            + &content_width
            + &padding_right;
        let height = &padding_top + &vertical_spacing.height + &padding_bottom;

        self.draw_staves(
            &staves,
            &(&padding_left + &instrument_name_width + &instrument_name_gap),
            &padding_top,
            &content_width,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        self.draw_names(
            &players,
            &(&padding_left + &instrument_name_width),
            &padding_top,
            &vertical_spacing,
            engrave,
            &converter,
            &mut instructions,
        );

        let _ = setup.call2(
            &JsValue::NULL,
            &converter.to_px(&height).as_jsvalue(),
            &converter.to_px(&width).as_jsvalue(),
        );

        for instruction in instructions {
            let _ = render.call1(&JsValue::NULL, &instruction.to_jsvalue());
        }
    }
}

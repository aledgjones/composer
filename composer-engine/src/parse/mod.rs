pub mod draw_braces;
pub mod draw_brackets;
pub mod draw_names;
pub mod draw_staves;
pub mod draw_sub_brackets;
pub mod draw_systemic_barline;
pub mod get_barlines;
pub mod get_vertical_spans;
pub mod get_written_durations;
pub mod measure_brackets_and_braces;
pub mod measure_instrument_names;
pub mod measure_vertical_spacing;

use crate::components::measurements::Point;
use crate::components::units::{Converter, Space};
use crate::score::engrave::LayoutType;
use crate::utils::log;
use crate::Engine;
use draw_braces::draw_braces;
use draw_brackets::draw_brackets;
use draw_names::draw_names;
use draw_staves::draw_staves;
use draw_sub_brackets::draw_sub_brackets;
use draw_systemic_barline::draw_systemic_barline;
use get_barlines::get_barlines;
use get_vertical_spans::get_vertical_spans;
use get_written_durations::get_written_durations;
use js_sys::Function;
use measure_brackets_and_braces::measure_brackets;
use measure_instrument_names::measure_instrument_names;
use measure_vertical_spacing::measure_vertical_spacing;
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

#[wasm_bindgen]
impl Engine {
    pub fn render(&self, flow_key: &str, px_per_mm: usize, measure: &Function) -> JsValue {
        let mut instructions: Vec<Instruction> = Vec::new();

        let engrave = self.get_engrave_by_type(LayoutType::Score).unwrap();
        let converter = Converter::new(px_per_mm as f32, engrave.space);

        let padding_top: Space = converter.mm_to_spaces(&engrave.frame_padding.top);
        let padding_bottom: Space = converter.mm_to_spaces(&engrave.frame_padding.bottom);
        let padding_left: Space = converter.mm_to_spaces(&engrave.frame_padding.left);
        let padding_right: Space = converter.mm_to_spaces(&engrave.frame_padding.right);
        let instrument_name_gap: Space = engrave.instrument_name.padding.right;

        let (flow, instruments, staves, tracks) = self.get_flow_instruments(flow_key);
        let flow_master = self.score.tracks.get(&flow.master).unwrap();

        let vertical_spans = get_vertical_spans(&instruments, engrave);
        let vertical_spacing = measure_vertical_spacing(&instruments, &flow.staves, engrave);
        let name_widths = measure_instrument_names(&instruments, engrave, &converter, measure);
        let bracket_widths = measure_brackets(&vertical_spacing, &vertical_spans, engrave);
        let barlines = get_barlines(flow.length, flow_master);
        let notation_by_track = get_written_durations(flow.length, &tracks, &barlines);

        // log(&format!("{:#?}", notation_by_track));

        let content_width: Space = 40.0;

        let width: Space = padding_left
            + name_widths
            + instrument_name_gap
            + bracket_widths
            + content_width
            + padding_right;
        let height: Space = padding_top + vertical_spacing.height + padding_bottom;

        draw_staves(
            &staves,
            &(padding_left + name_widths + instrument_name_gap + bracket_widths),
            &padding_top,
            &content_width,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        draw_names(
            &instruments,
            &(padding_left + name_widths),
            &padding_top,
            &vertical_spacing,
            engrave,
            &converter,
            &mut instructions,
        );
        draw_braces(
            &(padding_left + name_widths + instrument_name_gap + bracket_widths),
            &padding_top,
            &vertical_spans,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        draw_brackets(
            &(padding_left + name_widths + instrument_name_gap + bracket_widths),
            &padding_top,
            &vertical_spans,
            &vertical_spacing,
            engrave,
            &converter,
            &mut instructions,
        );
        draw_sub_brackets(
            &(padding_left + name_widths + instrument_name_gap + bracket_widths),
            &padding_top,
            &vertical_spans,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        draw_systemic_barline(
            &(padding_left + name_widths + instrument_name_gap + bracket_widths),
            &padding_top,
            &staves,
            &vertical_spacing,
            &converter,
            engrave,
            &mut instructions,
        );

        let width = converter.spaces_to_px(&width);
        let height = converter.spaces_to_px(&height);

        JsValue::from_serde(&(width, height, instructions)).unwrap()
    }
}

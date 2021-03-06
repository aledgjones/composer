mod draw_accidentals;
mod draw_barlines;
mod draw_beams;
mod draw_braces;
mod draw_brackets;
mod draw_clefs;
mod draw_dots;
mod draw_flags;
mod draw_key_signatures;
mod draw_ledger_lines;
mod draw_names;
mod draw_noteheads;
mod draw_rests;
mod draw_staves;
mod draw_stems;
mod draw_sub_brackets;
mod draw_systemic_barline;
mod draw_ties;
mod draw_time_signatures;
mod get_accidentals;
mod get_barlines;
pub mod get_bars;
pub mod get_beams;
mod get_dots;
mod get_shunts;
mod get_stem_directions;
mod get_stem_lengths;
mod get_tone_offsets;
mod get_vertical_spans;
pub mod get_written_durations;
mod measure_brackets_and_braces;
mod measure_horizontal_spacing;
mod measure_instrument_names;
mod measure_vertical_spacing;

use crate::components::measurements::{CurvePoint, Point};
use crate::components::units::{Converter, Space};
use crate::score::engrave::LayoutType;
use crate::Engine;
use draw_accidentals::draw_accidentals;
use draw_barlines::draw_barlines;
use draw_beams::draw_beams;
use draw_braces::draw_braces;
use draw_brackets::draw_brackets;
use draw_clefs::draw_clefs;
use draw_dots::draw_dots;
use draw_flags::draw_flags;
use draw_key_signatures::draw_key_signatures;
use draw_ledger_lines::draw_ledger_lines;
use draw_names::draw_names;
use draw_noteheads::draw_noteheads;
use draw_rests::draw_rests;
use draw_staves::draw_staves;
use draw_stems::draw_stems;
use draw_sub_brackets::draw_sub_brackets;
use draw_systemic_barline::draw_systemic_barline;
use draw_ties::draw_ties;
use draw_time_signatures::draw_time_signatures;
use get_accidentals::get_accidentals;
use get_barlines::get_barlines;
use get_bars::get_bars;
use get_beams::get_beams;
use get_dots::get_dots;
use get_shunts::get_note_shunts;
use get_stem_directions::get_stem_directions;
use get_stem_lengths::get_stem_lengths;
use get_tone_offsets::get_tone_offsets;
use get_vertical_spans::get_vertical_spans;
use get_written_durations::get_written_durations;
use js_sys::Function;
use measure_brackets_and_braces::measure_brackets;
use measure_horizontal_spacing::measure_horizontal_spacing;
use measure_instrument_names::measure_instrument_names;
use measure_vertical_spacing::measure_vertical_spacing;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Instruction {
    Circle {
        color: String,
        radius: f32,
        point: Point,
    },
    Line {
        color: String,
        width: f32,
        points: Vec<Point>,
    },
    Text {
        x: f32,
        y: f32,
        value: String,
        color: String,
        font: String,
        size: f32,
        justify: String,
        align: String,
    },
    Shape {
        color: String,
        points: Vec<Point>,
    },
    Curve {
        color: String,
        points: [CurvePoint; 3],
    },
}

#[wasm_bindgen]
impl Engine {
    pub fn render(&self, flow_key: &str, px_per_mm: usize, measure: &Function) -> JsValue {
        let mut instructions: Vec<Instruction> = Vec::new();

        let engrave = self.get_engrave_by_type(LayoutType::Score).unwrap();
        let converter = Converter::new(px_per_mm as f32, engrave.space);

        let padding_top: Space = converter.mm_to_spaces(engrave.frame_padding.top);
        let padding_bottom: Space = converter.mm_to_spaces(engrave.frame_padding.bottom);
        let padding_left: Space = converter.mm_to_spaces(engrave.frame_padding.left);
        let padding_right: Space = converter.mm_to_spaces(engrave.frame_padding.right);
        let instrument_name_gap: Space = engrave.instrument_name.padding.right;

        let (flow, instruments, staves, tracks) = self.get_flow_instruments(flow_key);

        let vertical_spans = get_vertical_spans(&instruments, engrave);
        let vertical_spacing = measure_vertical_spacing(&instruments, &flow.staves, engrave);
        let name_widths = measure_instrument_names(&instruments, engrave, &converter, measure);
        let bracket_widths = measure_brackets(&vertical_spacing, &vertical_spans, engrave);
        let bars = get_bars(flow, &self.score.tracks);
        let tone_offsets = get_tone_offsets(flow.length, &staves, &self.score.tracks);
        let barlines = get_barlines(flow, &self.score.tracks);

        let notations = get_written_durations(flow, &tracks, &bars);

        let beams = get_beams(&notations, &bars, flow.subdivisions);
        let stem_directions = get_stem_directions(&notations, &tone_offsets, &beams);
        let shunts = get_note_shunts(&notations, &tone_offsets, &stem_directions);
        let dots = get_dots(flow, &notations, &tone_offsets);
        let accidentals =
            get_accidentals(flow, &self.score.tracks, &notations, &bars, &tone_offsets);

        let horizontal_spacing = measure_horizontal_spacing(
            flow,
            &staves,
            &self.score.tracks,
            &barlines,
            &notations,
            &shunts,
            &beams,
            &accidentals,
            engrave,
        );

        let stem_lengths = get_stem_lengths(
            &notations,
            &tone_offsets,
            &horizontal_spacing,
            &stem_directions,
            &beams,
            engrave,
        );

        let width: Space = padding_left
            + name_widths
            + instrument_name_gap
            + bracket_widths
            + horizontal_spacing.width
            + padding_right;
        let height: Space = padding_top + vertical_spacing.height + padding_bottom;

        draw_names(
            &instruments,
            padding_left + name_widths,
            padding_top,
            &vertical_spacing,
            engrave,
            &converter,
            &mut instructions,
        );
        draw_braces(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &vertical_spans,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        draw_brackets(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &vertical_spans,
            &vertical_spacing,
            engrave,
            &converter,
            &mut instructions,
        );
        draw_sub_brackets(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &vertical_spans,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        draw_systemic_barline(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &vertical_spacing,
            &converter,
            engrave,
            &mut instructions,
        );
        draw_staves(
            &staves,
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            horizontal_spacing.width,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        draw_barlines(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &barlines,
            &staves,
            &vertical_spacing,
            &vertical_spans,
            &horizontal_spacing,
            &converter,
            &mut instructions,
        );

        draw_key_signatures(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            flow,
            &staves,
            &self.score.tracks,
            &vertical_spacing,
            &horizontal_spacing,
            &converter,
            &mut instructions,
        );
        draw_time_signatures(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            flow,
            &staves,
            &self.score.tracks,
            &vertical_spacing,
            &horizontal_spacing,
            &converter,
            &mut instructions,
        );
        draw_clefs(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &self.score.tracks,
            &vertical_spacing,
            &horizontal_spacing,
            &converter,
            &mut instructions,
        );

        draw_rests(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            flow,
            &staves,
            &notations,
            &horizontal_spacing,
            &vertical_spacing,
            &bars,
            &converter,
            &mut instructions,
        );

        draw_accidentals(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &notations,
            &horizontal_spacing,
            &vertical_spacing,
            &tone_offsets,
            &shunts,
            &accidentals,
            &converter,
            &mut instructions,
        );
        draw_ledger_lines(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &notations,
            &horizontal_spacing,
            &vertical_spacing,
            &tone_offsets,
            &shunts,
            &converter,
            &mut instructions,
        );
        draw_noteheads(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            flow,
            &staves,
            &notations,
            &horizontal_spacing,
            &vertical_spacing,
            &tone_offsets,
            &shunts,
            &converter,
            &mut instructions,
        );
        draw_dots(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &notations,
            &vertical_spacing,
            &horizontal_spacing,
            &dots,
            &shunts,
            &converter,
            &mut instructions,
        );
        draw_stems(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &vertical_spacing,
            &stem_lengths,
            &converter,
            &mut instructions,
        );
        draw_flags(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            flow,
            &staves,
            &notations,
            &stem_directions,
            &stem_lengths,
            &beams,
            &vertical_spacing,
            &converter,
            &mut instructions,
        );
        draw_beams(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &vertical_spacing,
            &stem_lengths,
            &beams,
            &converter,
            &mut instructions,
        );
        draw_ties(
            padding_left + name_widths + instrument_name_gap + bracket_widths,
            padding_top,
            &staves,
            &notations,
            &stem_directions,
            &vertical_spacing,
            &horizontal_spacing,
            &shunts,
            &tone_offsets,
            &converter,
            &mut instructions,
        );

        let width = converter.spaces_to_px(width);
        let height = converter.spaces_to_px(height);

        JsValue::from_serde(&(width, height, instructions)).unwrap()
    }
}

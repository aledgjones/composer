use super::measure_horizontal_spacing::{HorizontalSpacing, Position};
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::text::{Align, Justify};
use crate::components::units::Converter;
use crate::entries::clef::Clef;
use crate::score::flows::Flow;
use crate::score::stave::Stave;
use crate::score::tracks::Tracks;

fn draw_clef(
    x: f32,
    y: f32,
    clef: &Clef,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    if let Some(glyph) = clef.glyph() {
        instructions.push(Instruction::Text {
            x: converter.spaces_to_px(&x),
            y: converter.spaces_to_px(&(y + (0.5 * clef.offset as f32))),
            value: glyph,
            color: String::from("#000"),
            font: String::from("Bravura"),
            size: converter.spaces_to_px(&4.0),
            justify: Justify::Start.as_string(),
            align: Align::Middle.as_string(),
        })
    }
}

pub fn draw_clefs(
    x: &f32,
    y: &f32,
    flow: &Flow,
    staves: &Vec<&Stave>,
    tracks: &Tracks,
    vertical_spacing: &VerticalSpacing,
    horizontal_spacing: &HorizontalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let stave_master = tracks.get(&stave.master).unwrap();
        let top = vertical_spacing.staves.get(&stave.key).unwrap();

        for tick in 0..flow.length {
            if let Some(clef) = stave_master.get_clef_at_tick(&tick) {
                let left = horizontal_spacing.get(&tick, &Position::Clef).unwrap();
                draw_clef(x + left.x, y + top.y, &clef, converter, instructions)
            }
        }
    }
}

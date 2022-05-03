use std::cmp::Ordering;

use super::get_note_positions::Position;
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Instruction, Text};
use crate::components::text::{Align, Justify};
use crate::components::units::Converter;
use crate::entries::clef::{Clef, ClefDrawType};
use crate::entries::key_signature::KeySignature;
use crate::score::flows::Flow;
use crate::score::stave::Stave;
use crate::score::tracks::Tracks;
use crate::utils::log;

fn draw_key_signature(
    x: f32,
    y: f32,
    clef: &Clef,
    key_signature: &KeySignature,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let glyph = key_signature.glyph();
    if let Some(pattern) = key_signature.pattern(clef) {
        for i in 0..key_signature.offset.abs() {
            instructions.push(Instruction::Text(Text {
                x: converter.spaces_to_px(&(x + (i as f32))),
                y: converter.spaces_to_px(&(y + 0.5 * pattern[i as usize] as f32)),
                value: glyph.clone(),
                color: String::from("#000"),
                font: String::from("Bravura"),
                size: converter.spaces_to_px(&4.0),
                justify: Justify::Start.as_string(),
                align: Align::Middle.as_string(),
            }))
        }
    }
}

pub fn draw_key_signatures(
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
    let flow_master = tracks.get(&flow.master).unwrap();

    for stave in staves {
        let stave_master = tracks.get(&stave.master).unwrap();
        let mut clef = Clef::new(0, 60, 0, ClefDrawType::C);

        for tick in 0..flow.length {
            let key_signature = flow_master.get_key_signature_at_tick(&tick);
            if let Some(found) = stave_master.get_clef_at_tick(&tick) {
                clef = found;
            };

            if let Some(key) = key_signature {
                let top = vertical_spacing.staves.get(&stave.key).unwrap();
                let left = horizontal_spacing
                    .get(&(tick, Position::KeySignature))
                    .unwrap();

                draw_key_signature(x + left.x, y + top.y, &clef, &key, converter, instructions)
            }
        }
    }
}

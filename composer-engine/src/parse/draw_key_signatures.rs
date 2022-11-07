use super::measure_horizontal_spacing::{HorizontalSpacing, Position};
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::pitch::Accidental;
use crate::components::text::{Align, Justify};
use crate::components::units::{Converter, Space};
use crate::entries::clef::{Clef, ClefDrawType};
use crate::entries::key_signature::KeySignature;
use crate::score::flows::Flow;
use crate::score::stave::Stave;
use crate::score::tracks::Tracks;

fn draw_key_signature(
    x: Space,
    y: Space,
    clef: &Clef,
    key_signature: &KeySignature,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
    negate: bool,
) {
    let glyph = if negate {
        Accidental::Natural.to_glyph()
    } else {
        key_signature.glyph()
    };
    if let Some(pattern) = key_signature.pattern(clef) {
        for i in 0..key_signature.offset.abs() {
            instructions.push(Instruction::Text {
                x: converter.spaces_to_px(x + (i as f32)),
                y: converter.spaces_to_px(y + 0.5 * pattern[i as usize] as f32),
                value: glyph.clone(),
                color: String::from("#000"),
                font: String::from("Bravura"),
                size: converter.spaces_to_px(4.0),
                justify: Justify::Start.as_string(),
                align: Align::Middle.as_string(),
            })
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_key_signatures(
    x: Space,
    y: Space,
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
        let top = vertical_spacing.staves.get(&stave.key).unwrap();
        let mut clef = &Clef::new(0, 60, 0, ClefDrawType::C);

        for tick in 0..flow.length {
            if let Some(found) = stave_master.get_clef_at_tick(&tick) {
                clef = found;
            };

            if let Some(key_signature) = flow_master.get_key_signature_at_tick(&tick) {
                let is_offset_zero = key_signature.offset == 0;
                let left = horizontal_spacing
                    .get(&tick, &Position::KeySignature)
                    .unwrap();

                let key_signature = if is_offset_zero {
                    flow_master.get_key_signature_before_tick(tick)
                } else {
                    Some(key_signature)
                };

                if let Some(key_signature) = key_signature {
                    draw_key_signature(
                        x + left.x,
                        y + top.y,
                        clef,
                        key_signature,
                        converter,
                        instructions,
                        is_offset_zero,
                    )
                }
            }
        }
    }
}

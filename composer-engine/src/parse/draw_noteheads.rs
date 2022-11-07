use super::get_shunts::{Shunt, Shunts, ShuntsByTrack};
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use super::measure_horizontal_spacing::{HorizontalSpacing, Position};
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::misc::Tick;
use crate::components::text::{Align, Justify};
use crate::components::units::{Converter, Space};
use crate::entries::tone::Tone;
use crate::score::flows::Flow;
use crate::score::stave::Stave;

#[allow(clippy::too_many_arguments)]
fn draw_notehead(
    tick: Tick,
    x: Space,
    y: Space,
    flow: &Flow,
    entry: &Notation,
    tone: &Tone,
    horizontal_spacing: &HorizontalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    shunts: &Shunts,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let mut left = x + horizontal_spacing
        .get(&tick, &Position::NoteSpacing)
        .unwrap()
        .x;
    match shunts.by_key.get(&(tick, tone.key.clone())).unwrap() {
        Shunt::Pre => {
            left -= entry.notehead_width();
        }
        Shunt::None => (),
        Shunt::Post => {
            left += entry.notehead_width();
        }
    }

    let glyph = entry.glyph(flow.subdivisions);
    let offset = tone_offsets.get(&tone.key).unwrap();
    let top = y + (*offset as f32 / 2.0);

    instructions.push(Instruction::Text {
        x: converter.spaces_to_px(left),
        y: converter.spaces_to_px(top),
        value: glyph,
        color: String::from("#000"),
        font: String::from("Bravura"),
        size: converter.spaces_to_px(4.0),
        justify: Justify::Start.as_string(),
        align: Align::Middle.as_string(),
    });
}

#[allow(clippy::too_many_arguments)]
pub fn draw_noteheads(
    x: Space,
    y: Space,
    flow: &Flow,
    staves: &[&Stave],
    notation_by_track: &NotationByTrack,
    horizontal_spacing: &HorizontalSpacing,
    vertical_spacing: &VerticalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    shunts_by_track: &ShuntsByTrack,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let notation = notation_by_track.get(track_key).unwrap();
            let shunts = shunts_by_track.get(track_key).unwrap();

            for (tick, entry) in &notation.track {
                for tone in &entry.tones {
                    draw_notehead(
                        *tick,
                        x,
                        top,
                        flow,
                        entry,
                        tone,
                        horizontal_spacing,
                        tone_offsets,
                        shunts,
                        converter,
                        instructions,
                    );
                }
            }
        }
    }
}

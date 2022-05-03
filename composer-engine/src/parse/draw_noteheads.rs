use super::get_note_positions::TonePositions;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Instruction, Text};
use crate::components::misc::Tick;
use crate::components::text::{Align, Justify};
use crate::components::units::Converter;
use crate::entries::tone::Tone;
use crate::score::flows::Flow;
use crate::score::stave::Stave;

fn draw_notehead(
    tick: Tick,
    x: &f32,
    y: &f32,
    flow: &Flow,
    entry: &Notation,
    tone: &Tone,
    horizontal_spacing: &HorizontalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    tone_positions: &TonePositions,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let position = tone_positions.get(&(tick, tone.key.clone())).unwrap();
    let horizontal_offset = horizontal_spacing.get(&(tick, position.clone())).unwrap();
    let left = x + horizontal_offset.x;
    let glyph = entry.glyph(flow.subdivisions);
    let offset = tone_offsets.get(&tone.key).unwrap();
    let top = y + (*offset as f32 / 2.0);

    instructions.push(Instruction::Text(Text {
        x: converter.spaces_to_px(&left),
        y: converter.spaces_to_px(&top),
        value: glyph,
        color: String::from("#000"),
        font: String::from("Bravura"),
        size: converter.spaces_to_px(&4.0),
        justify: Justify::Start.as_string(),
        align: Align::Middle.as_string(),
    }));
}

pub fn draw_noteheads(
    x: &f32,
    y: &f32,
    flow: &Flow,
    staves: &[&Stave],
    notation_by_track: &NotationByTrack,
    horizontal_spacing: &HorizontalSpacing,
    vertical_spacing: &VerticalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    tone_positions: &TonePositions,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let notation = notation_by_track.get(track_key).unwrap();

            for (tick, entry) in &notation.track {
                for tone in &entry.tones {
                    draw_notehead(
                        *tick,
                        x,
                        &top,
                        flow,
                        entry,
                        tone,
                        horizontal_spacing,
                        tone_offsets,
                        tone_positions,
                        converter,
                        instructions,
                    );
                }
            }
        }
    }
}

use super::get_accidentals::{AccidentalNotation, Accidentals};
use super::get_note_positions::Position;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::NotationByTrack;
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Instruction, Text};
use crate::components::misc::Tick;
use crate::components::text::{Align, Justify};
use crate::components::units::Converter;
use crate::entries::tone::Tone;
use crate::score::stave::Stave;

fn draw_accidental(
    tick: &Tick,
    x: &f32,
    y: &f32,
    accidental: &AccidentalNotation,
    tone: &Tone,
    horizontal_spacing: &HorizontalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let horizontal_offset = horizontal_spacing
        .get(tick, &Position::PreNoteSlot)
        .unwrap();
    let left = x + horizontal_offset.x - 0.2 - (accidental.slot as f32 * 1.1);
    let glyph = tone.pitch.accidental.to_glyph();
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

pub fn draw_accidentals(
    x: &f32,
    y: &f32,
    staves: &[&Stave],
    notation_by_track: &NotationByTrack,
    horizontal_spacing: &HorizontalSpacing,
    vertical_spacing: &VerticalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    accidentals: &Accidentals,
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
                    if let Some(accidental) = accidentals.by_key.get(&(*tick, tone.key.clone())) {
                        draw_accidental(
                            tick,
                            x,
                            &top,
                            accidental,
                            tone,
                            horizontal_spacing,
                            tone_offsets,
                            converter,
                            instructions,
                        );
                    }
                }
            }
        }
    }
}

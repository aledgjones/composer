use super::get_note_positions::Position;
use super::get_note_positions::TonePositions;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use super::Line;
use crate::components::measurements::Point;
use crate::components::misc::Tick;
use crate::components::units::Converter;
use crate::score::stave::Stave;
use crate::score::stave::STAVE_LINE_WIDTH;

fn draw_lines<T: Iterator<Item = i8>>(
    range: T,
    tick: &Tick,
    x: &f32,
    y: &f32,
    horizontal_spacing: &HorizontalSpacing,
    tone_positions: &TonePositions,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let mut start_slot = Position::NoteSlot;
    let mut stop_slot = Position::NoteSlot;

    for offset in range {
        // get the furthest start position
        if let Some(position) = tone_positions.by_offset.get(&(*tick, offset)) {
            if position > &stop_slot {
                stop_slot = position.clone();
            }
            if position < &start_slot {
                start_slot = position.clone();
            }
        };

        if offset % 2 == 0 {
            let y = y + (offset as f32 / 2.0);

            let start_slot = horizontal_spacing.get(tick, &start_slot).unwrap();
            let stop_slot = horizontal_spacing.get(tick, &stop_slot).unwrap();
            let start = x + start_slot.x - 0.4;
            let stop = x + stop_slot.x + stop_slot.width + 0.4;

            instructions.push(Instruction::Line(Line {
                color: String::from("#000"),
                width: converter.spaces_to_px(&(STAVE_LINE_WIDTH * 1.5)),
                points: vec![
                    Point {
                        x: converter.spaces_to_px(&start),
                        y: converter.spaces_to_px(&y),
                    },
                    Point {
                        x: converter.spaces_to_px(&stop),
                        y: converter.spaces_to_px(&y),
                    },
                ],
            }));
        }
    }
}

fn draw_ledger_line(
    tick: &Tick,
    x: &f32,
    y: &f32,
    entry: &Notation,
    horizontal_spacing: &HorizontalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    tone_positions: &TonePositions,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let (highest, lowest, _) = entry.get_tone_offset_info(tone_offsets);

    let from = if highest > -5 { -5 } else { highest };
    let to = if lowest < 5 { 5 } else { lowest + 1 };

    // high ledger lines
    draw_lines(
        from..-5,
        tick,
        x,
        y,
        horizontal_spacing,
        tone_positions,
        converter,
        instructions,
    );

    // low ledger lines
    draw_lines(
        (5..to).rev(),
        tick,
        x,
        y,
        horizontal_spacing,
        tone_positions,
        converter,
        instructions,
    );
}

pub fn draw_ledger_lines(
    x: &f32,
    y: &f32,
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
                if !entry.is_rest() {
                    draw_ledger_line(
                        tick,
                        x,
                        &top,
                        entry,
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

use super::get_note_positions::{Position, TonePositions};
use super::get_stem_directions::{StemDirection, StemDirectionsByTrack};
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Instruction, Line};
use crate::components::measurements::Point;
use crate::components::misc::Tick;
use crate::components::units::Converter;
use crate::score::stave::{Stave, STAVE_LINE_WIDTH};

fn draw_ledger_line(
    tick: &Tick,
    x: &f32,
    y: &f32,
    entry: &Notation,
    horizontal_spacing: &HorizontalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    tone_positions: &TonePositions,
    stem_direction: &StemDirection,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let (highest, lowest, _) = entry.get_tone_offset_info(tone_offsets);

    let from = if highest > 0 { 0 } else { highest };
    let to = if lowest < 0 { 0 } else { lowest };

    for offset in from..to {
        if offset % 2 == 0 && (offset < -5 || offset > 5) {
            let y = y + (offset as f32 / 2.0);

            let slot = horizontal_spacing.get(tick, &Position::NoteSlot).unwrap();
            let start = x + slot.x - 0.4;
            let stop = x + slot.x + slot.width + 0.4;

            instructions.push(Instruction::Line(Line {
                color: String::from("#000"),
                width: converter.spaces_to_px(&(STAVE_LINE_WIDTH * 1.5)),
                points: vec![
                    Point(converter.spaces_to_px(&start), converter.spaces_to_px(&y)),
                    Point(converter.spaces_to_px(&stop), converter.spaces_to_px(&y)),
                ],
            }));
        }
    }
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
    stem_directions_by_track: &StemDirectionsByTrack,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let notation = notation_by_track.get(track_key).unwrap();
            let stem_directions = stem_directions_by_track.get(track_key).unwrap();

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
                        stem_directions.get(tick).unwrap(),
                        converter,
                        instructions,
                    );
                }
            }
        }
    }
}

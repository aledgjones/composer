use super::get_bars::Bars;
use super::get_written_durations::Notation;
use super::get_written_durations::NotationByTrack;
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_horizontal_spacing::Position;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Instruction, Text};
use crate::components::duration::NoteDuration;
use crate::components::misc::{Tick, Ticks};
use crate::components::text::{Align, Justify};
use crate::components::units::Converter;
use crate::score::flows::Flow;
use crate::score::stave::Stave;

fn draw_rest(
    x: &f32,
    y: &f32,
    tick: &Tick,
    entry: &Notation,
    subdivisions: &Ticks,
    is_full_bar: bool,
    horizontal_spacing: &HorizontalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let start = horizontal_spacing.get(tick, &Position::NoteSlot).unwrap();

    if is_full_bar {
        let end = horizontal_spacing
            .get(&(tick + entry.duration - 1), &Position::PaddingEnd)
            .unwrap();
        let left = (x + start.x + ((end.x - start.x) / 2.0)) - 1.0;

        instructions.push(Instruction::Text(Text {
            x: converter.spaces_to_px(&left),
            y: converter.spaces_to_px(&(y - 1.0)),
            value: String::from("\u{E4E3}"),
            color: String::from("#000"),
            font: String::from("Bravura"),
            size: converter.spaces_to_px(&4.0),
            justify: Justify::Start.as_string(),
            align: Align::Middle.as_string(),
        }));
    } else {
        let left = x + start.x;
        let base = entry.base_to_note_duration(subdivisions);
        let glyph = entry.glyph(subdivisions);
        let is_dotted = entry.is_dotted(subdivisions);
        let top = match base {
            Some(NoteDuration::Whole) => y - 1.0,
            _ => *y,
        };

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

        if is_dotted {
            todo!()
        }
    }
}

pub fn draw_rests(
    x: &f32,
    y: &f32,
    flow: &Flow,
    staves: &[&Stave],
    notation_by_track: &NotationByTrack,
    horizontal_spacing: &HorizontalSpacing,
    vertical_spacing: &VerticalSpacing,
    bars: &Bars,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let notation = notation_by_track.get(track_key).unwrap();

            for (tick, entry) in &notation.track {
                if entry.is_rest() {
                    let is_full_bar = bars.contains_key(tick)
                        && (bars.contains_key(&(tick + entry.duration))
                            || tick + entry.duration == flow.length);

                    draw_rest(
                        x,
                        &top,
                        tick,
                        entry,
                        &flow.subdivisions,
                        is_full_bar,
                        horizontal_spacing,
                        converter,
                        instructions,
                    );
                }
            }
        }
    }
}

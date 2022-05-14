use super::get_beams::BeamsByTrack;
use super::get_stem_directions::StemDirectionsByTrack;
use super::get_stem_lengths::StemLengthsByTrack;
use super::get_written_durations::NotationByTrack;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Instruction, Text};
use crate::components::text::{Align, Justify};
use crate::components::units::Converter;
use crate::score::flows::Flow;
use crate::score::stave::{Stave, STAVE_LINE_WIDTH};

pub fn draw_flags(
    x: &f32,
    y: &f32,
    flow: &Flow,
    staves: &[&Stave],
    notation_by_track: &NotationByTrack,
    stem_directions_by_track: &StemDirectionsByTrack,
    stem_lengths_by_track: &StemLengthsByTrack,
    beams_by_track: &BeamsByTrack,
    vertical_spacing: &VerticalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let notation = notation_by_track.get(track_key).unwrap();
            let stem_directions = stem_directions_by_track.get(track_key).unwrap();
            let stem_lengths = stem_lengths_by_track.get(track_key).unwrap();
            let beams = beams_by_track.get(track_key).unwrap();

            for (tick, entry) in &notation.track {
                if entry.is_flagged(beams, &flow.subdivisions) {
                    let stem_direction = stem_directions.get(tick).unwrap();
                    let glyph = entry.flag_glyph(stem_direction, &flow.subdivisions);

                    // TODO extend stem by amount of 'beams'
                    // let stem_length_modifier = stem_direction.to_modifier();

                    let tail = &stem_lengths.get(tick).unwrap().tail;
                    let left = x + tail.x - (STAVE_LINE_WIDTH / 2.0);
                    let top = top + tail.y;

                    instructions.push(Instruction::Text(Text {
                        x: converter.spaces_to_px(&left),
                        y: converter.spaces_to_px(&top),
                        value: glyph,
                        color: String::from("#000"),
                        font: String::from("Bravura"),
                        size: converter.spaces_to_px(&4.0),
                        justify: Justify::Start.as_string(),
                        align: Align::Middle.as_string(),
                    }))
                }
            }
        }
    }
}

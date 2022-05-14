use super::get_beams::BeamsByTrack;
use super::get_stem_lengths::{StemDef, StemLengthsByTrack};
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::score::stave::{Stave, STAVE_LINE_WIDTH};

fn draw_beam(
    x: &f32,
    y: &f32,
    start: &StemDef,
    stop: &StemDef,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let stem_tweek = STAVE_LINE_WIDTH / 2.0;

    instructions.push(Instruction::Shape {
        color: String::from("#000"),
        points: vec![
            Point {
                x: converter.spaces_to_px(&(x + start.tail.x - stem_tweek)),
                y: converter.spaces_to_px(&(y + start.tail.y)),
            },
            Point {
                x: converter.spaces_to_px(&(x + stop.tail.x + stem_tweek)),
                y: converter.spaces_to_px(&(y + stop.tail.y)),
            },
            Point {
                x: converter.spaces_to_px(&(x + stop.tail.x + stem_tweek)),
                y: converter.spaces_to_px(&(y + stop.tail.y + 0.5)),
            },
            Point {
                x: converter.spaces_to_px(&(x + start.tail.x - stem_tweek)),
                y: converter.spaces_to_px(&(y + start.tail.y + 0.5)),
            },
        ],
    });
}

pub fn draw_beams(
    x: &f32,
    y: &f32,
    staves: &[&Stave],
    vertical_spacing: &VerticalSpacing,
    stem_lengths_by_track: &StemLengthsByTrack,
    beams_by_track: &BeamsByTrack,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let stem_lengths = stem_lengths_by_track.get(track_key).unwrap();
            let beams = beams_by_track.get(track_key).unwrap();
            for beam in beams {
                let start = stem_lengths.get(&beam.start).unwrap();
                let stop = stem_lengths.get(&beam.stop).unwrap();

                draw_beam(x, &top, start, stop, converter, instructions);
            }
        }
    }
}

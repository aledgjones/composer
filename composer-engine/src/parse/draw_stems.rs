use super::get_stem_lengths::{StemDef, StemLengthsByTrack};
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::measurements::Point;
use crate::components::units::{Converter, Space};
use crate::score::stave::{Stave, STAVE_LINE_WIDTH};

fn draw_stem(
    x: Space,
    y: Space,
    def: &StemDef,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    instructions.push(Instruction::Line {
        color: String::from("#000"),
        width: converter.spaces_to_px(STAVE_LINE_WIDTH),
        points: vec![
            Point {
                x: converter.spaces_to_px(x + def.head.x),
                y: converter.spaces_to_px(y + def.head.y),
            },
            Point {
                x: converter.spaces_to_px(x + def.tail.x),
                y: converter.spaces_to_px(y + def.tail.y),
            },
        ],
    });
}

pub fn draw_stems(
    x: Space,
    y: Space,
    staves: &[&Stave],
    vertical_spacing: &VerticalSpacing,
    stem_lengths_by_track: &StemLengthsByTrack,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let lengths = stem_lengths_by_track.get(track_key).unwrap();
            for def in lengths.values() {
                draw_stem(x, top, def, converter, instructions);
            }
        }
    }
}

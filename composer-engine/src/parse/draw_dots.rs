use super::get_dots::DotsByTrack;
use super::get_shunts::ShuntsByTrack;
use super::get_written_durations::NotationByTrack;
use super::measure_horizontal_spacing::{HorizontalSpacing, Position};
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::measurements::Point;
use crate::components::units::{Converter, Space};
use crate::score::stave::Stave;

pub fn draw_dots(
    x: Space,
    y: Space,
    staves: &[&Stave],
    notations_by_track: &NotationByTrack,
    vertical_spacing: &VerticalSpacing,
    horizontal_spacing: &HorizontalSpacing,
    dots_by_track: &DotsByTrack,
    shunts_by_track: &ShuntsByTrack,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let notations = notations_by_track.get(track_key).unwrap();
            let dots = dots_by_track.get(track_key).unwrap();
            let shunts = shunts_by_track.get(track_key).unwrap();

            for (tick, offset) in dots {
                let entry = notations.track.get(tick).unwrap();
                let mut left = horizontal_spacing
                    .get(tick, &Position::NoteSpacing)
                    .unwrap()
                    .x;
                left += entry.notehead_width();
                if entry.has_post_shunt(shunts) {
                    left += entry.notehead_width();
                };

                instructions.push(Instruction::Circle {
                    color: String::from("#000"),
                    radius: converter.spaces_to_px(0.2),
                    point: Point {
                        x: converter.spaces_to_px(x + left + 0.5),
                        y: converter.spaces_to_px(top + (*offset as f32 / 2.0)),
                    },
                });
            }
        }
    }
}

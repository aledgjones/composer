use super::get_dots::DotsByTrack;
use super::get_note_positions::Position;
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Circle, Instruction};
use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::score::stave::Stave;

pub fn draw_dots(
    x: &f32,
    y: &f32,
    staves: &[&Stave],
    vertical_spacing: &VerticalSpacing,
    horizontal_spacing: &HorizontalSpacing,
    dots_by_track: &DotsByTrack,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let offset = vertical_spacing.staves.get(&stave.key).unwrap();
        let top = y + offset.y;

        for track_key in &stave.tracks {
            let dots = dots_by_track.get(track_key).unwrap();
            for (tick, offset) in dots {
                let position = horizontal_spacing.get(tick, &Position::Dot).unwrap().x;
                instructions.push(Instruction::Circle(Circle {
                    color: String::from("#000"),
                    radius: converter.spaces_to_px(&0.2),
                    point: Point {
                        x: converter.spaces_to_px(&(x + position + 0.75)),
                        y: converter.spaces_to_px(&(top + (*offset as f32 / 2.0))),
                    },
                }));
            }
        }
    }
}

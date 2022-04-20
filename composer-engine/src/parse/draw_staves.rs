use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use super::Line;
use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::components::units::Space;
use crate::score::stave::Stave;
use crate::Engine;

impl Engine {
    pub fn draw_staves(
        &self,
        staves: &[&Stave],
        x: &Space,
        y: &Space,
        width: &Space,
        vertical_spacing: &VerticalSpacing,
        converter: &Converter,
        instructions: &mut Vec<Instruction>,
    ) {
        for stave in staves {
            for (i, line) in stave.lines.iter().enumerate() {
                if line == &1 {
                    let top = y + vertical_spacing.staves[&stave.key].y
                        - (vertical_spacing.staves[&stave.key].height / 2.0)
                        + i as Space;

                    instructions.push(Instruction::Line(Line {
                        color: String::from("#000"),
                        width: converter.spaces_to_px(&0.125),
                        points: vec![
                            Point(converter.spaces_to_px(&x), converter.spaces_to_px(&top)),
                            Point(
                                converter.spaces_to_px(&(x + width)),
                                converter.spaces_to_px(&top),
                            ),
                        ],
                    }));
                }
            }
        }
    }
}

use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use super::Line;
use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::components::units::Unit;
use crate::score::stave::Stave;
use crate::Engine;

impl Engine {
    pub fn draw_staves(
        &self,
        staves: &Vec<&Stave>,
        x: Unit,
        y: Unit,
        width: Unit,
        vertical_spacing: &VerticalSpacing,
        converter: &Converter,
        instructions: &mut Vec<Instruction>,
    ) {
        for stave in staves {
            for (i, line) in stave.lines.iter().enumerate() {
                if line == &1 {
                    let top = y
                        + (vertical_spacing.staves[&stave.key].y
                            - vertical_spacing.staves[&stave.key].height / 2)
                        + Unit::Space(i as f32);

                    instructions.push(Instruction::Line(Line {
                        color: String::from("#000"),
                        width: converter.to_px(&Unit::Space(0.125)).to_f32(),
                        points: vec![
                            Point(converter.to_px(&x).to_f32(), converter.to_px(&top).to_f32()),
                            Point(
                                converter.to_px(&(x + width)).to_f32(),
                                converter.to_px(&top).to_f32(),
                            ),
                        ],
                    }));
                }
            }
        }
    }
}

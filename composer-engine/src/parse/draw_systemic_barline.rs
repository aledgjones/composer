use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::components::units::Space;
use crate::score::engrave::Engrave;
use crate::score::stave::Stave;
use crate::score::stave::STAVE_LINE_WIDTH;

pub fn draw_systemic_barline(
    x: &Space,
    y: &Space,
    staves: &[&Stave],
    vertical_spacing: &VerticalSpacing,
    converter: &Converter,
    engrave: &Engrave,
    instructions: &mut Vec<Instruction>,
) {
    if staves.len() > 1 || engrave.systemic_barline_single_instrument_system {
        let tweak_for_stave_line_width = STAVE_LINE_WIDTH / 2.0;
        instructions.push(Instruction::Line {
            color: String::from("#000"),
            width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
            points: vec![
                Point {
                    x: converter.spaces_to_px(x),
                    y: converter.spaces_to_px(&(y - tweak_for_stave_line_width)),
                },
                Point {
                    x: converter.spaces_to_px(x),
                    y: converter
                        .spaces_to_px(&(y + vertical_spacing.height + tweak_for_stave_line_width)),
                },
            ],
        });
    }
}

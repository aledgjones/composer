use super::get_vertical_spans::VerticalSpans;
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use super::Line;
use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::components::units::Space;
use crate::score::stave::STAVE_LINE_WIDTH;

pub fn draw_sub_brackets(
    x: &Space,
    y: &Space,
    vertical_spans: &VerticalSpans,
    vertical_spacing: &VerticalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for bracket in &vertical_spans.sub_brackets {
        let top = vertical_spacing.instruments.get(&bracket.start).unwrap();
        let bottom = vertical_spacing.instruments.get(&bracket.stop).unwrap();

        let top = y + (top.y);
        let bottom = y + bottom.y + bottom.height;

        instructions.push(Instruction::Line(Line {
            color: String::from("#000"),
            width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
            points: vec![
                Point {
                    x: converter.spaces_to_px(x),
                    y: converter.spaces_to_px(&top),
                },
                Point {
                    x: converter.spaces_to_px(&(x - 1.5)),
                    y: converter.spaces_to_px(&top),
                },
                Point {
                    x: converter.spaces_to_px(&(x - 1.5)),
                    y: converter.spaces_to_px(&bottom),
                },
                Point {
                    x: converter.spaces_to_px(x),
                    y: converter.spaces_to_px(&bottom),
                },
            ],
        }));
    }
}

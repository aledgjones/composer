use super::get_vertical_spans::VerticalSpans;
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::text::Align;
use crate::components::text::Justify;
use crate::components::units::Converter;
use crate::components::units::Space;

pub fn draw_braces(
    x: &Space,
    y: &Space,
    vertical_spans: &VerticalSpans,
    vertical_spacing: &VerticalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for brace in &vertical_spans.braces {
        let top = vertical_spacing.staves.get(&brace.start).unwrap();
        let bottom = vertical_spacing.staves.get(&brace.stop).unwrap();
        let height = (bottom.y + bottom.height) - top.y;
        let top = y + (bottom.y + bottom.height / 2.0);

        instructions.push(Instruction::Text {
            x: converter.spaces_to_px(&(x - 0.25)),
            y: converter.spaces_to_px(&top),
            value: String::from("\u{E000}"),
            color: String::from("#000"),
            font: String::from("Bravura"),
            size: converter.spaces_to_px(&height),
            justify: Justify::End.as_string(),
            align: Align::Middle.as_string(),
        })
    }
}

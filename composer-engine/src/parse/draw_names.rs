use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::units::Converter;
use crate::components::units::Space;
use crate::score::engrave::Engrave;
use crate::score::instruments::Instrument;

pub fn draw_names(
    instruments: &[&Instrument],
    x: Space,
    y: Space,
    vertical_spacing: &VerticalSpacing,
    engrave: &Engrave,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for instrument in instruments {
        let spacing = vertical_spacing.instruments.get(&instrument.key).unwrap();
        let top: Space = y + spacing.y + (spacing.height / 2.0);

        instructions.push(Instruction::Text {
            x: converter.spaces_to_px(x),
            y: converter.spaces_to_px(top),
            value: instrument.name(),
            color: String::from("#000"),
            font: engrave.instrument_name.font.clone(),
            size: converter.spaces_to_px(engrave.instrument_name.size),
            justify: engrave.instrument_name.justify.as_string(),
            align: engrave.instrument_name.align.as_string(),
        });
    }
}

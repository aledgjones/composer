use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use super::Text;
use crate::components::units::Converter;
use crate::components::units::Space;
use crate::score::engrave::Engrave;
use crate::score::players::Player;
use crate::Engine;

impl Engine {
    pub fn draw_names(
        &self,
        players: &[&Player],
        x: &Space,
        y: &Space,
        vertical_spacing: &VerticalSpacing,
        engrave: &Engrave,
        converter: &Converter,
        instructions: &mut Vec<Instruction>,
    ) {
        for player in players {
            for instrument_key in &player.instruments {
                let spacing = vertical_spacing.instruments.get(instrument_key).unwrap();
                let top: Space = y + spacing.y + (spacing.height / 2.0);

                instructions.push(Instruction::Text(Text {
                    x: converter.spaces_to_px(x),
                    y: converter.spaces_to_px(&top),
                    value: self.get_instrument_name(&player.key, instrument_key),
                    color: String::from("#000"),
                    font: engrave.instrument_name.font.clone(),
                    size: converter.spaces_to_px(&engrave.instrument_name.size),
                    justify: engrave.instrument_name.justify.as_string(),
                    align: engrave.instrument_name.align.as_string(),
                }));
            }
        }
    }
}

use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use super::Text;
use crate::components::units::Converter;
use crate::components::units::Unit;
use crate::score::engrave::Engrave;
use crate::score::players::Player;
use crate::Engine;

impl Engine {
    pub fn draw_names(
        &self,
        players: &[&Player],
        x: &Unit,
        y: &Unit,
        vertical_spacing: &VerticalSpacing,
        engrave: &Engrave,
        converter: &Converter,
        instructions: &mut Vec<Instruction>,
    ) {
        for player in players {
            for instrument_key in &player.instruments {
                let spacing = vertical_spacing.instruments.get(instrument_key).unwrap();
                let top = y + &spacing.y + &(spacing.height / 2);

                instructions.push(Instruction::Text(Text {
                    x: converter.to_px(&x).as_f32(),
                    y: converter.to_px(&top).as_f32(),
                    value: self.get_instrument_name(&player.key, instrument_key),
                    color: String::from("#000"),
                    font: engrave.instrument_name.font.clone(),
                    size: converter.to_px(&engrave.instrument_name.size).as_f32(),
                    justify: engrave.instrument_name.justify.as_string(),
                    align: engrave.instrument_name.align.as_string(),
                }));
            }
        }
    }
}

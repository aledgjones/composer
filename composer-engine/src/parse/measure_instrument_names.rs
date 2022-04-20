use crate::components::units::{Converter, Unit};
use crate::score::engrave::Engrave;
use crate::score::players::Player;
use crate::Engine;
use js_sys::Function;

impl Engine {
    pub fn measure_instrument_names(
        &self,
        players: &[&Player],
        engrave: &Engrave,
        converter: &Converter,
        measure: &Function,
    ) -> Unit {
        let mut max: f32 = 0.0;

        for player in players {
            for instrument_key in &player.instruments {
                let text = self.get_instrument_name(&player.key, instrument_key);
                let width = self.measure_text(
                    measure,
                    &text,
                    &engrave.instrument_name.size,
                    &engrave.instrument_name.font,
                    converter,
                );
                if width > max {
                    max = width;
                }
            }
        }

        converter.to_spaces(&Unit::Px(max))
    }
}

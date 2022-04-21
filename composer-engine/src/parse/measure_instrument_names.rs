use crate::components::text::measure_text;
use crate::components::units::{Converter, Px, Space};
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
    ) -> Space {
        let mut max: Px = 0.0;

        for player in players {
            for instrument_key in &player.instruments {
                let text = self.get_instrument_name(&player.key, instrument_key);
                let width = measure_text(
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

        converter.px_to_spaces(&max)
    }
}

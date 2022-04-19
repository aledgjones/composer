use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::components::units::{Converter, Unit};
use crate::score::engrave::Engrave;
use crate::score::players::Player;
use crate::Engine;

impl Engine {
    pub fn measure_instrument_names(
        &self,
        players: &Vec<&Player>,
        engrave: &Engrave,
        converter: &Converter,
        measure: &Function,
    ) -> Unit {
        let mut max: f32 = 0.0;

        for player in players {
            for instrument_key in &player.instruments {
                let name = self.get_instrument_name(&player.key, instrument_key);
                let size = converter.to_px(&engrave.instrument_name.size).to_f32();
                let font = &engrave.instrument_name.font;
                let result = measure
                    .call3(
                        &JsValue::NULL,
                        &JsValue::from_str(&name),
                        &JsValue::from_f64(size as f64),
                        &JsValue::from_str(font),
                    )
                    .unwrap()
                    .as_f64()
                    .unwrap() as f32;
                if result > max {
                    max = result;
                }
            }
        }

        converter.to_spaces(&Unit::Px(max))
    }
}

use std::collections::HashMap;

use crate::components::text::measure_text;
use crate::components::units::{Converter, Px, Space};
use crate::score::engrave::Engrave;
use crate::score::instruments::Instrument;
use js_sys::Function;

pub fn measure_instrument_names(
    instruments: &[&Instrument],
    engrave: &Engrave,
    converter: &Converter,
    measure: &Function,
) -> Space {
    let mut max: Px = 0.0;

    for instrument in instruments {
        let text = instrument.name();
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

    converter.px_to_spaces(&max)
}

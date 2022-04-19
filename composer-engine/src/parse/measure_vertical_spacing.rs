use crate::components::units::Unit;
use crate::score::engrave::Engrave;
use crate::score::instruments::Instrument;
use crate::score::stave::Stave;
use crate::Engine;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VerticalPosition {
    pub y: Unit,
    pub height: Unit,
}

#[derive(Debug)]
pub struct VerticalSpacing {
    pub height: Unit,
    pub instruments: HashMap<String, VerticalPosition>,
    pub staves: HashMap<String, VerticalPosition>,
}

impl VerticalSpacing {
    fn new() -> Self {
        Self {
            height: Unit::Space(0.0),
            instruments: HashMap::new(),
            staves: HashMap::new(),
        }
    }
}

impl Engine {
    pub fn measure_vertical_spacing(
        &self,
        instruments: &Vec<&Instrument>,
        staves: &HashMap<String, Stave>,
        engrave: &Engrave,
    ) -> VerticalSpacing {
        let mut output = VerticalSpacing::new();

        for (i, instrument) in instruments.iter().enumerate() {
            if i > 0 {
                output.height = output.height + engrave.instrument_spacing;
            }

            let mut instrument_entry = VerticalPosition {
                y: output.height,
                height: Unit::Space(0.0),
            };

            for (ii, stave_key) in instrument.staves.iter().enumerate() {
                let stave = staves.get(stave_key).unwrap();

                if 1 > 0 && ii > 0 {
                    output.height = output.height + engrave.stave_spacing;
                    instrument_entry.height = instrument_entry.height + engrave.stave_spacing;
                }

                let stave_entry = VerticalPosition {
                    y: output.height + Unit::Space(((stave.lines.len() - 1) / 2) as f32),
                    height: Unit::Space((stave.lines.len() - 1) as f32),
                };

                output.height = output.height + stave_entry.height;
                instrument_entry.height = instrument_entry.height + stave_entry.height;

                output.staves.insert(stave_key.clone(), stave_entry);
            }

            output
                .instruments
                .insert(instrument.key.clone(), instrument_entry);
        }

        output
    }
}

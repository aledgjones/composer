use crate::components::measurements::Padding;
use crate::components::text::{Font, Justify};
use crate::components::units::Unit;
use crate::utils::shortid;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Bracketing {
    None,
    Orchestral,
    SmallEnsemble,
}

#[derive(Debug)]
pub enum BracketStyle {
    None,
    Wing,
    Line,
}

#[derive(PartialEq, Debug)]
pub enum LayoutType {
    Score,
    Part,
    Custom,
}

#[derive(Debug)]
pub struct Engraves {
    pub by_key: HashMap<String, Engrave>,
}

impl Engraves {
    pub fn new() -> Self {
        let mut engraves = Engraves {
            by_key: HashMap::new(),
        };

        let score = Engrave::new(LayoutType::Score, String::from("Score"));
        engraves.by_key.insert(score.key.clone(), score);
        let part = Engrave::new(LayoutType::Part, String::from("Part"));
        engraves.by_key.insert(part.key.clone(), part);

        engraves
    }

    pub fn get_engrave_by_type(&self, layout_type: LayoutType) -> Option<&Engrave> {
        let result = self
            .by_key
            .iter()
            .find(|(_, val)| val.layout_type == layout_type);

        match result {
            Some((_, config)) => Some(config),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct Engrave {
    pub key: String,
    pub layout_type: LayoutType,
    pub display_name: String,

    pub space: Unit,

    pub frame_padding: Padding,
    pub instrument_spacing: Unit,
    pub stave_spacing: Unit,
    pub system_start_padding: Unit,

    pub instrument_name: Font,
    pub tempo_text: Font,

    pub systemic_barline_single_instrument_system: bool,
    pub bracketing: Bracketing,
    pub bracket_style: BracketStyle,
    pub bracket_single_staves: bool,
    pub sub_bracket: bool,

    pub minimum_note_spacing: Unit,
}

impl Engrave {
    pub fn new(layout_type: LayoutType, display_name: String) -> Engrave {
        Engrave {
            key: shortid(),
            layout_type,
            display_name,

            space: Unit::Mm(2.0),

            frame_padding: Padding(
                Unit::Mm(35.0),
                Unit::Mm(25.0),
                Unit::Mm(35.0),
                Unit::Mm(25.0),
            ),
            instrument_spacing: Unit::Space(8.0),
            stave_spacing: Unit::Space(6.0),
            system_start_padding: Unit::Space(0.75),

            instrument_name: Font {
                size: Unit::Space(1.75),
                font: String::from("Libre Baskerville"),
                align: Justify::End,
                padding: Padding(
                    Unit::Space(0.0),
                    Unit::Space(2.0),
                    Unit::Space(0.0),
                    Unit::Space(0.0),
                ),
            },
            tempo_text: Font {
                size: Unit::Space(1.75),
                font: String::from("Libre Baskerville"),
                align: Justify::Start,
                padding: Padding(
                    Unit::Space(0.0),
                    Unit::Space(0.0),
                    Unit::Space(2.0),
                    Unit::Space(0.0),
                ),
            },

            systemic_barline_single_instrument_system: false,
            bracketing: Bracketing::Orchestral,
            bracket_style: BracketStyle::Wing,
            bracket_single_staves: false,
            sub_bracket: true,

            minimum_note_spacing: Unit::Space(1.6),
        }
    }
}

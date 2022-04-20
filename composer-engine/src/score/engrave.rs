use crate::components::measurements::{PaddingMm, PaddingSpaces};
use crate::components::text::{Align, Font, Justify};
use crate::components::units::{Mm, Space};
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

    pub space: Mm,

    pub frame_padding: PaddingMm,
    pub instrument_spacing: Space,
    pub stave_spacing: Space,
    pub system_start_padding: Space,

    pub instrument_name: Font,
    pub tempo_text: Font,

    pub systemic_barline_single_instrument_system: bool,
    pub bracketing: Bracketing,
    pub bracket_style: BracketStyle,
    pub bracket_single_staves: bool,
    pub sub_bracket: bool,

    pub minimum_note_spacing: Space,
}

impl Engrave {
    pub fn new(layout_type: LayoutType, display_name: String) -> Engrave {
        Engrave {
            key: shortid(),
            layout_type,
            display_name,

            space: 2.0,

            frame_padding: PaddingMm::new(35.0, 25.0, 35.0, 25.0),
            instrument_spacing: 8.0,
            stave_spacing: 6.0,
            system_start_padding: 0.75,

            instrument_name: Font {
                size: 1.75,
                font: String::from("Libre Baskerville"),
                justify: Justify::End,
                align: Align::Middle,
                padding: PaddingSpaces::new(0.0, 2.0, 0.0, 0.0),
            },
            tempo_text: Font {
                size: 1.75,
                font: String::from("Libre Baskerville"),
                justify: Justify::Start,
                align: Align::Middle,
                padding: PaddingSpaces::new(0.0, 0.0, 2.0, 0.0),
            },

            systemic_barline_single_instrument_system: false,
            bracketing: Bracketing::Orchestral,
            bracket_style: BracketStyle::Wing,
            bracket_single_staves: false,
            sub_bracket: true,

            minimum_note_spacing: 1.6,
        }
    }
}

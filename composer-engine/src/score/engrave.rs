use crate::components::measurements::{PaddingMm, PaddingSpaces};
use crate::components::text::{Align, Font, Justify};
use crate::components::units::{Mm, Space};
use crate::utils::shortid;
use crate::Engine;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum BracketingApproach {
    None,
    Orchestral,
    SmallEnsemble,
}

#[derive(Debug, Serialize)]
#[wasm_bindgen]
pub enum BracketStyle {
    None,
    Wing,
    Line,
}

#[derive(PartialEq, Debug, Serialize)]
#[wasm_bindgen]
pub enum LayoutType {
    Score,
    Part,
    Custom,
}

#[derive(Debug)]
pub struct Engraves {
    pub order: Vec<String>,
    pub by_key: HashMap<String, Engrave>,
}

impl Engraves {
    pub fn new() -> Self {
        Self {
            order: Vec::new(),
            by_key: HashMap::new(),
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
    pub bracketing_approach: BracketingApproach,
    pub bracket_style: BracketStyle,
    pub bracket_single_staves: bool,
    pub sub_bracket: bool,

    pub base_note_space: Space,
    pub minimum_note_space: Space,
    pub minimum_tie_space: Space,
    pub note_space_ratio: f32,
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
            bracketing_approach: BracketingApproach::Orchestral,
            bracket_style: BracketStyle::Wing,
            bracket_single_staves: false,
            sub_bracket: true,

            base_note_space: 2.75,
            minimum_note_space: 0.6,
            minimum_tie_space: 2.0,
            note_space_ratio: 1.41,
        }
    }
}

impl Engine {
    pub fn get_engrave_by_type(&self, layout_type: LayoutType) -> Option<&Engrave> {
        let result = self
            .score
            .engrave
            .by_key
            .iter()
            .find(|(_, val)| val.layout_type == layout_type);

        match result {
            Some((_, config)) => Some(config),
            None => None,
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn create_engrave(&mut self, layout_type: LayoutType, name: &str) {
        let config = Engrave::new(layout_type, String::from(name));
        self.score.engrave.order.push(config.key.clone());
        self.score.engrave.by_key.insert(config.key.clone(), config);

        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn engraves(&self) -> JsValue {
        JsValue::from_serde(&self.score.engrave.order).unwrap()
    }

    pub fn get_systemic_barline_single_instrument_system(&self, key: &str) -> bool {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.systemic_barline_single_instrument_system
    }

    pub fn set_systemic_barline_single_instrument_system(&mut self, key: &str, value: bool) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.systemic_barline_single_instrument_system = value;

        self.emit();
    }

    pub fn get_bracketing_approach(&self, key: &str) -> BracketingApproach {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.bracketing_approach.clone()
    }

    pub fn set_bracketing_approach(&mut self, key: &str, value: BracketingApproach) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.bracketing_approach = value;

        self.emit();
    }

    pub fn get_bracke_single_staves(&self, key: &str) -> bool {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.bracket_single_staves
    }

    pub fn set_bracket_single_staves(&mut self, key: &str, value: bool) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.bracket_single_staves = value;

        self.emit();
    }

    pub fn get_sub_bracket(&self, key: &str) -> bool {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.sub_bracket
    }

    pub fn set_sub_bracket(&mut self, key: &str, value: bool) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.sub_bracket = value;

        self.emit();
    }

    pub fn get_base_note_space(&self, key: &str) -> f32 {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.base_note_space
    }

    pub fn set_base_note_space(&mut self, key: &str, value: f32) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.base_note_space = value;

        self.emit();
    }

    pub fn get_minimum_note_space(&self, key: &str) -> f32 {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.minimum_note_space
    }

    pub fn set_minimum_note_space(&mut self, key: &str, value: f32) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.minimum_note_space = value;

        self.emit();
    }

    pub fn get_minimum_tie_space(&self, key: &str) -> f32 {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.minimum_tie_space
    }

    pub fn set_minimum_tie_space(&mut self, key: &str, value: f32) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.minimum_tie_space = value;

        self.emit();
    }

    pub fn get_note_space_ratio(&self, key: &str) -> f32 {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.note_space_ratio
    }

    pub fn set_note_space_ratio(&mut self, key: &str, value: f32) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.note_space_ratio = value;

        self.emit();
    }

    pub fn get_space(&self, key: &str) -> f32 {
        let config = self.score.engrave.by_key.get(key).unwrap();
        config.space
    }

    pub fn set_space(&mut self, key: &str, value: f32) {
        let config = self.score.engrave.by_key.get_mut(key).unwrap();
        config.space = value;

        self.emit();
    }
}

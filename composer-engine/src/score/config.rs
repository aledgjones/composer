use wasm_bindgen::prelude::*;

use crate::Engine;

#[derive(Debug)]
#[wasm_bindgen]
pub enum AutoCountStyle {
    Arabic,
    Roman,
}

#[derive(Debug)]
pub struct AutoCount {
    pub solo: AutoCountStyle,
    pub section: AutoCountStyle,
}

impl AutoCount {
    pub fn new() -> AutoCount {
        AutoCount {
            solo: AutoCountStyle::Roman,
            section: AutoCountStyle::Roman,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub auto_count: AutoCount,
}

impl Config {
    pub fn new() -> Config {
        Config {
            auto_count: AutoCount::new(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    pub fn set_auto_count_style_solo(&mut self, value: AutoCountStyle) {
        self.score.config.auto_count.solo = value;

        self.calculate_counts();
        self.modify();
        self.emit();
    }
    pub fn set_auto_count_style_section(&mut self, value: AutoCountStyle) {
        self.score.config.auto_count.section = value;

        self.calculate_counts();
        self.modify();
        self.emit();
    }
}

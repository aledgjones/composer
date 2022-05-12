use crate::Engine;
use js_sys::Date;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    application_version: String,
    title: String,
    subtitle: String,
    composer: String,
    arranger: String,
    lyricist: String,
    copyright: String,
    created: f64,
}

impl Meta {
    pub fn new() -> Self {
        Meta {
            application_version: String::from("1.0.0"),
            title: String::from(""),
            subtitle: String::from(""),
            composer: String::from(""),
            arranger: String::from(""),
            lyricist: String::from(""),
            copyright: String::from(""),
            created: Date::now(),
        }
    }
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(getter)]
    pub fn application_version(&self) -> String {
        self.score.meta.application_version.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_application_version(&mut self, value: String) {
        self.score.meta.application_version = value;
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.score.meta.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: String) {
        self.score.meta.title = value;
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn subtitle(&self) -> String {
        self.score.meta.subtitle.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_subtitle(&mut self, value: String) {
        self.score.meta.subtitle = value;
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn composer(&self) -> String {
        self.score.meta.composer.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_composer(&mut self, value: String) {
        self.score.meta.composer = value;
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn arranger(&self) -> String {
        self.score.meta.arranger.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_arranger(&mut self, value: String) {
        self.score.meta.arranger = value;
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn lyricist(&self) -> String {
        self.score.meta.lyricist.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_lyricist(&mut self, value: String) {
        self.score.meta.lyricist = value;
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn copyright(&self) -> String {
        self.score.meta.copyright.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_copyright(&mut self, value: String) {
        self.score.meta.copyright = value;
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn created(&self) -> f64 {
        self.score.meta.created
    }

    #[wasm_bindgen(setter)]
    pub fn set_created(&mut self, value: f64) {
        self.score.meta.created = value;
        self.emit();
    }
}

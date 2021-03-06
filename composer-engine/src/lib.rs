pub mod components;
pub mod entries;
pub mod parse;
pub mod score;
pub mod utils;

use crate::utils::log;
use js_sys::Function;
use score::engrave::LayoutType;
use score::Score;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate lazy_static;

#[wasm_bindgen]
pub struct Engine {
    listener: Option<Function>,
    score: Score,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut engine = Self {
            listener: None,
            score: Score::new(),
        };

        let key = engine.create_flow();
        engine.create_engrave(LayoutType::Score, "Score");
        engine.create_engrave(LayoutType::Part, "Part");

        log(&key);

        engine
    }

    pub fn listen(&mut self, cb: Function) {
        self.listener = Some(cb);
        self.emit();
    }

    pub fn export(&self) -> JsValue {
        JsValue::from_serde(&self.score).unwrap()
    }

    pub fn import(&mut self, state: JsValue) {
        self.score = state.into_serde().unwrap();
        self.emit();
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> String {
        format!("{:#?}", self.score)
    }
}

impl Engine {
    fn emit(&self) {
        match &self.listener {
            Some(listener) => {
                let this = JsValue::NULL;
                let _ = listener.call0(&this);
            }
            None => (),
        };
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();
}

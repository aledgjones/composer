mod score;
mod utils;

use js_sys::{Date, Function};
use score::Score;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Engine {
    listener: Option<Function>,
    score: Score,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Engine {
            listener: None,
            score: Score::new(),
        }
    }

    pub fn listen(&mut self, cb: Function) {
        self.listener = Some(cb);
        self.emit();
    }
}

impl Engine {
    fn modify(&mut self) {
        self.score.meta.modified = Date::now();
    }
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

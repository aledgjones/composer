use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::components::units::{Converter, Unit};
use crate::Engine;

impl Engine {
    pub fn measure_text(
        &self,
        measure: &Function,
        text: &str,
        size: &Unit,
        font: &str,
        converter: &Converter,
    ) -> f32 {
        let result = measure
            .call3(
                &JsValue::NULL,
                &JsValue::from_str(text),
                &converter.to_px(size).as_jsvalue(),
                &JsValue::from_str(font),
            )
            .unwrap();
        result.as_f64().unwrap() as f32
    }
}

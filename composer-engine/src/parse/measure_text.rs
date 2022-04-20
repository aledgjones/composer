use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::components::units::{Converter, Px, Space};
use crate::Engine;

impl Engine {
    pub fn measure_text(
        &self,
        measure: &Function,
        text: &str,
        size: &Space,
        font: &str,
        converter: &Converter,
    ) -> Px {
        let result = measure
            .call3(
                &JsValue::NULL,
                &JsValue::from_str(text),
                &JsValue::from_f64(converter.spaces_to_px(size) as f64),
                &JsValue::from_str(font),
            )
            .unwrap();
        result.as_f64().unwrap() as Px
    }
}

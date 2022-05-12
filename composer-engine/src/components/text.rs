use super::measurements::PaddingSpaces;
use super::units::{Converter, Px, Space};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize)]
pub enum Justify {
    Start,
    Middle,
    End,
}

impl Justify {
    pub fn as_string(&self) -> String {
        match self {
            Justify::Start => String::from("left"),
            Justify::Middle => String::from("center"),
            Justify::End => String::from("right"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Align {
    Top,
    Middle,
    Bottom,
}

impl Align {
    pub fn as_string(&self) -> String {
        match self {
            Align::Top => String::from("top"),
            Align::Middle => String::from("middle"),
            Align::Bottom => String::from("bottom"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Font {
    pub size: Space,
    pub font: String,
    pub justify: Justify,
    pub align: Align,
    pub padding: PaddingSpaces,
}

pub fn measure_text(
    measure: &Function,
    text: &str,
    size: &Space,
    font: &str,
    converter: &Converter,
) -> Px {
    let size = converter.spaces_to_px(size);
    let result = measure
        .call3(
            &JsValue::NULL,
            &JsValue::from_str(text),
            &JsValue::from_f64(size as f64),
            &JsValue::from_str(font),
        )
        .unwrap()
        .as_f64()
        .unwrap();

    result as f32
}

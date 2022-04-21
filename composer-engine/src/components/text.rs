use super::measurements::PaddingSpaces;
use super::units::{Converter, Px, Space};
use js_sys::Function;
use serde::Serialize;
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

#[derive(Debug)]
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
    let size = converter.spaces_to_px(size) as f64;
    let result = measure
        .call3(
            &JsValue::NULL,
            &JsValue::from_str(text),
            &JsValue::from_f64(size),
            &JsValue::from_str(font),
        )
        .unwrap();

    result.as_f64().unwrap() as f32
}

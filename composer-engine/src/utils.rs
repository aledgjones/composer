use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-";

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn shortid() -> String {
    let mut rng = rand::thread_rng();
    let mut output: String = String::with_capacity(12);
    for _ in 0..12 {
        let i = rng.gen_range(0..63);
        let char = ALPHABET.chars().nth(i).unwrap();
        output.push(char);
    }
    output
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

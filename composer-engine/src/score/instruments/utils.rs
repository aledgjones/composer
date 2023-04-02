use crate::score::instruments::defs::INSTRUMENT_DEFS;
use rustc_hash::FxHashSet;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct FullPathReturn<'a> {
    path: &'a Vec<&'a str>,
    id: &'a str,
}

/// Get a full path to def from partial path
#[wasm_bindgen]
pub fn get_full_path_from_partial(selection: JsValue) -> JsValue {
    let selection: Vec<String> = serde_wasm_bindgen::from_value(selection).unwrap();

    let def = INSTRUMENT_DEFS.iter().find(|&def| {
        for (i, step) in selection.iter().enumerate() {
            if *step != def.path[i] {
                return false; // we have a mismatched path -- this isn't what we're looking for
            }
        }
        true // even if we have a partial match only the first def we encounter is what we want
    });

    match def {
        Some(def) => serde_wasm_bindgen::to_value(&FullPathReturn {
            path: &def.path,
            id: def.id,
        })
        .unwrap(),
        None => JsValue::UNDEFINED,
    }
}

/// Get a tree of instruments from a (possibly incomplete) path
#[wasm_bindgen]
pub fn def_tree(selection: JsValue) -> JsValue {
    let selection: Vec<String> = serde_wasm_bindgen::from_value(selection).unwrap();

    let mut ignore: FxHashSet<&str> = FxHashSet::default();
    let mut tree: [Vec<&str>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for def in INSTRUMENT_DEFS.iter() {
        for (i, step) in def.path.iter().enumerate() {
            if !ignore.contains(def.id) {
                if !tree[i].contains(step) {
                    tree[i].push(step);
                }
                if *step != selection[i] {
                    ignore.insert(def.id);
                }
            }
        }
    }

    serde_wasm_bindgen::to_value(&tree).unwrap()
}

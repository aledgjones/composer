use crate::score::instruments::defs::INSTRUMENT_DEFS;
use serde::Serialize;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct FullPathReturn<'a> {
    path: &'a Vec<&'a str>,
    id: &'a str,
}

/// Get a full path to def from partial path
#[wasm_bindgen]
pub fn get_full_path_from_partial(selection: &JsValue) -> JsValue {
    let selection: Vec<String> = selection.into_serde().unwrap();

    let def = INSTRUMENT_DEFS.iter().find(|&def| {
        for (i, step) in selection.iter().enumerate() {
            if String::from(step) != def.path[i] {
                return false; // we have a mismatched path -- this isn't what we're looking for
            }
        }
        true // even if we have a partial match only the first def we encounter is what we want
    });

    match def {
        Some(def) => JsValue::from_serde(&FullPathReturn {
            path: &def.path,
            id: def.id,
        })
        .unwrap(),
        None => JsValue::UNDEFINED,
    }
}

/// Get a tree of instruments from a (possibly incomplete) path
#[wasm_bindgen]
pub fn def_tree(selection: &JsValue) -> JsValue {
    let selection: Vec<String> = selection.into_serde().unwrap();

    let mut ignore: HashSet<&str> = HashSet::new();
    let mut tree: [Vec<&str>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for def in INSTRUMENT_DEFS.iter() {
        for (i, step) in def.path.iter().enumerate() {
            if !ignore.contains(def.id) {
                if !tree[i].contains(step) {
                    tree[i].push(step);
                }
                if step.to_string() != selection[i] {
                    ignore.insert(def.id);
                }
            }
        }
    }

    JsValue::from_serde(&tree).unwrap()
}

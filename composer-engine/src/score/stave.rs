use super::instruments::defs::StaveDef;
use crate::utils::shortid;

#[derive(Debug)]
pub struct Stave {
    pub key: String,
    pub lines: Vec<u8>,
    pub master: String,
    pub tracks: Vec<String>,
}

impl Stave {
    pub fn new(key: String, stave_def: &StaveDef) -> Stave {
        Stave {
            key,
            lines: stave_def.lines.clone(),
            master: shortid(),
            tracks: Vec::new(),
        }
    }
}

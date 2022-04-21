use super::instruments::defs::StaveDef;
use super::Track;

#[derive(Debug)]
pub struct Stave {
    pub key: String,
    pub lines: Vec<u8>,
    pub master: String,
    pub tracks: Vec<String>,
}

impl Stave {
    pub fn new(key: String, stave_def: &StaveDef, master: &Track) -> Stave {
        Stave {
            key,
            lines: stave_def.lines.clone(),
            master: master.key.clone(),
            tracks: Vec::new(),
        }
    }
}

pub const STAVE_LINE_WIDTH: f32 = 0.125;

mod config;
mod instruments;
mod meta;
mod players;

use std::collections::HashMap;

use config::Config;
use instruments::Instrument;
use meta::Meta;
use players::Players;

#[derive(Debug)]
pub struct Score {
    pub meta: Meta,
    pub config: Config,
    pub players: Players,
    pub instruments: HashMap<String, Instrument>,
}

impl Score {
    pub fn new() -> Self {
        Score {
            meta: Meta::new(),
            config: Config::new(),
            players: Players::new(),
            instruments: HashMap::new(),
        }
    }
}

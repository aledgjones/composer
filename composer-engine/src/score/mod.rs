mod config;
pub mod engrave;
mod flows;
mod instruments;
mod meta;
mod players;
mod stave;
mod tracks;

use config::Config;
use engrave::Engraves;
use flows::Flows;
use instruments::Instrument;
use meta::Meta;
use players::Players;
use std::collections::HashMap;
use tracks::Track;

#[derive(Debug)]
pub struct Score {
    pub meta: Meta,
    pub config: Config,
    pub engrave: Engraves,
    pub players: Players,
    pub instruments: HashMap<String, Instrument>,
    pub flows: Flows,
    pub tracks: HashMap<String, Track>,
}

impl Score {
    pub fn new() -> Self {
        Score {
            meta: Meta::new(),
            config: Config::new(),
            engrave: Engraves::new(),
            players: Players::new(),
            instruments: HashMap::new(),
            flows: Flows::new(),
            tracks: HashMap::new(),
        }
    }
}

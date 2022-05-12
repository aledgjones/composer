mod config;
pub mod engrave;
pub mod flows;
pub mod instruments;
mod meta;
pub mod players;
pub mod stave;
pub mod tracks;

use config::Config;
use engrave::Engraves;
use flows::Flows;
use instruments::Instrument;
use meta::Meta;
use players::Players;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tracks::Track;

use self::tracks::Tracks;

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub meta: Meta,
    pub config: Config,
    pub engrave: Engraves,
    pub players: Players,
    pub instruments: FxHashMap<String, Instrument>,
    pub flows: Flows,
    pub tracks: Tracks,
}

impl Score {
    pub fn new() -> Self {
        Score {
            meta: Meta::new(),
            config: Config::new(),
            engrave: Engraves::new(),
            players: Players::new(),
            instruments: FxHashMap::default(),
            flows: Flows::new(),
            tracks: FxHashMap::default(),
        }
    }
}

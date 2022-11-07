use crate::components::misc::Tick;
use crate::entries::Entry;
use crate::utils::shortid;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

pub type Tracks = FxHashMap<String, Track>;
pub type EntriesByTick = FxHashMap<Tick, Vec<String>>;
pub type EntriesByKey = FxHashMap<String, Entry>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entries {
    pub by_tick: EntriesByTick,
    pub by_key: EntriesByKey, // we can iterate the hashmap directly, so no order/by_key needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub key: String,
    pub entries: Entries,
}

impl Track {
    pub fn new() -> Track {
        Track {
            key: shortid(),
            entries: Entries {
                by_tick: FxHashMap::default(),
                by_key: FxHashMap::default(),
            },
        }
    }

    /// Insert and entry into the track
    pub fn insert(&mut self, entry: Entry) {
        let tick = self.entries.by_tick.entry(entry.tick()).or_default();
        tick.push(entry.key());
        self.entries.by_key.insert(entry.key(), entry);
    }

    /// Move an entry to a new tick
    pub fn shift(&mut self, key: &str, new_tick: Tick) {
        let entry = match self.entries.by_key.get_mut(key) {
            Some(entry) => entry,
            None => return,
        };

        let old_tick = entry.tick();

        // move the entry tp the new tick only if it has actually moved
        if old_tick != new_tick {
            entry.set_tick(new_tick);
            // move the entry key to the new tick
            if let Some(keys) = self.entries.by_tick.get_mut(&old_tick) {
                keys.retain(|item| item != key);
            };
            let tick = self.entries.by_tick.entry(new_tick).or_default();
            tick.push(String::from(key));
        }
    }

    /// remove an entry and return the removed entry
    pub fn remove(&mut self, key: &str) -> Option<Entry> {
        let entry = match self.entries.by_key.get(key) {
            Some(entry) => entry,
            None => return None,
        };

        if let Some(keys) = self.entries.by_tick.get_mut(&entry.tick()) {
            keys.retain(|item| item != key);
        };

        self.entries.by_key.remove(key)
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

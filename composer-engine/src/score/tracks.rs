use crate::entries::Entry;
use crate::utils::shortid;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Entries {
    pub by_tick: HashMap<u32, Vec<String>>,
    pub by_key: HashMap<String, Entry>,
}

#[derive(Debug)]
pub struct Track {
    pub key: String,
    pub entries: Entries,
}

impl Track {
    pub fn new() -> Track {
        Track {
            key: shortid(),
            entries: Entries {
                by_tick: HashMap::new(),
                by_key: HashMap::new(),
            },
        }
    }

    /// Insert and entry into the track
    pub fn insert(&mut self, entry: Entry) {
        let tick = self
            .entries
            .by_tick
            .entry(entry.tick())
            .or_insert(Vec::new());
        tick.push(entry.key());
        self.entries.by_key.insert(entry.key(), entry);
    }

    /// Move an entry to a new tick
    pub fn _shift(&mut self, key: &str, new_tick: u32) {
        let entry = match self.entries.by_key.get_mut(key) {
            Some(entry) => entry,
            None => return (),
        };

        let old_tick = entry.tick();

        // move the entry tp the new tick only if it has actually moved
        if old_tick != new_tick {
            entry.set_tick(new_tick);
            // move the entry key to the new tick
            match self.entries.by_tick.get_mut(&old_tick) {
                Some(keys) => {
                    keys.retain(|item| item != key);
                }
                None => (),
            };
            let tick = self.entries.by_tick.entry(new_tick).or_insert(Vec::new());
            tick.push(String::from(key));
        }
    }

    /// remove an entry and return the removed entry
    pub fn _remove(&mut self, key: &str) -> Option<Entry> {
        let entry = match self.entries.by_key.get(key) {
            Some(entry) => entry,
            None => return None,
        };

        match self.entries.by_tick.get_mut(&entry.tick()) {
            Some(keys) => {
                keys.retain(|item| item != key);
            }
            None => (),
        };
        self.entries.by_key.remove(key)
    }
}

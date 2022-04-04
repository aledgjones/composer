pub mod clef;

use clef::Clef;

pub enum Entry {
    Clef(Clef),
}

impl Entry {
    /// Get the entries key without having to manually unwrap the entry
    pub fn key(&self) -> String {
        match self {
            Entry::Clef(clef) => clef.key.clone(),
        }
    }

    /// Get the entries tick without having to manually unwrap the entry
    pub fn tick(&self) -> u32 {
        match self {
            Entry::Clef(clef) => clef.tick,
        }
    }

    /// Updates the entries tick value
    ///
    /// This shouldn't be called directly. It will most likely be called via the
    /// Track interface.
    pub fn set_tick(&mut self, tick: u32) {
        match self {
            Entry::Clef(clef) => clef.tick = tick,
        }
    }
}

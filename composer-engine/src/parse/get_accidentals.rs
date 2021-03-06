use rustc_hash::{FxHashMap, FxHashSet};

use super::get_bars::Bars;
use super::get_written_durations::{Notation, NotationTrack};
use super::{get_tone_offsets::ToneVerticalOffsets, get_written_durations::NotationByTrack};
use crate::components::misc::Tick;
use crate::components::pitch::{Accidental, Pitch};
use crate::entries::key_signature::{KeySignature, KeySignatureMode};
use crate::entries::tone::Tone;
use crate::score::flows::Flow;
use crate::score::tracks::{Track, Tracks};

type SlotsByTick = FxHashMap<Tick, u8>;

#[derive(Debug)]
pub struct AccidentalNotation {
    pub tone_key: String,
    pub slot: u8,
}

#[derive(Debug)]
pub struct Accidentals {
    pub by_key: FxHashMap<(Tick, String), AccidentalNotation>,
    pub slots_by_tick: SlotsByTick,
}

impl Accidentals {
    pub fn new() -> Self {
        Self {
            by_key: FxHashMap::default(),
            slots_by_tick: FxHashMap::default(),
        }
    }
}

impl Default for Accidentals {
    fn default() -> Self {
        Self::new()
    }
}

pub type AccidentalsByTrack = FxHashMap<String, Accidentals>;

pub fn fits_in_slot(
    i: u8,
    accidental_offset: &i8,
    slots: &[AccidentalNotation],
    tone_offsets: &ToneVerticalOffsets,
) -> bool {
    for entry in slots {
        if entry.slot == i {
            let entry_offset = tone_offsets.get(&entry.tone_key).unwrap();
            let diff = (entry_offset - accidental_offset).abs();
            if diff < 6 {
                return false;
            }
        };
    }
    true
}

pub fn find_slot(
    tone_key: &str,
    slots: &[AccidentalNotation],
    tone_offsets: &ToneVerticalOffsets,
) -> u8 {
    let accidental_offset = tone_offsets.get(tone_key).unwrap();

    let mut i: u8 = 1;
    loop {
        if fits_in_slot(i, accidental_offset, slots, tone_offsets) {
            return i;
        } else {
            i += 1;
        }
    }
}

pub fn sort_accidentals_alternate(accidentals: &[String]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let mut i = 0;
    let mut j = accidentals.len() - 1;
    while i < j {
        output.push(accidentals.get(j).unwrap().to_string());
        output.push(accidentals.get(i).unwrap().to_string());
        j -= 1;
        i += 1;
    }

    if accidentals.len() % 2 != 0 {
        output.push(accidentals.get(i).unwrap().to_string());
    }

    output
}

pub fn is_accidental_needed(
    tone: &Tone,
    previous_tones: &FxHashSet<String>,
    altered_pitches: &FxHashSet<Pitch>,
    key_signature: &KeySignature,
) -> bool {
    // ignore tied notes (the tones would have already been seen)
    if previous_tones.contains(&tone.key) {
        return false;
    }

    // altered accidentals
    for pitch in altered_pitches {
        // octave aware
        if tone.pitch.letter() == pitch.letter() && tone.pitch.octave() == pitch.octave() {
            return tone.pitch.accidental != pitch.accidental;
        }
    }

    // key accidentals
    for pitch in key_signature.accidentals() {
        // not octave aware
        if tone.pitch.letter() == pitch.letter() {
            return tone.pitch.accidental != pitch.accidental;
        }
    }

    // finally if it's just an accidental not covered by the above two cases
    if tone.pitch.accidental != Accidental::Natural {
        return true;
    }

    false
}

/// returns the *unordered* tones needing accidentals
pub fn get_tones_needing_accidentals(
    entry: &Notation,
    previous_tones: &mut FxHashSet<String>,
    altered_pitches: &mut FxHashSet<Pitch>,
    key_signature: &KeySignature,
    tone_offsets: &ToneVerticalOffsets,
) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for tone in &entry.sort_tones(tone_offsets) {
        if is_accidental_needed(tone, previous_tones, altered_pitches, key_signature) {
            output.push(tone.key.clone());
            altered_pitches.retain(|pitch| -> bool {
                !(pitch.letter() == tone.pitch.letter() && pitch.octave() == tone.pitch.octave())
            });
            altered_pitches.insert(tone.pitch.clone());
        }
        previous_tones.insert(tone.key.clone());
    }

    output
}

pub fn get_accidentals_in_track(
    notation: &NotationTrack,
    master: &Track,
    barlines: &Bars,
    tone_offsets: &ToneVerticalOffsets,
) -> Accidentals {
    let mut output = Accidentals::new();

    let mut key_signature = &KeySignature::new(0, KeySignatureMode::Major, 0);
    let mut altered_pitches: FxHashSet<Pitch> = FxHashSet::default();
    let mut previous_tones: FxHashSet<String> = FxHashSet::default();

    for tick in 0..notation.length {
        // look for a key signature at this tick
        let found = match master.get_key_signature_at_tick(&tick) {
            Some(entry) => {
                key_signature = entry;
                true
            }
            None => false,
        };

        // clear alterations at barlines and key changes
        if found || barlines.contains_key(&tick) {
            altered_pitches.clear();
        }

        let entry = match notation.track.get(&tick) {
            Some(entry) => entry,
            None => {
                continue;
            }
        };

        let accidentals = get_tones_needing_accidentals(
            entry,
            &mut previous_tones,
            &mut altered_pitches,
            key_signature,
            tone_offsets,
        );

        if accidentals.is_empty() {
            continue;
        }

        let mut slots: Vec<AccidentalNotation> = Vec::new();
        for tone_key in sort_accidentals_alternate(&accidentals) {
            let slot = find_slot(&tone_key, &slots, tone_offsets);
            slots.push(AccidentalNotation { tone_key, slot })
        }

        let mut max_slot = 1;
        for entry in slots {
            if entry.slot > max_slot {
                max_slot = entry.slot;
            }
            output.by_key.insert((tick, entry.tone_key.clone()), entry);
        }
        output.slots_by_tick.insert(tick, max_slot);
    }

    output
}

pub fn get_accidentals(
    flow: &Flow,
    tracks: &Tracks,
    notation_by_track: &NotationByTrack,
    bars: &Bars,
    tone_offsets: &ToneVerticalOffsets,
) -> AccidentalsByTrack {
    let mut output: AccidentalsByTrack = FxHashMap::default();

    let master = tracks.get(&flow.master).unwrap();

    for (track_key, notation) in notation_by_track {
        let accidentals = get_accidentals_in_track(notation, master, bars, tone_offsets);
        output.insert(track_key.clone(), accidentals);
    }

    output
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;
    use rustc_hash::FxHashSet;

    use crate::components::articulation::Articulation;
    use crate::components::pitch::Accidental;
    use crate::components::pitch::Pitch;
    use crate::components::velocity::Velocity;
    use crate::entries::key_signature::KeySignature;
    use crate::entries::key_signature::KeySignatureMode;
    use crate::entries::tone::Tone;
    use crate::parse::get_accidentals::find_slot;
    use crate::parse::get_accidentals::is_accidental_needed;
    use crate::parse::get_accidentals::AccidentalNotation;

    #[test]
    fn is_accidental_needed_test_1() {
        let tone = Tone::new(
            String::from("a"),
            0,
            0,
            Pitch::new(60, Accidental::Natural),
            Velocity::new(100),
            Articulation::None,
        );
        let previous_tones = FxHashSet::default();
        let altered_pitches = FxHashSet::default();
        let key_signature = KeySignature::new(0, KeySignatureMode::Major, 0);
        let result = is_accidental_needed(&tone, &previous_tones, &altered_pitches, &key_signature);
        assert!(!result);
    }

    #[test]
    fn is_accidental_needed_test_2() {
        let tone = Tone::new(
            String::from("a"),
            0,
            0,
            Pitch::new(61, Accidental::Sharp),
            Velocity::new(100),
            Articulation::None,
        );
        let previous_tones = FxHashSet::default();
        let altered_pitches = FxHashSet::default();
        let key_signature = KeySignature::new(0, KeySignatureMode::Major, 0);
        let result = is_accidental_needed(&tone, &previous_tones, &altered_pitches, &key_signature);
        assert!(result);
    }

    #[test]
    fn is_accidental_needed_test_3() {
        let tone = Tone::new(
            String::from("a"),
            0,
            0,
            Pitch::new(60, Accidental::Natural),
            Velocity::new(100),
            Articulation::None,
        );

        let mut previous_tones = FxHashSet::default();
        previous_tones.insert(String::from("a"));

        let altered_pitches = FxHashSet::default();

        let key_signature = KeySignature::new(0, KeySignatureMode::Major, 0);
        let result = is_accidental_needed(&tone, &previous_tones, &altered_pitches, &key_signature);
        assert!(!result);
    }

    #[test]
    fn is_accidental_needed_test_4() {
        let tone = Tone::new(
            String::from("a"),
            0,
            0,
            Pitch::new(67, Accidental::Natural),
            Velocity::new(100),
            Articulation::None,
        );

        let previous_tones = FxHashSet::default();
        let mut altered_pitches = FxHashSet::default();
        altered_pitches.insert(Pitch {
            int: 68,
            accidental: Accidental::Sharp,
        });

        let key_signature = KeySignature::new(0, KeySignatureMode::Major, 0);
        let result = is_accidental_needed(&tone, &previous_tones, &altered_pitches, &key_signature);
        assert!(result);
    }

    #[test]
    fn is_accidental_needed_test_5() {
        let tone = Tone::new(
            String::from("a"),
            0,
            0,
            Pitch::new(68, Accidental::Sharp),
            Velocity::new(100),
            Articulation::None,
        );
        let previous_tones = FxHashSet::default();
        let mut altered_pitches = FxHashSet::default();
        altered_pitches.insert(Pitch {
            int: 68,
            accidental: Accidental::Sharp,
        });
        let key_signature = KeySignature::new(0, KeySignatureMode::Major, 0);
        let result = is_accidental_needed(&tone, &previous_tones, &altered_pitches, &key_signature);
        assert!(!result);
    }

    #[test]
    fn is_accidental_needed_test_6() {
        let tone = Tone::new(
            String::from("a"),
            0,
            0,
            Pitch::new(60, Accidental::Natural),
            Velocity::new(100),
            Articulation::None,
        );
        let previous_tones = FxHashSet::default();
        let altered_pitches = FxHashSet::default();
        let key_signature = KeySignature::new(0, KeySignatureMode::Major, 2);
        let result = is_accidental_needed(&tone, &previous_tones, &altered_pitches, &key_signature);
        assert!(result);
    }

    #[test]
    fn is_accidental_needed_test_7() {
        let tone = Tone::new(
            String::from("a"),
            0,
            0,
            Pitch::new(61, Accidental::Sharp),
            Velocity::new(100),
            Articulation::None,
        );
        let previous_tones = FxHashSet::default();
        let altered_pitches = FxHashSet::default();
        let key_signature = KeySignature::new(0, KeySignatureMode::Major, 2);
        let result = is_accidental_needed(&tone, &previous_tones, &altered_pitches, &key_signature);
        assert!(!result);
    }

    #[test]
    fn find_slot_test_1() {
        let mut tone_offsets = FxHashMap::default();
        tone_offsets.insert(String::from("a"), 0);
        let result = find_slot("a", &Vec::new(), &tone_offsets);
        assert_eq!(result, 1);
    }

    #[test]
    fn find_slot_test_2() {
        let mut tone_offsets = FxHashMap::default();
        tone_offsets.insert(String::from("a"), 0);
        tone_offsets.insert(String::from("b"), 2);
        let result = find_slot(
            "b",
            &[AccidentalNotation {
                tone_key: String::from("a"),
                slot: 1,
            }],
            &tone_offsets,
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn find_slot_test_3() {
        let mut tone_offsets = FxHashMap::default();
        tone_offsets.insert(String::from("a"), 0);
        tone_offsets.insert(String::from("b"), -4);
        tone_offsets.insert(String::from("c"), 2);
        let result = find_slot(
            "c",
            &[
                AccidentalNotation {
                    tone_key: String::from("a"),
                    slot: 1,
                },
                AccidentalNotation {
                    tone_key: String::from("b"),
                    slot: 2,
                },
            ],
            &tone_offsets,
        );
        assert_eq!(result, 2);
    }
}

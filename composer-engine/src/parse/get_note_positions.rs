use super::get_stem_directions::StemDirectionsByTrack;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use crate::components::misc::{StemDirection, Tick};
use rustc_hash::FxHashMap;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Shunt {
    Pre,
    None,
    Post,
}

pub struct NoteheadShunts {
    pub by_key: FxHashMap<(Tick, String), Shunt>,
    pub by_offset: FxHashMap<(Tick, i8), Shunt>,
}

pub fn note_shunts_in_chord(
    tick: &Tick,
    entry: &Notation,
    tone_offsets: &ToneVerticalOffsets,
    stem_direction: &StemDirection,
) -> NoteheadShunts {
    let mut shunts = NoteheadShunts {
        by_key: FxHashMap::default(),
        by_offset: FxHashMap::default(),
    };

    let clusters = entry.get_clusters(tone_offsets);

    for cluster in clusters {
        let is_odd_length = cluster.len() % 2 != 0;
        let first_note_shunted = stem_direction == &StemDirection::Up || is_odd_length;

        for (i, tone) in cluster.iter().enumerate() {
            // alternate between shunted/not shunted based on the position
            // of the first tone in the cluster
            let shunted = match first_note_shunted {
                true => i % 2 > 0,
                false => i % 2 == 0,
            };

            let shunt = match shunted {
                true => match stem_direction {
                    StemDirection::Up => Shunt::Post,
                    StemDirection::Down => Shunt::Pre,
                },
                false => Shunt::None,
            };

            let offset = tone_offsets.get(&tone.key).unwrap();

            shunts
                .by_key
                .insert((*tick, tone.key.clone()), shunt.clone());
            shunts.by_offset.insert((*tick, *offset), shunt.clone());
        }
    }

    shunts
}

pub fn get_note_shunts(
    notation_by_track: &NotationByTrack,
    tone_offsets: &ToneVerticalOffsets,
    stem_directions_by_track: &StemDirectionsByTrack,
) -> NoteheadShunts {
    let mut shunts = NoteheadShunts {
        by_key: FxHashMap::default(),
        by_offset: FxHashMap::default(),
    };

    for (track_key, notation) in notation_by_track {
        let stem_directions = stem_directions_by_track.get(track_key).unwrap();
        for (tick, entry) in &notation.track {
            if !entry.is_rest() {
                let stem_direction = stem_directions.get(tick).unwrap();
                let positions = note_shunts_in_chord(tick, entry, tone_offsets, stem_direction);
                for (key, position) in positions.by_key {
                    shunts.by_key.insert(key, position);
                }
                for (key, position) in positions.by_offset {
                    shunts.by_offset.insert(key, position);
                }
            }
        }
    }

    shunts
}

#[cfg(test)]
mod tests {
    use super::{note_shunts_in_chord, NoteheadShunts};
    use crate::components::misc::StemDirection;
    use crate::entries::tone::Tone;
    use crate::parse::get_note_positions::Shunt;
    use crate::parse::get_written_durations::Notation;
    use rustc_hash::{FxHashMap, FxHashSet};

    fn run(config: Vec<(&str, i8)>, stem_direction: &StemDirection) -> NoteheadShunts {
        let mut tone_offsets = FxHashMap::default();
        let mut notation = Notation {
            tick: 0,
            tones: Vec::new(),
            duration: 0,
            ties: FxHashSet::default(),
        };

        for (key, offset) in config {
            notation.tones.push(Tone::tester(key));
            tone_offsets.insert(key.to_string(), offset);
        }

        note_shunts_in_chord(&0, &notation, &tone_offsets, stem_direction)
    }

    #[test]
    /// no shunts (1 tone, up)
    fn notehead_shunts_in_chord_test_1() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::None);

        let result = run(vec![("a", 0)], &StemDirection::Up);
        assert_eq!(result.by_key, expected);
    }

    #[test]
    /// no shunts (1 tone, down)
    fn notehead_shunts_in_chord_test_2() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::None);

        let result = run(vec![("a", 0)], &StemDirection::Down);
        assert_eq!(result.by_key, expected);
    }

    #[test]
    /// shunts (2 tones, up)
    fn notehead_shunts_in_chord_test_3() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::None);
        expected.insert((0, String::from("b")), Shunt::Post);

        let result = run(vec![("a", 0), ("b", -1)], &StemDirection::Up);
        assert_eq!(result.by_key, expected);
    }

    #[test]
    /// shunts (2 tones, up)
    fn notehead_shunts_in_chord_test_4() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::Pre);
        expected.insert((0, String::from("b")), Shunt::None);

        let result = run(vec![("a", 0), ("b", -1)], &StemDirection::Down);
        assert_eq!(result.by_key, expected);
    }

    #[test]
    /// shunts (3 tones, up)
    fn notehead_shunts_in_chord_test_5() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::None);
        expected.insert((0, String::from("b")), Shunt::Post);
        expected.insert((0, String::from("c")), Shunt::None);

        let result = run(vec![("a", 0), ("b", -1), ("c", -2)], &StemDirection::Up);
        assert_eq!(result.by_key, expected);
    }

    #[test]
    /// shunts (3 tones, up)
    fn notehead_shunts_in_chord_test_6() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::None);
        expected.insert((0, String::from("b")), Shunt::Pre);
        expected.insert((0, String::from("c")), Shunt::None);

        let result = run(vec![("a", 0), ("b", -1), ("c", -2)], &StemDirection::Down);
        assert_eq!(result.by_key, expected);
    }

    #[test]
    /// shunts (3 tones, 2 clusters, up)
    fn notehead_shunts_in_chord_test_7() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::None);
        expected.insert((0, String::from("b")), Shunt::None);
        expected.insert((0, String::from("c")), Shunt::Post);

        let result = run(vec![("a", 0), ("b", -2), ("c", -3)], &StemDirection::Up);
        assert_eq!(result.by_key, expected);
    }

    #[test]
    /// shunts (3 tones, 2 clusters, up)
    fn notehead_shunts_in_chord_test_8() {
        let mut expected = FxHashMap::default();
        expected.insert((0, String::from("a")), Shunt::None);
        expected.insert((0, String::from("b")), Shunt::Pre);
        expected.insert((0, String::from("c")), Shunt::None);

        let result = run(vec![("a", 0), ("b", -2), ("c", -3)], &StemDirection::Down);
        assert_eq!(result.by_key, expected);
    }
}

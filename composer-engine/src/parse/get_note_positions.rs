use super::get_stem_directions::{StemDirection, StemDirectionsByTrack};
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use crate::components::misc::Tick;
use std::collections::HashMap;
use std::ops::{Add, Index, IndexMut};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Position {
    PaddingStart = 0,
    EndRepeat,
    Clef,
    Barline,
    KeySignature,
    TimeSignature,
    StartRepeat,
    Accidentals,
    PreNoteSlot,
    NoteSlot,
    PostNoteSlot,
    NoteSpacing,
    PaddingEnd,
}

impl Add<Position> for usize {
    type Output = usize;

    fn add(self, other: Position) -> usize {
        self + other as usize
    }
}

impl From<usize> for Position {
    fn from(int: usize) -> Position {
        match int {
            0 => Position::PaddingStart,
            1 => Position::EndRepeat,
            2 => Position::Clef,
            3 => Position::Barline,
            4 => Position::KeySignature,
            5 => Position::TimeSignature,
            6 => Position::StartRepeat,
            7 => Position::Accidentals,
            8 => Position::PreNoteSlot,
            9 => Position::NoteSlot,
            10 => Position::PostNoteSlot,
            11 => Position::NoteSpacing,
            12 => Position::PaddingEnd,
            _ => Position::PaddingStart,
        }
    }
}

impl Index<Position> for [f32] {
    type Output = f32;

    fn index(&self, position: Position) -> &Self::Output {
        &self[position as usize]
    }
}

impl IndexMut<Position> for [f32] {
    fn index_mut(&mut self, position: Position) -> &mut f32 {
        &mut self[position as usize]
    }
}

pub type TonePositions = HashMap<(Tick, String), Position>;

pub fn note_positions_in_chord(
    tick: &Tick,
    entry: &Notation,
    tone_offsets: &ToneVerticalOffsets,
    stem_direction: &StemDirection,
) -> TonePositions {
    let mut shunts: TonePositions = HashMap::new();

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

            let position = match shunted {
                true => match stem_direction {
                    StemDirection::Up => Position::PostNoteSlot,
                    StemDirection::Down => Position::PreNoteSlot,
                },
                false => Position::NoteSlot,
            };

            shunts.insert((*tick, tone.key.clone()), position);
        }
    }

    shunts
}

pub fn get_note_positions(
    notation_by_track: &NotationByTrack,
    tone_offsets: &ToneVerticalOffsets,
    stem_directions_by_track: &StemDirectionsByTrack,
) -> TonePositions {
    let mut shunts: TonePositions = HashMap::new();

    for (track_key, notation) in notation_by_track {
        let stem_directions = stem_directions_by_track.get(track_key).unwrap();
        for (tick, entry) in &notation.track {
            if !entry.is_rest() {
                let stem_direction = stem_directions.get(tick).unwrap();
                let positions = note_positions_in_chord(tick, entry, tone_offsets, stem_direction);
                for (key, position) in positions {
                    shunts.insert(key, position);
                }
            }
        }
    }

    shunts
}

#[cfg(test)]
mod tests {
    use super::{note_positions_in_chord, Position, TonePositions};
    use crate::entries::tone::Tone;
    use crate::parse::get_stem_directions::StemDirection;
    use crate::parse::get_written_durations::Notation;
    use std::collections::{HashMap, HashSet};

    fn run(config: Vec<(&str, i8)>, stem_direction: &StemDirection) -> TonePositions {
        let mut tone_offsets = HashMap::new();
        let mut notation = Notation {
            tones: Vec::new(),
            duration: 0,
            ties: HashSet::new(),
        };

        for (key, offset) in config {
            notation.tones.push(Tone::tester(key));
            tone_offsets.insert(key.to_string(), offset);
        }

        note_positions_in_chord(&0, &notation, &tone_offsets, stem_direction)
    }

    #[test]
    /// no shunts (1 tone, up)
    fn notehead_positions_in_chord_test_1() {
        let result = run(vec![("a", 0)], &StemDirection::Up);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::NoteSlot}
        );
    }

    #[test]
    /// no shunts (1 tone, down)
    fn notehead_positions_in_chord_test_2() {
        let result = run(vec![("a", 0)], &StemDirection::Down);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::NoteSlot}
        );
    }

    #[test]
    /// shunts (2 tones, up)
    fn notehead_positions_in_chord_test_3() {
        let result = run(vec![("a", 0), ("b", -1)], &StemDirection::Up);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::NoteSlot, (0,String::from("b")) => Position::PostNoteSlot}
        );
    }

    #[test]
    /// shunts (2 tones, up)
    fn notehead_positions_in_chord_test_4() {
        let result = run(vec![("a", 0), ("b", -1)], &StemDirection::Down);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::PreNoteSlot, (0,String::from("b")) => Position::NoteSlot}
        );
    }

    #[test]
    /// shunts (3 tones, up)
    fn notehead_positions_in_chord_test_5() {
        let result = run(vec![("a", 0), ("b", -1), ("c", -2)], &StemDirection::Up);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::NoteSlot, (0,String::from("b")) => Position::PostNoteSlot, (0,String::from("c")) => Position::NoteSlot}
        );
    }

    #[test]
    /// shunts (3 tones, up)
    fn notehead_positions_in_chord_test_6() {
        let result = run(vec![("a", 0), ("b", -1), ("c", -2)], &StemDirection::Down);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::NoteSlot, (0,String::from("b")) => Position::PreNoteSlot, (0,String::from("c")) => Position::NoteSlot}
        );
    }

    #[test]
    /// shunts (3 tones, 2 clusters, up)
    fn notehead_positions_in_chord_test_7() {
        let result = run(vec![("a", 0), ("b", -2), ("c", -3)], &StemDirection::Up);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::NoteSlot, (0,String::from("b")) => Position::NoteSlot, (0,String::from("c")) => Position::PostNoteSlot}
        );
    }

    #[test]
    /// shunts (3 tones, 2 clusters, up)
    fn notehead_positions_in_chord_test_8() {
        let result = run(vec![("a", 0), ("b", -2), ("c", -3)], &StemDirection::Down);
        assert_eq!(
            result,
            hashmap! {(0,String::from("a")) => Position::NoteSlot, (0,String::from("b")) => Position::PreNoteSlot, (0,String::from("c")) => Position::NoteSlot}
        );
    }
}

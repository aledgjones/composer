use super::get_dots::DotsByTrack;
use super::get_stem_directions::StemDirectionsByTrack;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::misc::StemDirection;
use crate::components::units::Converter;
use crate::score::flows::Flow;
use crate::score::stave::Stave;
use rustc_hash::FxHashMap;

type TieDirections = FxHashMap<String, StemDirection>;

pub fn get_tie_directions(
    entry: &Notation,
    stem_direction: &StemDirection,
    tone_offsets: &ToneVerticalOffsets,
) -> TieDirections {
    let mut output: TieDirections = FxHashMap::default();

    let count = entry.ties.len();

    let middle = match stem_direction {
        StemDirection::Up => (count as f32 / 2.0).ceil() as usize,
        StemDirection::Down => (count as f32 / 2.0).floor() as usize,
    };

    let sorted = entry.sort_tones(tone_offsets);
    let sorted = sorted.iter().filter(|tone| entry.ties.contains(&tone.key));

    for (i, tone) in sorted.enumerate() {
        if i < middle {
            output.insert(tone.key.clone(), StemDirection::Down);
        } else {
            output.insert(tone.key.clone(), StemDirection::Up);
        }
    }

    output
}

pub fn draw_ties(
    x: &f32,
    y: &f32,
    flow: &Flow,
    staves: &[&Stave],
    notation_by_track: &NotationByTrack,
    stem_directions_by_track: &StemDirectionsByTrack,
    dots_by_track: &DotsByTrack,
    vertical_spacing: &VerticalSpacing,
    horizontal_spacing: &HorizontalSpacing,
    tone_offsets: &ToneVerticalOffsets,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let top = vertical_spacing.staves.get(&stave.key).unwrap();

        for track_key in &stave.tracks {
            let notation = notation_by_track.get(track_key).unwrap();
            let stem_directions = stem_directions_by_track.get(track_key).unwrap();
            let dots = dots_by_track.get(track_key).unwrap();

            for (tick, entry) in &notation.track {
                if entry.has_tie() {
                    let stem_direction = stem_directions.get(tick).unwrap();
                    let tie_directions = get_tie_directions(entry, stem_direction, tone_offsets);

                    for tone_key in tie_directions {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_tie_directions, TieDirections};
    use crate::components::misc::StemDirection;
    use crate::entries::tone::Tone;
    use crate::parse::get_written_durations::Notation;
    use rustc_hash::{FxHashMap, FxHashSet};

    fn run_tie_directions_test(
        config: Vec<(&str, i8)>,
        stem_direction: &StemDirection,
    ) -> TieDirections {
        let mut tone_offsets = FxHashMap::default();
        let mut notation = Notation {
            tick: 0,
            tones: Vec::new(),
            duration: 0,
            ties: FxHashSet::default(),
        };

        for (key, offset) in config {
            notation.tones.push(Tone::tester(key));
            notation.ties.insert(String::from(key));
            tone_offsets.insert(key.to_string(), offset);
        }

        get_tie_directions(&notation, stem_direction, &tone_offsets)
    }

    #[test]
    fn tie_directions_test_1() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), StemDirection::Down);

        let result = run_tie_directions_test(vec![("a", 0)], &StemDirection::Up);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_2() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), StemDirection::Up);

        let result = run_tie_directions_test(vec![("a", 0)], &StemDirection::Down);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_3() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), StemDirection::Up);
        expected.insert(String::from("b"), StemDirection::Down);

        let result = run_tie_directions_test(vec![("a", 0), ("b", 1)], &StemDirection::Up);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_4() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), StemDirection::Up);
        expected.insert(String::from("b"), StemDirection::Down);

        let result = run_tie_directions_test(vec![("a", 0), ("b", 1)], &StemDirection::Down);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_5() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), StemDirection::Up);
        expected.insert(String::from("b"), StemDirection::Down);
        expected.insert(String::from("c"), StemDirection::Down);

        let result =
            run_tie_directions_test(vec![("a", 0), ("b", 1), ("c", 2)], &StemDirection::Up);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_6() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), StemDirection::Up);
        expected.insert(String::from("b"), StemDirection::Up);
        expected.insert(String::from("c"), StemDirection::Down);

        let result =
            run_tie_directions_test(vec![("a", 0), ("b", 1), ("c", 2)], &StemDirection::Down);
        assert_eq!(result, expected);
    }
}

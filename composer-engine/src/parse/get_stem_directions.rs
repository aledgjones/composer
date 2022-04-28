use super::get_beams::Beam;
use super::get_beams::Beams;
use super::get_beams::BeamsByTrack;
use super::get_tone_offsets::get_tone_offset_info;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::Notation;
use super::get_written_durations::NotationTrack;
use super::get_written_durations::NotationTracks;
use crate::components::misc::Tick;
use crate::entries::tone::Tone;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum StemDirection {
    Up,
    Down,
}

impl StemDirection {
    pub fn to_modifier(&self) -> i8 {
        match self {
            StemDirection::Up => 1,
            StemDirection::Down => -1,
        }
    }
}

pub type StemDirections = HashMap<Tick, StemDirection>;
pub type StemDirectionsByTrack = HashMap<String, StemDirections>;

pub fn get_span_stem_direction(
    span: &Beam,
    notation: &NotationTrack,
    tone_offsets: &ToneVerticalOffsets,
) -> StemDirection {
    let mut up_count = 0;
    let mut down_count = 0;
    let mut tones: Vec<Tone> = Vec::new();

    for tick in span {
        let entry = notation.track.get(tick).unwrap();

        let direction = entry.get_stem_direction(tone_offsets);
        match direction {
            StemDirection::Up => up_count += 1,
            StemDirection::Down => down_count += 1,
        }

        for tone in &entry.tones {
            tones.push(tone.clone());
        }
    }

    match up_count.cmp(&down_count) {
        Ordering::Greater => StemDirection::Up,
        Ordering::Less => StemDirection::Down,
        Ordering::Equal => {
            let (_, _, furthest) = get_tone_offset_info(&tones, tone_offsets);

            if furthest > 0 {
                StemDirection::Up
            } else {
                StemDirection::Down
            }
        }
    }
}

pub fn get_stem_directions_in_track(
    notation: &NotationTrack,
    tone_offsets: &ToneVerticalOffsets,
    beams: &Beams,
) -> StemDirections {
    let mut output = HashMap::new();

    // natural stem directions
    for (at, entry) in &notation.track {
        if !entry.is_rest() {
            output.insert(*at, entry.get_stem_direction(tone_offsets));
        };
    }

    // stem spans
    for span in beams {
        let direction = get_span_stem_direction(span, notation, tone_offsets);
        for tick in span {
            output.insert(*tick, direction.clone());
        }
    }

    output
}

pub fn get_stem_directions(
    tracks: &NotationTracks,
    tone_offsets: &ToneVerticalOffsets,
    beams_by_track: &BeamsByTrack,
) -> StemDirectionsByTrack {
    let mut output = HashMap::new();

    for (track_key, track) in tracks {
        let beams = beams_by_track.get(track_key).unwrap();
        let stem_directions = get_stem_directions_in_track(track, tone_offsets, beams);
        output.insert(track_key.clone(), stem_directions);
    }

    output
}

impl Notation {
    pub fn get_stem_direction(&self, tone_offsets: &ToneVerticalOffsets) -> StemDirection {
        let (_, _, furthest) = get_tone_offset_info(&self.tones, tone_offsets);

        if furthest > 0 {
            StemDirection::Up
        } else {
            StemDirection::Down
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_span_stem_direction;
    use super::StemDirection;
    use super::ToneVerticalOffsets;
    use crate::components::articulation::Articulation;
    use crate::components::misc::Tick;
    use crate::components::misc::Ticks;
    use crate::components::pitch::Pitch;
    use crate::components::velocity::Velocity;
    use crate::entries::tone::Tone;
    use crate::parse::get_written_durations::Notation;
    use crate::parse::get_written_durations::NotationTrack;
    use std::collections::{HashMap, HashSet};

    fn run_get_stem_direction_test(tones: Vec<(&str, i8)>) -> StemDirection {
        let mut notation = Notation {
            tones: Vec::new(),
            duration: 0,
            ties: HashSet::new(),
        };
        let mut tone_offsets: ToneVerticalOffsets = HashMap::new();

        for (key, offset) in tones {
            tone_offsets.insert(key.to_string(), offset);
            notation.tones.push(Tone::new(
                key.to_string(),
                0,
                0,
                Pitch::from_int(60),
                Velocity::new(100),
                Articulation::None,
            ));
        }

        notation.get_stem_direction(&tone_offsets)
    }

    #[test]
    fn get_stem_direction_1() {
        let result = run_get_stem_direction_test(vec![("a", 0)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_stem_direction_2() {
        let result = run_get_stem_direction_test(vec![("a", 2)]);
        assert_eq!(result, StemDirection::Up);
    }

    #[test]
    fn get_stem_direction_3() {
        let result = run_get_stem_direction_test(vec![("a", -2)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_stem_direction_4() {
        let result = run_get_stem_direction_test(vec![("a", -2), ("b", 2)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_stem_direction_5() {
        let result = run_get_stem_direction_test(vec![("a", 2), ("b", -2)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_stem_direction_6() {
        let result = run_get_stem_direction_test(vec![("a", -2), ("b", 3)]);
        assert_eq!(result, StemDirection::Up);
    }

    #[test]
    fn get_stem_direction_7() {
        let result = run_get_stem_direction_test(vec![("a", -2), ("b", 3), ("c", -7)]);
        assert_eq!(result, StemDirection::Down);
    }

    fn run_get_span_stem_direction_test(tones: Vec<(&str, i8)>) -> StemDirection {
        let mut tone_offsets: ToneVerticalOffsets = HashMap::new();
        let mut beam = Vec::new();

        let mut track = NotationTrack::new(tones.len() as Ticks);
        for (tick, (key, offset)) in tones.iter().enumerate() {
            track.insert(
                tick as Tick,
                Notation {
                    tones: vec![Tone::new(
                        key.to_string(),
                        tick as Tick,
                        0,
                        Pitch::from_int(60),
                        Velocity::new(100),
                        Articulation::None,
                    )],
                    duration: 1,
                    ties: HashSet::new(),
                },
            );

            tone_offsets.insert(key.to_string(), *offset);
            beam.push(tick as Tick);
        }

        get_span_stem_direction(&beam, &track, &tone_offsets)
    }

    #[test]
    fn get_span_stem_direction_test_1() {
        let result = run_get_span_stem_direction_test(vec![("a", 0), ("b", 0)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_span_stem_direction_test_2() {
        let result = run_get_span_stem_direction_test(vec![("a", -1), ("b", -1)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_span_stem_direction_test_3() {
        let result = run_get_span_stem_direction_test(vec![("a", 1), ("b", 1)]);
        assert_eq!(result, StemDirection::Up);
    }

    #[test]
    fn get_span_stem_direction_test_4() {
        let result = run_get_span_stem_direction_test(vec![("a", -1), ("b", 1)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_span_stem_direction_test_5() {
        let result = run_get_span_stem_direction_test(vec![("a", 2), ("b", -2)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_span_stem_direction_test_6() {
        let result = run_get_span_stem_direction_test(vec![("a", -5), ("b", 4)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_span_stem_direction_test_7() {
        let result = run_get_span_stem_direction_test(vec![("a", 3), ("b", -4)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_span_stem_direction_test_8() {
        let result = run_get_span_stem_direction_test(vec![("a", -4), ("b", 5)]);
        assert_eq!(result, StemDirection::Up);
    }

    #[test]
    fn get_span_stem_direction_test_9() {
        let result = run_get_span_stem_direction_test(vec![("a", 4), ("b", -3)]);
        assert_eq!(result, StemDirection::Up);
    }

    #[test]
    fn get_span_stem_direction_test_10() {
        let result = run_get_span_stem_direction_test(vec![("a", 1), ("b", -1), ("c", 1)]);
        assert_eq!(result, StemDirection::Up);
    }

    #[test]
    fn get_span_stem_direction_test_11() {
        let result =
            run_get_span_stem_direction_test(vec![("a", -2), ("b", 0), ("c", -1), ("d", 2)]);
        assert_eq!(result, StemDirection::Down);
    }

    #[test]
    fn get_span_stem_direction_test_12() {
        let result =
            run_get_span_stem_direction_test(vec![("a", 1), ("b", 2), ("c", 3), ("d", -3)]);
        assert_eq!(result, StemDirection::Up);
    }
}

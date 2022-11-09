use super::get_beams::Beam;
use super::get_beams::Beams;
use super::get_beams::BeamsByTrack;
use super::get_tone_offsets::get_tone_offset_info;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::Notation;
use super::get_written_durations::NotationByTrack;
use super::get_written_durations::NotationTrack;
use crate::components::misc::Direction;
use crate::components::misc::Tick;
use crate::entries::tone::Tone;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

pub type StemDirections = FxHashMap<Tick, Direction>;
pub type StemDirectionsByTrack = FxHashMap<String, StemDirections>;

pub fn get_span_stem_direction(
    span: &Beam,
    notation: &NotationTrack,
    tone_offsets: &ToneVerticalOffsets,
) -> Direction {
    let mut up_count = 0;
    let mut down_count = 0;
    let mut tones: Vec<Tone> = Vec::new();

    for tick in span.ticks.keys() {
        let entry = notation.track.get(tick).unwrap();

        let direction = entry.get_stem_direction(tone_offsets);

        match direction {
            Direction::Up => up_count += 1,
            Direction::Down => down_count += 1,
        }

        for tone in &entry.tones {
            tones.push(tone.clone());
        }
    }

    match up_count.cmp(&down_count) {
        Ordering::Greater => Direction::Up,
        Ordering::Less => Direction::Down,
        Ordering::Equal => {
            let (_, _, furthest) = get_tone_offset_info(&tones, tone_offsets);

            if furthest > 0 {
                Direction::Up
            } else {
                Direction::Down
            }
        }
    }
}

pub fn get_stem_directions_in_track(
    notation: &NotationTrack,
    tone_offsets: &ToneVerticalOffsets,
    beams: &Beams,
) -> StemDirections {
    let mut output = FxHashMap::default();

    // natural stem directions
    for (at, entry) in &notation.track {
        if !entry.is_rest() {
            output.insert(*at, entry.get_stem_direction(tone_offsets));
        };
    }

    // stem spans
    for span in beams {
        let direction = get_span_stem_direction(span, notation, tone_offsets);
        for tick in span.ticks.keys() {
            output.insert(*tick, direction.clone());
        }
    }

    output
}

pub fn get_stem_directions(
    tracks: &NotationByTrack,
    tone_offsets: &ToneVerticalOffsets,
    beams_by_track: &BeamsByTrack,
) -> StemDirectionsByTrack {
    let mut output = FxHashMap::default();

    for (track_key, track) in tracks {
        let beams = beams_by_track.get(track_key).unwrap();
        let stem_directions = get_stem_directions_in_track(track, tone_offsets, beams);
        output.insert(track_key.clone(), stem_directions);
    }

    output
}

impl Notation {
    pub fn get_stem_direction(&self, tone_offsets: &ToneVerticalOffsets) -> Direction {
        let (_, _, furthest) = self.get_tone_offset_info(tone_offsets);

        if furthest > 0 {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;
    use rustc_hash::FxHashSet;

    use super::get_span_stem_direction;
    use super::Direction;
    use super::ToneVerticalOffsets;
    use crate::components::articulation::Articulation;
    use crate::components::misc::Tick;
    use crate::components::misc::Ticks;
    use crate::components::pitch::Pitch;
    use crate::components::velocity::Velocity;
    use crate::entries::tone::Tone;
    use crate::parse::get_beams::Beam;
    use crate::parse::get_written_durations::Notation;
    use crate::parse::get_written_durations::NotationTrack;

    fn run_get_stem_direction_test(tones: Vec<(&str, i8)>) -> Direction {
        let mut notation = Notation {
            tick: 0,
            tones: Vec::new(),
            duration: 0,
            ties: FxHashSet::default(),
        };
        let mut tone_offsets: ToneVerticalOffsets = FxHashMap::default();

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
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_stem_direction_2() {
        let result = run_get_stem_direction_test(vec![("a", 2)]);
        assert_eq!(result, Direction::Up);
    }

    #[test]
    fn get_stem_direction_3() {
        let result = run_get_stem_direction_test(vec![("a", -2)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_stem_direction_4() {
        let result = run_get_stem_direction_test(vec![("a", -2), ("b", 2)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_stem_direction_5() {
        let result = run_get_stem_direction_test(vec![("a", 2), ("b", -2)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_stem_direction_6() {
        let result = run_get_stem_direction_test(vec![("a", -2), ("b", 3)]);
        assert_eq!(result, Direction::Up);
    }

    #[test]
    fn get_stem_direction_7() {
        let result = run_get_stem_direction_test(vec![("a", -2), ("b", 3), ("c", -7)]);
        assert_eq!(result, Direction::Down);
    }

    fn run_get_span_stem_direction_test(tones: Vec<(&str, i8)>) -> Direction {
        let mut tone_offsets: ToneVerticalOffsets = FxHashMap::default();
        let mut beam: Beam = Beam {
            ticks: FxHashMap::default(),
            start: 0,
            stop: tones.len() as Tick - 1,
        };

        let mut track = NotationTrack::new(tones.len() as Ticks);
        for (tick, (key, offset)) in tones.iter().enumerate() {
            track.insert(
                tick as Tick,
                Notation {
                    tick: 0,
                    tones: vec![Tone::new(
                        key.to_string(),
                        tick as Tick,
                        0,
                        Pitch::from_int(60),
                        Velocity::new(100),
                        Articulation::None,
                    )],
                    duration: 1,
                    ties: FxHashSet::default(),
                },
            );

            tone_offsets.insert(key.to_string(), *offset);
            beam.ticks.insert(tick as Tick, 1);
        }

        get_span_stem_direction(&beam, &track, &tone_offsets)
    }

    #[test]
    fn get_span_stem_direction_test_1() {
        let result = run_get_span_stem_direction_test(vec![("a", 0), ("b", 0)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_span_stem_direction_test_2() {
        let result = run_get_span_stem_direction_test(vec![("a", -1), ("b", -1)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_span_stem_direction_test_3() {
        let result = run_get_span_stem_direction_test(vec![("a", 1), ("b", 1)]);
        assert_eq!(result, Direction::Up);
    }

    #[test]
    fn get_span_stem_direction_test_4() {
        let result = run_get_span_stem_direction_test(vec![("a", -1), ("b", 1)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_span_stem_direction_test_5() {
        let result = run_get_span_stem_direction_test(vec![("a", 2), ("b", -2)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_span_stem_direction_test_6() {
        let result = run_get_span_stem_direction_test(vec![("a", -5), ("b", 4)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_span_stem_direction_test_7() {
        let result = run_get_span_stem_direction_test(vec![("a", 3), ("b", -4)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_span_stem_direction_test_8() {
        let result = run_get_span_stem_direction_test(vec![("a", -4), ("b", 5)]);
        assert_eq!(result, Direction::Up);
    }

    #[test]
    fn get_span_stem_direction_test_9() {
        let result = run_get_span_stem_direction_test(vec![("a", 4), ("b", -3)]);
        assert_eq!(result, Direction::Up);
    }

    #[test]
    fn get_span_stem_direction_test_10() {
        let result = run_get_span_stem_direction_test(vec![("a", 1), ("b", -1), ("c", 1)]);
        assert_eq!(result, Direction::Up);
    }

    #[test]
    fn get_span_stem_direction_test_11() {
        let result =
            run_get_span_stem_direction_test(vec![("a", -2), ("b", 0), ("c", -1), ("d", 2)]);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn get_span_stem_direction_test_12() {
        let result =
            run_get_span_stem_direction_test(vec![("a", 1), ("b", 2), ("c", 3), ("d", -3)]);
        assert_eq!(result, Direction::Up);
    }
}

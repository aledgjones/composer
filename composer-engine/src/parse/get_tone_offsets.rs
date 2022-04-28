use crate::components::misc::Ticks;
use crate::components::pitch::Pitch;
use crate::entries::clef::Clef;
use crate::entries::tone::Tone;
use crate::score::stave::Stave;
use crate::score::tracks::Tracks;
use std::collections::HashMap;

pub type ToneVerticalOffsets = HashMap<String, i8>;

pub fn get_tone_offsets(
    flow_length: Ticks,
    staves: &[&Stave],
    tracks: &Tracks,
) -> ToneVerticalOffsets {
    let mut output: ToneVerticalOffsets = HashMap::new();

    for stave in staves {
        let master = tracks.get(&stave.master).unwrap();
        let mut clef: Option<Clef> = None;

        for tick in 0..flow_length {
            if let Some(found) = master.get_clef_at_tick(&tick) {
                clef = Some(found);
            };

            if let Some(clef) = &clef {
                for stave_key in &stave.tracks {
                    let track = tracks.get(stave_key).unwrap();
                    for tone in track.get_tones_at_tick(&tick) {
                        let offset = Pitch::steps_between(&clef.pitch, &tone.pitch) + clef.offset;
                        output.insert(tone.key.clone(), offset);
                    }
                }
            }
        }
    }

    output
}

pub fn get_tone_offset_info(tones: &[Tone], tone_offsets: &ToneVerticalOffsets) -> (i8, i8, i8) {
    // this will never happen but for safety:
    if tones.is_empty() {
        return (0, 0, 0);
    }

    let mut minimum: Option<i8> = None;
    let mut maximum: Option<i8> = None;

    for tone in tones {
        let offset = *tone_offsets.get(&tone.key).unwrap();
        match minimum {
            Some(value) => {
                if offset < value {
                    minimum = Some(offset)
                }
            }
            None => minimum = Some(offset),
        };
        match maximum {
            Some(value) => {
                if offset > value {
                    maximum = Some(offset)
                }
            }
            None => maximum = Some(offset),
        };
    }

    let min = minimum.unwrap();
    let max = maximum.unwrap();

    let furthest = if max.abs() > min.abs() { max } else { min };

    (min, max, furthest)
}

#[cfg(test)]
mod tests {
    use super::get_tone_offset_info;
    use super::ToneVerticalOffsets;
    use crate::components::articulation::Articulation;
    use crate::components::pitch::Pitch;
    use crate::components::velocity::Velocity;
    use crate::entries::tone::Tone;
    use crate::parse::get_written_durations::Notation;
    use std::collections::{HashMap, HashSet};

    fn run(tones: Vec<(&str, i8)>) -> (i8, i8, i8) {
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

        get_tone_offset_info(&notation.tones, &tone_offsets)
    }

    #[test]
    fn empty() {
        let result = run(Vec::new());
        assert_eq!(result, (0, 0, 0));
    }

    #[test]
    /// middle of stave
    fn single_tone_1() {
        let result = run(vec![("a", 0)]);
        assert_eq!(result, (0, 0, 0));
    }

    #[test]
    /// above middle
    fn single_tone_2() {
        let result = run(vec![("a", 2)]);
        assert_eq!(result, (2, 2, 2));
    }

    #[test]
    /// below middle
    fn single_tone_3() {
        let result = run(vec![("a", -2)]);
        assert_eq!(result, (-2, -2, -2));
    }

    #[test]
    /// same pitch - middle
    fn multi_tone_1() {
        let result = run(vec![("a", 0), ("b", 0)]);
        assert_eq!(result, (0, 0, 0));
    }

    #[test]
    /// same pitch - high
    fn multi_tone_2() {
        let result = run(vec![("a", 2), ("b", 2)]);
        assert_eq!(result, (2, 2, 2));
    }

    #[test]
    /// same pitch - low
    fn multi_tone_3() {
        let result = run(vec![("a", -2), ("b", -2)]);
        assert_eq!(result, (-2, -2, -2));
    }

    #[test]
    /// same pitch - even spread
    fn multi_tone_4() {
        let result = run(vec![("a", 2), ("b", -2)]);
        assert_eq!(result, (-2, 2, -2));
    }

    #[test]
    /// same pitch - even spread
    fn multi_tone_5() {
        let result = run(vec![("a", -2), ("b", 2)]);
        assert_eq!(result, (-2, 2, -2));
    }

    #[test]
    fn multi_tone_6() {
        let result = run(vec![("a", 1), ("b", -2), ("c", -1)]);
        assert_eq!(result, (-2, 1, -2));
    }

    #[test]
    fn multi_tone_7() {
        let result = run(vec![("a", -1), ("b", 2), ("c", 1)]);
        assert_eq!(result, (-1, 2, 2));
    }
}

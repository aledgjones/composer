use crate::components::misc::Ticks;
use crate::components::pitch::Pitch;
use crate::entries::clef::{Clef, ClefDrawType};
use crate::entries::tone::Tone;
use crate::score::stave::Stave;
use crate::score::tracks::Tracks;
use rustc_hash::FxHashMap;

pub type ToneVerticalOffsets = FxHashMap<String, i8>;

pub fn get_tone_offsets(
    flow_length: Ticks,
    staves: &[&Stave],
    tracks: &Tracks,
) -> ToneVerticalOffsets {
    let mut output: ToneVerticalOffsets = FxHashMap::default();

    for stave in staves {
        let master = tracks.get(&stave.master).unwrap();
        let mut clef: Clef = Clef::new(0, 60, 0, ClefDrawType::C);

        for tick in 0..flow_length {
            if let Some(found) = master.get_clef_at_tick(&tick) {
                clef = found;
            };

            for stave_key in &stave.tracks {
                let track = tracks.get(stave_key).unwrap();
                for tone in track.get_tones_at_tick(&tick) {
                    let offset = Pitch::steps_between(&tone.pitch, &clef.pitch) + clef.offset;
                    output.insert(tone.key.clone(), offset);
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

    let mut highest: Option<i8> = None;
    let mut lowest: Option<i8> = None;

    for tone in tones {
        let offset = *tone_offsets.get(&tone.key).unwrap();
        match highest {
            Some(value) => {
                if offset < value {
                    highest = Some(offset)
                }
            }
            None => highest = Some(offset),
        };
        match lowest {
            Some(value) => {
                if offset > value {
                    lowest = Some(offset)
                }
            }
            None => lowest = Some(offset),
        };
    }

    let highest = highest.unwrap();
    let lowest = lowest.unwrap();

    let furthest = if lowest.abs() > highest.abs() {
        lowest
    } else {
        highest
    };

    (highest, lowest, furthest)
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;
    use rustc_hash::FxHashSet;

    use super::get_tone_offset_info;
    use super::get_tone_offsets;
    use super::ToneVerticalOffsets;
    use crate::components::articulation::Articulation;
    use crate::components::pitch::Pitch;
    use crate::components::velocity::Velocity;
    use crate::entries::clef::Clef;
    use crate::entries::clef::ClefDrawType;
    use crate::entries::tone::Tone;
    use crate::entries::Entry;
    use crate::parse::get_written_durations::Notation;
    use crate::score::instruments::defs::StaveDef;
    use crate::score::stave::Stave;
    use crate::score::tracks::Track;

    fn run_get_tone_offsets(clef: Clef, tone: (&str, u8)) -> ToneVerticalOffsets {
        let mut track = Track::new();
        track.insert(Entry::Tone(Tone::new(
            String::from(tone.0),
            0,
            16,
            Pitch::from_int(tone.1),
            Velocity::new(100),
            Articulation::None,
        )));

        let mut master = Track::new();
        master.insert(Entry::Clef(clef.clone()));

        let mut stave = Stave::new(
            String::from("a"),
            &StaveDef {
                lines: vec![1, 1, 1, 1, 1],
                clef,
            },
            &master,
        );
        stave.tracks.push(track.key.clone());

        let mut tracks = FxHashMap::default();
        tracks.insert(track.key.clone(), track);
        tracks.insert(master.key.clone(), master);

        get_tone_offsets(16, &[&stave], &tracks)
    }

    #[test]
    fn get_tone_offsets_test_1() {
        let result = run_get_tone_offsets(Clef::new(0, 60, 0, ClefDrawType::C), ("a", 60));
        let mut expected = FxHashMap::default();
        expected.insert(String::from("a"), 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_offsets_test_2() {
        let result = run_get_tone_offsets(Clef::new(0, 60, 0, ClefDrawType::C), ("a", 64));
        let mut expected = FxHashMap::default();
        expected.insert(String::from("a"), -2);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_tone_offsets_test_3() {
        let mut expected = FxHashMap::default();
        expected.insert(String::from("a"), 2);
        let result = run_get_tone_offsets(Clef::new(0, 60, 0, ClefDrawType::C), ("a", 57));
        assert_eq!(result, expected);
    }

    fn run_get_tone_offset_info(tones: Vec<(&str, i8)>) -> (i8, i8, i8) {
        let mut notation = Notation {
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

        get_tone_offset_info(&notation.tones, &tone_offsets)
    }

    #[test]
    fn empty() {
        let result = run_get_tone_offset_info(Vec::new());
        assert_eq!(result, (0, 0, 0));
    }

    #[test]
    /// middle of stave
    fn single_tone_1() {
        let result = run_get_tone_offset_info(vec![("a", 0)]);
        assert_eq!(result, (0, 0, 0));
    }

    #[test]
    /// above middle
    fn single_tone_2() {
        let result = run_get_tone_offset_info(vec![("a", 2)]);
        assert_eq!(result, (2, 2, 2));
    }

    #[test]
    /// below middle
    fn single_tone_3() {
        let result = run_get_tone_offset_info(vec![("a", -2)]);
        assert_eq!(result, (-2, -2, -2));
    }

    #[test]
    /// same pitch - middle
    fn multi_tone_1() {
        let result = run_get_tone_offset_info(vec![("a", 0), ("b", 0)]);
        assert_eq!(result, (0, 0, 0));
    }

    #[test]
    /// same pitch - high
    fn multi_tone_2() {
        let result = run_get_tone_offset_info(vec![("a", 2), ("b", 2)]);
        assert_eq!(result, (2, 2, 2));
    }

    #[test]
    /// same pitch - low
    fn multi_tone_3() {
        let result = run_get_tone_offset_info(vec![("a", -2), ("b", -2)]);
        assert_eq!(result, (-2, -2, -2));
    }

    #[test]
    /// same pitch - even spread
    fn multi_tone_4() {
        let result = run_get_tone_offset_info(vec![("a", 2), ("b", -2)]);
        assert_eq!(result, (-2, 2, -2));
    }

    #[test]
    /// same pitch - even spread
    fn multi_tone_5() {
        let result = run_get_tone_offset_info(vec![("a", -2), ("b", 2)]);
        assert_eq!(result, (-2, 2, -2));
    }

    #[test]
    fn multi_tone_6() {
        let result = run_get_tone_offset_info(vec![("a", 1), ("b", -2), ("c", -1)]);
        assert_eq!(result, (-2, 1, -2));
    }

    #[test]
    fn multi_tone_7() {
        let result = run_get_tone_offset_info(vec![("a", -1), ("b", 2), ("c", 1)]);
        assert_eq!(result, (-1, 2, 2));
    }
}

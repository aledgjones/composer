use super::get_shunts::{Shunt, Shunts, ShuntsByTrack};
use super::get_stem_directions::StemDirectionsByTrack;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack};
use super::measure_horizontal_spacing::{HorizontalSpacing, Position};
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::measurements::CurvePoint;
use crate::components::misc::Direction;
use crate::components::units::{Converter, Space};
use crate::score::stave::{Stave, STAVE_LINE_WIDTH};
use rustc_hash::FxHashMap;

type TieDirections = FxHashMap<String, Direction>;

pub fn get_tie_directions(
    entry: &Notation,
    stem_direction: &Direction,
    tone_offsets: &ToneVerticalOffsets,
) -> TieDirections {
    let mut output: TieDirections = FxHashMap::default();

    let count = entry.ties.len();

    let middle = match stem_direction {
        Direction::Up => (count as f32 / 2.0).ceil() as usize,
        Direction::Down => (count as f32 / 2.0).floor() as usize,
    };

    let sorted = entry.sort_tones(tone_offsets);
    let sorted = sorted.iter().filter(|tone| entry.ties.contains(&tone.key));

    for (i, tone) in sorted.enumerate() {
        if i < middle {
            output.insert(tone.key.clone(), Direction::Down);
        } else {
            output.insert(tone.key.clone(), Direction::Up);
        }
    }

    output
}

fn tie_points_y(
    y: Space,
    start: &Notation,
    tie_direction: &Direction,
    width: Space,
    offset: i8,
) -> [f32; 3] {
    let is_on_line = offset % 2 == 0;
    let is_wide = width > 10.0;
    let direction_modifier = tie_direction.to_modifier() as f32 * -1.0;

    let (ends_tweak, middle_tweak) = match start.is_chord() {
        true => {
            let ends_tweak = 0.25 * direction_modifier;
            let middle_tweak = match is_wide {
                true => match is_on_line {
                    true => 0.75 * direction_modifier,
                    false => 0.5 * direction_modifier,
                },
                false => 0.5 * direction_modifier,
            };
            (ends_tweak, middle_tweak)
        }
        false => {
            let ends_tweak = 0.74 * direction_modifier;
            let middle_tweak = match is_wide {
                true => match is_on_line {
                    true => 0.75 * direction_modifier,
                    false => 0.5 * direction_modifier,
                },
                false => 0.5 * direction_modifier,
            };
            (ends_tweak, middle_tweak)
        }
    };

    let ends = y + (offset as f32 / 2.0) - ends_tweak;
    let middle = ends - middle_tweak;

    [ends, middle, ends]
}

fn start_x(
    start: &Notation,
    horizontal_spacing: &HorizontalSpacing,
    shunts: &Shunts,
    tie_direction: &Direction,
    stem_direction: &Direction,
    offset: i8,
) -> f32 {
    let x = horizontal_spacing
        .get(&start.tick, &Position::NoteSpacing)
        .unwrap()
        .x;

    let after_pre = x + STAVE_LINE_WIDTH + 0.2;
    let after_note = x + start.notehead_width() + 0.2;
    let after_post = x + (start.notehead_width() * 2.0) + 0.2;

    let shunt = shunts.by_offset.get(&(start.tick, offset)).unwrap();
    let next_shunt = shunts
        .by_offset
        .get(&(start.tick, offset + tie_direction.to_modifier()));

    match stem_direction {
        Direction::Up => {
            if let Shunt::Post = shunt {
                return after_post;
            }

            if let Some(Shunt::Post) = next_shunt {
                return after_post;
            }
        }
        Direction::Down => {
            if shunt == &Shunt::Pre && next_shunt.is_none() {
                return after_pre;
            }
        }
    };

    after_note
}

fn stop_x(
    stop: &Notation,
    horizontal_spacing: &HorizontalSpacing,
    shunts: &Shunts,
    tie_direction: &Direction,
    stem_direction: &Direction,
    offset: i8,
) -> f32 {
    let x = horizontal_spacing
        .get(&stop.tick, &Position::NoteSpacing)
        .unwrap()
        .x;

    let before_pre = x - stop.notehead_width() - 0.2;
    let before_note = x - 0.2;
    let before_post = x + stop.notehead_width() - STAVE_LINE_WIDTH - 0.2;

    let shunt = shunts.by_offset.get(&(stop.tick, offset)).unwrap();
    let next_shunt = shunts
        .by_offset
        .get(&(stop.tick, offset + tie_direction.to_modifier()));

    match stem_direction {
        Direction::Down => {
            if let Shunt::Pre = shunt {
                return before_pre;
            }

            if let Some(Shunt::Pre) = next_shunt {
                return before_pre;
            }
        }
        Direction::Up => {
            if shunt == &Shunt::Post && next_shunt.is_none() {
                return before_post;
            }
        }
    };

    before_note
}

fn tie_points_x(
    x: Space,
    start: &Notation,
    stop: &Notation,
    horizontal_spacing: &HorizontalSpacing,
    shunts: &Shunts,
    tie_direction: &Direction,
    stem_direction: &Direction,
    offset: i8,
) -> [f32; 3] {
    let (start_x, stop_x) = match start.is_chord() {
        true => (
            x + start_x(
                start,
                horizontal_spacing,
                shunts,
                tie_direction,
                stem_direction,
                offset,
            ),
            x + stop_x(
                stop,
                horizontal_spacing,
                shunts,
                tie_direction,
                stem_direction,
                offset,
            ),
        ),
        false => {
            let start_x = x
                + horizontal_spacing
                    .get(&start.tick, &Position::NoteSpacing)
                    .unwrap()
                    .x
                + (start.notehead_width() / 2.0)
                + 0.1;

            let stop_x = x
                + horizontal_spacing
                    .get(&stop.tick, &Position::NoteSpacing)
                    .unwrap()
                    .x
                + (start.notehead_width() / 2.0)
                - 0.1;

            (start_x, stop_x)
        }
    };

    let middle_x = start_x + ((stop_x - start_x) / 2.0);

    [start_x, middle_x, stop_x]
}

pub fn draw_tie(
    x: Space,
    y: Space,
    tone_key: &String,
    start: &Notation,
    stop: &Notation,
    tie_direction: &Direction,
    stem_direction: &Direction,
    horizontal_spacing: &HorizontalSpacing,
    shunts: &Shunts,
    tone_offsets: &ToneVerticalOffsets,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let offset = *tone_offsets.get(tone_key).unwrap();

    let [start_x, middle_x, stop_x] = tie_points_x(
        x,
        start,
        stop,
        horizontal_spacing,
        shunts,
        tie_direction,
        stem_direction,
        offset,
    );
    let [start_y, middle_y, stop_y] =
        tie_points_y(y, start, tie_direction, stop_x - start_x, offset);

    instructions.push(Instruction::Curve {
        color: String::from("#000"),
        points: [
            CurvePoint {
                x: converter.spaces_to_px(start_x),
                y: converter.spaces_to_px(start_y),
                thickness: converter.spaces_to_px(0.125),
            },
            CurvePoint {
                x: converter.spaces_to_px(middle_x),
                y: converter.spaces_to_px(middle_y),
                thickness: converter.spaces_to_px(0.2),
            },
            CurvePoint {
                x: converter.spaces_to_px(stop_x),
                y: converter.spaces_to_px(stop_y),
                thickness: converter.spaces_to_px(0.125),
            },
        ],
    })
}

pub fn draw_ties(
    x: Space,
    y: Space,
    staves: &[&Stave],
    notation_by_track: &NotationByTrack,
    stem_directions_by_track: &StemDirectionsByTrack,
    vertical_spacing: &VerticalSpacing,
    horizontal_spacing: &HorizontalSpacing,
    shunts_by_track: &ShuntsByTrack,
    tone_offsets: &ToneVerticalOffsets,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let top = y + vertical_spacing.staves.get(&stave.key).unwrap().y;

        for track_key in &stave.tracks {
            let notation = notation_by_track.get(track_key).unwrap();
            let stem_directions = stem_directions_by_track.get(track_key).unwrap();
            let shunts = shunts_by_track.get(track_key).unwrap();

            for (tick, entry) in &notation.track {
                if entry.has_tie() {
                    let stem_direction = stem_directions.get(tick).unwrap();
                    let tie_directions = get_tie_directions(entry, stem_direction, tone_offsets);

                    for (tone_key, tie_direction) in tie_directions {
                        draw_tie(
                            x,
                            top,
                            &tone_key,
                            entry,
                            notation.track.get(&(tick + entry.duration)).unwrap(),
                            &tie_direction,
                            stem_direction,
                            horizontal_spacing,
                            shunts,
                            tone_offsets,
                            converter,
                            instructions,
                        )
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_tie_directions, TieDirections};
    use crate::components::misc::Direction;
    use crate::entries::tone::Tone;
    use crate::parse::get_written_durations::Notation;
    use rustc_hash::{FxHashMap, FxHashSet};

    fn run_tie_directions_test(
        config: Vec<(&str, i8)>,
        stem_direction: &Direction,
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
        expected.insert(String::from("a"), Direction::Down);

        let result = run_tie_directions_test(vec![("a", 0)], &Direction::Up);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_2() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), Direction::Up);

        let result = run_tie_directions_test(vec![("a", 0)], &Direction::Down);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_3() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), Direction::Up);
        expected.insert(String::from("b"), Direction::Down);

        let result = run_tie_directions_test(vec![("a", 0), ("b", 1)], &Direction::Up);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_4() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), Direction::Up);
        expected.insert(String::from("b"), Direction::Down);

        let result = run_tie_directions_test(vec![("a", 0), ("b", 1)], &Direction::Down);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_5() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), Direction::Up);
        expected.insert(String::from("b"), Direction::Down);
        expected.insert(String::from("c"), Direction::Down);

        let result = run_tie_directions_test(vec![("a", 0), ("b", 1), ("c", 2)], &Direction::Up);
        assert_eq!(result, expected);
    }

    #[test]
    fn tie_directions_test_6() {
        let mut expected: TieDirections = FxHashMap::default();
        expected.insert(String::from("a"), Direction::Up);
        expected.insert(String::from("b"), Direction::Up);
        expected.insert(String::from("c"), Direction::Down);

        let result = run_tie_directions_test(vec![("a", 0), ("b", 1), ("c", 2)], &Direction::Down);
        assert_eq!(result, expected);
    }
}

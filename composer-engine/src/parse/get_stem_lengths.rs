use super::get_beams::{Beam, Beams, BeamsByTrack};
use super::get_note_positions::Position;
use super::get_stem_directions::{StemDirection, StemDirections, StemDirectionsByTrack};
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack, NotationTrack};
use super::measure_horizontal_spacing::HorizontalSpacing;
use crate::components::measurements::Point;
use crate::components::misc::Tick;
use crate::score::engrave::Engrave;
use crate::score::stave::STAVE_LINE_WIDTH;
use std::collections::HashMap;

#[derive(Debug)]
pub enum BeamSlant {
    Up = -1,
    None = 0,
    Down = 1,
}

#[derive(Debug, Clone)]
pub struct StemDef {
    pub head: Point,
    pub tail: Point,
}
pub type StemLengths = HashMap<Tick, StemDef>;
pub type StemLengthsByTrack = HashMap<String, StemLengths>;

fn adjust_to_beam(
    beam: &Beam,
    high: &StemDef,
    low: &StemDef,
    max_slant: f32,
    output: &mut StemLengths,
) {
    // cap at max beam slant
    let diff = low.tail.y - high.tail.y;
    let low_y = if diff > max_slant {
        high.tail.y + 0.5
    } else {
        low.tail.y
    };

    // join inner stems to the angles beam
    let mut adjustment = 0.0;
    let tan_angle = (low_y - high.tail.y) / (low.tail.x - high.tail.x);
    for i in 0..beam.len() {
        let tick = beam.get(i).unwrap();
        let original = output.get_mut(tick).unwrap();

        let opp = tan_angle * (low.tail.x - original.tail.x);
        let beam_dictated_tail = low_y - opp;

        if beam_dictated_tail.abs() < original.tail.y.abs() {
            let offset = original.tail.y - beam_dictated_tail;
            if offset > adjustment {
                adjustment = offset;
            }
        }

        original.tail.y = beam_dictated_tail;
    }

    // make sure stems aren't squashed less than natural lengths
    for i in 0..beam.len() {
        let tick = beam.get(i).unwrap();
        let original = output.get_mut(tick).unwrap();
        original.tail.y += adjustment;
    }
}

fn get_beam_slant(
    beam: &Beam,
    notation: &NotationTrack,
    stem_direction: &StemDirection,
    tone_offsets: &ToneVerticalOffsets,
) -> BeamSlant {
    let start = notation.track.get(beam.first().unwrap()).unwrap();
    let stop = notation.track.get(beam.last().unwrap()).unwrap();

    let start_guide = start.get_beam_guide_note(stem_direction, tone_offsets);
    let stop_guide = stop.get_beam_guide_note(stem_direction, tone_offsets);

    if start_guide == stop_guide {
        return BeamSlant::None;
    }

    // check the melodic shape, we flatten in certain conditions
    for i in 1..(beam.len() - 1) {
        let tick = beam.get(i).unwrap();
        let entry = notation.track.get(tick).unwrap();
        let guide = entry.get_beam_guide_note(stem_direction, tone_offsets);

        match stem_direction {
            StemDirection::Up => {
                if guide < start_guide && guide < stop_guide {
                    return BeamSlant::None;
                }
            }
            StemDirection::Down => {
                if guide > start_guide && guide > stop_guide {
                    return BeamSlant::None;
                }
            }
        }
    }

    if start_guide < stop_guide {
        BeamSlant::Down
    } else {
        BeamSlant::Up
    }
}

fn get_furthest_tail(stems: &[StemDef], stem_direction: &StemDirection) -> f32 {
    let mut furthest = 0.0;
    for def in stems {
        match stem_direction {
            StemDirection::Up => {
                if def.tail.y < furthest {
                    furthest = def.tail.y;
                }
            }
            StemDirection::Down => {
                if def.tail.y > furthest {
                    furthest = def.tail.y;
                }
            }
        }
    }
    furthest
}

fn get_natural_stem_length(
    tick: &Tick,
    entry: &Notation,
    tone_offsets: &ToneVerticalOffsets,
    stem_direction: &StemDirection,
    horizontal_spacing: &HorizontalSpacing,
) -> StemDef {
    let (highest, lowest, _) = entry.get_tone_offset_info(tone_offsets);

    let (x, head, tail) = match stem_direction {
        StemDirection::Up => {
            let head = (lowest as f32 - 0.5) / 2.0;
            let mut tail = (highest as f32 - 0.5) / 2.0 - 3.25;
            if tail > 0.0 {
                tail = 0.0
            }
            let x = horizontal_spacing
                .get(tick, &Position::PostNoteSlot)
                .unwrap()
                .x;
            (x - (STAVE_LINE_WIDTH / 2.0), head, tail)
        }
        StemDirection::Down => {
            let head = (highest as f32 + 0.5) / 2.0;
            let mut tail = (lowest as f32 + 0.5) / 2.0 + 3.25;
            if tail < 0.0 {
                tail = 0.0
            }
            let x = horizontal_spacing.get(tick, &Position::NoteSlot).unwrap().x;
            (x + (STAVE_LINE_WIDTH / 2.0), head, tail)
        }
    };

    StemDef {
        head: Point { x, y: head },
        tail: Point { x, y: tail },
    }
}

pub fn get_stem_lengths_in_track(
    notation: &NotationTrack,
    stem_directions: &StemDirections,
    tone_offsets: &ToneVerticalOffsets,
    horizontal_spacing: &HorizontalSpacing,
    beams: &Beams,
    engrave: &Engrave,
) -> StemLengths {
    let mut output: StemLengths = HashMap::new();

    // get natural lengths of stems
    for (tick, entry) in &notation.track {
        if !entry.is_rest() {
            let stem_direction = stem_directions.get(tick).unwrap();
            output.insert(
                *tick,
                get_natural_stem_length(
                    tick,
                    entry,
                    tone_offsets,
                    stem_direction,
                    horizontal_spacing,
                ),
            );
        }
    }

    // for each beam adjust stems to meet beam angle
    for beam in beams {
        let start_tick = beam.first().unwrap();
        let stem_direction = stem_directions.get(start_tick).unwrap();
        let slant = get_beam_slant(beam, notation, stem_direction, tone_offsets);

        let stems: Vec<StemDef> = beam
            .iter()
            .map(|tick| output.get(tick).unwrap().clone())
            .collect();

        match slant {
            BeamSlant::Up => match stem_direction {
                StemDirection::Up => {
                    let low = stems.first().unwrap();
                    let high = stems.last().unwrap();
                    adjust_to_beam(beam, high, low, engrave.max_beam_slant, &mut output);
                }
                StemDirection::Down => {
                    let low = stems.first().unwrap();
                    let high = stems.last().unwrap();
                    adjust_to_beam(beam, high, low, engrave.max_beam_slant, &mut output);
                }
            },
            BeamSlant::None => {
                let furthest = get_furthest_tail(&stems, stem_direction);
                for tick in beam {
                    let original = output.get_mut(tick).unwrap();
                    original.tail.y = furthest;
                }
            }
            BeamSlant::Down => match stem_direction {
                StemDirection::Up => {
                    let high = stems.first().unwrap();
                    let low = stems.last().unwrap();
                    adjust_to_beam(beam, high, low, engrave.max_beam_slant, &mut output);
                }
                StemDirection::Down => {
                    let high = stems.first().unwrap();
                    let low = stems.last().unwrap();
                    adjust_to_beam(beam, high, low, engrave.max_beam_slant, &mut output);
                }
            },
        }
    }

    output
}

pub fn get_stem_lengths(
    tracks: &NotationByTrack,
    tone_offsets: &ToneVerticalOffsets,
    horizontal_spacing: &HorizontalSpacing,
    stem_directions_by_track: &StemDirectionsByTrack,
    beams_by_track: &BeamsByTrack,
    engrave: &Engrave,
) -> StemLengthsByTrack {
    let mut output: StemLengthsByTrack = HashMap::new();

    for (track_key, track) in tracks {
        let stem_directions = stem_directions_by_track.get(track_key).unwrap();
        let beams = beams_by_track.get(track_key).unwrap();
        let stem_lengths = get_stem_lengths_in_track(
            track,
            stem_directions,
            tone_offsets,
            horizontal_spacing,
            beams,
            engrave,
        );
        output.insert(track_key.clone(), stem_lengths);
    }

    output
}
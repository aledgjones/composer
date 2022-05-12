use super::get_bars::Bars;
use super::get_written_durations::{Notation, NotationByTrack, NotationTrack};
use crate::components::duration::NoteDuration;
use crate::components::misc::{Tick, Ticks};
use crate::entries::time_signature::TimeSignature;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::{Debug, Formatter, Result};
use std::iter::FromIterator;

pub struct Beam {
    pub ticks: FxHashSet<Tick>,
    pub start: Tick,
    pub stop: Tick,
}

impl Debug for Beam {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output = Vec::new();
        for tick in &self.ticks {
            output.push(tick);
        }

        write!(f, "{:?}", output)
    }
}

pub type Beams = Vec<Beam>;
pub type BeamsByTrack = FxHashMap<String, Beams>;
pub type Span = Vec<Tick>;

fn grouping_is_beamable(
    notation: &NotationTrack,
    start: Tick,
    stop: Tick,
    sixteenth: Ticks,
) -> bool {
    for tick in start..stop {
        if let Some(entry) = notation.track.get(&tick) {
            // TODO: make this more sophisticated, -eee | eee- can be beamed -ee- cannot, for example
            // for now just bail out if there are rests
            if entry.is_rest() || entry.duration <= sixteenth {
                return false;
            }
        }
    }

    true
}

fn assign_span(spans: &mut Beams, span: Span) -> Span {
    if span.len() > 1 {
        let start = *span.first().unwrap();
        let stop = *span.last().unwrap();
        let ticks = FxHashSet::from_iter(span);
        spans.push(Beam { ticks, start, stop });
    }

    Vec::new()
}

pub fn get_beams_in_track(notation: &NotationTrack, barlines: &Bars, subdivisions: Ticks) -> Beams {
    let mut output: Beams = Vec::new();

    let mut current_span: Span = Vec::new();
    let mut time_signature = &TimeSignature::default();
    let mut boundries = time_signature.groupings_to_ticks(&0, subdivisions);
    let mut break_at_beats = false;

    for tick in 0..notation.length {
        if let Some(entry) = barlines.get(&tick) {
            time_signature = entry;
            boundries = time_signature.groupings_to_ticks(&tick, subdivisions);
        }

        if boundries.contains(&tick) {
            current_span = assign_span(&mut output, current_span);
            break_at_beats = match time_signature.beat_type {
                // quarters break if rhythm has durations < sixteenth
                NoteDuration::Quarter => {
                    let i = boundries.iter().position(|entry| entry == &tick).unwrap();
                    let stop = boundries.get(i + 1).unwrap();
                    let sixteenth = NoteDuration::Sixteenth.to_ticks(subdivisions);
                    !grouping_is_beamable(notation, tick, *stop, sixteenth)
                }
                // larger beats break, smaller don't
                _ => {
                    let quarter = NoteDuration::Quarter.to_ticks(subdivisions);
                    time_signature.ticks_per_beat(subdivisions) > quarter
                }
            }
        }

        if break_at_beats && time_signature.is_on_beat(tick, subdivisions) {
            current_span = assign_span(&mut output, current_span);
        }

        if let Some(entry) = notation.track.get(&tick) {
            if entry.is_beamable(subdivisions) {
                current_span.push(tick);
            } else {
                current_span = assign_span(&mut output, current_span);
            }
        }
    }

    assign_span(&mut output, current_span);

    output
}

pub fn get_beams(tracks: &NotationByTrack, bars: &Bars, subdivisions: Ticks) -> BeamsByTrack {
    let mut output: BeamsByTrack = FxHashMap::default();

    for (track_key, track) in tracks {
        let beams = get_beams_in_track(track, bars, subdivisions);
        output.insert(track_key.clone(), beams);
    }

    output
}

pub fn get_has_beam(tick: &Tick, beams: &Beams) -> bool {
    for beam in beams {
        if beam.ticks.contains(tick) {
            return true;
        }
    }

    false
}

impl Notation {
    pub fn is_beamable(&self, subdivisions: Ticks) -> bool {
        if self.is_rest() {
            false
        } else {
            match self.base_to_ticks(subdivisions) {
                Some(base) => base <= NoteDuration::Eighth.to_ticks(subdivisions),
                None => false,
            }
        }
    }
}

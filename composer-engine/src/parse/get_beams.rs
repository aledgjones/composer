use super::get_bars::Bars;
use super::get_written_durations::{Notation, NotationByTrack, NotationTrack};
use crate::components::duration::NoteDuration;
use crate::components::misc::{Tick, Ticks};
use crate::entries::time_signature::TimeSignature;
use rustc_hash::FxHashMap;
use std::fmt::{Debug, Formatter, Result};

pub type BeamTicks = FxHashMap<Tick, u8>;

#[derive(Clone)]
pub struct Beam {
    pub ticks: BeamTicks,
    pub start: Tick,
    pub stop: Tick,
}

impl Beam {
    pub fn new() -> Self {
        Self {
            ticks: FxHashMap::default(),
            start: 0,
            stop: 0,
        }
    }
}

impl Default for Beam {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Beam {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output = Vec::new();
        for tick in &self.ticks {
            output.push((self.start, tick));
        }

        write!(f, "{:?}", output)
    }
}

pub type Beams = Vec<Beam>;
pub type BeamsByTrack = FxHashMap<String, Beams>;

enum EighthType {
    Rest,
    Note,
}

/// eighths can beamed in certain patterns
fn is_pattern(
    pattern: &[EighthType],
    notation: &NotationTrack,
    start: Tick,
    subdivisions: Ticks,
) -> bool {
    let eighth = NoteDuration::Eighth.to_ticks(subdivisions);

    for (i, eighth_type) in pattern.iter().enumerate() {
        match notation.track.get(&(start + (i as u32 * eighth))) {
            Some(entry) => match eighth_type {
                EighthType::Rest => {
                    if !(entry.is_rest() && entry.duration == eighth) {
                        return false;
                    }
                }
                EighthType::Note => {
                    if entry.is_rest() || entry.duration != eighth {
                        return false;
                    }
                }
            },
            None => return false,
        };
    }

    true
}

fn grouping_is_beamable(
    beats: u8,
    notation: &NotationTrack,
    start: Tick,
    stop: Tick,
    subdivisions: Ticks,
) -> bool {
    // beat specific patterns
    if let 4 = beats {
        // -eee
        let pattern = vec![
            EighthType::Rest,
            EighthType::Note,
            EighthType::Note,
            EighthType::Note,
        ];
        if is_pattern(&pattern, notation, start, subdivisions) {
            return true;
        }

        // eee-
        let pattern = vec![
            EighthType::Note,
            EighthType::Note,
            EighthType::Note,
            EighthType::Rest,
        ];
        if is_pattern(&pattern, notation, start, subdivisions) {
            return true;
        }
    }

    let sixteenth = NoteDuration::Sixteenth.to_ticks(subdivisions);
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

fn assign_beam(beams: &mut Beams, ticks: BeamTicks) -> BeamTicks {
    if ticks.len() > 1 {
        let mut min: Option<Tick> = None;
        let mut max: Option<Tick> = None;

        for tick in ticks.keys() {
            match min {
                None => min = Some(*tick),
                Some(val) => {
                    if tick < &val {
                        min = Some(*tick)
                    }
                }
            }

            match max {
                None => max = Some(*tick),
                Some(val) => {
                    if tick > &val {
                        max = Some(*tick)
                    }
                }
            }
        }

        beams.push(Beam {
            ticks,
            start: min.unwrap(),
            stop: max.unwrap(),
        });
    }

    FxHashMap::default()
}

pub fn get_beams_in_track(notation: &NotationTrack, barlines: &Bars, subdivisions: Ticks) -> Beams {
    let mut output: Beams = Vec::new();

    let mut ticks: BeamTicks = FxHashMap::default();
    let mut time_signature = &TimeSignature::default();
    let mut boundries = time_signature.groupings_to_ticks(0, subdivisions);
    let mut break_at_beats = false;

    for tick in 0..notation.length {
        if let Some(entry) = barlines.get(&tick) {
            time_signature = entry;
            boundries = time_signature.groupings_to_ticks(tick, subdivisions);
        }

        if boundries.contains(&tick) {
            ticks = assign_beam(&mut output, ticks);

            break_at_beats = match time_signature.beat_type {
                // quarters break if rhythm has durations < sixteenth
                NoteDuration::Quarter => {
                    let i = boundries.iter().position(|entry| entry == &tick).unwrap();
                    let stop = boundries.get(i + 1).unwrap();
                    !grouping_is_beamable(time_signature.beats, notation, tick, *stop, subdivisions)
                }
                // larger beats break, smaller don't
                _ => {
                    let quarter = NoteDuration::Quarter.to_ticks(subdivisions);
                    time_signature.ticks_per_beat(subdivisions) > quarter
                }
            }
        }

        if break_at_beats && time_signature.is_on_beat(tick, subdivisions) {
            ticks = assign_beam(&mut output, ticks);
        }

        if let Some(entry) = notation.track.get(&tick) {
            let count = entry.get_beam_count(subdivisions);
            if count > 0 {
                ticks.insert(tick, count);
            } else {
                ticks = assign_beam(&mut output, ticks);
            }
        }
    }

    assign_beam(&mut output, ticks);

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

impl Notation {
    pub fn get_beam_count(&self, subdivisions: Ticks) -> u8 {
        if self.is_rest() {
            0
        } else {
            match self.base_to_ticks(subdivisions) {
                Some(base) => (NoteDuration::Eighth.to_ticks(subdivisions) / base) as u8,
                None => 0,
            }
        }
    }
}

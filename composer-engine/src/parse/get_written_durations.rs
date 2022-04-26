use super::get_barlines::Barlines;
use crate::components::duration::is_writable;
use crate::components::duration::NoteDuration;
use crate::components::duration::NOTE_DURATIONS;
use crate::components::misc::Tick;
use crate::components::misc::Ticks;
use crate::entries::tone::Tone;
use crate::score::tracks::Track;
use crate::utils::log;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Notation {
    pub tones: Vec<Tone>,
    pub duration: Ticks,
    pub ties: HashSet<String>,
}

impl Notation {
    pub fn new(length: Ticks) -> Self {
        Self {
            tones: Vec::new(),
            duration: length,
            ties: HashSet::new(),
        }
    }

    pub fn is_rest(&self) -> bool {
        self.tones.is_empty()
    }

    pub fn longest_written_duration(&self, subdivisions: u8) -> NoteDuration {
        for option in NOTE_DURATIONS {
            if option.to_ticks(subdivisions) < self.duration {
                return option;
            }
        }

        NoteDuration::Whole
    }

    pub fn is_writable(&self, subdivisions: u8) -> bool {
        if is_writable(self.duration, subdivisions) {
            true
        } else {
            let dotted = (self.duration as f32 / 3.0) * 2.0;
            // ensure we don't get a false match when we lose a fraction
            // while converting to u32
            if dotted.fract() == 0.0 {
                is_writable(dotted as u32, subdivisions)
            } else {
                false
            }
        }
    }
}

pub struct NotationTrack {
    pub length: u32,
    pub track: HashMap<Tick, Notation>,
}

impl NotationTrack {
    pub fn new(length: Ticks) -> Self {
        let mut track = HashMap::new();
        track.insert(
            0,
            Notation {
                tones: Vec::new(),
                duration: length,
                ties: HashSet::new(),
            },
        );
        Self { length, track }
    }

    pub fn get_previous_notation(&self, at: Tick) -> Option<(Tick, Notation)> {
        for tick in (0..at + 1).rev() {
            match self.track.get(&tick) {
                Some(notation) => return Some((tick, notation.clone())),
                None => continue,
            }
        }

        None
    }

    pub fn insert(&mut self, tick: Tick, notation: Notation) {
        self.track.insert(tick, notation);
    }

    pub fn add_tone(&mut self, tick: Tick, tone: &Tone) {
        let entry = self.track.get_mut(&tick).unwrap();
        entry.tones.push(tone.clone());
    }

    pub fn split(&mut self, split_at: Tick) {
        if let Some((event_at, notation)) = self.get_previous_notation(split_at) {
            // only split if:
            // 1. split index not already the start of an event.
            // 2. split index is not the end of an event (ie; end of flow);
            if event_at != split_at && split_at != event_at + notation.duration {
                self.insert(
                    event_at,
                    Notation {
                        tones: notation.tones.clone(),
                        duration: split_at - event_at,
                        ties: notation.tones.iter().map(|tone| tone.key.clone()).collect(),
                    },
                );

                self.insert(
                    split_at,
                    Notation {
                        tones: notation.tones.clone(),
                        duration: event_at + notation.duration - split_at,
                        ties: notation.ties,
                    },
                );
            }
        }
    }

    pub fn split_at_tone_events(&mut self, track: &Track) {
        for tick in 0..self.length {
            let tones = track.get_tones_at_tick(tick);
            if !tones.is_empty() {
                self.split(tick);
                for tone in tones {
                    self.split(tick + tone.duration);
                    self.add_tone(tick, &tone);
                }
            }
        }
    }

    pub fn get_beat_group_boundries(
        start: Tick,
        ticks_per_beat: Ticks,
        groupings: &[u8],
    ) -> Vec<Tick> {
        let mut output: Vec<Tick> = vec![start];

        let mut progress = start;
        for grouping in groupings {
            let ticks_in_grouping = *grouping as u32 * ticks_per_beat;
            progress += ticks_in_grouping;
            output.push(progress);
        }

        output
    }

    pub fn is_empty(&self, start: Tick, stop: Tick) -> bool {
        for tick in start + 1..stop {
            match self.track.get(&tick) {
                Some(_) => return false,
                None => continue,
            }
        }

        true
    }

    pub fn split_unit(
        &mut self,
        start: Tick,
        stop: Tick,
        subdivisions: u8,
        beats: u8,
        beat_type: &NoteDuration,
        ticks_per_origional_beat: Ticks,
        groupings: &Vec<u8>,
        is_full_bar: bool,
    ) {
        let ticks_per_beat = beat_type.to_ticks(subdivisions);
        let boundries = NotationTrack::get_beat_group_boundries(start, ticks_per_beat, groupings);

        // if the unit is empty we stop the reccursion as there is no need for higher fidelity
        if self.is_empty(start, stop) {
            let entry = self.track.get(&start).unwrap();
            if is_full_bar && !entry.is_rest() && !entry.is_writable(subdivisions) {
                let last_beat = *boundries.get(boundries.len() - 2).unwrap();
                self.split(last_beat);
            }
            return;
        }

        if boundries.len() == 2 || boundries.len() == 4 {}

        if boundries.len() == 3 {
            let first_beat = *boundries.get(0).unwrap();
            let second_beat = *boundries.get(0).unwrap();
            let third_beat = *boundries.get(0).unwrap();

            // split all rests at beats
            for boundry in boundries {
                if let Some((_, entry)) = self.get_previous_notation(boundry) {
                    if entry.is_rest() {
                        self.split(boundry);
                    }
                }
            }

            // make sure it doesn't look compound! (c. at end of bar)
            let middle = start + ((stop - start) as f32 / 2.0) as u32;
            if self.track.contains_key(&middle) && self.is_empty(middle, stop) {
                self.split(third_beat);
            }

            // sustain two beats into one
            if !self.is_empty(first_beat, second_beat) {
                self.split(second_beat);
            }

            // if we haven't made any splits we split at the first boundry
            if !self.track.contains_key(&second_beat) && !self.track.contains_key(&third_beat) {
                self.split(third_beat);
            }
        }
    }

    pub fn split_measures(&mut self, barlines: &Barlines) {
        for tick in barlines.keys() {
            self.split(*tick);
        }
    }

    pub fn split_as_per_meter(&mut self, barlines: &Barlines) {
        for (tick, time_signature) in barlines {
            let subdivisions = time_signature.subdivisions;

            let start = *tick;
            let stop = tick + time_signature.ticks_per_bar();

            self.split_unit(
                start,
                stop,
                subdivisions,
                time_signature.beats,
                &time_signature.beat_type,
                time_signature.ticks_per_beat(),
                &time_signature.groupings,
                true,
            );
        }
    }

    pub fn split_unwritable(&mut self, flow_length: Ticks, barlines: &Barlines) {
        let mut time = barlines.get(&0).unwrap();
        for tick in 0..flow_length {
            if barlines.contains_key(&tick) {
                time = barlines.get(&tick).unwrap();
            }

            if let Some(entry) = self.track.get(&tick) {
                if !entry.is_rest() && !entry.is_writable(time.subdivisions) {
                    let longest = entry.longest_written_duration(time.subdivisions);
                    self.split(tick + longest.to_ticks(time.subdivisions));
                }
            };
        }
    }
}

impl Debug for NotationTrack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output: Vec<char> = Vec::new();
        output.resize_with(self.length as usize, || '.');

        for entry in &self.track {
            let (tick, notation) = entry;
            let is_rest = notation.tones.is_empty();
            let has_tie = !notation.ties.is_empty();

            let start = *tick as usize;
            let stop = (tick + notation.duration) as usize;
            for i in start..stop {
                if i == start {
                    if is_rest {
                        output[i] = 'r';
                    } else {
                        output[i] = 'o';
                    }
                } else if i == stop - 1 && !has_tie {
                    output[i] = ':';
                } else if has_tie {
                    output[i] = '_';
                } else {
                    output[i] = '-';
                }
            }
        }

        write!(f, "{}", String::from_iter(output))
    }
}

pub type NotationTracks = HashMap<String, NotationTrack>;

pub fn get_written_durations(
    flow_length: Ticks,
    tracks: &[&Track],
    barlines: &Barlines,
) -> NotationTracks {
    let mut entries = NotationTracks::new();

    for track in tracks {
        let notation = track.to_notation_track(flow_length, barlines);
        entries.insert(track.key.clone(), notation);
    }

    entries
}

impl Track {
    pub fn to_notation_track(&self, flow_length: Ticks, barlines: &Barlines) -> NotationTrack {
        let mut notation = NotationTrack::new(flow_length);
        notation.split_at_tone_events(self);
        notation.split_measures(barlines);
        notation.split_as_per_meter(barlines);
        notation.split_unwritable(flow_length, barlines);
        notation
    }
}

use super::get_barlines::Barlines;
use super::get_tone_offsets::ToneVerticalOffsets;
use crate::components::duration::is_writable;
use crate::components::duration::NOTE_DURATIONS;
use crate::components::misc::Tick;
use crate::components::misc::Ticks;
use crate::entries::time_signature::TimeSignature;
use crate::entries::time_signature::TimeSignatureDrawType;
use crate::entries::tone::Tone;
use crate::score::tracks::Track;
use crate::utils::log;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::FromIterator;

pub type Clusters<'a> = Vec<Cluster<'a>>;
pub type Cluster<'a> = Vec<&'a Tone>;
pub type NotationTracks = HashMap<String, NotationTrack>;

#[derive(Debug, Clone)]
pub struct Notation {
    pub tones: Vec<Tone>,
    pub duration: Ticks,
    pub ties: HashSet<String>,
}

impl Notation {
    pub fn is_rest(&self) -> bool {
        self.tones.is_empty()
    }

    pub fn longest_written_duration(&self, subdivisions: u8) -> Ticks {
        for option in NOTE_DURATIONS {
            let ticks = option.to_ticks(subdivisions);
            if ticks < self.duration {
                return ticks;
            }
        }

        0
    }

    // gets base duration from a possibly dotted duration
    pub fn base_duration(&self, subdivisions: u8) -> Option<Ticks> {
        if is_writable(self.duration, subdivisions) {
            // original duration is directly writable
            Some(self.duration)
        } else {
            // see if duration is dotted
            let base_duration = (self.duration as f32 / 3.0) * 2.0;
            if base_duration.fract() == 0.0 && is_writable(base_duration as u32, subdivisions) {
                Some(base_duration as Tick)
            } else {
                None
            }
        }
    }

    pub fn is_writable(&self, subdivisions: u8) -> bool {
        self.base_duration(subdivisions).is_some()
    }

    /// creates a new vec of sort tones by offset -- ascending *pitch*
    pub fn sort_tones(&self, tone_offsets: &ToneVerticalOffsets) -> Vec<Tone> {
        let mut tones = self.tones.clone();
        tones.sort_by(|a, b| {
            let a = tone_offsets.get(&a.key).unwrap();
            let b = tone_offsets.get(&b.key).unwrap();
            b.cmp(a)
        });
        tones
    }

    pub fn get_clusters(&self, tone_offsets: &ToneVerticalOffsets) -> Clusters {
        let tones = self.sort_tones(tone_offsets);

        let mut clusters: Clusters = Vec::new();
        let mut cluster: Cluster = Vec::new();
        let mut previous_tone = self.tones.first().unwrap();

        for i in 1..tones.len() {
            let current_tone = self.tones.get(i).unwrap();
            let current_offset = tone_offsets.get(&current_tone.key).unwrap();
            let previous_offset = tone_offsets.get(&previous_tone.key).unwrap();

            cluster.push(previous_tone);

            // if not a cluster
            if previous_offset - current_offset > 1 {
                clusters.push(cluster);
                cluster = Vec::new();
            }

            previous_tone = current_tone;
        }

        if !tones.is_empty() {
            cluster.push(previous_tone);
            clusters.push(cluster);
        }

        clusters
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

    pub fn is_range_empty(&self, start: &Tick, stop: &Tick) -> bool {
        for tick in start + 1..*stop {
            match self.track.get(&tick) {
                Some(_) => return false,
                None => continue,
            }
        }

        true
    }

    pub fn is_tick_empty(&self, tick: &Tick) -> bool {
        self.track.get(tick).is_none()
    }

    pub fn is_tick_rest(&self, tick: &Tick) -> bool {
        match self.track.get(tick) {
            Some(entry) => entry.is_rest(),
            None => false,
        }
    }

    // [qhq] patterns dont't split middle
    pub fn is_qhq_pattern(&self, start: &Tick, time_signature: &TimeSignature) -> bool {
        let beat_one = *start;
        let beat_two = time_signature.get_tick_at_beat(start, 2);
        let beat_four = time_signature.get_tick_at_beat(start, 4);

        !self.is_tick_empty(&beat_one)
            && !self.is_tick_empty(&beat_two)
            && !self.is_tick_empty(&beat_four)
            && (!self.is_tick_rest(&beat_one) && !self.is_tick_rest(&beat_four))
            && !self.is_tick_rest(&beat_two)
            && self.is_range_empty(&beat_one, &beat_two)
            && self.is_range_empty(&beat_two, &beat_four)
    }

    /// [qm.] patterns don't split middle
    pub fn is_qmdot_pattern(&self, start: &Tick, time_signature: &TimeSignature) -> bool {
        let beat_one = *start;
        let beat_two = time_signature.get_tick_at_beat(start, 2);
        let end = start + time_signature.ticks_per_bar();

        !self.is_tick_empty(&beat_one)
            && !self.is_tick_empty(&beat_two)
            && !self.is_tick_rest(&beat_two)
            && self.is_range_empty(&beat_one, &beat_two)
            && self.is_range_empty(&beat_two, &end)
    }

    /// [m.q] patterns don't split middle
    pub fn is_mdotq_pattern(
        &self,
        start: &Tick,
        time_signature: &TimeSignature,
        original_time_signature: &TimeSignature,
    ) -> bool {
        let beat_one = *start;
        let beat_four = time_signature.get_tick_at_beat(start, 4);

        let middle = start + (time_signature.ticks_per_bar() / 2);

        let is_pattern = !self.is_tick_empty(&beat_one)
            && !self.is_tick_empty(&beat_four)
            && self.is_range_empty(&beat_one, &beat_four);

        let are_both_notes = !self.is_tick_rest(&beat_one) && !self.is_tick_rest(&beat_four);

        let is_allowed_with_rest = !self.is_tick_rest(&beat_one)
            && self.is_tick_rest(&beat_four)
            && match self.track.get(&beat_one) {
                Some(entry) => {
                    entry.duration != (original_time_signature.ticks_per_beat() as f32 * 1.5) as u32
                }
                None => false,
            };

        let intersect_beat = original_time_signature.is_on_beat(middle);

        is_pattern && (are_both_notes || is_allowed_with_rest || !intersect_beat)
    }

    // time_signature represents the unit as if it were a "measure"
    pub fn split_unit(
        &mut self,
        start: &u32,
        time_signature: &TimeSignature,
        original_time_signature: &TimeSignature,
    ) {
        let stop = start + time_signature.ticks_per_bar();

        // self.debug(*start, stop);

        // we stop once there are no events in the range
        if self.is_range_empty(start, &stop) {
            return;
        }

        // convert 2 beats to 4 at hgher fidelity
        if time_signature.beats == 2 {
            return self.split_unit(
                start,
                &TimeSignature::new(
                    time_signature.tick,
                    4,
                    time_signature.beat_type.half(),
                    TimeSignatureDrawType::Normal,
                    None,
                ),
                original_time_signature,
            );
        }

        match time_signature.beats {
            3 => {
                let beat_one = *start;
                let beat_two = time_signature.get_tick_at_beat(start, 2);
                let beat_three = time_signature.get_tick_at_beat(start, 3);

                // split all rests at beats
                for beat in [beat_one, beat_two, beat_three] {
                    if let Some((_, entry)) = self.get_previous_notation(beat) {
                        if entry.is_rest() {
                            self.split(beat);
                        }
                    }
                }

                // make sure it doesn't look compound! (c. at end of bar)
                let middle = start + (time_signature.ticks_per_bar() / 2);
                if !self.is_tick_empty(&middle) && self.is_range_empty(&middle, &stop) {
                    self.split(beat_three);
                };

                // allow sustaining two beats into one
                if !self.is_range_empty(&beat_one, &beat_two) {
                    self.split(beat_two);
                }

                // if we haven't made any splits we split at the third beat
                if !self.is_tick_empty(&beat_two) && !self.is_tick_empty(&beat_three) {
                    self.split(beat_three);
                }

                let next = TimeSignature::new(
                    time_signature.tick,
                    4,
                    time_signature.beat_type.half().half(),
                    TimeSignatureDrawType::Normal,
                    None,
                );

                self.split_unit(&beat_one, &next, original_time_signature);
                self.split_unit(&beat_two, &next, original_time_signature);
                self.split_unit(&beat_three, &next, original_time_signature);
            }
            4 => {
                let middle = start + (time_signature.ticks_per_bar() / 2);

                if !self.is_qhq_pattern(start, time_signature)
                    && !self.is_qmdot_pattern(start, time_signature)
                    && !self.is_mdotq_pattern(start, time_signature, original_time_signature)
                {
                    self.split(middle);
                }

                let next = TimeSignature::new(
                    time_signature.tick,
                    4,
                    time_signature.beat_type.half(),
                    TimeSignatureDrawType::Normal,
                    None,
                );

                self.split_unit(start, &next, original_time_signature);
                self.split_unit(&middle, &next, original_time_signature);
            }
            _ => (),
        }
    }

    pub fn split_as_per_meter(&mut self, barlines: &Barlines) {
        for (tick, time_signature) in barlines {
            self.split_unit(tick, time_signature, time_signature);
        }
    }

    pub fn split_measures(&mut self, barlines: &Barlines) {
        for tick in barlines.keys() {
            self.split(*tick);
        }
    }

    pub fn split_at_tone_events(&mut self, track: &Track) {
        for tick in 0..self.length {
            let tones = track.get_tones_at_tick(&tick);
            if !tones.is_empty() {
                self.split(tick);
                for tone in tones {
                    self.split(tick + tone.duration);
                    self.add_tone(tick, &tone);
                }
            }
        }
    }

    pub fn split_unwritable(&mut self, barlines: &Barlines) {
        for (i, time) in barlines {
            for tick in *i..*i + time.ticks_per_bar() {
                if let Some(entry) = self.track.get(&tick) {
                    if !entry.is_rest() && !entry.is_writable(time.subdivisions) {
                        let longest = entry.longest_written_duration(time.subdivisions);
                        self.split(tick + longest);
                    }
                };
            }
        }
    }

    pub fn debug(&self, start: Tick, stop: Tick) {
        log(&format!("{:?}", self));

        let mut annotation = String::new();
        for i in 0..stop {
            if i >= start && i < stop {
                annotation.push('^');
            } else {
                annotation.push(' ');
            }
        }
        log(&annotation);
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
        notation.split_unwritable(barlines);
        notation
    }
}

#[cfg(test)]
mod tests {
    use crate::entries::tone::Tone;
    use crate::parse::get_written_durations::Clusters;
    use crate::parse::get_written_durations::Notation;
    use std::collections::HashSet;

    #[test]
    fn sort_tones_test() {
        let notation = Notation {
            tones: vec![
                Tone::tester("a"),
                Tone::tester("b"),
                Tone::tester("c"),
                Tone::tester("d"),
                Tone::tester("e"),
            ],
            duration: 0,
            ties: HashSet::new(),
        };

        let tone_offsets = hashmap! {
            String::from("a") => 0,
            String::from("b") => 1,
            String::from("c") => -1,
            String::from("d") => 2,
            String::from("e") => -2
        };

        let result = notation.sort_tones(&tone_offsets);
        let expected = ["d", "b", "a", "c", "e"];
        for (i, tone) in result.iter().enumerate() {
            assert_eq!(&tone.key, expected[i]);
        }
    }

    #[test]
    fn get_clusters_test() {
        let notation = Notation {
            tones: vec![
                Tone::tester("a"),
                Tone::tester("b"),
                Tone::tester("c"),
                Tone::tester("d"),
                Tone::tester("e"),
                Tone::tester("f"),
            ],
            duration: 0,
            ties: HashSet::new(),
        };

        let tone_offsets = hashmap! {
            String::from("a") => 2,
            String::from("b") => 1,
            String::from("c") => -1,
            String::from("d") => -3,
            String::from("e") => -4,
            String::from("f") => -5
        };

        let result: Clusters = notation.get_clusters(&tone_offsets);
        assert_eq!(result.len(), 3);

        let expected: Vec<Vec<&str>> = vec![vec!["a", "b"], vec!["c"], vec!["d", "e", "f"]];
        for (i, expected_cluster) in expected.iter().enumerate() {
            let result_cluster = result.get(i).unwrap();
            for (ii, expected_key) in expected_cluster.iter().enumerate() {
                let result_tone = result_cluster.get(ii).unwrap();
                assert_eq!(&result_tone.key, expected_key);
            }
        }
    }
}

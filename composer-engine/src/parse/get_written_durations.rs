use super::get_bars::Bars;
use super::get_beams::Beams;
use super::get_shunts::Shunt;
use super::get_shunts::Shunts;
use super::get_tone_offsets::get_tone_offset_info;
use super::get_tone_offsets::ToneVerticalOffsets;
use crate::components::duration::is_writable;
use crate::components::duration::NoteDuration;
use crate::components::duration::NOTE_DURATIONS;
use crate::components::measurements::BoundingBox;
use crate::components::measurements::PaddingSpaces;
use crate::components::misc::Direction;
use crate::components::misc::Tick;
use crate::components::misc::Ticks;
use crate::components::units::Space;
use crate::entries::time_signature::TimeSignature;
use crate::entries::time_signature::TimeSignatureDrawType;
use crate::entries::tone::Tone;
use crate::score::engrave::Engrave;
use crate::score::flows::Flow;
use crate::score::tracks::Track;
use crate::utils::log;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::iter::FromIterator;

pub type Clusters = Vec<Cluster>;
pub type Cluster = Vec<Tone>;
pub type NotationByTrack = FxHashMap<String, NotationTrack>;

#[derive(Debug)]
pub struct Notation {
    pub tick: Tick,
    pub tones: Vec<Tone>,
    pub duration: Ticks,
    pub ties: FxHashSet<String>,
}

impl Notation {
    pub fn is_rest(&self) -> bool {
        self.tones.is_empty()
    }

    pub fn is_chord(&self) -> bool {
        self.tones.len() > 1
    }

    pub fn longest_written_duration(&self, subdivisions: Ticks) -> Ticks {
        for option in NOTE_DURATIONS {
            let ticks = option.to_ticks(subdivisions);
            if ticks < self.duration {
                return ticks;
            }
        }

        0
    }

    // gets base duration from a possibly dotted duration
    pub fn base_to_ticks(&self, subdivisions: Ticks) -> Option<Ticks> {
        if is_writable(&self.duration, subdivisions) {
            // original duration is directly writable
            Some(self.duration)
        } else {
            // see if duration is dotted
            let base_duration = (self.duration as f32 / 3.0) * 2.0;
            if base_duration.fract() == 0.0 && is_writable(&(base_duration as u32), subdivisions) {
                Some(base_duration as Tick)
            } else {
                None
            }
        }
    }

    pub fn base_to_note_duration(&self, subdivisions: Ticks) -> Option<NoteDuration> {
        match self.base_to_ticks(subdivisions) {
            Some(base) => NoteDuration::from_ticks(base, subdivisions),
            None => None,
        }
    }

    pub fn glyph(&self, subdivisions: Ticks) -> String {
        if self.is_rest() {
            match self.base_to_note_duration(subdivisions) {
                Some(base) => match base {
                    NoteDuration::Whole => String::from("\u{E4E3}"),
                    NoteDuration::Half => String::from("\u{E4E4}"),
                    NoteDuration::Quarter => String::from("\u{E4E5}"),
                    NoteDuration::Eighth => String::from("\u{E4E6}"),
                    NoteDuration::Sixteenth => String::from("\u{E4E7}"),
                    NoteDuration::ThirtySecond => String::from("\u{E4E8}"),
                    NoteDuration::SixtyFourth => todo!(),
                    NoteDuration::HudredTwentyEighth => todo!(),
                },
                None => String::from("\u{E4E5}"),
            }
        } else {
            match self.base_to_note_duration(subdivisions) {
                Some(base) => match base {
                    NoteDuration::Whole => String::from("\u{E0A4}"),
                    NoteDuration::Half => String::from("\u{E0A3}"),
                    NoteDuration::Quarter => String::from("\u{E0A4}"),
                    NoteDuration::Eighth => String::from("\u{E0A4}"),
                    NoteDuration::Sixteenth => String::from("\u{E0A4}"),
                    NoteDuration::ThirtySecond => String::from("\u{E0A4}"),
                    NoteDuration::SixtyFourth => String::from("\u{E0A4}"),
                    NoteDuration::HudredTwentyEighth => String::from("\u{E0A4}"),
                },
                None => String::from("\u{E0A4}"),
            }
        }
    }

    pub fn is_writable(&self, subdivisions: Ticks) -> bool {
        self.base_to_ticks(subdivisions).is_some()
    }

    pub fn is_dotted(&self, subdivisions: Ticks) -> bool {
        if is_writable(&self.duration, subdivisions) {
            false
        } else {
            let base_duration = (self.duration as f32 / 3.0) * 2.0;
            base_duration.fract() == 0.0 && is_writable(&(base_duration as u32), subdivisions)
        }
    }

    pub fn is_flagged(&self, beams: &Beams, subdivisions: Ticks) -> bool {
        !self.is_rest()
            && !self.has_beam(beams)
            && self.duration < NoteDuration::Quarter.to_ticks(subdivisions)
    }

    pub fn flag_glyph(&self, stem_direction: &Direction, subdivisions: Ticks) -> String {
        let base = self.base_to_note_duration(subdivisions);
        match base {
            Some(duration) => String::from(duration.to_flag_glyph(stem_direction)),
            None => String::from(""),
        }
    }

    pub fn has_beam(&self, beams: &Beams) -> bool {
        for beam in beams {
            if beam.ticks.contains(&self.tick) {
                return true;
            }
        }

        false
    }

    pub fn has_pre_shunt(&self, shunts: &Shunts) -> bool {
        for tone in &self.tones {
            if let Shunt::Pre = shunts
                .by_key
                .get(&(self.tick, tone.key.to_string()))
                .unwrap()
            {
                return true;
            }
        }

        false
    }

    pub fn has_post_shunt(&self, shunts: &Shunts) -> bool {
        for tone in &self.tones {
            if let Shunt::Post = shunts.by_key.get(&(self.tick, tone.key.clone())).unwrap() {
                return true;
            }
        }

        false
    }

    pub fn get_beam_guide_note(
        &self,
        stem_direction: &Direction,
        tone_offsets: &ToneVerticalOffsets,
    ) -> i8 {
        let (highest, lowest, _) = self.get_tone_offset_info(tone_offsets);
        match stem_direction {
            Direction::Up => highest,
            Direction::Down => lowest,
        }
    }

    pub fn has_tie(&self) -> bool {
        !self.ties.is_empty()
    }

    pub fn notehead_width(&self) -> Space {
        1.175
    }

    pub fn min_spacing(
        &self,
        shunts: &Shunts,
        subdivisions: Ticks,
        engrave: &Engrave,
        beams: &Beams,
    ) -> Space {
        let mut min_space = self.notehead_width();
        if self.has_post_shunt(shunts) {
            min_space *= 2.0;
        }

        if self.is_dotted(subdivisions) || self.is_flagged(beams, subdivisions) {
            min_space += 1.0
        }

        if self.has_tie() {
            min_space += engrave.minimum_tie_space;
        } else {
            min_space += engrave.minimum_note_space;
        };

        min_space
    }

    pub fn metrics(
        &self,
        shunts: &Shunts,
        subdivisions: Ticks,
        engrave: &Engrave,
        beams: &Beams,
    ) -> BoundingBox {
        let min = self.min_spacing(shunts, subdivisions, engrave, beams);
        let spacing = match self.base_to_note_duration(subdivisions) {
            Some(base) => {
                let space = engrave.base_note_space
                    * base.spacing_ratio(engrave.note_space_ratio, self.is_dotted(subdivisions));
                if space > min {
                    space
                } else {
                    min
                }
            }
            None => min,
        };

        BoundingBox {
            width: 0.0,
            height: 1.0,
            padding: PaddingSpaces::new(0.0, spacing, 0.0, 0.0),
        }
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

    pub fn get_tone_offset_info(&self, tone_offsets: &ToneVerticalOffsets) -> (i8, i8, i8) {
        get_tone_offset_info(&self.tones, tone_offsets)
    }

    pub fn get_clusters(&self, tone_offsets: &ToneVerticalOffsets) -> Clusters {
        let tones = self.sort_tones(tone_offsets);

        let mut clusters: Clusters = Vec::new();
        let mut cluster: Cluster = Vec::new();
        let mut previous_tone = tones.first().unwrap();

        for i in 1..tones.len() {
            let current_tone = tones.get(i).unwrap();
            let current_offset = tone_offsets.get(&current_tone.key).unwrap();
            let previous_offset = tone_offsets.get(&previous_tone.key).unwrap();

            cluster.push(previous_tone.clone());

            // if not a cluster
            if previous_offset - current_offset > 1 {
                clusters.push(cluster);
                cluster = Vec::new();
            }

            previous_tone = current_tone;
        }

        if !tones.is_empty() {
            cluster.push(previous_tone.clone());
            clusters.push(cluster);
        }

        clusters
    }
}

pub struct NotationTrack {
    pub length: u32,
    pub track: FxHashMap<Tick, Notation>,
}

impl NotationTrack {
    pub fn new(length: Ticks) -> Self {
        let mut track = FxHashMap::default();
        track.insert(
            0,
            Notation {
                tick: 0,
                tones: Vec::new(),
                duration: length,
                ties: FxHashSet::default(),
            },
        );
        Self { length, track }
    }

    pub fn get_previous_notation(&self, at: Tick) -> Option<(Tick, &Notation)> {
        for tick in (0..at + 1).rev() {
            match self.track.get(&tick) {
                Some(notation) => return Some((tick, notation)),
                None => continue,
            }
        }

        None
    }

    pub fn get_next_notation(&self, from: &Tick) -> Option<(Tick, &Notation)> {
        for tick in from + 1..self.length {
            match self.track.get(&tick) {
                Some(notation) => return Some((tick, notation)),
                None => continue,
            }
        }

        None
    }

    pub fn insert(&mut self, tick: Tick, notation: Notation) {
        self.track.insert(tick, notation);
    }

    pub fn add_tone(&mut self, tick: &Tick, tone: &Tone) {
        let entry = self.track.get_mut(tick).unwrap();
        entry.tones.push(tone.clone());
    }

    pub fn split(&mut self, split_at: Tick) {
        if let Some((event_at, notation)) = self.get_previous_notation(split_at) {
            // only split if:
            // 1. split index not already the start of an event.
            // 2. split index is not the end of an event (ie; end of flow);
            if event_at != split_at && split_at != event_at + notation.duration {
                let tones = notation.tones.clone();
                let ties = notation.ties.clone();
                let duration = notation.duration;

                self.insert(
                    event_at,
                    Notation {
                        tick: event_at,
                        tones: tones.clone(),
                        duration: split_at - event_at,
                        ties: tones.iter().map(|tone| tone.key.clone()).collect(),
                    },
                );

                self.insert(
                    split_at,
                    Notation {
                        tick: split_at,
                        tones,
                        duration: event_at + duration - split_at,
                        ties,
                    },
                );
            }
        }
    }

    pub fn is_range_empty(&self, start: Tick, stop: Tick) -> bool {
        for tick in start + 1..stop {
            match self.track.contains_key(&tick) {
                true => return false,
                false => continue,
            }
        }

        true
    }

    pub fn is_tick_empty(&self, tick: Tick) -> bool {
        self.track.get(&tick).is_none()
    }

    pub fn is_tick_rest(&self, tick: &Tick) -> bool {
        match self.track.get(tick) {
            Some(entry) => entry.is_rest(),
            None => false,
        }
    }

    // [qhq] patterns dont't split middle
    pub fn is_qhq_pattern(
        &self,
        start: Tick,
        time_signature: &TimeSignature,
        subdivisions: Ticks,
    ) -> bool {
        let beat_one = start;
        let beat_two = time_signature.get_tick_at_beat(start, 2, subdivisions);
        let beat_four = time_signature.get_tick_at_beat(start, 4, subdivisions);

        !self.is_tick_empty(beat_one)
            && !self.is_tick_empty(beat_two)
            && !self.is_tick_empty(beat_four)
            && (!self.is_tick_rest(&beat_one) && !self.is_tick_rest(&beat_four))
            && !self.is_tick_rest(&beat_two)
            && self.is_range_empty(beat_one, beat_two)
            && self.is_range_empty(beat_two, beat_four)
    }

    /// [qm.] patterns don't split middle
    pub fn is_qmdot_pattern(
        &self,
        start: Tick,
        time_signature: &TimeSignature,
        subdivisions: Ticks,
    ) -> bool {
        let beat_one = start;
        let beat_two = time_signature.get_tick_at_beat(start, 2, subdivisions);
        let end = start + time_signature.ticks_per_bar(subdivisions);

        !self.is_tick_empty(beat_one)
            && !self.is_tick_empty(beat_two)
            && !self.is_tick_rest(&beat_two)
            && self.is_range_empty(beat_one, beat_two)
            && self.is_range_empty(beat_two, end)
    }

    /// [m.q] patterns don't split middle
    pub fn is_mdotq_pattern(
        &self,
        start: Tick,
        time_signature: &TimeSignature,
        original_time_signature: &TimeSignature,
        subdivisions: Ticks,
    ) -> bool {
        let beat_one = start;
        let beat_four = time_signature.get_tick_at_beat(start, 4, subdivisions);

        let middle = start + (time_signature.ticks_per_bar(subdivisions) / 2);

        let is_pattern = !self.is_tick_empty(beat_one)
            && !self.is_tick_empty(beat_four)
            && self.is_range_empty(beat_one, beat_four);

        let are_both_notes = !self.is_tick_rest(&beat_one) && !self.is_tick_rest(&beat_four);

        let is_allowed_with_rest = !self.is_tick_rest(&beat_one)
            && self.is_tick_rest(&beat_four)
            && match self.track.get(&beat_one) {
                Some(entry) => {
                    entry.duration
                        != (original_time_signature.ticks_per_beat(subdivisions) as f32 * 1.5)
                            as u32
                }
                None => false,
            };

        let intersect_beat = original_time_signature.is_on_beat(middle, subdivisions);

        is_pattern && (are_both_notes || is_allowed_with_rest || !intersect_beat)
    }

    // time_signature represents the unit as if it were a "measure"
    pub fn split_unit(
        &mut self,
        start: u32,
        time_signature: &TimeSignature,
        original_time_signature: &TimeSignature,
        subdivisions: Ticks,
    ) {
        let stop = start + time_signature.ticks_per_bar(subdivisions);

        // we stop once there are no events in the range
        if self.is_range_empty(start, stop) {
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
                    TimeSignatureDrawType::Regular,
                    None,
                ),
                original_time_signature,
                subdivisions,
            );
        }

        match time_signature.beats {
            3 => {
                let beat_one = start;
                let beat_two = time_signature.get_tick_at_beat(start, 2, subdivisions);
                let beat_three = time_signature.get_tick_at_beat(start, 3, subdivisions);

                // split all rests at beats
                for beat in [beat_one, beat_two, beat_three] {
                    if let Some((_, entry)) = self.get_previous_notation(beat) {
                        if entry.is_rest() {
                            self.split(beat);
                        }
                    }
                }

                // make sure it doesn't look compound! (c. at end of bar)
                let middle = start + (time_signature.ticks_per_bar(subdivisions) / 2);
                if !self.is_tick_empty(middle) && self.is_range_empty(middle, stop) {
                    self.split(beat_three);
                };

                // allow sustaining two beats into one
                if !self.is_range_empty(beat_one, beat_two) {
                    self.split(beat_two);
                }

                // if we haven't made any splits we split at the third beat
                if !self.is_tick_empty(beat_two) && !self.is_tick_empty(beat_three) {
                    self.split(beat_three);
                }

                let next = TimeSignature::new(
                    time_signature.tick,
                    4,
                    time_signature.beat_type.half().half(),
                    TimeSignatureDrawType::Regular,
                    None,
                );

                self.split_unit(beat_one, &next, original_time_signature, subdivisions);
                self.split_unit(beat_two, &next, original_time_signature, subdivisions);
                self.split_unit(beat_three, &next, original_time_signature, subdivisions);
            }
            4 => {
                let middle = start + (time_signature.ticks_per_bar(subdivisions) / 2);

                if !self.is_qhq_pattern(start, time_signature, subdivisions)
                    && !self.is_qmdot_pattern(start, time_signature, subdivisions)
                    && !self.is_mdotq_pattern(
                        start,
                        time_signature,
                        original_time_signature,
                        subdivisions,
                    )
                {
                    self.split(middle);
                }

                let next = TimeSignature::new(
                    time_signature.tick,
                    4,
                    time_signature.beat_type.half(),
                    TimeSignatureDrawType::Regular,
                    None,
                );

                self.split_unit(start, &next, original_time_signature, subdivisions);
                self.split_unit(middle, &next, original_time_signature, subdivisions);
            }
            _ => (),
        }
    }

    pub fn split_as_per_meter(&mut self, barlines: &Bars, subdivisions: Ticks) {
        for (tick, time_signature) in barlines {
            self.split_unit(*tick, time_signature, time_signature, subdivisions);
        }
    }

    pub fn split_measures(&mut self, barlines: &Bars) {
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
                    for inner_tick in tick..tick + tone.duration {
                        if self.track.contains_key(&inner_tick) {
                            self.add_tone(&inner_tick, tone);
                        }
                    }
                }
            }
        }
    }

    pub fn split_unwritable(&mut self, barlines: &Bars, subdivisions: Ticks) {
        for (i, time) in barlines {
            for tick in *i..*i + time.ticks_per_bar(subdivisions) {
                if let Some(entry) = self.track.get(&tick) {
                    if !entry.is_rest() && !entry.is_writable(subdivisions) {
                        let longest = entry.longest_written_duration(subdivisions);
                        self.split(tick + longest);
                    }
                };
            }
        }
    }

    pub fn debug(&self, start: &Tick, stop: &Tick) {
        let mut annotation = String::new();
        for i in 0..*stop {
            if i >= *start && i < *stop {
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

            for (i, char) in output.iter_mut().enumerate().take(stop).skip(start) {
                if i == start {
                    if is_rest {
                        *char = 'r';
                    } else {
                        *char = 'o';
                    }
                } else if i == stop - 1 && !has_tie {
                    *char = ':';
                } else if has_tie {
                    *char = '_';
                } else {
                    *char = '-';
                }
            }
        }

        write!(f, "{}", String::from_iter(output))
    }
}

pub fn get_written_durations(flow: &Flow, tracks: &[&Track], bars: &Bars) -> NotationByTrack {
    let mut entries = NotationByTrack::default();

    for track in tracks {
        let notation = track.to_notation_track(&flow.length, bars, flow.subdivisions);
        entries.insert(track.key.clone(), notation);
    }

    entries
}

impl Track {
    pub fn to_notation_track(
        &self,
        flow_length: &Ticks,
        barlines: &Bars,
        subdivisions: Ticks,
    ) -> NotationTrack {
        let mut notation = NotationTrack::new(*flow_length);
        notation.split_at_tone_events(self);
        notation.split_measures(barlines);
        notation.split_as_per_meter(barlines, subdivisions);
        notation.split_unwritable(barlines, subdivisions);
        notation
    }
}

#[cfg(test)]
mod tests {
    use crate::entries::tone::Tone;
    use crate::parse::get_written_durations::Clusters;
    use crate::parse::get_written_durations::Notation;
    use rustc_hash::FxHashMap;
    use rustc_hash::FxHashSet;

    #[test]
    fn sort_tones_test() {
        let notation = Notation {
            tick: 0,
            tones: vec![
                Tone::tester("a"),
                Tone::tester("b"),
                Tone::tester("c"),
                Tone::tester("d"),
                Tone::tester("e"),
            ],
            duration: 0,
            ties: FxHashSet::default(),
        };

        let mut tone_offsets = FxHashMap::default();
        tone_offsets.insert(String::from("a"), 0);
        tone_offsets.insert(String::from("b"), 1);
        tone_offsets.insert(String::from("c"), -1);
        tone_offsets.insert(String::from("d"), 2);
        tone_offsets.insert(String::from("e"), -2);

        let result = notation.sort_tones(&tone_offsets);
        let expected = ["d", "b", "a", "c", "e"];
        for (i, tone) in result.iter().enumerate() {
            assert_eq!(&tone.key, expected[i]);
        }
    }

    #[test]
    fn get_clusters_test() {
        let notation = Notation {
            tick: 0,
            tones: vec![
                Tone::tester("a"),
                Tone::tester("b"),
                Tone::tester("c"),
                Tone::tester("d"),
                Tone::tester("e"),
                Tone::tester("f"),
            ],
            duration: 0,
            ties: FxHashSet::default(),
        };

        let mut tone_offsets = FxHashMap::default();
        tone_offsets.insert(String::from("a"), 2);
        tone_offsets.insert(String::from("b"), 1);
        tone_offsets.insert(String::from("c"), -1);
        tone_offsets.insert(String::from("d"), -3);
        tone_offsets.insert(String::from("e"), -4);
        tone_offsets.insert(String::from("f"), -5);

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

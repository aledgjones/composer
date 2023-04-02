use super::get_barlines::Barlines;
use super::get_beams::BeamsByTrack;
use super::get_written_durations::NotationByTrack;
use super::{get_accidentals::AccidentalsByTrack, get_shunts::ShuntsByTrack};
use crate::components::measurements::BoundingBox;
use crate::components::misc::Tick;
use crate::components::units::Space;
use crate::entries::barline::BarlineDrawType;
use crate::entries::Entry;
use crate::score::engrave::Engrave;
use crate::score::flows::Flow;
use crate::score::stave::Stave;
use crate::score::tracks::Track;
use rustc_hash::FxHashMap;
use std::ops::{Add, Index, IndexMut};

pub const POSITION_COUNT: u32 = 12;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Position {
    PaddingStart = 0,
    EndRepeat,
    Clef,
    Barline,
    KeySignature,
    TimeSignature,
    StartRepeat,
    Accidentals, // only used when at begining of measure (there is no previous space to cut into)
    PreNoteSlot, // as above
    NoteSpacing,
    PaddingEnd,
}

impl Add<Position> for usize {
    type Output = usize;

    fn add(self, other: Position) -> usize {
        self + other as usize
    }
}

impl From<usize> for Position {
    fn from(int: usize) -> Position {
        match int {
            0 => Position::PaddingStart,
            1 => Position::EndRepeat,
            2 => Position::Clef,
            3 => Position::Barline,
            4 => Position::KeySignature,
            5 => Position::TimeSignature,
            6 => Position::StartRepeat,
            7 => Position::Accidentals,
            8 => Position::PreNoteSlot,
            9 => Position::NoteSpacing,
            10 => Position::PaddingEnd,
            _ => Position::PaddingStart,
        }
    }
}

impl Index<Position> for [f32] {
    type Output = f32;

    fn index(&self, position: Position) -> &Self::Output {
        &self[position as usize]
    }
}

impl IndexMut<Position> for [f32] {
    fn index_mut(&mut self, position: Position) -> &mut f32 {
        &mut self[position as usize]
    }
}

#[derive(Debug)]
pub struct Spacing {
    pub width: Space,
    pub x: Space,
}
pub struct HorizontalSpacing {
    widths: Vec<Spacing>,
    pub width: f32,
}

impl HorizontalSpacing {
    pub fn new() -> Self {
        Self {
            widths: Vec::new(),
            width: 0.0,
        }
    }
    pub fn get(&self, tick: &Tick, position: &Position) -> Option<&Spacing> {
        let start = (tick * POSITION_COUNT) as usize;
        let i = start + position.clone() as usize;
        self.widths.get(i)
    }
}

impl Default for HorizontalSpacing {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn measure_horizontal_spacing(
    flow: &Flow,
    staves: &[&Stave],
    tracks: &FxHashMap<String, Track>,
    barlines: &Barlines,
    notations_by_track: &NotationByTrack,
    shunts_by_track: &ShuntsByTrack,
    beams_by_track: &BeamsByTrack,
    accidentals_by_track: &AccidentalsByTrack,
    engrave: &Engrave,
) -> HorizontalSpacing {
    let mut widths: Vec<f32> = vec![0.0; (flow.length * POSITION_COUNT) as usize];
    let flow_master = tracks.get(&flow.master).unwrap();

    widths[0 + Position::PaddingStart] = 1.0;

    for entry in flow_master.entries.by_key.values() {
        match entry {
            Entry::KeySignature(key_signature) => {
                let start = (entry.tick() * POSITION_COUNT) as usize;
                let metrics = if key_signature.offset == 0 {
                    // find width needed to cancel the previous key signature
                    match flow_master.get_key_signature_before_tick(key_signature.tick) {
                        Some(previous) => previous.metrics(),
                        None => BoundingBox::none(),
                    }
                } else {
                    key_signature.metrics()
                };
                widths[start + Position::KeySignature] = metrics.width + metrics.padding.right;
            }
            Entry::TimeSignature(time_signature) => {
                let start = (entry.tick() * POSITION_COUNT) as usize;
                let metrics = time_signature.metrics(flow.subdivisions);
                widths[start + Position::TimeSignature] = metrics.width + metrics.padding.right;
            }
            _ => (),
        }
    }

    for (tick, def) in barlines {
        let start = (tick * POSITION_COUNT) as usize;
        let time = flow_master.get_time_signature_at_tick(tick);
        let key = flow_master.get_key_signature_at_tick(tick);

        if def.end_repeat {
            let metrics = BarlineDrawType::EndRepeat.metrics();
            if time.is_some() || key.is_some() {
                widths[start + Position::EndRepeat] = metrics.width + metrics.padding.right - 0.5;
            } else {
                widths[start + Position::EndRepeat] = metrics.width + metrics.padding.right;
            }
        }

        if let Some(draw_type) = &def.draw_type {
            let metrics = draw_type.metrics();
            if time.is_some() || key.is_some() {
                widths[start + Position::Barline] = metrics.width + metrics.padding.right - 0.5;
            } else {
                widths[start + Position::Barline] = metrics.width + metrics.padding.right;
            }
        }

        if def.start_repeat {
            let metrics = BarlineDrawType::StartRepeat.metrics();
            if time.is_some() {
                widths[start + Position::StartRepeat] = metrics.width + metrics.padding.right - 1.0;
            } else {
                widths[start + Position::StartRepeat] = metrics.width + metrics.padding.right;
            }
        }
    }

    for stave in staves {
        let stave_master = tracks.get(&stave.master).unwrap();
        for entry in stave_master.entries.by_key.values() {
            if let Entry::Clef(clef) = entry {
                let start = (clef.tick * POSITION_COUNT) as usize;
                let metrics = clef.metrics();
                widths[start + Position::Clef] = metrics.width + metrics.padding.right;
            }
        }
    }

    for (track_key, notation) in notations_by_track {
        for (tick, entry) in &notation.track {
            let start = (tick * POSITION_COUNT) as usize;
            let shunts = shunts_by_track.get(track_key).unwrap();

            if *tick == 0 && entry.has_pre_shunt(shunts) {
                widths[start + Position::PreNoteSlot] = entry.notehead_width();
            }

            let accidentals = accidentals_by_track.get(track_key).unwrap();
            let beams = beams_by_track.get(track_key).unwrap();

            let mut spacing = entry
                .metrics(shunts, flow.subdivisions, engrave, beams)
                .padding
                .right;

            // ACCIDENTALS
            if *tick == 0 || barlines.contains_key(tick) {
                // start of bars has no previous spacing to extend so we use the accidentals slot
                if let Some(slots) = accidentals.slots_by_tick.get(tick) {
                    let current = widths[start + Position::Accidentals];
                    let width = (*slots as f32) * 1.1; // TODO: width of accidental
                    if width > current {
                        widths[start + Position::Accidentals] = width;
                    }
                };
            }

            // extend spacing to accomodate accidentals + pre shunts (if needed)
            if let Some((next_tick, next_entry)) = notation.get_next_notation(tick) {
                if !barlines.contains_key(&next_tick) {
                    let min = entry.min_spacing(shunts, flow.subdivisions, engrave, beams);

                    let pre_shunt = match next_entry.has_pre_shunt(shunts) {
                        true => next_entry.notehead_width(),
                        false => 0.0,
                    };

                    let accidentals = match accidentals.slots_by_tick.get(&next_tick) {
                        Some(slots) => (*slots as f32) * 1.1,
                        None => 0.0,
                    };

                    let min = min + pre_shunt + accidentals;
                    if min > spacing {
                        spacing = min
                    }
                }
            };

            let note_spacing_per_tick = spacing / entry.duration as f32;
            let end = tick + entry.duration;
            for i in *tick..end {
                let start = (i * POSITION_COUNT) as usize;
                if note_spacing_per_tick > widths[start + Position::NoteSpacing] {
                    widths[start + Position::NoteSpacing] = note_spacing_per_tick;
                }
            }
        }
    }

    // assign the spacing to hashmap for easy lookup & accumulate widths to get x positions
    let mut output = HorizontalSpacing::new();
    let mut x: f32 = 0.0;
    for width in widths {
        output.widths.push(Spacing { width, x });
        x += width;
    }
    output.width = x + BarlineDrawType::Final.metrics().width;
    output
}

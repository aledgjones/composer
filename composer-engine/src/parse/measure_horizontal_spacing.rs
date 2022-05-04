use super::get_accidentals::Accidentals;
use super::get_beams::BeamsByTrack;
use super::get_note_positions::{Position, TonePositions};
use super::get_stem_directions::StemDirectionsByTrack;
use super::get_written_durations::NotationByTrack;
use crate::components::measurements::BoundingBox;
use crate::components::misc::Tick;
use crate::components::units::Space;
use crate::score::engrave::Engrave;
use crate::score::flows::Flow;
use crate::score::stave::Stave;
use crate::score::tracks::Track;
use std::collections::HashMap;

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
        let start = (tick * 13) as usize;
        let i = start + position.clone() as usize;
        self.widths.get(i)
    }
}

impl Default for HorizontalSpacing {
    fn default() -> Self {
        Self::new()
    }
}

pub fn measure_horizontal_spacing(
    flow: &Flow,
    staves: &[&Stave],
    tracks: &HashMap<String, Track>,
    notations_by_track: &NotationByTrack,
    tone_positions: &TonePositions,
    beams_by_track: &BeamsByTrack,
    stem_directions_by_track: &StemDirectionsByTrack,
    accidentals: &Accidentals,
    engraving: &Engrave,
) -> HorizontalSpacing {
    let mut widths: Vec<f32> = vec![0.0; (flow.length * 13) as usize];
    for tick in 0..flow.length {
        let start = (tick * 13) as usize;

        if tick == 0 {
            widths[start + Position::PaddingStart] = 1.0;
        }

        let flow_master = tracks.get(&flow.master).unwrap();

        let key_signature = flow_master.get_key_signature_at_tick(&tick);
        // TODO: barline spacing
        // let barline = flow_master.get_barline_at_tick(&tick);

        // KEY SIGNATURE
        let key = match key_signature.clone() {
            Some(key) => {
                let metrics = if key.offset == 0 {
                    // find width needed to cancel the previous key signature
                    match flow_master.get_key_signature_before_tick(tick) {
                        Some(previous) => previous.metrics(),
                        None => BoundingBox::none(),
                    }
                } else {
                    key.metrics()
                };
                widths[start + Position::KeySignature] = metrics.width + metrics.padding.right;
                Some(key)
            }
            _ => None,
        };

        // TIME SIGNATURE
        let time = match flow_master.get_time_signature_at_tick(&tick) {
            Some(time) => {
                let metrics = time.metrics(flow.subdivisions);
                widths[start + Position::TimeSignature] = metrics.width + metrics.padding.right;
                Some(time)
            }
            _ => None,
        };

        // ACCIDENTALS
        if let Some(slots) = accidentals.slots_by_tick.get(&tick) {
            if slots > &0 {
                widths[start + Position::Accidentals] = ((*slots as f32) * 1.1) + 0.2;
            }
        };

        let is_first_beat = match time {
            Some(time) => time.is_on_first_beat(tick, flow.subdivisions),
            None => false,
        };

        for stave in staves {
            let stave_master = tracks.get(&stave.master).unwrap();

            // CLEF
            if let Some(clef) = stave_master.get_clef_at_tick(&tick) {
                let metrics = clef.metrics();
                widths[start + Position::Clef] = metrics.width + metrics.padding.right;
            }

            for track_key in &stave.tracks {
                let notation = notations_by_track.get(track_key).unwrap();
                if let Some(entry) = notation.track.get(&tick) {
                    let notehead_width: Space = 1.175;

                    if entry.is_rest() {
                        // rests are always at the not slot position
                        widths[start + Position::NoteSlot] = notehead_width;
                    } else {
                        for tone in &entry.tones {
                            // notes can be shunted, we need to set the width at the right position for each tone
                            let position = tone_positions.get(&(tick, tone.key.clone())).unwrap();
                            widths[start + position.clone()] = notehead_width;
                        }
                    }

                    let beams = beams_by_track.get(track_key).unwrap();
                    let stem_directions = stem_directions_by_track.get(track_key).unwrap();
                    let stem_direction = stem_directions.get(&tick);

                    let note_spacing =
                        entry.spacing(&tick, engraving, flow.subdivisions, &stem_direction, beams);

                    let note_spacing_per_tick = note_spacing / entry.duration as f32;
                    let end = tick + entry.duration;
                    for i in tick..end {
                        let start = (i * 13) as usize;
                        if note_spacing_per_tick > widths[start + Position::NoteSpacing] {
                            widths[start + Position::NoteSpacing] = note_spacing_per_tick;
                        }
                    }
                };
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
    output.width = x;
    output
}

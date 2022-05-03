use super::get_accidentals::Accidentals;
use super::get_note_positions::{Position, TonePositions};
use super::get_stem_directions::StemDirectionsByTrack;
use super::get_written_durations::NotationByTrack;
use super::{get_barlines::Barlines, get_beams::BeamsByTrack};
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
pub type HorizontalSpacing = HashMap<(Tick, Position), Spacing>;

pub fn measure_horizontal_spacing(
    flow: &Flow,
    staves: &[&Stave],
    tracks: &HashMap<String, Track>,
    barlines: &Barlines,
    notations: &NotationByTrack,
    tone_positions: &TonePositions,
    beams: &BeamsByTrack,
    stem_directions: &StemDirectionsByTrack,
    accidentals: &Accidentals,
    engraving: &Engrave,
) -> (HorizontalSpacing, f32) {
    let mut widths: Vec<f32> = vec![0.0; (flow.length * 13) as usize];

    for tick in 0..flow.length {
        let slice_start = (tick * 13) as usize;
        let spacing = &mut widths[slice_start..slice_start + 13];

        if tick == 0 {
            spacing[Position::PaddingStart] = 1.0;
        }

        let flow_master = tracks.get(&flow.master).unwrap();

        let time_signature = flow_master.get_time_signature_at_tick(&tick);
        let key_signature = flow_master.get_key_signature_at_tick(&tick);
        let barline = flow_master.get_barline_at_tick(&tick);

        // KEY SIGNATURE
        if let Some(key) = key_signature.clone() {
            let metrics = if key.offset == 0 {
                // find width needed to cancel the previous key signature
                match flow_master.get_key_signature_before_tick(tick) {
                    Some(previous) => previous.metrics(),
                    None => BoundingBox::none(),
                }
            } else {
                key.metrics()
            };
            spacing[Position::KeySignature] = metrics.width + metrics.padding.right;
        };

        // TIME SIGNATURE
        if let Some(time) = time_signature {
            let metrics = time.metrics();
            spacing[Position::TimeSignature] = metrics.width + metrics.padding.right;
        };

        for stave in staves {
            let stave_master = tracks.get(&stave.master).unwrap();

            // CLEF
            if let Some(clef) = stave_master.get_clef_at_tick(&tick) {
                let metrics = clef.metrics();
                spacing[Position::Clef] = metrics.width + metrics.padding.right;
            }

            for track_key in &stave.tracks {
                let notation = notations.get(track_key).unwrap();
                if let Some(entry) = notation.track.get(&tick) {
                    let notehead_width: Space = 1.175;
                    if entry.is_rest() {
                        spacing[Position::NoteSlot] = notehead_width;
                    } else {
                        for tone in &entry.tones {
                            let position = tone_positions.get(&(tick, tone.key.clone())).unwrap();
                            spacing[position.clone()] = notehead_width;
                        }
                    }
                };
            }
        }
    }

    // assign the spacing to hashmap for easy lookup & accumulate widths to get x positions
    let mut output: HorizontalSpacing = HashMap::new();
    let mut x: f32 = 0.0;
    for (i, width) in widths.iter().enumerate() {
        let tick = (i / 13) as Tick;
        let position = Position::from(i % 13);
        output.insert((tick, position), Spacing { width: *width, x });
        x += width;
    }
    (output, x)
}

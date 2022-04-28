use super::get_beams::Beams;
use super::get_beams::BeamsByTrack;
use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::NotationTrack;
use super::get_written_durations::NotationTracks;
use crate::components::misc::Tick;
use crate::components::misc::Ticks;
use std::collections::HashMap;

pub enum StemDirection {
    Up,
    Down,
}

impl StemDirection {
    pub fn to_modifier(&self) -> i8 {
        match self {
            StemDirection::Up => 1,
            StemDirection::Down => -1,
        }
    }
}

pub type StemDirections = HashMap<Tick, StemDirection>;
pub type StemDirectionsByTrack = HashMap<String, StemDirections>;

pub fn get_stem_directions_in_track(
    flow_length: Ticks,
    notation: &NotationTrack,
    tone_offsets: &ToneVerticalOffsets,
    beams: &Beams,
) -> StemDirections {
    let mut output = HashMap::new();

    // TODO: do this function

    output
}

pub fn get_stem_directions(
    flow_length: Ticks,
    tracks: &NotationTracks,
    tone_offsets: &ToneVerticalOffsets,
    beams_by_track: &BeamsByTrack,
) -> StemDirectionsByTrack {
    let mut output = HashMap::new();

    for (track_key, track) in tracks {
        let beams = beams_by_track.get(track_key).unwrap();
        let stem_directions = get_stem_directions_in_track(flow_length, track, tone_offsets, beams);
        output.insert(track_key.clone(), stem_directions);
    }

    output
}

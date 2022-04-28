use crate::components::misc::Ticks;
use crate::components::pitch::Pitch;
use crate::entries::clef::Clef;
use crate::score::stave::Stave;
use crate::score::tracks::Tracks;
use std::collections::HashMap;

pub type ToneVerticalOffsets = HashMap<String, i8>;

pub fn get_tone_offsets(
    flow_length: Ticks,
    staves: &[&Stave],
    tracks: &Tracks,
) -> ToneVerticalOffsets {
    let mut output: ToneVerticalOffsets = HashMap::new();

    for stave in staves {
        let master = tracks.get(&stave.master).unwrap();
        let mut clef: Option<Clef> = None;

        for tick in 0..flow_length {
            if let Some(found) = master.get_clef_at_tick(&tick) {
                clef = Some(found);
            };

            if let Some(clef) = &clef {
                for stave_key in &stave.tracks {
                    let track = tracks.get(stave_key).unwrap();
                    for tone in track.get_tones_at_tick(&tick) {
                        let offset = Pitch::steps_between(&clef.pitch, &tone.pitch) + clef.offset;
                        output.insert(tone.key.clone(), offset);
                    }
                }
            }
        }
    }

    output
}

use crate::components::misc::Tick;
use crate::entries::time_signature::TimeSignature;
use crate::score::flows::Flow;
use crate::score::tracks::Tracks;
use std::collections::HashMap;

pub type Barlines = HashMap<Tick, TimeSignature>;

pub fn get_barlines(flow: &Flow, tracks: &Tracks) -> Barlines {
    let mut barlines: Barlines = HashMap::new();

    let master = tracks.get(&flow.master).unwrap();
    let mut time_signature = TimeSignature::default();

    for tick in 0..flow.length {
        if let Some(entry) = master.get_time_signature_at_tick(&tick) {
            time_signature = entry;
        }

        if time_signature.distance_from_barline(tick, flow.subdivisions) == 0 {
            barlines.insert(tick, time_signature.clone());
        }
    }

    barlines
}

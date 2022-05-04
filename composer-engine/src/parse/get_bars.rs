use crate::components::misc::Tick;
use crate::entries::time_signature::TimeSignature;
use crate::score::flows::Flow;
use crate::score::tracks::Tracks;
use std::collections::HashMap;

pub type Bars = HashMap<Tick, TimeSignature>;

pub fn get_bars(flow: &Flow, tracks: &Tracks) -> Bars {
    let mut output: Bars = HashMap::new();

    let master = tracks.get(&flow.master).unwrap();
    let mut time_signature = TimeSignature::default();

    for tick in 0..flow.length {
        if let Some(entry) = master.get_time_signature_at_tick(&tick) {
            time_signature = entry;
        }

        if time_signature.is_on_first_beat(tick, flow.subdivisions) {
            output.insert(tick, time_signature.clone());
        }
    }

    output
}

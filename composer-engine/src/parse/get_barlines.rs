use crate::components::misc::Tick;
use crate::components::misc::Ticks;
use crate::entries::time_signature::TimeSignature;
use crate::score::tracks::Track;
use std::collections::HashMap;

pub type Barlines = HashMap<Tick, TimeSignature>;

pub fn get_barlines(flow_length: Ticks, master: &Track) -> Barlines {
    let mut barlines: Barlines = HashMap::new();

    let mut time_signature = TimeSignature::default();

    for tick in 0..flow_length {
        if let Some(entry) = master.get_time_signature_at_tick(&tick) {
            time_signature = entry;
        }

        if time_signature.distance_from_barline(tick) == 0 {
            barlines.insert(tick, time_signature.clone());
        }
    }

    barlines
}

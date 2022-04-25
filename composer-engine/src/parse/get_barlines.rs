use crate::components::duration::NoteDuration;
use crate::components::misc::Tick;
use crate::components::misc::Ticks;
use crate::entries::time_signature::TimeSignature;
use crate::entries::time_signature::TimeSignatureDrawType;
use crate::score::tracks::Track;
use std::collections::HashMap;

pub type Barlines = HashMap<Tick, TimeSignature>;

pub fn get_barlines(flow_length: Ticks, master: &Track) -> Barlines {
    let mut barlines: Barlines = HashMap::new();

    let mut time = TimeSignature::new(
        0,
        4,
        NoteDuration::Quarter,
        TimeSignatureDrawType::Hidden,
        None,
    );

    for tick in 0..flow_length {
        if let Some(time_signature) = master.get_time_signature_at_tick(tick) {
            time = time_signature;
        }

        if time.distance_from_barline(tick) == 0 {
            barlines.insert(tick, time.clone());
        }
    }

    barlines
}

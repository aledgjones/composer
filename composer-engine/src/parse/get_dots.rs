use super::get_tone_offsets::ToneVerticalOffsets;
use super::get_written_durations::{Notation, NotationByTrack, NotationTrack};
use crate::components::misc::{Tick, Ticks};
use crate::score::flows::Flow;
use rustc_hash::{FxHashMap, FxHashSet};

pub type Dot = (Tick, i8);
pub type Dots = FxHashSet<Dot>;
pub type DotsByTrack = FxHashMap<String, Dots>;

fn best_effort_fallback(tick: &Tick, entry: &Notation, tone_offsets: &ToneVerticalOffsets) -> Dots {
    let mut output: Dots = FxHashSet::default();

    // put a dot at each space with a tone
    for tone in &entry.tones {
        let offset = tone_offsets.get(&tone.key).unwrap();
        // is space
        if offset % 2 != 0 {
            output.insert((*tick, *offset));
        }
    }

    for tone in &entry.tones {
        let offset = tone_offsets.get(&tone.key).unwrap();
        // is space
        if offset % 2 == 0 {
            match output.get(&(*tick, offset - 1)) {
                Some(_) => {
                    output.insert((*tick, offset + 1));
                }
                None => {
                    output.insert((*tick, offset - 1));
                }
            }
        }
    }

    output
}

fn is_close_enough(slot: i8, entry: &Notation, tone_offsets: &ToneVerticalOffsets) -> bool {
    for tone in &entry.tones {
        let offset = tone_offsets.get(&tone.key).unwrap();
        if slot >= offset - 2 && slot <= offset + 2 {
            return true;
        }
    }

    false
}

/// try and write all dots that are needed, we fallback to the best effort apporach
/// if the dots get too far away from the note heads
fn get_dots_at_tick(tick: &Tick, entry: &Notation, tone_offsets: &ToneVerticalOffsets) -> Dots {
    let mut output: Dots = FxHashSet::default();

    // put a dot at each space with a tone
    for tone in &entry.tones {
        let offset = tone_offsets.get(&tone.key).unwrap();
        // is space
        if offset % 2 != 0 {
            output.insert((*tick, *offset));
        }
    }

    for tone in &entry.tones {
        let offset = tone_offsets.get(&tone.key).unwrap();
        // is line
        if offset % 2 == 0 {
            let mut n = 0;
            let mut direction = -1;
            let mut slot = offset + (1 + n * 2) * direction;

            // find the nearest available slot
            while output.contains(&(*tick, slot)) {
                if direction == 1 {
                    n += 1;
                }
                direction *= -1;
                slot = offset + (1 + n * 2) * direction;
            }

            // check if the slot is close enough
            if is_close_enough(slot, entry, tone_offsets) {
                output.insert((*tick, slot));
            } else {
                // if you can't fit the dot close enough to the end
                // of the cluster try moving it to the other end
                direction *= -1;
                // find last space we can fit dot in
                loop {
                    slot += 2 * direction;
                    if !output.contains(&(*tick, slot)) {
                        break;
                    }
                }
                if is_close_enough(slot, entry, tone_offsets) {
                    output.insert((*tick, slot));
                } else {
                    return best_effort_fallback(tick, entry, tone_offsets);
                }
            }
        }
    }

    output
}

fn get_dots_in_track(
    notation: &NotationTrack,
    tone_offsets: &ToneVerticalOffsets,
    subdivisions: Ticks,
) -> Dots {
    let mut output: Dots = FxHashSet::default();

    for (tick, entry) in &notation.track {
        if !entry.is_rest() && entry.is_dotted(subdivisions) {
            let dots: Dots = get_dots_at_tick(tick, entry, tone_offsets);
            for dot in dots {
                output.insert(dot);
            }
        }
    }

    output
}

pub fn get_dots(
    flow: &Flow,
    notation_by_track: &NotationByTrack,
    tone_offsets: &ToneVerticalOffsets,
) -> DotsByTrack {
    let mut output: DotsByTrack = FxHashMap::default();

    for (track_key, notation) in notation_by_track {
        let dots: Dots = get_dots_in_track(notation, tone_offsets, flow.subdivisions);
        output.insert(track_key.clone(), dots);
    }

    output
}

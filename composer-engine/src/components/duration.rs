use super::misc::Ticks;
use wasm_bindgen::prelude::wasm_bindgen;

pub const NOTE_DURATIONS: [NoteDuration; 7] = [
    NoteDuration::Whole,
    NoteDuration::Half,
    NoteDuration::Quarter,
    NoteDuration::Eighth,
    NoteDuration::Sixteenth,
    NoteDuration::ThirtySecond,
    NoteDuration::SixtyFourth,
];

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl NoteDuration {
    /// convert ticks to NoteDuration. this may fail so wrap in option
    pub fn from_ticks(ticks: Ticks, subdivisions: Ticks) -> Option<NoteDuration> {
        if ticks == subdivisions * 4 {
            return Some(NoteDuration::Whole);
        }

        if ticks == subdivisions * 2 {
            return Some(NoteDuration::Half);
        }

        if ticks == subdivisions {
            return Some(NoteDuration::Quarter);
        }

        if ticks == subdivisions / 2 {
            return Some(NoteDuration::Eighth);
        }

        if ticks == subdivisions / 4 {
            return Some(NoteDuration::Sixteenth);
        }

        if ticks == subdivisions / 8 {
            return Some(NoteDuration::ThirtySecond);
        }

        if ticks == subdivisions / 16 {
            return Some(NoteDuration::SixtyFourth);
        }

        None
    }

    pub fn to_ticks(&self, subdivisions: Ticks) -> Ticks {
        match self {
            NoteDuration::Whole => subdivisions * 4,
            NoteDuration::Half => subdivisions * 2,
            NoteDuration::Quarter => subdivisions,
            NoteDuration::Eighth => subdivisions / 2,
            NoteDuration::Sixteenth => subdivisions / 4,
            NoteDuration::ThirtySecond => subdivisions / 8,
            NoteDuration::SixtyFourth => subdivisions / 16,
        }
    }

    pub fn spacing_ratio(&self, ratio: f32, is_dotted: bool) -> f32 {
        let base = match self {
            NoteDuration::Whole => ratio * 2.0,
            NoteDuration::Half => ratio,
            NoteDuration::Quarter => 1.0,
            NoteDuration::Eighth => 1.0 / ratio,
            NoteDuration::Sixteenth => 1.0 / (ratio * 2.0),
            NoteDuration::ThirtySecond => 1.0 / (ratio * 4.0),
            NoteDuration::SixtyFourth => 1.0 / (ratio * 8.0),
        };

        if is_dotted {
            base * 1.2
        } else {
            base
        }
    }

    pub fn to_glyph(&self) -> &str {
        match self {
            NoteDuration::Whole => "\u{1D15D}",
            NoteDuration::Half => "\u{1D15E}",
            NoteDuration::Quarter => "\u{1D15F}",
            NoteDuration::Eighth => "\u{1D160}",
            NoteDuration::Sixteenth => "\u{1D161}",
            NoteDuration::ThirtySecond => "\u{1D162}",
            NoteDuration::SixtyFourth => "\u{1D162}",
        }
    }

    pub fn double(&self) -> NoteDuration {
        match self {
            NoteDuration::Whole => NoteDuration::Whole,
            NoteDuration::Half => NoteDuration::Whole,
            NoteDuration::Quarter => NoteDuration::Half,
            NoteDuration::Eighth => NoteDuration::Quarter,
            NoteDuration::Sixteenth => NoteDuration::Eighth,
            NoteDuration::ThirtySecond => NoteDuration::Sixteenth,
            NoteDuration::SixtyFourth => NoteDuration::ThirtySecond,
        }
    }

    pub fn half(&self) -> NoteDuration {
        match self {
            NoteDuration::Whole => NoteDuration::Half,
            NoteDuration::Half => NoteDuration::Quarter,
            NoteDuration::Quarter => NoteDuration::Eighth,
            NoteDuration::Eighth => NoteDuration::Sixteenth,
            NoteDuration::Sixteenth => NoteDuration::ThirtySecond,
            NoteDuration::ThirtySecond => NoteDuration::SixtyFourth,
            NoteDuration::SixtyFourth => NoteDuration::SixtyFourth,
        }
    }
}

pub fn is_writable(duration: Ticks, subdivisions: Ticks) -> bool {
    for option in NOTE_DURATIONS {
        if duration == option.to_ticks(subdivisions) {
            return true;
        }
    }
    false
}

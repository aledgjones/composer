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
    pub fn from_ticks(ticks: &Ticks, subdivisions: Ticks) -> Option<NoteDuration> {
        let quarters = *ticks as f32 / subdivisions as f32;
        match quarters {
            4.0 => Some(NoteDuration::Whole),
            2.0 => Some(NoteDuration::Half),
            1.0 => Some(NoteDuration::Quarter),
            0.5 => Some(NoteDuration::Eighth),
            0.25 => Some(NoteDuration::Sixteenth),
            0.125 => Some(NoteDuration::ThirtySecond),
            0.0625 => Some(NoteDuration::SixtyFourth),
            _ => None,
        }
    }

    pub fn to_quarters(&self) -> f32 {
        match self {
            NoteDuration::Whole => 4.0,
            NoteDuration::Half => 2.0,
            NoteDuration::Quarter => 1.0,
            NoteDuration::Eighth => 0.5,
            NoteDuration::Sixteenth => 0.25,
            NoteDuration::ThirtySecond => 0.125,
            NoteDuration::SixtyFourth => 0.0625,
        }
    }

    pub fn to_ticks(&self, subdivisions: Ticks) -> Ticks {
        (self.to_quarters() * subdivisions as f32) as Ticks
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
            _ => 1.0,
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
    let quarters = duration as f32 / subdivisions as f32;
    for option in NOTE_DURATIONS {
        if quarters == option.to_quarters() {
            return true;
        }
    }
    false
}

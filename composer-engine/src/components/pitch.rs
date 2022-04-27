use serde::Serialize;
use wasm_bindgen::prelude::*;

const C0: u8 = 12;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Accidental {
    DoubleSharp,
    Sharp,
    Natural,
    Flat,
    DoubleFlat,
}

impl Accidental {
    // TODO: Make this better by working it out within the context of a key
    /// When there is no user defined accidental, we work it out from the pitch
    pub fn default(int: u8) -> Accidental {
        let step = (int - 12) % 12;
        match step {
            0 | 2 | 4 | 5 | 7 | 9 | 11 => Accidental::Natural,
            _ => Accidental::Sharp,
        }
    }

    /// Convert an accidental to a token
    pub fn to_token(&self) -> &str {
        match self {
            Accidental::DoubleSharp => "${double-sharp}",
            Accidental::Sharp => "${sharp}",
            Accidental::Natural => "${natural}",
            Accidental::Flat => "${flat}",
            Accidental::DoubleFlat => "${double-flat}",
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize)]
pub struct Pitch {
    pub int: u8, // the midi number
    pub accidental: Accidental,
}

impl Pitch {
    /// Create a pitch from a MIDI number and accidental
    pub fn new(int: u8, accidental: Accidental) -> Self {
        Pitch { int, accidental }
    }

    /// gets the base note pitch (natural) for an accidental note
    /// ie 61 (C#) -> 60 (C), 61 (D flat) -> 62 (D)
    fn base(&self) -> u8 {
        let int = self.int;
        match self.accidental {
            Accidental::DoubleSharp => (int - 2),
            Accidental::Sharp => (int - 1),
            Accidental::Natural => int,
            Accidental::Flat => (int + 1),
            Accidental::DoubleFlat => (int + 2),
        }
    }

    pub fn half_steps(&self) -> u8 {
        (self.base() - C0) % 12
    }

    pub fn steps(&self) -> u8 {
        match self.half_steps() {
            0 => 0,
            2 => 1,
            4 => 2,
            5 => 3,
            7 => 4,
            9 => 5,
            11 => 6,
            _ => 0,
        }
    }

    /// Gets the base note letter for a pitch
    /// ie 61 (C#) -> C, 61 (D flat) -> D
    pub fn letter(&self) -> &str {
        match self.half_steps() {
            0 => "C",
            2 => "D",
            4 => "E",
            5 => "F",
            7 => "G",
            9 => "A",
            11 => "B",
            _ => "",
        }
    }

    pub fn octave(&self) -> u8 {
        const C0: f32 = 12.0;
        let natural = self.base() as f32;
        ((natural - C0) / 12.0).floor() as u8
    }

    /// Get the scientific notation parts for the pitch in form (pitch: String, accidental: Accidental, octave: u8)
    /// eg. ("c", Accidental::Sharp, 0) == Pitch(60);
    pub fn to_scientific_notation(&self) -> String {
        format!(
            "{}{}{}",
            self.letter(),
            self.accidental.to_token(),
            self.octave()
        )
    }

    pub fn to_frequency(&self) -> f64 {
        let a: f64 = 440.0;
        (a / 32.0) * (2.0_f64.powf((self.int as f64 - 9.0) / 12.0))
    }

    pub fn steps_between(a: &Pitch, b: &Pitch) -> i8 {
        let octave_offset = (b.octave() as i8 - a.octave() as i8) * 7;
        println!("{}+{}-{}", octave_offset, b.steps(), a.steps());
        octave_offset + b.steps() as i8 - a.steps() as i8
    }
}

#[cfg(test)]
mod tests {
    use crate::components::pitch::Accidental;
    use crate::components::pitch::Pitch;

    #[test]
    fn step_cb() {
        assert_eq!(Pitch::new(59, Accidental::Flat).steps(), 0);
    }

    #[test]
    fn step_c() {
        assert_eq!(Pitch::new(60, Accidental::Natural).steps(), 0);
    }

    #[test]
    fn step_cs() {
        assert_eq!(Pitch::new(61, Accidental::Sharp).steps(), 0);
    }

    #[test]
    fn step_db() {
        assert_eq!(Pitch::new(61, Accidental::Flat).steps(), 1);
    }

    #[test]
    fn step_d() {
        assert_eq!(Pitch::new(62, Accidental::Natural).steps(), 1);
    }

    #[test]
    fn step_ds() {
        assert_eq!(Pitch::new(63, Accidental::Sharp).steps(), 1);
    }

    #[test]
    fn step_e() {
        assert_eq!(Pitch::new(64, Accidental::Natural).steps(), 2);
    }

    #[test]
    fn step_f() {
        assert_eq!(Pitch::new(65, Accidental::Natural).steps(), 3);
    }

    #[test]
    fn step_g() {
        assert_eq!(Pitch::new(67, Accidental::Natural).steps(), 4);
    }

    #[test]
    fn step_a() {
        assert_eq!(Pitch::new(69, Accidental::Natural).steps(), 5);
    }

    #[test]
    fn step_bb() {
        assert_eq!(Pitch::new(70, Accidental::Flat).steps(), 6);
    }

    #[test]
    fn step_b() {
        assert_eq!(Pitch::new(71, Accidental::Natural).steps(), 6);
    }

    #[test]
    fn step_bs() {
        assert_eq!(Pitch::new(72, Accidental::Sharp).steps(), 6);
    }

    #[test]
    fn steps_1() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(62, Accidental::Natural)
            ),
            1
        );
    }

    #[test]
    fn steps_2() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(61, Accidental::Flat)
            ),
            1
        );
    }

    #[test]
    fn steps_3() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(63, Accidental::Sharp)
            ),
            1
        );
    }

    #[test]
    fn steps_4() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(59, Accidental::Natural)
            ),
            -1
        );
    }

    #[test]
    fn steps_5() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(60, Accidental::Sharp)
            ),
            -1
        );
    }

    #[test]
    fn steps_6() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(60, Accidental::DoubleFlat)
            ),
            1
        );
    }

    #[test]
    fn steps_7() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(72, Accidental::DoubleFlat)
            ),
            8
        );
    }

    #[test]
    fn steps_8() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(58, Accidental::Flat)
            ),
            -1
        );
    }

    #[test]
    fn steps_9() {
        assert_eq!(
            Pitch::steps_between(
                &Pitch::new(60, Accidental::Natural),
                &Pitch::new(59, Accidental::Natural)
            ),
            -1
        );
    }
}

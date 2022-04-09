#[derive(Debug)]
pub enum Unit {
    Px(f32),
    Mm(f32),
    Space(f32),
}

pub struct Converter {
    pub px_per_mm: f32,
    pub mm_per_space: f32,
}

impl Converter {
    pub fn new(px_per_mm: f32, mm_per_space: f32) -> Self {
        Converter {
            px_per_mm,
            mm_per_space,
        }
    }

    pub fn to_px(&self, unit: &Unit) -> f32 {
        match unit {
            Unit::Px(value) => *value,
            Unit::Mm(value) => value * self.px_per_mm,
            Unit::Space(value) => value * self.mm_per_space * self.px_per_mm,
        }
    }

    pub fn to_mm(&self, unit: &Unit) -> f32 {
        match unit {
            Unit::Px(value) => value / self.px_per_mm,
            Unit::Mm(value) => *value,
            Unit::Space(value) => value * self.mm_per_space,
        }
    }

    pub fn to_spaces(&self, unit: &Unit) -> f32 {
        match unit {
            Unit::Px(value) => value / self.px_per_mm / self.mm_per_space,
            Unit::Mm(value) => value / self.mm_per_space,
            Unit::Space(value) => *value,
        }
    }
}

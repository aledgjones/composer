use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum Unit {
    Px(f32),
    Mm(f32),
    Space(f32),
}

impl Unit {
    pub fn to_f32(&self) -> f32 {
        match self {
            Unit::Px(num) => *num,
            Unit::Mm(num) => *num,
            Unit::Space(num) => *num,
        }
    }
}

impl Add for Unit {
    type Output = Unit;

    fn add(self, other: Unit) -> Unit {
        match self {
            Unit::Px(self_num) => match other {
                Unit::Px(other_num) => Unit::Px(self_num + other_num),
                Unit::Mm(_) => panic!("cannot add mm to px unit"),
                Unit::Space(_) => panic!("cannot add space to px unit"),
            },
            Unit::Mm(self_num) => match other {
                Unit::Px(_) => panic!("cannot add px to mm unit"),
                Unit::Mm(other_num) => Unit::Mm(self_num + other_num),
                Unit::Space(_) => panic!("cannot add space to mm unit"),
            },
            Unit::Space(self_num) => match other {
                Unit::Px(_) => panic!("cannot add px to spaces unit"),
                Unit::Mm(_) => panic!("cannot add mm to spaces unit"),
                Unit::Space(other_num) => Unit::Space(self_num + other_num),
            },
        }
    }
}

impl Sub for Unit {
    type Output = Unit;

    fn sub(self, other: Unit) -> Unit {
        match self {
            Unit::Px(self_num) => match other {
                Unit::Px(other_num) => Unit::Px(self_num - other_num),
                Unit::Mm(_) => panic!("cannot add mm to px unit"),
                Unit::Space(_) => panic!("cannot add space to px unit"),
            },
            Unit::Mm(self_num) => match other {
                Unit::Px(_) => panic!("cannot add px to mm unit"),
                Unit::Mm(other_num) => Unit::Mm(self_num - other_num),
                Unit::Space(_) => panic!("cannot add space to mm unit"),
            },
            Unit::Space(self_num) => match other {
                Unit::Px(_) => panic!("cannot add px to spaces unit"),
                Unit::Mm(_) => panic!("cannot add mm to spaces unit"),
                Unit::Space(other_num) => Unit::Space(self_num - other_num),
            },
        }
    }
}

impl Div<u8> for Unit {
    type Output = Unit;

    fn div(self, other: u8) -> Unit {
        match self {
            Unit::Px(num) => Unit::Px(num / other as f32),
            Unit::Mm(num) => Unit::Mm(num / other as f32),
            Unit::Space(num) => Unit::Space(num / other as f32),
        }
    }
}

impl Mul<u8> for Unit {
    type Output = Unit;

    fn mul(self, other: u8) -> Unit {
        match self {
            Unit::Px(num) => Unit::Px(num * other as f32),
            Unit::Mm(num) => Unit::Mm(num * other as f32),
            Unit::Space(num) => Unit::Space(num * other as f32),
        }
    }
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

    pub fn to_px(&self, unit: &Unit) -> Unit {
        match unit {
            Unit::Px(value) => Unit::Px(*value),
            Unit::Mm(value) => Unit::Px(value * self.px_per_mm),
            Unit::Space(value) => Unit::Px(value * self.mm_per_space * self.px_per_mm),
        }
    }

    pub fn to_mm(&self, unit: &Unit) -> Unit {
        match unit {
            Unit::Px(value) => Unit::Mm(value / self.px_per_mm),
            Unit::Mm(value) => Unit::Mm(*value),
            Unit::Space(value) => Unit::Mm(value * self.mm_per_space),
        }
    }

    pub fn to_spaces(&self, unit: &Unit) -> Unit {
        match unit {
            Unit::Px(value) => Unit::Space(value / self.px_per_mm / self.mm_per_space),
            Unit::Mm(value) => Unit::Space(value / self.mm_per_space),
            Unit::Space(value) => Unit::Space(*value),
        }
    }
}

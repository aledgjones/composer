use serde::Serialize;

use super::units::{Mm, Space};

pub struct BoundingBox {
    pub width: Space,
    pub height: Space,
    pub padding: PaddingSpaces,
}

#[derive(Debug)]
pub struct PaddingSpaces {
    pub top: Space,
    pub right: Space,
    pub bottom: Space,
    pub left: Space,
}

impl PaddingSpaces {
    pub fn new(top: Space, right: Space, bottom: Space, left: Space) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[derive(Debug)]
pub struct PaddingMm {
    pub top: Mm,
    pub right: Mm,
    pub bottom: Mm,
    pub left: Mm,
}

impl PaddingMm {
    pub fn new(top: Mm, right: Mm, bottom: Mm, left: Mm) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[derive(Serialize)]
pub struct Point(pub Space, pub Space);

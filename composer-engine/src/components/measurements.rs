use serde::Serialize;

use super::units::{Mm, Space};

pub struct BoundingBox {
    pub width: Space,
    pub height: Space,
    pub padding: PaddingSpaces,
}

impl BoundingBox {
    pub fn none() -> Self {
        BoundingBox {
            width: 0.0,
            height: 0.0,
            padding: PaddingSpaces {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
        }
    }
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

#[derive(Debug, Clone, Serialize)]
pub struct Point {
    pub x: Space,
    pub y: Space,
}

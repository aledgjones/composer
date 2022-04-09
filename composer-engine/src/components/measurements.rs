use serde::Serialize;

use super::units::Unit;

pub struct BoundingBox {
    pub width: Unit,
    pub height: Unit,
    pub padding: Padding,
}

pub struct Padding(pub Unit, pub Unit, pub Unit, pub Unit);

#[derive(Serialize)]
pub struct Point(pub f32, pub f32);

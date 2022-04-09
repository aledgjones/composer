use crate::components::measurements::Point;
use serde::Serialize;

#[derive(Serialize)]
pub struct Line {
    pub color: String,
    pub width: f32,
    pub points: Vec<Point>,
}

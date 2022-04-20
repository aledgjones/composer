use super::measurements::Padding;
use super::units::Unit;

#[derive(Debug)]
pub enum Justify {
    Start,
    Middle,
    End,
}

impl Justify {
    pub fn as_string(&self) -> String {
        match self {
            Justify::Start => String::from("left"),
            Justify::Middle => String::from("center"),
            Justify::End => String::from("right"),
        }
    }
}

#[derive(Debug)]
pub enum Align {
    Top,
    Middle,
    Bottom,
}

impl Align {
    pub fn as_string(&self) -> String {
        match self {
            Align::Top => String::from("top"),
            Align::Middle => String::from("middle"),
            Align::Bottom => String::from("bottom"),
        }
    }
}

#[derive(Debug)]
pub struct Font {
    pub size: Unit,
    pub font: String,
    pub justify: Justify,
    pub align: Align,
    pub padding: Padding,
}

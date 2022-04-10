use super::{measurements::Padding, units::Unit};

#[derive(Debug)]
pub enum Justify {
    Start,
    Middle,
    End,
}

impl Justify {
    fn to_string(&self) -> String {
        match self {
            Justify::Start => String::from("flex-start"),
            Justify::Middle => String::from("center"),
            Justify::End => String::from("flex-end"),
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
    fn to_string(&self) -> String {
        match self {
            Align::Top => String::from("flex-start"),
            Align::Middle => String::from("center"),
            Align::Bottom => String::from("flex-end"),
        }
    }
}

#[derive(Debug)]
pub struct Font {
    pub size: Unit,
    pub font: String,
    pub align: Justify,
    pub padding: Padding,
}

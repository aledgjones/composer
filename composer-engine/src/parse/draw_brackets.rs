use super::get_vertical_spans::VerticalSpans;
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::measurements::Point;
use crate::components::text::Align;
use crate::components::text::Justify;
use crate::components::units::Converter;
use crate::components::units::Space;
use crate::score::engrave::BracketStyle;
use crate::score::engrave::Engrave;
use crate::score::stave::STAVE_LINE_WIDTH;

const TWEEK_FOR_WING: f32 = 0.3125;

pub fn draw_brackets(
    x: &Space,
    y: &Space,
    vertical_spans: &VerticalSpans,
    vertical_spacing: &VerticalSpacing,
    engrave: &Engrave,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let tweek_for_stave_line = STAVE_LINE_WIDTH / 2.0;

    for bracket in &vertical_spans.brackets {
        // short circuit if its a single stave and we don't want single staves bracketed.
        if !engrave.bracket_single_staves && bracket.start == bracket.stop {
            continue;
        }

        let top = vertical_spacing.instruments.get(&bracket.start).unwrap();
        let bottom = vertical_spacing.instruments.get(&bracket.stop).unwrap();

        let top = y + (top.y);
        let bottom = y + bottom.y + bottom.height;

        match engrave.bracket_style {
            BracketStyle::None => {
                // thick line
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&0.5),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x - 0.75)),
                            y: converter.spaces_to_px(&(top - tweek_for_stave_line)),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x - 0.75)),
                            y: converter.spaces_to_px(&(bottom + tweek_for_stave_line)),
                        },
                    ],
                });
            }
            BracketStyle::Line => {
                // think line
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&0.5),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x - 0.75)),
                            y: converter.spaces_to_px(&(top - tweek_for_stave_line)),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x - 0.75)),
                            y: converter.spaces_to_px(&(bottom + tweek_for_stave_line)),
                        },
                    ],
                });
                // thin lines
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&0.125),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x - 1.0)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&top),
                        },
                    ],
                });
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&0.125),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x - 1.0)),
                            y: converter.spaces_to_px(&bottom),
                        },
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
            }
            BracketStyle::Wing => {
                // think line
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&0.5),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x - 0.75)),
                            y: converter.spaces_to_px(&(top - TWEEK_FOR_WING)),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x - 0.75)),
                            y: converter.spaces_to_px(&(bottom + TWEEK_FOR_WING)),
                        },
                    ],
                });
                // wings
                instructions.push(Instruction::Text {
                    x: converter.spaces_to_px(&(x - 1.0)),
                    y: converter.spaces_to_px(&(top - TWEEK_FOR_WING)),
                    value: String::from("\u{E003}"),
                    color: String::from("#000"),
                    font: String::from("Bravura"),
                    size: converter.spaces_to_px(&4.0),
                    justify: Justify::Start.as_string(),
                    align: Align::Middle.as_string(),
                });
                instructions.push(Instruction::Text {
                    x: converter.spaces_to_px(&(x - 1.0)),
                    y: converter.spaces_to_px(&(bottom + TWEEK_FOR_WING)),
                    value: String::from("\u{E004}"),
                    color: String::from("#000"),
                    font: String::from("Bravura"),
                    size: converter.spaces_to_px(&4.0),
                    justify: Justify::Start.as_string(),
                    align: Align::Middle.as_string(),
                });
            }
        }
    }
}

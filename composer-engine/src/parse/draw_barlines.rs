use super::get_barlines::Barlines;
use super::get_vertical_spans::VerticalSpans;
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_horizontal_spacing::Position;
use super::measure_vertical_spacing::VerticalSpacing;
use super::Instruction;
use crate::components::measurements::Point;
use crate::components::units::Converter;
use crate::entries::barline::BarlineDrawType;
use crate::score::stave::Stave;
use crate::score::stave::STAVE_LINE_WIDTH;

fn draw_barline_dots(
    x: &f32,
    y: &f32,
    staves: &[&Stave],
    vertical_spacing: &VerticalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for stave in staves {
        let top = y + vertical_spacing.staves.get(&stave.key).unwrap().y;
        instructions.push(Instruction::Circle {
            color: String::from("#000"),
            radius: converter.spaces_to_px(&0.25),
            point: Point {
                x: converter.spaces_to_px(x),
                y: converter.spaces_to_px(&(top - 0.5)),
            },
        });
        instructions.push(Instruction::Circle {
            color: String::from("#000"),
            radius: converter.spaces_to_px(&0.25),
            point: Point {
                x: converter.spaces_to_px(x),
                y: converter.spaces_to_px(&(top + 0.5)),
            },
        });
    }
}

fn draw_barline(
    x: &f32,
    y: &f32,
    draw_type: &BarlineDrawType,
    staves: &[&Stave],
    vertical_spacing: &VerticalSpacing,
    vertical_spans: &VerticalSpans,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let tweak_for_stave_line_width = STAVE_LINE_WIDTH / 2.0;

    for span in &vertical_spans.barlines {
        let start = vertical_spacing.instruments.get(&span.start).unwrap();
        let stop = vertical_spacing.instruments.get(&span.stop).unwrap();

        let top = y + start.y - tweak_for_stave_line_width;
        let bottom = y + stop.y + stop.height + tweak_for_stave_line_width;

        match draw_type {
            BarlineDrawType::Single => {
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
            }
            BarlineDrawType::Double => {
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 0.5)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 0.5)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
            }
            BarlineDrawType::EndRepeat => {
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 1.0)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 1.0)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&(STAVE_LINE_WIDTH * 4.0)),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 1.75)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 1.75)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
            }
            BarlineDrawType::EndStartRepeat => {
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 1.0)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 1.0)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&(STAVE_LINE_WIDTH * 4.0)),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 1.75)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 1.75)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 2.5)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 2.5)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
            }
            BarlineDrawType::StartRepeat => {
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&(STAVE_LINE_WIDTH * 4.0)),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 0.25)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 0.25)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 1.0)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 1.0)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
            }
            BarlineDrawType::Final => {
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&STAVE_LINE_WIDTH),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(x),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
                instructions.push(Instruction::Line {
                    color: String::from("#000"),
                    width: converter.spaces_to_px(&(STAVE_LINE_WIDTH * 4.0)),
                    points: vec![
                        Point {
                            x: converter.spaces_to_px(&(x + 0.75)),
                            y: converter.spaces_to_px(&top),
                        },
                        Point {
                            x: converter.spaces_to_px(&(x + 0.75)),
                            y: converter.spaces_to_px(&bottom),
                        },
                    ],
                });
            }
        }
    }

    match draw_type {
        BarlineDrawType::EndRepeat => {
            draw_barline_dots(
                &(x + 0.25),
                y,
                staves,
                vertical_spacing,
                converter,
                instructions,
            );
        }
        BarlineDrawType::EndStartRepeat => {
            draw_barline_dots(
                &(x + 0.25),
                y,
                staves,
                vertical_spacing,
                converter,
                instructions,
            );
            draw_barline_dots(
                &(x + 3.25),
                y,
                staves,
                vertical_spacing,
                converter,
                instructions,
            );
        }
        BarlineDrawType::StartRepeat => {
            draw_barline_dots(
                &(x + 1.75),
                y,
                staves,
                vertical_spacing,
                converter,
                instructions,
            );
        }
        _ => (),
    }
}

pub fn draw_barlines(
    x: &f32,
    y: &f32,
    barlines: &Barlines,
    staves: &Vec<&Stave>,
    vertical_spacing: &VerticalSpacing,
    vertical_spans: &VerticalSpans,
    spacing: &HorizontalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    for (tick, def) in barlines {
        if def.end_repeat {
            let offset = spacing.get(tick, &Position::EndRepeat).unwrap().x;
            draw_barline(
                &(x + offset),
                y,
                &BarlineDrawType::EndRepeat,
                staves,
                vertical_spacing,
                vertical_spans,
                converter,
                instructions,
            );
        }

        if let Some(draw_type) = &def.draw_type {
            let offset = spacing.get(tick, &Position::Barline).unwrap().x;
            draw_barline(
                &(x + offset),
                y,
                draw_type,
                staves,
                vertical_spacing,
                vertical_spans,
                converter,
                instructions,
            );
        }

        if def.start_repeat {
            let offset = spacing.get(tick, &Position::StartRepeat).unwrap().x;
            draw_barline(
                &(x + offset),
                y,
                &BarlineDrawType::StartRepeat,
                staves,
                vertical_spacing,
                vertical_spans,
                converter,
                instructions,
            );
        }
    }

    draw_barline(
        &(x + spacing.width - BarlineDrawType::Final.metrics().width),
        y,
        &BarlineDrawType::Final,
        staves,
        vertical_spacing,
        vertical_spans,
        converter,
        instructions,
    );
}

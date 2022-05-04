use super::get_note_positions::Position;
use super::measure_horizontal_spacing::HorizontalSpacing;
use super::measure_vertical_spacing::VerticalSpacing;
use super::{Instruction, Text};
use crate::components::duration::NoteDuration;
use crate::components::misc::Key;
use crate::components::text::{Align, Justify};
use crate::components::units::Converter;
use crate::entries::time_signature::{TimeSignature, TimeSignatureDrawType};
use crate::score::flows::Flow;
use crate::score::stave::Stave;
use crate::score::tracks::Tracks;

fn digit_to_glyph(input: char) -> String {
    match input {
        '0' => String::from("\u{E080}"),
        '1' => String::from("\u{E081}"),
        '2' => String::from("\u{E082}"),
        '3' => String::from("\u{E083}"),
        '4' => String::from("\u{E084}"),
        '5' => String::from("\u{E085}"),
        '6' => String::from("\u{E086}"),
        '7' => String::from("\u{E087}"),
        '8' => String::from("\u{E088}"),
        '9' => String::from("\u{E089}"),
        _ => String::new(),
    }
}

pub fn number_to_glyph(input: u8) -> String {
    let mut output: Vec<String> = Vec::new();
    let str = input.to_string();
    for digit in str.chars() {
        output.push(digit_to_glyph(digit));
    }
    output.concat()
}

pub fn beat_type_to_glyph(input: NoteDuration) -> String {
    let number = match input {
        NoteDuration::Whole => 1,
        NoteDuration::Half => 2,
        NoteDuration::Quarter => 4,
        NoteDuration::Eighth => 8,
        NoteDuration::Sixteenth => 16,
        NoteDuration::ThirtySecond => 32,
        NoteDuration::SixtyFourth => 64,
    };
    number_to_glyph(number)
}

fn draw_time_signature(
    x: f32,
    y: f32,
    time_signature: &TimeSignature,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    match time_signature.draw_type {
        TimeSignatureDrawType::Hidden => (),
        TimeSignatureDrawType::CommonTime => instructions.push(Instruction::Text(Text {
            x: converter.spaces_to_px(&x),
            y: converter.spaces_to_px(&y),
            value: String::from("\u{E08A}"),
            color: String::from("#000"),
            font: String::from("Bravura"),
            size: converter.spaces_to_px(&4.0),
            justify: Justify::Middle.as_string(),
            align: Align::Middle.as_string(),
        })),
        TimeSignatureDrawType::SplitCommonTime => instructions.push(Instruction::Text(Text {
            x: converter.spaces_to_px(&x),
            y: converter.spaces_to_px(&y),
            value: String::from("\u{E08B}"),
            color: String::from("#000"),
            font: String::from("Bravura"),
            size: converter.spaces_to_px(&4.0),
            justify: Justify::Middle.as_string(),
            align: Align::Middle.as_string(),
        })),
        TimeSignatureDrawType::Open => instructions.push(Instruction::Text(Text {
            x: converter.spaces_to_px(&x),
            y: converter.spaces_to_px(&y),
            value: String::from("\u{E09C}"),
            color: String::from("#000"),
            font: String::from("Bravura"),
            size: converter.spaces_to_px(&4.0),
            justify: Justify::Middle.as_string(),
            align: Align::Middle.as_string(),
        })),
        TimeSignatureDrawType::Regular => {
            let beats = number_to_glyph(time_signature.beats);
            let beat_type = beat_type_to_glyph(time_signature.beat_type);
            instructions.push(Instruction::Text(Text {
                x: converter.spaces_to_px(&x),
                y: converter.spaces_to_px(&(y - 1.0)),
                value: beats,
                color: String::from("#000"),
                font: String::from("Bravura"),
                size: converter.spaces_to_px(&4.0),
                justify: Justify::Middle.as_string(),
                align: Align::Middle.as_string(),
            }));
            instructions.push(Instruction::Text(Text {
                x: converter.spaces_to_px(&x),
                y: converter.spaces_to_px(&(y + 1.0)),
                value: beat_type,
                color: String::from("#000"),
                font: String::from("Bravura"),
                size: converter.spaces_to_px(&4.0),
                justify: Justify::Middle.as_string(),
                align: Align::Middle.as_string(),
            }));
        }
    };
}

pub fn draw_time_signatures(
    x: &f32,
    y: &f32,
    flow: &Flow,
    staves: &Vec<&Stave>,
    tracks: &Tracks,
    vertical_spacing: &VerticalSpacing,
    horizontal_spacing: &HorizontalSpacing,
    converter: &Converter,
    instructions: &mut Vec<Instruction>,
) {
    let flow_master = tracks.get(&flow.master).unwrap();

    for stave in staves {
        let top = vertical_spacing.staves.get(&stave.key).unwrap();

        for tick in 0..flow.length {
            if let Some(time_signature) = flow_master.get_time_signature_at_tick(&tick) {
                let left = horizontal_spacing
                    .get(&Key::TickPosition(tick, Position::TimeSignature))
                    .unwrap();
                let offset = time_signature.metrics(flow.subdivisions).width / 2.0;

                draw_time_signature(
                    x + left.x + offset,
                    y + top.y,
                    &time_signature,
                    converter,
                    instructions,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::number_to_glyph;

    #[test]
    fn number_to_glyph_test_1() {
        let result = number_to_glyph(1);
        assert_eq!(result, String::from("\u{E081}"))
    }

    #[test]
    fn number_to_glyph_test_2() {
        let result = number_to_glyph(145);
        assert_eq!(result, String::from("\u{E081}\u{E084}\u{E085}"))
    }
}

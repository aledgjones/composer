use super::get_vertical_spans::VerticalSpans;
use super::measure_vertical_spacing::VerticalSpacing;
use crate::components::units::Space;
use crate::score::engrave::Engrave;

pub fn measure_brackets_and_braces(
    vertical_spacing: &VerticalSpacing,
    spans: &VerticalSpans,
    engrave: &Engrave,
) -> Space {
    let mut max: Space = 0.0;

    // BRACKET of fixed width 1.0
    for bracket in &spans.brackets {
        // we need at least on instrument to bracket single staves
        if engrave.bracket_single_staves || bracket.stop != bracket.start {
            max = 1.0;
            break;
        }
    }

    // SUB-BRACKETS of fixed width .5
    if !spans.sub_brackets.is_empty() {
        max += 0.5;
    }

    // BRACES of variable widths
    for brace in &spans.braces {
        let top = vertical_spacing.staves.get(&brace.start).unwrap();
        let bottom = vertical_spacing.staves.get(&brace.stop).unwrap();
        let height: Space = (bottom.y + bottom.height) - top.y;
        let width = height * 0.1;
        if width > max {
            max = width;
        }
    }

    max
}

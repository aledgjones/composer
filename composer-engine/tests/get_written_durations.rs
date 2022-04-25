use composer_engine::components::articulation::Articulation;
use composer_engine::components::duration::NoteDuration;
use composer_engine::components::pitch::{Accidental, Pitch};
use composer_engine::components::velocity::Velocity;
use composer_engine::entries::time_signature::{TimeSignature, TimeSignatureDrawType};
use composer_engine::entries::tone::Tone;
use composer_engine::entries::Entry;
use composer_engine::parse::get_barlines::get_barlines;
use composer_engine::parse::get_written_durations::NotationTrack;
use composer_engine::score::tracks::Track;
use composer_engine::utils::shortid;

const QUARTER: u32 = 16;
const EIGTH: u32 = 8;

fn run(length: u32, time_signature: (u8, NoteDuration), tones: Vec<(u32, u32)>) -> NotationTrack {
    let mut master = Track::new();
    master.insert(Entry::TimeSignature(TimeSignature::new(
        0,
        time_signature.0,
        time_signature.1,
        TimeSignatureDrawType::Normal,
        None,
    )));

    let mut track = Track::new();
    for (tick, duration) in tones {
        track.insert(Entry::Tone(Tone::new(
            shortid(),
            tick,
            duration,
            Pitch::new(60, Accidental::Natural),
            Velocity::new(100),
            Articulation::None,
        )));
    }

    let barlines = get_barlines(length, &master);
    track.to_notation_track(length, &barlines)
}

#[test]
/// "splits notes at barlines only - 2/4"
fn test_1() {
    let result = run(
        QUARTER * 4,
        (2, NoteDuration::Quarter),
        vec![(0, QUARTER * 4)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o_______________________________o------------------------------:"
    );
}

#[test]
/// "splits rests at barlines only - 2/4"
fn test_2() {
    let result = run(QUARTER * 4, (2, NoteDuration::Quarter), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r------------------------------:r------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 2/4"
fn test_3() {
    let result = run(QUARTER * 2, (2, NoteDuration::Quarter), vec![]);
    assert_eq!(format!("{:?}", result), "r------------------------------:");
}

#[test]
/// "renders a full bar rest as such - 6/8"
fn test_4() {
    let result = run(EIGTH * 6, (6, NoteDuration::Eighth), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r----------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 3/4"
fn test_5() {
    let result = run(QUARTER * 3, (3, NoteDuration::Quarter), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r----------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 9/8"
fn test_6() {
    let result = run(EIGTH * 9, (9, NoteDuration::Eighth), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r----------------------------------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 4/4"
fn test_7() {
    let result = run(QUARTER * 4, (4, NoteDuration::Quarter), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r--------------------------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 12/8"
fn test_8() {
    let result = run(EIGTH * 12, (12, NoteDuration::Eighth), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r----------------------------------------------------------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 5/8"
fn test_9() {
    let result = run(EIGTH * 5, (5, NoteDuration::Eighth), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r--------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 7/8"
fn test_10() {
    let result = run(EIGTH * 7, (7, NoteDuration::Eighth), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r------------------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 2/4"
fn test_11() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, QUARTER * 2)],
    );
    assert_eq!(format!("{:?}", result), "o------------------------------:");
}

#[test]
/// "renders a full bar rest as such - 6/8"
fn test_12() {
    let result = run(EIGTH * 6, (6, NoteDuration::Eighth), vec![(0, EIGTH * 6)]);
    assert_eq!(
        format!("{:?}", result),
        "o----------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 3/4"
fn test_13() {
    let result = run(
        QUARTER * 3,
        (3, NoteDuration::Quarter),
        vec![(0, QUARTER * 3)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o----------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 9/8"
fn test_14() {
    let result = run(EIGTH * 9, (9, NoteDuration::Eighth), vec![(0, EIGTH * 9)]);
    assert_eq!(
        format!("{:?}", result),
        "o_______________________________________________o----------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 4/4"
fn test_15() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(0, QUARTER * 4)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o--------------------------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 12/8"
fn test_16() {
    let result = run(
        EIGTH * 12,
        (12, NoteDuration::Eighth),
        vec![(0, EIGTH * 12)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o----------------------------------------------------------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 5/8"
fn test_17() {
    let result = run(EIGTH * 5, (5, NoteDuration::Eighth), vec![(0, EIGTH * 5)]);
    assert_eq!(
        format!("{:?}", result),
        "o_______________________o--------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 7/8"
fn test_18() {
    let result = run(EIGTH * 7, (7, NoteDuration::Eighth), vec![(0, EIGTH * 7)]);
    assert_eq!(
        format!("{:?}", result),
        "o_______________________o------------------------------:"
    );
}

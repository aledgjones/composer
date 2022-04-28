use composer_engine::components::articulation::Articulation;
use composer_engine::components::duration::NoteDuration;
use composer_engine::components::pitch::Accidental;
use composer_engine::components::pitch::Pitch;
use composer_engine::components::velocity::Velocity;
use composer_engine::entries::time_signature::TimeSignature;
use composer_engine::entries::time_signature::TimeSignatureDrawType;
use composer_engine::entries::tone::Tone;
use composer_engine::entries::Entry;
use composer_engine::parse::get_barlines::get_barlines;
use composer_engine::parse::get_beams::get_beams_in_track;
use composer_engine::parse::get_beams::Beams;
use composer_engine::score::tracks::Track;
use composer_engine::utils::shortid;

const QUARTER: u32 = 16;
const EIGHTH: u32 = 8;
const SIXTEENTH: u32 = 4;

fn run(length: u32, time_signature: (u8, NoteDuration), tones: Vec<(u32, u32)>) -> Beams {
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
    let notation = track.to_notation_track(length, &barlines);
    get_beams_in_track(length, &notation, &barlines)
}

#[test]
/// "beam groups, full 4/4"
fn test_44_1() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![
            (0, EIGHTH),
            (EIGHTH, EIGHTH),
            (EIGHTH * 2, EIGHTH),
            (EIGHTH * 3, EIGHTH),
            (EIGHTH * 4, EIGHTH),
            (EIGHTH * 5, EIGHTH),
            (EIGHTH * 6, EIGHTH),
            (EIGHTH * 7, EIGHTH),
        ],
    );
    assert_eq!(
        format!("{:?}", result),
        "[[0, 8, 16, 24], [32, 40, 48, 56]]"
    );
}

#[test]
/// "beam groups, partial 4/4"
fn test_44_2() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![
            (EIGHTH, EIGHTH),
            (EIGHTH * 2, EIGHTH),
            (EIGHTH * 3, EIGHTH),
            (EIGHTH * 4, EIGHTH),
            (EIGHTH * 5, EIGHTH),
            (EIGHTH * 6, EIGHTH),
        ],
    );
    assert_eq!(format!("{:?}", result), "[[16, 24], [32, 40]]");
}

#[test]
/// "beam groups, sixteenths break at beat 4/4"
fn test_44_3() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![
            (0, EIGHTH),
            (EIGHTH, EIGHTH),
            (EIGHTH * 2, EIGHTH),
            (EIGHTH * 3, EIGHTH),
            (EIGHTH * 4, SIXTEENTH),
            (SIXTEENTH * 9, SIXTEENTH),
            (EIGHTH * 5, EIGHTH),
            (EIGHTH * 6, EIGHTH),
            (EIGHTH * 7, EIGHTH),
        ],
    );
    assert_eq!(
        format!("{:?}", result),
        "[[0, 8, 16, 24], [32, 36, 40], [48, 56]]"
    );
}

#[test]
/// "beam groups, full 3/4"
fn test_34_1() {
    let result = run(
        QUARTER * 3,
        (3, NoteDuration::Quarter),
        vec![
            (0, EIGHTH),
            (EIGHTH, EIGHTH),
            (EIGHTH * 2, EIGHTH),
            (EIGHTH * 3, EIGHTH),
            (EIGHTH * 4, EIGHTH),
            (EIGHTH * 5, EIGHTH),
        ],
    );
    assert_eq!(format!("{:?}", result), "[[0, 8, 16, 24, 32, 40]]");
}

#[test]
/// "beam groups, partial 3/4"
fn test_34_2() {
    let result = run(
        QUARTER * 3,
        (3, NoteDuration::Quarter),
        vec![
            (EIGHTH, EIGHTH),
            (EIGHTH * 2, EIGHTH),
            (EIGHTH * 3, EIGHTH),
            (EIGHTH * 4, EIGHTH),
            (EIGHTH * 5, EIGHTH),
        ],
    );
    assert_eq!(format!("{:?}", result), "[[16, 24], [32, 40]]");
}

#[test]
/// "beam groups, sixteenths break at beat 3/4"
fn test_34_3() {
    let result = run(
        QUARTER * 3,
        (3, NoteDuration::Quarter),
        vec![
            (0, EIGHTH),
            (EIGHTH, EIGHTH),
            (EIGHTH * 2, EIGHTH),
            (EIGHTH * 3, EIGHTH),
            (EIGHTH * 4, SIXTEENTH),
            (SIXTEENTH * 9, SIXTEENTH),
            (EIGHTH * 5, EIGHTH),
        ],
    );
    assert_eq!(format!("{:?}", result), "[[0, 8], [16, 24], [32, 36, 40]]");
}

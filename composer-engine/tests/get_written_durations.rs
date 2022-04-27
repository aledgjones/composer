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
const EIGHTH: u32 = 8;
const SIXTEENTH: u32 = 4;

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
/// "renders a full bar rest as such - 2/4"
fn test_24_1() {
    let result = run(QUARTER * 2, (2, NoteDuration::Quarter), vec![]);
    assert_eq!(format!("{:?}", result), "r------------------------------:");
}

#[test]
/// "renders a full bar rest as such - 2/4"
fn test_24_2() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, QUARTER * 2)],
    );
    assert_eq!(format!("{:?}", result), "o------------------------------:");
}

#[test]
/// "renders correctly - 4/4 [e---]"
fn test_24_3() {
    let result = run(QUARTER * 2, (2, NoteDuration::Quarter), vec![(0, EIGHTH)]);
    assert_eq!(format!("{:?}", result), "o------:r------:r--------------:");
}

#[test]
/// "renders correctly - 2/4 [e--e]"
fn test_24_4() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, EIGHTH), (EIGHTH * 3, EIGHTH)],
    );
    assert_eq!(format!("{:?}", result), "o------:r------:r------:o------:");
}

#[test]
/// "renders correctly - 2/4 [---e]"
fn test_24_5() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(EIGHTH * 3, EIGHTH)],
    );
    assert_eq!(format!("{:?}", result), "r--------------:r------:o------:");
}

#[test]
/// "renders correctly - 2/4 [-q.]"
fn test_24_6() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(EIGHTH, EIGHTH * 3)],
    );
    assert_eq!(format!("{:?}", result), "r------:o----------------------:");
}

#[test]
/// "renders correctly - 2/4 [-q-]"
fn test_24_7() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(EIGHTH, QUARTER)],
    );
    assert_eq!(format!("{:?}", result), "r------:o_______o------:r------:");
}

#[test]
/// "renders correctly - 2/4 [q.-]"
fn test_24_8() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, EIGHTH * 3)],
    );
    assert_eq!(format!("{:?}", result), "o_______________o------:r------:");
}

#[test]
/// "renders correctly - 2/4 [eq.]"
fn test_24_9() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, EIGHTH), (EIGHTH, EIGHTH * 3)],
    );
    assert_eq!(format!("{:?}", result), "o------:o----------------------:");
}

#[test]
/// "renders correctly - 2/4 [eqe]"
fn test_24_10() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, EIGHTH), (EIGHTH, QUARTER), (EIGHTH * 3, EIGHTH)],
    );
    assert_eq!(format!("{:?}", result), "o------:o--------------:o------:");
}

#[test]
/// "renders correctly - 2/4 [q.e]"
fn test_24_11() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, EIGHTH * 3), (EIGHTH * 3, EIGHTH)],
    );
    assert_eq!(format!("{:?}", result), "o----------------------:o------:");
}

#[test]
/// "renders correctly - 2/4 [sq._c]"
fn test_24_12() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(0, SIXTEENTH), (SIXTEENTH, SIXTEENTH * 7)],
    );
    assert_eq!(format!("{:?}", result), "o--:o___________o--------------:");
}

#[test]
/// "renders correctly - 2/4 [----s]"
fn test_24_13() {
    let result = run(
        QUARTER * 2,
        (2, NoteDuration::Quarter),
        vec![(SIXTEENTH * 7, SIXTEENTH)],
    );
    assert_eq!(format!("{:?}", result), "r--------------:r----------:o--:");
}

#[test]
/// "renders a full bar rest as such - 4/4"
fn test_44_1() {
    let result = run(QUARTER * 4, (4, NoteDuration::Quarter), vec![]);
    assert_eq!(
        format!("{:?}", result),
        "r--------------------------------------------------------------:"
    );
}

#[test]
/// "renders a full bar rest as such - 4/4"
fn test_44_2() {
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
/// "renders correctly - 4/4 [q---]"
fn test_44_3() {
    let result = run(QUARTER * 4, (4, NoteDuration::Quarter), vec![(0, QUARTER)]);
    assert_eq!(
        format!("{:?}", result),
        "o--------------:r--------------:r------------------------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [q--q]"
fn test_44_4() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(0, QUARTER), (QUARTER * 3, QUARTER)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o--------------:r--------------:r--------------:o--------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [---q]"
fn test_44_5() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(QUARTER * 3, QUARTER)],
    );
    assert_eq!(
        format!("{:?}", result),
        "r------------------------------:r--------------:o--------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [-m.]"
fn test_44_6() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(QUARTER, QUARTER * 3)],
    );
    assert_eq!(
        format!("{:?}", result),
        "r--------------:o----------------------------------------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [-m-]"
fn test_44_7() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(QUARTER, QUARTER * 2)],
    );
    assert_eq!(
        format!("{:?}", result),
        "r--------------:o_______________o--------------:r--------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [m.-]"
fn test_44_8() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(0, QUARTER * 3)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o----------------------------------------------:r--------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [qm.]"
fn test_44_9() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(0, QUARTER), (QUARTER, QUARTER * 3)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o--------------:o----------------------------------------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [qmq]"
fn test_44_10() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(0, QUARTER), (QUARTER, QUARTER * 2), (QUARTER * 3, QUARTER)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o--------------:o------------------------------:o--------------:"
    );
}

#[test]
/// "renders correctly - 4/4 [m.q]"
fn test_44_11() {
    let result = run(
        QUARTER * 4,
        (4, NoteDuration::Quarter),
        vec![(0, QUARTER * 3), (QUARTER * 3, QUARTER)],
    );
    assert_eq!(
        format!("{:?}", result),
        "o----------------------------------------------:o--------------:"
    );
}

// #[test]
// /// "renders a full bar rest as such - 6/8"
// fn test_4() {
//     let result = run(EIGTH * 6, (6, NoteDuration::Eighth), vec![]);
//     assert_eq!(
//         format!("{:?}", result),
//         "r----------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 3/4"
// fn test_5() {
//     let result = run(QUARTER * 3, (3, NoteDuration::Quarter), vec![]);
//     assert_eq!(
//         format!("{:?}", result),
//         "r----------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 9/8"
// fn test_6() {
//     let result = run(EIGTH * 9, (9, NoteDuration::Eighth), vec![]);
//     assert_eq!(
//         format!("{:?}", result),
//         "r----------------------------------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 12/8"
// fn test_8() {
//     let result = run(EIGTH * 12, (12, NoteDuration::Eighth), vec![]);
//     assert_eq!(
//         format!("{:?}", result),
//         "r----------------------------------------------------------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 5/8"
// fn test_9() {
//     let result = run(EIGTH * 5, (5, NoteDuration::Eighth), vec![]);
//     assert_eq!(
//         format!("{:?}", result),
//         "r--------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 7/8"
// fn test_10() {
//     let result = run(EIGTH * 7, (7, NoteDuration::Eighth), vec![]);
//     assert_eq!(
//         format!("{:?}", result),
//         "r------------------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 6/8"
// fn test_12() {
//     let result = run(EIGTH * 6, (6, NoteDuration::Eighth), vec![(0, EIGTH * 6)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o----------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 3/4"
// fn test_13() {
//     let result = run(
//         QUARTER * 3,
//         (3, NoteDuration::Quarter),
//         vec![(0, QUARTER * 3)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "o----------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 9/8"
// fn test_14() {
//     let result = run(EIGTH * 9, (9, NoteDuration::Eighth), vec![(0, EIGTH * 9)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o_______________________________________________o----------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 12/8"
// fn test_16() {
//     let result = run(
//         EIGTH * 12,
//         (12, NoteDuration::Eighth),
//         vec![(0, EIGTH * 12)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "o----------------------------------------------------------------------------------------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 5/8"
// fn test_17() {
//     let result = run(EIGTH * 5, (5, NoteDuration::Eighth), vec![(0, EIGTH * 5)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o_______________________o--------------:"
//     );
// }

// #[test]
// /// "renders a full bar rest as such - 7/8"
// fn test_18() {
//     let result = run(EIGTH * 7, (7, NoteDuration::Eighth), vec![(0, EIGTH * 7)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o_______________________o------------------------------:"
//     );
// }

// #[test]
// /// "pattern in 6/8"
// fn test_19() {
//     let result = run(
//         EIGTH * 12,
//         (6, NoteDuration::Eighth),
//         vec![
//             (0, EIGTH * 3),
//             (EIGTH * 3, EIGTH),
//             (EIGTH * 4, EIGTH),
//             (EIGTH * 5, EIGTH),
//             (EIGTH * 6, EIGTH * 2),
//             (EIGTH * 8, EIGTH),
//             (EIGTH * 9, EIGTH * 3),
//         ],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "o----------------------:o------:o------:o------:o--------------:o------:o----------------------:"
//     );
// }

// #[test]
// /// "pattern in 3/4"
// fn test_20() {
//     let result = run(
//         EIGTH * 12,
//         (3, NoteDuration::Quarter),
//         vec![
//             (0, EIGTH * 3),
//             (EIGTH * 3, EIGTH),
//             (EIGTH * 4, EIGTH),
//             (EIGTH * 5, EIGTH),
//             (EIGTH * 6, EIGTH * 2),
//             (EIGTH * 8, EIGTH),
//             (EIGTH * 9, EIGTH * 3),
//         ],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "o----------------------:o------:o------:o------:o--------------:o------:o_______o--------------:"
//     );
// }

// #[test]
// /// "renders correctly - 6/8 [q-----]"
// fn test_24() {
//     let result = run(EIGTH * 6, (6, NoteDuration::Eighth), vec![(0, EIGTH)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o------:r------:r------:r----------------------:"
//     );
// }

// #[test]
// /// "renders correctly - 6/8 [c--c]"
// fn test_25() {
//     let result = run(
//         EIGTH * 6,
//         (6, NoteDuration::Eighth),
//         vec![(0, QUARTER), (EIGTH * 4, QUARTER)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "o--------------:r------:r------:o--------------:"
//     );
// }

// #[test]
// /// "renders correctly - 6/8 [-----q]"
// fn test_26() {
//     let result = run(
//         EIGTH * 6,
//         (6, NoteDuration::Eighth),
//         vec![(EIGTH * 5, EIGTH)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "r----------------------:r------:r------:o------:"
//     );
// }

// #[test]
// /// "renders correctly - 12/8 [q-----------]"
// fn test_27() {
//     let result = run(EIGTH * 12, (12, NoteDuration::Eighth), vec![(0, EIGTH)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o------:r------:r------:r----------------------:r----------------------------------------------:"
//     );
// }

// #[test]
// /// "renders correctly - 12/8 [q----------q]"
// fn test_28() {
//     let result = run(
//         EIGTH * 12,
//         (12, NoteDuration::Eighth),
//         vec![(0, EIGTH), (EIGTH * 11, EIGTH)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "o------:r------:r------:r----------------------:r----------------------:r------:r------:o------:"
//     );
// }

// #[test]
// /// "renders correctly - 12/8 [-----------q]"
// fn test_29() {
//     let result = run(
//         EIGTH * 12,
//         (12, NoteDuration::Eighth),
//         vec![(EIGTH * 11, EIGTH)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "r----------------------------------------------:r----------------------:r------:r------:o------:"
//     );
// }

// #[test]
// /// "renders correctly - 3/4 [c--]"
// fn test_30() {
//     let result = run(QUARTER * 3, (3, NoteDuration::Quarter), vec![(0, QUARTER)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o--------------:r--------------:r--------------:"
//     );
// }

// #[test]
// /// "renders correctly - 3/4 [c--]"
// fn test_31() {
//     let result = run(
//         QUARTER * 3,
//         (3, NoteDuration::Quarter),
//         vec![(QUARTER * 2, QUARTER)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "r--------------:r--------------:o--------------:"
//     );
// }

// #[test]
// /// "renders correctly - 9/8 [c.------]"
// fn test_32() {
//     let result = run(EIGTH * 9, (9, NoteDuration::Eighth), vec![(0, EIGTH * 3)]);
//     assert_eq!(
//         format!("{:?}", result),
//         "o----------------------:r----------------------:r----------------------:"
//     );
// }

// #[test]
// /// "renders correctly - 9/8 [------c.]"
// fn test_33() {
//     let result = run(
//         EIGTH * 9,
//         (9, NoteDuration::Eighth),
//         vec![(EIGTH * 6, EIGTH * 3)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "r----------------------:r----------------------:o----------------------:"
//     );
// }

// #[test]
// /// "renders correctly - 6/8 [c_ss---]"
// fn test_35() {
//     let result = run(
//         EIGTH * 6,
//         (6, NoteDuration::Eighth),
//         vec![(0, SIXTEENTH * 5), (SIXTEENTH * 5, SIXTEENTH)],
//     );
//     assert_eq!(
//         format!("{:?}", result),
//         "o_______________o--:o--:r----------------------:"
//     );
// }

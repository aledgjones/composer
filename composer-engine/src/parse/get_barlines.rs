use crate::components::misc::Tick;
use crate::entries::barline::BarlineDrawType;
use crate::entries::time_signature::TimeSignature;
use crate::score::flows::Flow;
use crate::score::tracks::Tracks;
use rustc_hash::FxHashMap;

/// context can mean barlines need to be split over positions
/// esspecially repeat marks, this def allows info about when this happens
#[derive(Debug, PartialEq)]
pub struct BarlineDrawDef {
    /// seperate end repeat
    pub end_repeat: bool,
    /// seperate start repeat
    pub start_repeat: bool,
    /// other draw type if needed (including a combined endstart repeat)
    pub draw_type: Option<BarlineDrawType>,
}

impl BarlineDrawDef {
    pub fn new(end_repeat: bool, start_repeat: bool, draw_type: Option<BarlineDrawType>) -> Self {
        Self {
            end_repeat,
            start_repeat,
            draw_type,
        }
    }
}

pub type Barlines = FxHashMap<Tick, BarlineDrawDef>;

pub fn get_barlines(flow: &Flow, tracks: &Tracks) -> Barlines {
    let mut output: Barlines = FxHashMap::default();

    let master = tracks.get(&flow.master).unwrap();
    let mut time_signature = TimeSignature::default();

    for tick in 0..flow.length {
        // these aren't context aware...
        let key_signature = master.get_key_signature_at_tick(&tick);
        let barline = master.get_barline_at_tick(&tick);
        // ...this is, we need to maintin ref so we can work out where the natural barlines are
        if let Some(entry) = master.get_time_signature_at_tick(&tick) {
            time_signature = entry;
        }

        let mut def = BarlineDrawDef::new(false, false, None);

        match barline {
            Some(barline) => {
                // draw the use defined barline to veto default
                match barline.barline_type {
                    BarlineDrawType::EndRepeat => {
                        def.end_repeat = true;
                    }
                    BarlineDrawType::EndStartRepeat => {
                        // repeat barlines are seperated if there is a key or time sig :|#3/4|:
                        // else we can draw as one barline :||:
                        if key_signature.is_some() || time_signature.tick == tick {
                            def.start_repeat = true;
                            def.end_repeat = true;
                        } else {
                            def.draw_type = Some(BarlineDrawType::EndStartRepeat);
                        }
                    }
                    BarlineDrawType::StartRepeat => {
                        if tick > 0 {
                            def.start_repeat = true;
                            // time sig use an aditional single barline
                            if time_signature.tick == tick {
                                def.draw_type = Some(BarlineDrawType::Single);
                            }
                            // key sig uses an aditional double barline
                            if key_signature.is_some() {
                                def.draw_type = Some(BarlineDrawType::Double);
                            }
                        }
                    }
                    _ => {
                        output.insert(
                            tick,
                            BarlineDrawDef::new(false, false, Some(barline.barline_type)),
                        );
                    }
                }
            }
            None => {
                if tick > 0 {
                    if key_signature.is_some() {
                        // key signatures take double barlines
                        def.draw_type = Some(BarlineDrawType::Double);
                    } else if time_signature.is_on_first_beat(tick, flow.subdivisions) {
                        // this is just your standard barline
                        def.draw_type = Some(BarlineDrawType::Single);
                    }
                }
            }
        }

        if def.end_repeat || def.start_repeat || def.draw_type.is_some() {
            output.insert(tick, def);
        }
    }

    output
}

// #[cfg(test)]
// mod tests {
//     use super::get_barlines;
//     use super::BarlineDrawDef;
//     use super::Barlines;
//     use crate::components::duration::NoteDuration;
//     use crate::entries::barline::Barline;
//     use crate::entries::barline::BarlineDrawType;
//     use crate::entries::key_signature::KeySignature;
//     use crate::entries::key_signature::KeySignatureMode;
//     use crate::entries::time_signature::TimeSignature;
//     use crate::entries::time_signature::TimeSignatureDrawType;
//     use crate::entries::Entry;
//     use crate::score::flows::Flow;
//     use crate::score::tracks::Track;

//     fn run(
//         barline: Option<Barline>,
//         time: Option<TimeSignature>,
//         key: Option<KeySignature>,
//     ) -> Barlines {
//         let mut master = Track::new();
//         let mut flow = Flow::new(&master);
//         flow.length = 16 * 4 * 2;

//         master.insert(Entry::TimeSignature(TimeSignature::new(
//             0,
//             4,
//             NoteDuration::Quarter,
//             TimeSignatureDrawType::Regular,
//             None,
//         )));

//         if let Some(barline) = barline {
//             master.insert(Entry::Barline(barline));
//         }

//         if let Some(time) = time {
//             master.insert(Entry::TimeSignature(time));
//         }

//         if let Some(key) = key {
//             master.insert(Entry::KeySignature(key));
//         }

//         get_barlines(&flow, &hashmap! {master.key.clone() => master})
//     }

//     #[test]
//     /// single barline at first beat
//     fn barlines_1() {
//         let result = run(None, None, None);
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: false, draw_type: Some(BarlineDrawType::Single) }}
//         )
//     }

//     #[test]
//     /// double if manually set (vetos single)
//     fn barlines_2() {
//         let result = run(Some(Barline::new(64, BarlineDrawType::Double)), None, None);
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: false, draw_type: Some(BarlineDrawType::Double) }}
//         )
//     }

//     #[test]
//     /// double if key sig (vetos single)
//     fn barlines_3() {
//         let result = run(
//             None,
//             None,
//             Some(KeySignature::new(64, KeySignatureMode::Major, 2)),
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: false, draw_type: Some(BarlineDrawType::Double) }}
//         )
//     }

//     #[test]
//     /// single if time sig
//     fn barlines_4() {
//         let result = run(
//             None,
//             Some(TimeSignature::new(
//                 64,
//                 4,
//                 NoteDuration::Quarter,
//                 TimeSignatureDrawType::Regular,
//                 None,
//             )),
//             None,
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: false, draw_type: Some(BarlineDrawType::Single) }}
//         )
//     }

//     #[test]
//     /// end repeat
//     fn barlines_5() {
//         let result = run(
//             Some(Barline::new(64, BarlineDrawType::EndRepeat)),
//             None,
//             None,
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: true, start_repeat: false, draw_type: None }}
//         )
//     }

//     #[test]
//     /// start repeat
//     fn barlines_6() {
//         let result = run(
//             Some(Barline::new(64, BarlineDrawType::StartRepeat)),
//             None,
//             None,
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: true, draw_type: None }}
//         )
//     }

//     #[test]
//     /// double and start if key sig
//     fn barlines_7() {
//         let result = run(
//             Some(Barline::new(64, BarlineDrawType::StartRepeat)),
//             None,
//             Some(KeySignature::new(64, KeySignatureMode::Major, 2)),
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: true, draw_type: Some(BarlineDrawType::Double) }}
//         )
//     }

//     #[test]
//     /// single and start if time sig
//     fn barlines_8() {
//         let result = run(
//             Some(Barline::new(64, BarlineDrawType::StartRepeat)),
//             Some(TimeSignature::new(
//                 64,
//                 4,
//                 NoteDuration::Quarter,
//                 TimeSignatureDrawType::Regular,
//                 None,
//             )),
//             None,
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: true, draw_type: Some(BarlineDrawType::Single) }}
//         )
//     }

//     #[test]
//     /// combined endstart if no time/key sig
//     fn barlines_9() {
//         let result = run(
//             Some(Barline::new(64, BarlineDrawType::EndStartRepeat)),
//             None,
//             None,
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: false, start_repeat: false, draw_type: Some(BarlineDrawType::EndStartRepeat) }}
//         )
//     }

//     #[test]
//     /// split endstart if key sig
//     fn barlines_10() {
//         let result = run(
//             Some(Barline::new(64, BarlineDrawType::EndStartRepeat)),
//             None,
//             Some(KeySignature::new(64, KeySignatureMode::Major, 2)),
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: true, start_repeat: true, draw_type: None }}
//         )
//     }

//     #[test]
//     /// split endstart if time sig
//     fn barlines_11() {
//         let result = run(
//             Some(Barline::new(64, BarlineDrawType::EndStartRepeat)),
//             Some(TimeSignature::new(
//                 64,
//                 4,
//                 NoteDuration::Quarter,
//                 TimeSignatureDrawType::Regular,
//                 None,
//             )),
//             None,
//         );
//         assert_eq!(
//             result,
//             hashmap! {64 => BarlineDrawDef { end_repeat: true, start_repeat: true, draw_type: None }}
//         )
//     }
// }

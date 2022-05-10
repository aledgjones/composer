use std::collections::HashMap;

use crate::entries::clef::{Clef, ClefDrawType};
use wasm_bindgen::prelude::*;

#[derive(Hash, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Expression {
    Natural,
    Pizzicato,
    Spiccato,
    Staccato,
    Tremolo,
    Mute, // may have to convert to MuteStaccato etc. later for mixed expressions
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub enum InstrumentType {
    Melodic,
    Percussive,
}

pub struct StaveDef {
    pub lines: Vec<u8>,
    pub clef: Clef,
}

pub struct InstrumentDef {
    pub id: &'static str,
    pub instrument_type: InstrumentType,
    pub path: Vec<&'static str>,
    pub long_name: &'static str,
    pub short_name: &'static str,
    pub staves: Vec<StaveDef>,
    pub solo_patches: HashMap<Expression, &'static str>,
    pub section_patches: HashMap<Expression, &'static str>,
}

lazy_static! {
    pub static ref INSTRUMENT_DEFS: Vec<InstrumentDef> = {
        vec![
            InstrumentDef {
                id: "brass.bass-trombone",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Brass", "Bass Trombone"],
                long_name: "Bass Trombone",
                short_name: "B. Tbn.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/bass-trombone/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/bass-trombone/natural.json"
                },
            },
            InstrumentDef {
                id: "brass.horn.f",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Brass", "Horn", "F"],
                long_name: "Horn in F",
                short_name: "F Hn.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/horn/natural.json",
                    Expression::Staccato => "/patches/horn/staccato.json",
                    Expression::Mute => "/patches/horn/mute.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/horn/natural.json",
                    Expression::Staccato => "/patches/horn/staccato.json",
                    Expression::Mute => "/patches/horn/mute.json",
                },
            },
            InstrumentDef {
                id: "brass.trombone",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Brass", "Trombone"],
                long_name: "Trombone",
                short_name: "Tbn.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/trombone/natural.json",
                    Expression::Staccato => "/patches/trombone/staccato.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/trombone/natural.json",
                    Expression::Staccato => "/patches/trombone/staccato.json"
                },
            },
            InstrumentDef {
                id: "brass.trumpet.b-flat",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Brass", "Trumpet", "B${flat}"],
                long_name: "Trumpet in B${flat}",
                short_name: "B${flat} Tpt.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/trumpet/natural.json",
                    Expression::Staccato => "/patches/trumpet/staccato.json",
                    Expression::Mute => "/patches/trumpet/mute.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/trumpet/natural.json",
                    Expression::Staccato => "/patches/trumpet/staccato.json",
                    Expression::Mute => "/patches/trumpet/mute.json"
                },
            },
            InstrumentDef {
                id: "brass.trumpet.c",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Brass", "Trumpet", "C"],
                long_name: "Trumpet in C",
                short_name: "C Tpt.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/trumpet/natural.json",
                    Expression::Staccato => "/patches/trumpet/staccato.json",
                    Expression::Mute => "/patches/trumpet/mute.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/trumpet/natural.json",
                    Expression::Staccato => "/patches/trumpet/staccato.json",
                    Expression::Mute => "/patches/trumpet/mute.json"
                },
            },
            InstrumentDef {
                id: "brass.tuba",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Brass", "Tuba"],
                long_name: "Tuba",
                short_name: "Tba.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/tuba/natural.json",
                    Expression::Staccato => "/patches/tuba/staccato.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/tuba/natural.json",
                    Expression::Staccato => "/patches/tuba/staccato.json"
                },
            },
            InstrumentDef {
                id: "guitar.acoustic",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Guitar", "Acoustic Guitar"],
                long_name: "Acoustic Guitar",
                short_name: "A. Gtr.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/acoustic-guitar/natural.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/acoustic-guitar/natural.json",
                },
            },
            InstrumentDef {
                id: "guitar.bass",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Guitar", "Bass Guitar"],
                long_name: "Bass Guitar",
                short_name: "B. Gtr.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/bass-guitar/natural.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/bass-guitar/natural.json",
                },
            },
            InstrumentDef {
                id: "guitar.distortion",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Guitar", "Distortion Guitar"],
                long_name: "Distortion Guitar",
                short_name: "Gtr.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/distortion-guitar/natural.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/distortion-guitar/natural.json",
                },
            },
            InstrumentDef {
                id: "unpitched-percussion.crash-cymbal",
                instrument_type: InstrumentType::Percussive,
                path: vec!["Unpitched Percussion", "Crash Cymbal"],
                long_name: "Crash Cymbal",
                short_name: "Cym.",
                staves: vec![StaveDef {
                    lines: vec![1],
                    clef: Clef::new(0, 60, 0, ClefDrawType::Percussion),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/kit-crash/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/kit-crash/natural.json"
                },
            },
            InstrumentDef {
                id: "unpitched-percussion.hi-hat",
                instrument_type: InstrumentType::Percussive,
                path: vec!["Unpitched Percussion", "Hi-Hat"],
                long_name: "Hi-Hat",
                short_name: "HH.",
                staves: vec![StaveDef {
                    lines: vec![1],
                    clef: Clef::new(0, 60, 0, ClefDrawType::Percussion),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/kit-hihat/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/kit-hihat/natural.json"
                },
            },
            InstrumentDef {
                id: "unpitched-percussion.kick",
                instrument_type: InstrumentType::Percussive,
                path: vec!["Unpitched Percussion", "Kick Drum"],
                long_name: "Kick Drum",
                short_name: "K Drm.",
                staves: vec![StaveDef {
                    lines: vec![1],
                    clef: Clef::new(0, 60, 0, ClefDrawType::Percussion),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/kit-kicks/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/kit-kicks/natural.json"
                },
            },
            InstrumentDef {
                id: "unpitched-percussion.snare",
                instrument_type: InstrumentType::Percussive,
                path: vec!["Unpitched Percussion", "Snare"],
                long_name: "Snare",
                short_name: "Sn.",
                staves: vec![StaveDef {
                    lines: vec![1],
                    clef: Clef::new(0, 60, 0, ClefDrawType::Percussion),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/snare/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/snare/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.glockenspiel",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Pitched Percussion", "Glockenspiel"],
                long_name: "Glokenspiel",
                short_name: "Glock.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/glockenspiel/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/glockenspiel/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.harp",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Pitched Percussion", "Harp"],
                long_name: "Harp",
                short_name: "Hrp.",
                staves: vec![
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 67, 2, ClefDrawType::G),
                    },
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 53, -2, ClefDrawType::F),
                    },
                ],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/harp/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/harp/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.marimba",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Pitched Percussion", "Marimba"],
                long_name: "Marimba",
                short_name: "Mrm.",
                staves: vec![
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 67, 2, ClefDrawType::G),
                    },
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 53, -2, ClefDrawType::F),
                    },
                ],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/marimba/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/marimba/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.timpani",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Pitched Percussion", "Timpani"],
                long_name: "Timpani",
                short_name: "Timp.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/timpani/natural.json",
                    Expression::Tremolo => "/patches/timpani/roll.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/timpani/natural.json",
                    Expression::Tremolo => "/patches/timpani/roll.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.vibraphone",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Pitched Percussion", "Vibraphone"],
                long_name: "Vibraphone",
                short_name: "Vib.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/vibraphone/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/vibraphone/natural.json"
                },
            },
            InstrumentDef {
                id: "pitched-percussion.xylophone",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Pitched Percussion", "Xylophone"],
                long_name: "Xylophone",
                short_name: "Xyl.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 79, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/xylophone/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/xylophone/natural.json"
                },
            },
            InstrumentDef {
                id: "keyboard.celesta",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Keyboards", "Celesta"],
                long_name: "Celesta",
                short_name: "Cel.",
                staves: vec![
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 79, 2, ClefDrawType::G),
                    },
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 65, -2, ClefDrawType::F),
                    },
                ],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/celesta/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/celesta/natural.json"
                },
            },
            InstrumentDef {
                id: "keyboard.piano",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Keyboards", "Piano"],
                long_name: "Piano",
                short_name: "Pno.",
                staves: vec![
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 67, 2, ClefDrawType::G),
                    },
                    StaveDef {
                        lines: vec![1, 1, 1, 1, 1],
                        clef: Clef::new(0, 53, -2, ClefDrawType::F),
                    },
                ],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/piano/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/piano/natural.json"
                },
            },
            InstrumentDef {
                id: "strings.contrabass",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Strings", "Contrabass"],
                long_name: "Contrabass",
                short_name: "Cb.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 41, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/contrabass/natural.json",
                    Expression::Pizzicato => "/patches/contrabass/pizzicato.json",
                    Expression::Staccato => "/patches/contrabass/spiccato.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/contrabass-section/natural.json",
                    Expression::Pizzicato => "/patches/contrabass-section/pizzicato.json",
                    Expression::Staccato => "/patches/contrabass-section/spiccato.json"
                },
            },
            InstrumentDef {
                id: "strings.viola",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Strings", "Viola"],
                long_name: "Viola",
                short_name: "Vla.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 60, 0, ClefDrawType::C),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/viola/natural.json",
                    Expression::Pizzicato => "/patches/viola/pizzicato.json",
                    Expression::Staccato => "/patches/viola/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/viola-section/natural.json",
                    Expression::Pizzicato => "/patches/viola-section/pizzicato.json",
                    Expression::Staccato => "/patches/viola-section/staccato.json",
                },
            },
            InstrumentDef {
                id: "strings.violin",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Strings", "Violin"],
                long_name: "Violin",
                short_name: "Vln.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/violin/natural.json",
                    Expression::Pizzicato => "/patches/violin/pizzicato.json",
                    Expression::Staccato => "/patches/violin/spiccato.json",
                    Expression::Tremolo => "/patches/violin/tremolo.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/violin-section/natural.json",
                    Expression::Pizzicato => "/patches/violin-section/pizzicato.json",
                    Expression::Staccato => "/patches/violin-section/spiccato.json",
                    Expression::Tremolo => "/patches/violin-section/tremolo.json"
                },
            },
            InstrumentDef {
                id: "strings.violoncello",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Strings", "Violoncello"],
                long_name: "Violoncello",
                short_name: "Vc.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/violoncello/natural.json",
                    Expression::Pizzicato => "/patches/violoncello/pizzicato.json",
                    Expression::Staccato => "/patches/violoncello/staccato.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/violoncello-section/natural.json",
                    Expression::Pizzicato => "/patches/violoncello-section/pizzicato.json",
                    Expression::Staccato => "/patches/violoncello-section/staccato.json"
                },
            },
            InstrumentDef {
                id: "woodwinds.alto-flute",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Alto Flute"],
                long_name: "Alto Flute",
                short_name: "A. Fl.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/alto-flute/natural.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/alto-flute/natural.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.alto-sxophone",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Alto Saxophone"],
                long_name: "Alto Saxophone",
                short_name: "A. Sax.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/alto-saxophone/natural.json",
                    Expression::Staccato => "/patches/alto-saxophone/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/alto-saxophone/natural.json",
                    Expression::Staccato => "/patches/alto-saxophone/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.bassoon",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Bassoon"],
                long_name: "Bassoon",
                short_name: "Bsn.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/bassoon/natural.json",
                    Expression::Staccato => "/patches/bassoon/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/bassoon/natural.json",
                    Expression::Staccato => "/patches/bassoon/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.bass-clarinet",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Bass Clarinet"],
                long_name: "Bass Clarinet",
                short_name: "B. Cl.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/bass-clarinet/natural.json",
                    Expression::Staccato => "/patches/bass-clarinet/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/bass-clarinet/natural.json",
                    Expression::Staccato => "/patches/bass-clarinet/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.clarinet.a",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Clarinet", "A"],
                long_name: "Clarinet in A",
                short_name: "A Cl.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.clarinet.b-flat",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Clarinet", "B Flat"],
                long_name: "Clarinet in B${flat}",
                short_name: "B${flat} Cl.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/clarinet/natural.json",
                    Expression::Staccato => "/patches/clarinet/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.contrabassoon",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Contrabassoon"],
                long_name: "Contrabasson",
                short_name: "Cbsn.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 53, -2, ClefDrawType::F),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/contrabassoon/natural.json"
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/contrabassoon/natural.json"
                },
            },
            InstrumentDef {
                id: "woodwinds.english-horn",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "English Horn"],
                long_name: "English Horn",
                short_name: "E Hn.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/cor-anglais/natural.json",
                    Expression::Staccato => "/patches/cor-anglais/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/cor-anglais/natural.json",
                    Expression::Staccato => "/patches/cor-anglais/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.flute",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Flute"],
                long_name: "Flute",
                short_name: "Fl.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/flute/natural.json",
                    Expression::Staccato => "/patches/flute/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/flute/natural.json",
                    Expression::Staccato => "/patches/flute/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.oboe",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Oboe"],
                long_name: "Oboe",
                short_name: "Ob.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 67, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/oboe/natural.json",
                    Expression::Staccato => "/patches/oboe/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/oboe/natural.json",
                    Expression::Staccato => "/patches/oboe/staccato.json",
                },
            },
            InstrumentDef {
                id: "woodwinds.piccolo",
                instrument_type: InstrumentType::Melodic,
                path: vec!["Woodwinds", "Piccolo"],
                long_name: "Piccolo",
                short_name: "Pc.",
                staves: vec![StaveDef {
                    lines: vec![1, 1, 1, 1, 1],
                    clef: Clef::new(0, 79, 2, ClefDrawType::G),
                }],
                solo_patches: hashmap! {
                    Expression::Natural => "/patches/piccolo/natural.json",
                    Expression::Staccato => "/patches/piccolo/staccato.json",
                },
                section_patches: hashmap! {
                    Expression::Natural => "/patches/piccolo/natural.json",
                    Expression::Staccato => "/patches/piccolo/staccato.json",
                },
            },
        ]
    };
}

pub fn get_def(id: &str) -> Option<&InstrumentDef> {
    INSTRUMENT_DEFS.iter().find(|&def| def.id == id)
}

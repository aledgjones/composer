use crate::score::engrave::{BracketingApproach, Engrave};
use crate::score::instruments::Instrument;

#[derive(PartialEq)]
enum BracketSpan {
    None,
    Start,
    Continue,
}

#[derive(Debug)]
pub struct VerticalSpan {
    pub start: String,
    pub stop: String,
}

#[derive(Debug)]
pub struct VerticalSpans {
    pub brackets: Vec<VerticalSpan>,
    pub sub_brackets: Vec<VerticalSpan>,
    pub braces: Vec<VerticalSpan>,
    pub barlines: Vec<VerticalSpan>,
}

impl VerticalSpans {
    fn new() -> Self {
        Self {
            brackets: Vec::new(),
            sub_brackets: Vec::new(),
            braces: Vec::new(),
            barlines: Vec::new(),
        }
    }
}

fn get_span_type(
    instrument: &Instrument,
    previous_instrument: &Option<&Instrument>,
    engrave: &Engrave,
) -> BracketSpan {
    match engrave.bracketing_approach {
        BracketingApproach::None => BracketSpan::None,
        BracketingApproach::SmallEnsemble => {
            if instrument.staves.len() > 1 {
                return BracketSpan::None;
            }
            match previous_instrument {
                None => BracketSpan::Start,
                Some(instrument) => {
                    if instrument.staves.len() > 1 {
                        BracketSpan::Start
                    } else {
                        BracketSpan::Continue
                    }
                }
            }
        }
        BracketingApproach::Orchestral => {
            let family = instrument.family();
            let previous_family = match previous_instrument {
                None => "",
                Some(instrument) => instrument.family(),
            };

            match family {
                "strings" | "woodwinds" | "brass" => {
                    if family == previous_family {
                        BracketSpan::Continue
                    } else {
                        BracketSpan::Start
                    }
                }
                _ => BracketSpan::None,
            }
        }
    }
}

pub fn get_vertical_spans(instruments: &[&Instrument], engrave: &Engrave) -> VerticalSpans {
    let mut output = VerticalSpans::new();

    let mut previous_instrument: Option<&Instrument> = None;

    for instrument in instruments {
        let span_type = get_span_type(instrument, &previous_instrument, engrave);

        // BRACKETS
        match span_type {
            BracketSpan::Start => output.brackets.push(VerticalSpan {
                start: instrument.key.clone(),
                stop: instrument.key.clone(),
            }),
            BracketSpan::Continue => {
                let last = output.brackets.last_mut().unwrap();
                last.stop = instrument.key.clone();
            }
            _ => {}
        };

        // SUB-BRACKETS
        if engrave.sub_bracket && span_type == BracketSpan::Continue {
            if let Some(previous_instrument) = previous_instrument {
                if instrument.id == previous_instrument.id {
                    match output.sub_brackets.last_mut() {
                        None => output.sub_brackets.push(VerticalSpan {
                            start: previous_instrument.key.clone(),
                            stop: instrument.key.clone(),
                        }),
                        Some(entry) => {
                            if entry.stop == previous_instrument.key {
                                // extend the span another instrument
                                entry.stop = instrument.key.clone();
                            } else {
                                // start a span from the previous instrument
                                output.sub_brackets.push(VerticalSpan {
                                    start: previous_instrument.key.clone(),
                                    stop: instrument.key.clone(),
                                });
                            }
                        }
                    }
                }
            };
        };

        // BRACES
        if instrument.staves.len() > 1 {
            output.braces.push(VerticalSpan {
                start: String::from(instrument.staves.first().unwrap()),
                stop: String::from(instrument.staves.last().unwrap()),
            });
        }

        // BARLINES
        match span_type {
            BracketSpan::Start => output.barlines.push(VerticalSpan {
                start: instrument.key.clone(),
                stop: instrument.key.clone(),
            }),
            BracketSpan::Continue => {
                let last = output.barlines.last_mut().unwrap();
                last.stop = instrument.key.clone();
            }
            _ => output.barlines.push(VerticalSpan {
                start: instrument.key.clone(),
                stop: instrument.key.clone(),
            }),
        };

        previous_instrument = Some(instrument);
    }

    output
}

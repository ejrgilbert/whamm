use crate::parser::types::Location;
use orca_wasm::ir::types::Tag;
use pest::error::LineColLocation;

type TagData = Vec<u8>;

pub fn get_probe_tag_data(loc: &Option<Location>, op_idx_end: u32) -> TagData {
    if let Some(loc) = loc {
        match loc.line_col {
            LineColLocation::Pos(_) => panic!("A probe should be associated with a span location"),
            LineColLocation::Span(lc0, lc1) => Reason::UserProbe {
                lc0: LineCol::from(lc0),
                lc1: LineCol::from(lc1),
                op_idx_end,
            }
            .into(),
        }
    } else {
        Reason::WhammProbe { op_idx_end }.into()
    }
}
pub fn get_tag_for(loc: &Option<Location>) -> Tag {
    Tag::new(Reason::from(loc).into())
}
pub fn get_reasons_from_tag(tag: &mut TagData) -> Vec<Reason> {
    Reason::from_bytes(tag)
}

#[derive(Clone, Debug)]
pub enum Reason {
    // There's a reason in the Whamm script for this addition
    // it's due to a single character.
    UserPos {
        lc: LineCol,
    },
    // There's a reason in the Whamm script for this addition
    // it's due to a span in the script.
    UserSpan {
        lc0: LineCol,
        lc1: LineCol,
    },
    // There's a reason in the Whamm script for this addition
    // it's due to a probe.
    UserProbe {
        lc0: LineCol,
        lc1: LineCol,

        // If there are multiple probes at a single tag,
        // they will be in order of injection.
        // Stores the index in the vec<op> where this probe ends.
        op_idx_end: u32,
    },
    // The injection was for the Whamm language runtime
    Whamm,
    WhammProbe {
        op_idx_end: u32,
    },
}
impl From<&Option<Location>> for Reason {
    fn from(loc: &Option<Location>) -> Self {
        match loc {
            Some(loc) => match loc.line_col {
                LineColLocation::Pos(lc) => Reason::UserPos {
                    lc: LineCol::from(lc),
                },
                LineColLocation::Span(lc0, lc1) => Reason::UserSpan {
                    lc0: LineCol::from(lc0),
                    lc1: LineCol::from(lc1),
                },
            },
            None => Reason::Whamm,
        }
    }
}
impl From<Reason> for Vec<u8> {
    fn from(reason: Reason) -> Self {
        let mut data = vec![reason.id()];
        match reason {
            Reason::UserPos { lc } => data.extend::<Vec<u8>>(lc.into()),
            Reason::UserSpan { lc0, lc1 } => {
                data.extend::<Vec<u8>>(lc0.into());
                data.extend::<Vec<u8>>(lc1.into());
            }
            Reason::UserProbe {
                lc0,
                lc1,
                op_idx_end,
            } => {
                data.extend::<Vec<u8>>(lc0.into());
                data.extend::<Vec<u8>>(lc1.into());
                data.extend(op_idx_end.to_le_bytes());
            }
            Reason::Whamm => {}
            Reason::WhammProbe { op_idx_end } => data.extend(op_idx_end.to_le_bytes()),
        }
        data
    }
}
impl Reason {
    fn id(&self) -> u8 {
        match self {
            Reason::Whamm => 0,
            Reason::WhammProbe { .. } => 1,
            Reason::UserPos { .. } => 2,
            Reason::UserSpan { .. } => 3,
            Reason::UserProbe { .. } => 4,
        }
    }
    fn from_bytes(bytes: &mut Vec<u8>) -> Vec<Reason> {
        let mut reasons = vec![];
        let id = read_le_u8(bytes);
        match id {
            0 => reasons.push(Self::Whamm),
            1 => reasons.push(Self::WhammProbe {
                op_idx_end: read_le_u32(bytes),
            }),
            2 => reasons.push(Self::UserPos {
                lc: LineCol::read(bytes),
            }),
            3 => reasons.push(Self::UserSpan {
                lc0: LineCol::read(bytes),
                lc1: LineCol::read(bytes),
            }),
            4 => reasons.push(Self::UserProbe {
                lc0: LineCol::read(bytes),
                lc1: LineCol::read(bytes),
                op_idx_end: read_le_u32(bytes),
            }),
            _ => panic!("Invalid reason ID in tag: {id}"),
        }
        // assert_eq!(0, bytes.len());
        reasons
    }
}

#[derive(Clone, Debug)]
pub struct LineCol {
    l: u32,
    c: u32,
}
impl From<(usize, usize)> for LineCol {
    fn from(lc: (usize, usize)) -> Self {
        Self {
            l: lc.0 as u32,
            c: lc.1 as u32,
        }
    }
}
impl LineCol {
    fn read(bytes: &mut Vec<u8>) -> Self {
        Self {
            l: read_le_u32(bytes),
            c: read_le_u32(bytes),
        }
    }
}
impl From<LineCol> for Vec<u8> {
    fn from(lc: LineCol) -> Self {
        let mut data = vec![];

        data.extend(lc.l.to_le_bytes());
        data.extend(lc.c.to_le_bytes());

        data
    }
}

fn read_le_u32(input: &mut Vec<u8>) -> u32 {
    let (int_bytes, rest) = input.split_at(size_of::<u32>());
    let res = u32::from_le_bytes(int_bytes.try_into().unwrap());
    *input = rest.to_vec();
    res
}

fn read_le_u8(input: &mut Vec<u8>) -> u8 {
    let (int_bytes, rest) = input.split_at(size_of::<u8>());
    let res = u8::from_le_bytes(int_bytes.try_into().unwrap());
    *input = rest.to_vec();
    res
}

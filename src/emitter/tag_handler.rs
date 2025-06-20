use pest::error::LineColLocation;
use crate::parser::types::Location;

type TagData = Vec<u8>;

pub fn get_probe_tag_data(loc: &Location, num_ops: u32) -> TagData {
    match loc.line_col {
        LineColLocation::Pos(_) => panic!("A probe should be associated with a span location"),
        LineColLocation::Span(lc0, lc1) => Reason::UserProbe {
            lc0: LineCol::from(lc0),
            lc1: LineCol::from(lc1),
            num_ops,
        }.into()
    }

}
pub fn get_tag_data_for(loc: &Option<Location>) -> TagData {
    Reason::from(loc).into()
}
pub fn get_reasons_from_tag(tag: &TagData) -> Vec<Reason> {
    Reason::from_bytes(tag)
}

pub enum Reason {
    // There's a reason in the Whamm script for this addition
    // it's due to a single character.
    UserPos {
        lc: LineCol
    },
    // There's a reason in the Whamm script for this addition
    // it's due to a span in the script.
    UserSpan {
        lc0: LineCol,
        lc1: LineCol
    },
    // There's a reason in the Whamm script for this addition
    // it's due to a probe.
    UserProbe {
        lc0: LineCol,
        lc1: LineCol,

        // number of opcodes devoted to this probe
        // If there are multiple probes at a single tag,
        // they will be in order of injection.
        // num_ops0 + ... + num_opsN = total_ops
        num_ops: u32
    },
    // The injection was for the Whamm language runtime
    Whamm
}
impl From<&Option<Location>> for Reason {
    fn from(loc: &Option<Location>) -> Self {
        match loc {
            Some(loc) => {
                match loc.line_col {
                    LineColLocation::Pos(lc) => Reason::UserPos { lc: LineCol::from(lc) },
                    LineColLocation::Span(lc0, lc1) => Reason::UserSpan {
                        lc0: LineCol::from(lc0),
                        lc1: LineCol::from(lc1)
                    },
                }
            },
            None => Reason::Whamm
        }
    }
}
impl Into<Vec<u8>> for Reason {
    fn into(self) -> Vec<u8> {
        let mut data = vec![self.id()];
        match self {
            Reason::UserPos {lc} => data.extend::<Vec<u8>>(lc.into()),
            Reason::UserSpan {lc0, lc1}=> {
                data.extend::<Vec<u8>>(lc0.into());
                data.extend::<Vec<u8>>(lc1.into());
            },
            Reason::UserProbe {lc0, lc1, num_ops}=> {
                data.extend::<Vec<u8>>(lc0.into());
                data.extend::<Vec<u8>>(lc1.into());
                data.extend(num_ops.to_le_bytes());
            },
            Reason::Whamm => {}
        }
        data
    }
}
impl Reason {
    fn id(&self) -> u8 {
        match self {
            Reason::Whamm {..} => 0,
            Reason::UserPos {..} => 1,
            Reason::UserSpan {..} => 2,
            Reason::UserProbe {..} => 3,
        }
    }
    fn from_bytes(mut bytes: &[u8]) -> Vec<Reason> {
        let mut reasons = vec![];
        let id = read_le_u8(&mut bytes);
        match id {
            0 => reasons.push(Self::Whamm),
            1 => reasons.push(Self::UserPos {
                lc: LineCol::from(bytes),
            }),
            2 => reasons.push(Self::UserSpan {
                lc0: LineCol::from(bytes),
                lc1: LineCol::from(bytes)
            }),
            3 => reasons.push(Self::UserProbe {
                lc0: LineCol::from(bytes),
                lc1: LineCol::from(bytes),
                num_ops: read_le_u32(&mut bytes)
            }),
            _ => panic!("Invalid reason ID in tag: {id}")
        }
        reasons
    }
}

pub struct LineCol {
    l: u32,
    c: u32
}
impl From<(usize, usize)> for LineCol {
    fn from(lc: (usize, usize)) -> Self {
        Self {
            l: lc.0 as u32,
            c: lc.1 as u32
        }
    }
}
impl From<&[u8]> for LineCol {
    fn from(mut bytes: &[u8]) -> Self {
        Self {
            l: read_le_u32(&mut bytes),
            c: read_le_u32(&mut bytes)
        }
    }
}
impl Into<Vec<u8>> for LineCol {
    fn into(self) -> Vec<u8> {
        let mut data = vec![];

        data.extend(self.l.to_le_bytes());
        data.extend(self.c.to_le_bytes());

        data
    }
}

fn read_le_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(size_of::<u32>());
    *input = rest;
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}

fn read_le_u8(input: &mut &[u8]) -> u8 {
    let (int_bytes, rest) = input.split_at(size_of::<u8>());
    *input = rest;
    u8::from_le_bytes(int_bytes.try_into().unwrap())
}

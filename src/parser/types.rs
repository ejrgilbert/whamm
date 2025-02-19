#![allow(clippy::borrowed_box)]

use pest::error::LineColLocation;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use termcolor::{Buffer, ColorChoice, WriteColor};

use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{green, grey_italics, long_line, magenta, white, yellow};
use crate::parser::rules::{
    print_provider_docs, provider_factory, Event, Package, Probe, Provider, WhammProvider,
};
use orca_wasm::ir::types::DataType as OrcaType;
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use termcolor::BufferWriter;

#[derive(Parser)]
#[grammar = "./parser/whamm.pest"] // Path relative to base `src` dir
pub struct WhammParser;

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        // Follows: https://en.cppreference.com/w/c/language/operator_precedence
        PrattParser::new()
            .op(Op::infix(and, Left) | Op::infix(or, Left)) // LOGOP
            .op(Op::infix(binary_or, Left)) // bitwise OR
            .op(Op::infix(binary_xor, Left)) // bitwise XOR
            .op(Op::infix(binary_and, Left)) // bitwise AND
            .op(Op::infix(eq, Left)                         // RELOP
                | Op::infix(ne, Left)
                | Op::infix(ge, Left)
                | Op::infix(gt, Left)
                | Op::infix(le, Left)
                | Op::infix(lt, Left)
            ).op(Op::infix(lshift, Left) | Op::infix(rshift, Left)) // Bitwise left shift and right shift
            .op(Op::infix(add, Left) | Op::infix(subtract, Left)) // SUMOP
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left)) // MULOP
            .op(Op::prefix(neg) | Op::prefix(binary_not)) // Logical NOT and bitwise NOT
            .op(Op::postfix(cast))
    };
}

const UNEXPECTED_ERR_MSG: &str =
    "WhammParser: Looks like you've found a bug...please report this behavior! Exiting now...";

// ===============
// ==== Types ====
// ===============

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Location {
    /// Line/column within the input string
    pub line_col: LineColLocation,
    pub path: Option<String>,
}
impl Location {
    pub fn from(loc0: &LineColLocation, loc1: &LineColLocation, path: Option<String>) -> Self {
        let pos0 = match loc0 {
            LineColLocation::Pos(pos0) => pos0,
            LineColLocation::Span(span0, ..) => span0,
        };

        let pos1 = match loc1 {
            LineColLocation::Pos(pos0) => pos0,
            LineColLocation::Span(.., span1) => span1,
        };

        Location {
            line_col: LineColLocation::Span(*pos0, *pos1),
            path,
        }
    }

    pub fn span_between(loc0: &Location, loc1: &Location) -> LineColLocation {
        let pos0 = match &loc0.line_col {
            LineColLocation::Pos(pos0) | LineColLocation::Span(pos0, ..) => *pos0,
        };

        let pos1 = match &loc1.line_col {
            LineColLocation::Pos(end1) | LineColLocation::Span(.., end1) => *end1,
        };

        LineColLocation::Span(pos0, pos1)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
pub enum DataType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    Boolean,
    Null,
    Str,
    Tuple {
        ty_info: Vec<Box<DataType>>,
    },
    Map {
        key_ty: Box<DataType>,
        val_ty: Box<DataType>,
    },
    Unknown,
    AssumeGood,
}
impl Hash for DataType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // use any distinct number as an enum variant identifier
        match self {
            DataType::U8
            | DataType::I8
            | DataType::U16
            | DataType::I16
            | DataType::U32
            | DataType::I32
            | DataType::F32
            | DataType::U64
            | DataType::I64
            | DataType::F64
            | DataType::Boolean
            | DataType::Null
            | DataType::Str
            | DataType::AssumeGood
            | DataType::Unknown => {
                state.write_u8(self.id() as u8);
            }
            DataType::Tuple { ty_info } => {
                for ty in ty_info {
                    state.write_u8(self.id() as u8);
                    ty.hash(state);
                }
            }
            DataType::Map { key_ty, val_ty } => {
                state.write_u8(self.id() as u8);
                key_ty.hash(state);
                val_ty.hash(state);
            }
        }
    }
}
impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::U8, DataType::U8)
            | (DataType::I8, DataType::I8)
            | (DataType::U16, DataType::U16)
            | (DataType::I16, DataType::I16)
            | (DataType::U32, DataType::U32)
            | (DataType::I32, DataType::I32)
            | (DataType::U64, DataType::U64)
            | (DataType::I64, DataType::I64)
            | (DataType::F32, DataType::F32)
            | (DataType::F64, DataType::F64)
            | (DataType::Boolean, DataType::Boolean)
            | (DataType::Null, DataType::Null)
            | (DataType::Str, DataType::Str)
            | (DataType::Unknown, DataType::Unknown)
            | (_, DataType::AssumeGood)
            | (DataType::AssumeGood, _) => true,
            (DataType::Tuple { ty_info: ty_info0 }, DataType::Tuple { ty_info: ty_info1 }) => {
                // println!("ty_info0: {:?}", ty_info0);
                // println!("ty_info1: {:?}", ty_info1);
                let res = ty_info0.len() == ty_info1.len()
                    && ty_info0
                        .iter()
                        .zip(ty_info1.iter())
                        .all(|(ty0, ty1)| ty0 == ty1);
                // println!("res: {res}");
                res
            }
            (
                DataType::Map {
                    key_ty: key_ty0,
                    val_ty: val_ty0,
                },
                DataType::Map {
                    key_ty: key_ty1,
                    val_ty: val_ty1,
                },
            ) => key_ty0 == key_ty1 && val_ty0 == val_ty1,
            _ => false,
        }
    }
}
impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::U8 => write!(f, "u8"),
            DataType::I8 => write!(f, "i8"),
            DataType::U16 => write!(f, "u16"),
            DataType::I16 => write!(f, "i16"),
            DataType::U32 => write!(f, "u32"),
            DataType::I32 => write!(f, "i32"),
            DataType::F32 => write!(f, "f32"),
            DataType::U64 => write!(f, "u64"),
            DataType::I64 => write!(f, "i64"),
            DataType::F64 => write!(f, "f64"),
            DataType::Boolean => write!(f, "bool"),
            DataType::Null => write!(f, "null"),
            DataType::Str => write!(f, "str"),
            DataType::Tuple { ty_info } => {
                let mut s = "".to_string();
                s += "(";
                s += &ty_info
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                s += ")";
                write!(f, "{s}")
            }
            DataType::Map { key_ty, val_ty, .. } => {
                write!(f, "map<{},{}>", key_ty, val_ty)
            }
            DataType::AssumeGood => write!(f, "assume_good"),
            DataType::Unknown => write!(f, "unknown"),
        }
    }
}
impl DataType {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::U64
                | DataType::I64
                | DataType::F32
                | DataType::F64
        )
    }
    pub fn is_compatible_with(&self, other: &DataType) -> bool {
        match self {
            DataType::U8
            | DataType::I8
            | DataType::U16
            | DataType::I16
            | DataType::U32
            | DataType::I32
            | DataType::Boolean => other.as_i32_in_wasm(),
            DataType::U64 | DataType::I64 => other.as_i64_in_wasm(),
            DataType::F32
            | DataType::F64
            | DataType::Null
            | DataType::Str
            | DataType::AssumeGood
            | DataType::Unknown
            | DataType::Tuple { .. }
            | DataType::Map { .. } => *other == *self,
        }
    }
    fn as_i32_in_wasm(&self) -> bool {
        self.to_wasm_type() == vec![OrcaType::I32]
    }
    fn as_i64_in_wasm(&self) -> bool {
        self.to_wasm_type() == vec![OrcaType::I64]
    }
    pub fn to_wasm_type(&self) -> Vec<OrcaType> {
        match self {
            DataType::U8
            | DataType::I8
            | DataType::U16
            | DataType::I16
            | DataType::I32
            | DataType::U32
            | DataType::Boolean => vec![OrcaType::I32],
            DataType::F32 => vec![OrcaType::F32],
            DataType::I64 | DataType::U64 => vec![OrcaType::I64],
            DataType::F64 => vec![OrcaType::F64],
            // the ID used to track this var in the lib
            DataType::Map { .. } => vec![OrcaType::I32],
            DataType::Null => unimplemented!(),
            DataType::Str => vec![OrcaType::I32, OrcaType::I32],
            DataType::Tuple { .. } => unimplemented!(),
            DataType::Unknown => unimplemented!(),
            DataType::AssumeGood => unimplemented!(),
        }
    }
    pub fn from_wasm_type(ty: &OrcaType) -> Self {
        match ty {
            OrcaType::I8 => DataType::I8,
            OrcaType::I16 => DataType::I16,
            OrcaType::I32 => DataType::I32,
            OrcaType::I64 => DataType::I64,
            OrcaType::F32 => DataType::F32,
            OrcaType::F64 => DataType::F64,
            OrcaType::FuncRef
            | OrcaType::FuncRefNull
            | OrcaType::Cont
            | OrcaType::NoCont
            | OrcaType::ExternRef
            | OrcaType::ExternRefNull
            | OrcaType::Any
            | OrcaType::AnyNull
            | OrcaType::None
            | OrcaType::NoExtern
            | OrcaType::NoFunc
            | OrcaType::Eq
            | OrcaType::EqNull
            | OrcaType::Struct
            | OrcaType::StructNull
            | OrcaType::Array
            | OrcaType::ArrayNull
            | OrcaType::I31
            | OrcaType::I31Null
            | OrcaType::Exn
            | OrcaType::NoExn
            | OrcaType::Module { .. }
            | OrcaType::RecGroup(_)
            | OrcaType::CoreTypeId(_)
            | OrcaType::V128 => unimplemented!(),
        }
    }
    pub fn can_implicitly_cast(&self) -> bool {
        match self {
            DataType::U8
            | DataType::I8
            | DataType::U16
            | DataType::I16
            | DataType::U32
            | DataType::I32
            | DataType::F32
            | DataType::U64
            | DataType::I64
            | DataType::F64
            | DataType::Unknown => true,
            DataType::Tuple { ty_info } => {
                // check for numeric types
                for ty in ty_info.iter() {
                    if ty.can_implicitly_cast() {
                        return true;
                    }
                }
                false
            }
            DataType::Boolean
            | DataType::Null
            | DataType::Str
            // | DataType::Tuple { .. }
            | DataType::Map { .. }
            | DataType::AssumeGood => false,
        }
    }
    pub fn id(&self) -> i32 {
        match self {
            DataType::U8 => 0,
            DataType::I8 => 1,
            DataType::U16 => 2,
            DataType::I16 => 3,
            DataType::U32 => 4,
            DataType::I32 => 5,
            DataType::F32 => 6,
            DataType::U64 => 7,
            DataType::I64 => 8,
            DataType::F64 => 9,
            DataType::Boolean => 10,
            DataType::Null => 11,
            DataType::Str => 12,
            DataType::Tuple { .. } => 13,
            DataType::Map { .. } => 14,
            DataType::AssumeGood => 15,
            DataType::Unknown => 16,
        }
    }
    pub fn num_bytes(&self) -> Option<usize> {
        match self {
            DataType::U8 | DataType::I8 => Some(1),
            DataType::U16 | DataType::I16 => Some(2),
            DataType::U32 |
            DataType::I32 |
            DataType::F32 |
            DataType::Boolean |
            // We save the map ID as u32!
            DataType::Map { .. } => Some(4),
            DataType::U64 |
            DataType::I64 |
            DataType::F64 => Some(8),
            DataType::Tuple { ty_info } => {
                let mut size = 0;
                for ty in ty_info.iter() {
                    size += ty.num_bytes().unwrap_or_default();
                }
                Some(size)
            }
            DataType::Str |
            DataType::Null |
            DataType::AssumeGood |
            DataType::Unknown => {
                // TODO -- is this okay for AssumeGood?
                // size should be determined respective to the context!
                None
            }
        }
    }

    pub fn print(&self, buffer: &mut Buffer) {
        match self {
            DataType::U8 => {
                yellow(true, "u8".to_string(), buffer);
            }
            DataType::I8 => {
                yellow(true, "i8".to_string(), buffer);
            }
            DataType::U16 => {
                yellow(true, "u16".to_string(), buffer);
            }
            DataType::I16 => {
                yellow(true, "i16".to_string(), buffer);
            }
            DataType::U32 => {
                yellow(true, "u32".to_string(), buffer);
            }
            DataType::I32 => {
                yellow(true, "i32".to_string(), buffer);
            }
            DataType::F32 => {
                yellow(true, "f32".to_string(), buffer);
            }
            DataType::U64 => {
                yellow(true, "u64".to_string(), buffer);
            }
            DataType::I64 => {
                yellow(true, "i64".to_string(), buffer);
            }
            DataType::F64 => {
                yellow(true, "f64".to_string(), buffer);
            }
            DataType::Boolean => {
                yellow(true, "bool".to_string(), buffer);
            }
            DataType::Null => {
                yellow(true, "null".to_string(), buffer);
            }
            DataType::Str => {
                yellow(true, "str".to_string(), buffer);
            }
            DataType::Tuple { ty_info } => {
                white(true, "(".to_string(), buffer);
                let mut is_first = true;
                for ty in ty_info.iter() {
                    if !is_first {
                        white(true, ", ".to_string(), buffer);
                    }
                    ty.print(buffer);
                    is_first = false;
                }

                white(true, ")".to_string(), buffer);
            }
            DataType::Map { key_ty, val_ty } => {
                yellow(true, "map".to_string(), buffer);
                white(true, "<".to_string(), buffer);
                key_ty.print(buffer);
                white(true, ",".to_string(), buffer);
                val_ty.print(buffer);
                white(true, ">".to_string(), buffer);
            }
            DataType::AssumeGood => {
                yellow(true, "assume_good, not type checked".to_string(), buffer);
            }
            DataType::Unknown => {
                yellow(true, "unknown, not type checked".to_string(), buffer);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumLit {
    I8 { val: i8 },
    U8 { val: u8 },
    I16 { val: i16 },
    U16 { val: u16 },
    I32 { val: i32 },
    U32 { val: u32 },
    I64 { val: i64 },
    U64 { val: u64 },
    F32 { val: f32 },
    F64 { val: f64 },
}
impl NumLit {
    fn implicit_cast(&mut self, target: &DataType) -> Result<(), String> {
        match target {
            DataType::U8 => self.as_u8(),
            DataType::I8 => self.as_i8(),
            DataType::U16 => self.as_u16(),
            DataType::I16 => self.as_i16(),
            DataType::U32 => self.as_u32(),
            DataType::I32 => self.as_i32(),
            DataType::U64 => self.as_u64(),
            DataType::I64 => self.as_i64(),
            DataType::F32 => self.as_f32(),
            DataType::F64 => self.as_f64(),
            _ => Err(format!("{} to {}", self.ty(), target)),
        }
    }
    pub fn as_u8(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as u8,
            NumLit::U8 { val } => *val,
            NumLit::I16 { val } => (*val & 0xFF) as u8,
            NumLit::U16 { val } => (*val & 0xFF) as u8,
            NumLit::I32 { val } => (*val & 0xFF) as u8,
            NumLit::U32 { val } => (*val & 0xFF) as u8,
            NumLit::I64 { val } => (*val & 0xFF) as u8,
            NumLit::U64 { val } => (*val & 0xFF) as u8,
            NumLit::F32 { val } => val.trunc() as u8,
            NumLit::F64 { val } => val.trunc() as u8,
        };
        *self = Self::u8(new);
        Ok(())
    }
    pub fn as_i8(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val,
            NumLit::U8 { val } => *val as i8,
            NumLit::I16 { val } => (*val & 0xFF) as i8,
            NumLit::U16 { val } => (*val & 0xFF) as i8,
            NumLit::I32 { val } => (*val & 0xFF) as i8,
            NumLit::U32 { val } => (*val & 0xFF) as i8,
            NumLit::I64 { val } => (*val & 0xFF) as i8,
            NumLit::U64 { val } => (*val & 0xFF) as i8,
            NumLit::F32 { val } => val.trunc() as i8,
            NumLit::F64 { val } => val.trunc() as i8,
        };
        *self = Self::i8(new);
        Ok(())
    }
    pub fn as_u16(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as u16,
            NumLit::U8 { val } => *val as u16,
            NumLit::I16 { val } => *val as u16,
            NumLit::U16 { val } => *val,
            NumLit::I32 { val } => (*val & 0xFFFF) as u16,
            NumLit::U32 { val } => (*val & 0xFFFF) as u16,
            NumLit::I64 { val } => (*val & 0xFFFF) as u16,
            NumLit::U64 { val } => (*val & 0xFFFF) as u16,
            NumLit::F32 { val } => val.trunc() as u16,
            NumLit::F64 { val } => val.trunc() as u16,
        };
        *self = Self::u16(new);
        Ok(())
    }
    pub fn as_i16(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as i16,
            NumLit::U8 { val } => *val as i16,
            NumLit::I16 { val } => *val,
            NumLit::U16 { val } => *val as i16,
            NumLit::I32 { val } => (*val & 0xFFFF) as i16,
            NumLit::U32 { val } => (*val & 0xFFFF) as i16,
            NumLit::I64 { val } => (*val & 0xFFFF) as i16,
            NumLit::U64 { val } => (*val & 0xFFFF) as i16,
            NumLit::F32 { val } => val.trunc() as i16,
            NumLit::F64 { val } => val.trunc() as i16,
        };
        *self = Self::i16(new);
        Ok(())
    }
    pub fn as_u32(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as u32,
            NumLit::U8 { val } => *val as u32,
            NumLit::I16 { val } => *val as u32,
            NumLit::U16 { val } => *val as u32,
            NumLit::I32 { val } => {
                // always fits
                *val as u32
            }
            NumLit::U32 { .. } => return Ok(()),
            NumLit::I64 { val } => {
                if *val < u32::MIN as i64 || *val > u32::MAX as i64 {
                    return Err("out of min/max range".to_string());
                }
                *val as u32
            }
            NumLit::U64 { val } => {
                if *val < u32::MIN as u64 || *val > u32::MAX as u64 {
                    return Err("out of min/max range".to_string());
                }
                *val as u32
            }
            NumLit::F32 { val } => {
                if *val < u32::MIN as f32 || *val > u32::MAX as f32 {
                    return Err("out of min/max range".to_string());
                }
                *val as u32
            }
            NumLit::F64 { val } => {
                if *val < u32::MIN as f64 || *val > u32::MAX as f64 {
                    return Err("out of min/max range".to_string());
                }
                *val as u32
            }
        };
        *self = Self::u32(new);
        Ok(())
    }
    pub fn as_i32(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as i32,
            NumLit::U8 { val } => *val as i32,
            NumLit::I16 { val } => *val as i32,
            NumLit::U16 { val } => *val as i32,
            NumLit::I32 { .. } => return Ok(()),
            NumLit::U32 { val } => {
                if *val > i32::MAX as u32 {
                    return Err("out of min/max range".to_string());
                }
                *val as i32
            }
            NumLit::I64 { val } => {
                if *val < i32::MIN as i64 || *val > i32::MAX as i64 {
                    return Err("out of min/max range".to_string());
                }
                *val as i32
            }
            NumLit::U64 { val } => {
                if *val > i32::MAX as u64 {
                    return Err("out of min/max range".to_string());
                }
                *val as i32
            }
            NumLit::F32 { val } => {
                if *val < i32::MIN as f32 || *val > i32::MAX as f32 {
                    return Err("out of min/max range".to_string());
                }
                *val as i32
            }
            NumLit::F64 { val } => {
                if *val < i32::MIN as f64 || *val > i32::MAX as f64 {
                    return Err("out of min/max range".to_string());
                }
                *val as i32
            }
        };
        *self = Self::i32(new);
        Ok(())
    }
    pub fn as_u64(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as u64,
            NumLit::U8 { val } => *val as u64,
            NumLit::I16 { val } => *val as u64,
            NumLit::U16 { val } => *val as u64,
            NumLit::I32 { val } => {
                // always fits
                *val as u64
            }
            NumLit::U32 { val } => {
                // always fits
                *val as u64
            }
            NumLit::I64 { val } => {
                // always fits
                *val as u64
            }
            NumLit::U64 { .. } => return Ok(()),
            Self::F32 { val } => {
                if *val < u64::MIN as f32 || *val > u64::MAX as f32 {
                    return Err("out of min/max range".to_string());
                }
                *val as u64
            }
            Self::F64 { val } => {
                if *val < u64::MIN as f64 || *val > u64::MAX as f64 {
                    return Err("out of min/max range".to_string());
                }
                *val as u64
            }
        };
        *self = Self::u64(new);
        Ok(())
    }
    pub fn as_i64(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as i64,
            NumLit::U8 { val } => *val as i64,
            NumLit::I16 { val } => *val as i64,
            NumLit::U16 { val } => *val as i64,
            NumLit::I32 { val } => {
                // always fits
                *val as i64
            }
            NumLit::U32 { val } => {
                // always fits
                *val as i64
            }
            NumLit::I64 { .. } => return Ok(()),
            NumLit::U64 { val } => {
                if *val > i64::MAX as u64 {
                    return Err("out of min/max range".to_string());
                }
                *val as i64
            }
            NumLit::F32 { val } => {
                if *val < i64::MIN as f32 || *val > i64::MAX as f32 {
                    return Err("out of min/max range".to_string());
                }
                *val as i64
            }
            NumLit::F64 { val } => {
                if *val < i64::MIN as f64 || *val > i64::MAX as f64 {
                    return Err("out of min/max range".to_string());
                }
                *val as i64
            }
        };
        *self = Self::i64(new);
        Ok(())
    }
    pub fn as_f32(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as f32,
            NumLit::U8 { val } => *val as f32,
            NumLit::I16 { val } => *val as f32,
            NumLit::U16 { val } => *val as f32,
            NumLit::I32 { val } => {
                if *val < f32::MIN as i32 || *val > f32::MAX as i32 {
                    return Err("out of min/max range".to_string());
                }
                *val as f32
            }
            NumLit::U32 { val } => {
                if *val < f32::MIN as u32 || *val > f32::MAX as u32 {
                    return Err("out of min/max range".to_string());
                }
                *val as f32
            }
            NumLit::I64 { val } => {
                if *val < f32::MIN as i64 || *val > f32::MAX as i64 {
                    return Err("out of min/max range".to_string());
                }
                *val as f32
            }
            NumLit::U64 { val } => {
                if *val < f32::MIN as u64 || *val > f32::MAX as u64 {
                    return Err("out of min/max range".to_string());
                }
                *val as f32
            }
            NumLit::F32 { .. } => return Ok(()),
            NumLit::F64 { val } => {
                if *val < f32::MIN as f64 || *val > f32::MAX as f64 {
                    return Err("out of min/max range".to_string());
                }
                *val as f32
            }
        };
        *self = Self::f32(new);
        Ok(())
    }
    pub fn as_f64(&mut self) -> Result<(), String> {
        let new = match self {
            NumLit::I8 { val } => *val as f64,
            NumLit::U8 { val } => *val as f64,
            NumLit::I16 { val } => *val as f64,
            NumLit::U16 { val } => *val as f64,
            NumLit::I32 { val } => {
                if *val < f64::MIN as i32 || *val > f64::MAX as i32 {
                    return Err("out of min/max range".to_string());
                }
                *val as f64
            }
            NumLit::U32 { val } => {
                if *val < f64::MIN as u32 || *val > f64::MAX as u32 {
                    return Err("out of min/max range".to_string());
                }
                *val as f64
            }
            NumLit::I64 { val } => {
                if *val < f64::MIN as i64 || *val > f64::MAX as i64 {
                    return Err("out of min/max range".to_string());
                }
                *val as f64
            }
            NumLit::U64 { val } => {
                if *val < f64::MIN as u64 || *val > f64::MAX as u64 {
                    return Err("out of min/max range".to_string());
                }
                *val as f64
            }
            Self::F32 { val } => {
                // always fits
                *val as f64
            }
            Self::F64 { .. } => return Ok(()),
        };
        *self = Self::f64(new);
        Ok(())
    }
    pub fn is_true_ish(&self) -> bool {
        match self {
            NumLit::I8 { val } => *val != 0,
            NumLit::U8 { val } => *val != 0,
            NumLit::I16 { val } => *val != 0,
            NumLit::U16 { val } => *val != 0,
            NumLit::I32 { val } => *val != 0,
            NumLit::U32 { val } => *val != 0,
            NumLit::I64 { val } => *val != 0,
            NumLit::U64 { val } => *val != 0,
            NumLit::F32 { val } => *val != 0f32,
            NumLit::F64 { val } => *val != 0f64,
        }
    }
    pub fn i8(val: i8) -> Self {
        Self::I8 { val }
    }
    pub fn u8(val: u8) -> Self {
        Self::U8 { val }
    }
    pub fn i16(val: i16) -> Self {
        Self::I16 { val }
    }
    pub fn u16(val: u16) -> Self {
        Self::U16 { val }
    }
    pub fn i32(val: i32) -> Self {
        Self::I32 { val }
    }
    pub fn u32(val: u32) -> Self {
        Self::U32 { val }
    }
    pub fn i64(val: i64) -> Self {
        Self::I64 { val }
    }
    pub fn u64(val: u64) -> Self {
        Self::U64 { val }
    }
    pub fn f32(val: f32) -> Self {
        Self::F32 { val }
    }
    pub fn f64(val: f64) -> Self {
        Self::F64 { val }
    }
    pub fn ty(&self) -> DataType {
        match self {
            NumLit::I8 { .. } => DataType::I8,
            NumLit::U8 { .. } => DataType::U8,
            NumLit::I16 { .. } => DataType::I16,
            NumLit::U16 { .. } => DataType::U16,
            NumLit::I32 { .. } => DataType::I32,
            NumLit::U32 { .. } => DataType::U32,
            NumLit::I64 { .. } => DataType::I64,
            NumLit::U64 { .. } => DataType::U64,
            NumLit::F32 { .. } => DataType::F32,
            NumLit::F64 { .. } => DataType::F64,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NumFmt {
    Bin,
    Hex,
    Dec,
    NA, // not applicable (created by compiler)
}
impl NumFmt {
    pub fn base(&self) -> u32 {
        match self {
            Self::Bin => 2,
            Self::Hex => 16,
            Self::Dec => 10,
            Self::NA => u32::MAX,
        }
    }
}

// Values
#[derive(Clone, Debug)]
pub enum Value {
    Number {
        val: NumLit,
        ty: DataType,
        token: String,
        fmt: NumFmt,
    },
    Boolean {
        val: bool,
    },
    Str {
        val: String,
    },
    Tuple {
        ty: DataType,
        vals: Vec<Expr>,
    },
    U32U32Map {
        val: Box<HashMap<u32, u32>>,
    },
}
impl Value {
    pub fn gen_u8(val: u8) -> Self {
        Self::gen_num(NumLit::u8(val), DataType::U8)
    }
    pub fn gen_i8(val: i8) -> Self {
        Self::gen_num(NumLit::i8(val), DataType::I8)
    }
    pub fn gen_u16(val: u16) -> Self {
        Self::gen_num(NumLit::u16(val), DataType::U16)
    }
    pub fn gen_i16(val: i16) -> Self {
        Self::gen_num(NumLit::i16(val), DataType::I16)
    }
    pub fn gen_u32(val: u32) -> Self {
        Self::gen_num(NumLit::u32(val), DataType::U32)
    }
    pub fn gen_i32(val: i32) -> Self {
        Self::gen_num(NumLit::i32(val), DataType::I32)
    }
    pub fn gen_u64(val: u64) -> Self {
        Self::gen_num(NumLit::u64(val), DataType::U64)
    }
    pub fn gen_i64(val: i64) -> Self {
        Self::gen_num(NumLit::i64(val), DataType::I64)
    }
    pub fn gen_f32(val: f32) -> Self {
        Self::gen_num(NumLit::f32(val), DataType::F32)
    }
    pub fn gen_f64(val: f64) -> Self {
        Self::gen_num(NumLit::f64(val), DataType::F64)
    }
    fn gen_num(val: NumLit, ty: DataType) -> Self {
        // generated by the compiler
        Self::Number {
            val,
            ty,
            token: "".to_string(),
            fmt: NumFmt::NA,
        }
    }
    pub fn ty(&self) -> DataType {
        match self {
            Value::Number { ty, .. } => ty.clone(),
            Value::Boolean { .. } => DataType::Boolean,
            Value::Str { .. } => DataType::Str,
            Value::Tuple { ty, .. } => ty.clone(),
            Value::U32U32Map { .. } => DataType::Map {
                key_ty: Box::new(DataType::U32),
                val_ty: Box::new(DataType::U32),
            },
        }
    }
    pub fn implicit_cast(&mut self, target: &DataType) -> Result<(), String> {
        match self {
            Value::Number {
                val, token, fmt, ..
            } => {
                let new_val = match val.implicit_cast(target) {
                    Ok(_) => val.to_owned(),
                    Err(msg) => return Err(msg),
                };

                *self = Value::Number {
                    val: new_val,
                    ty: target.clone(),
                    token: token.to_owned(),
                    fmt: fmt.to_owned(),
                };
                Ok(())
            }
            Value::Tuple { vals, ty } => {
                // constraints on the target data type
                if let DataType::Tuple { ty_info } = target {
                    if vals.len() != ty_info.len() {
                        return Err(format!("{ty} to {target}"));
                    }

                    let mut success = false;
                    let mut msg = "".to_string();
                    for (i, val) in vals.iter_mut().enumerate() {
                        match val.internal_implicit_cast(ty_info.get(i).unwrap()) {
                            Ok(()) => success = true, // do nothing
                            Err(e) => msg = e, // ignore (might be able to cast other indices of the tuple)
                        }
                    }
                    if !success {
                        Err(msg)
                    } else {
                        Ok(())
                    }
                } else {
                    Err(format!("{ty} to {target}"))
                }
            }
            _ => Err("non-numeric values".to_string()),
        }
    }
    pub fn check_explicit_cast(&mut self, target: &DataType) -> Result<(), String> {
        self.explicit_cast(target, false)
    }
    pub fn do_explicit_cast(&mut self, target: &DataType) -> Result<(), String> {
        self.explicit_cast(target, true)
    }
    fn explicit_cast(&mut self, target: &DataType, perform_cast: bool) -> Result<(), String> {
        if matches!(target, DataType::Boolean) {
            // first make the value represented as an i32
            match self.implicit_cast(&DataType::I32) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }
        match self {
            Value::Number { val, .. } => {
                if target.can_implicitly_cast() {
                    // can just go ahead and do the implicit cast whether
                    // perform_cast is true...it's just a primitive...
                    // which is a local cast operation by nature.
                    self.implicit_cast(target)
                } else if matches!(target, DataType::Boolean) {
                    if perform_cast {
                        *self = Self::Boolean {
                            val: val.is_true_ish(),
                        };
                    }
                    Ok(())
                } else {
                    Err(format!("{} to {}", self.ty(), target))
                }
            }
            Value::Boolean { val } => {
                let num_rep = if *val { 1 } else { 0 };
                if !perform_cast {
                    return Ok(());
                }
                *self = match target {
                    DataType::U8 | DataType::I8 | DataType::U16 | DataType::I16 | DataType::U32 => {
                        Value::gen_num(NumLit::u32(num_rep), target.clone())
                    }
                    DataType::I32 => Value::gen_num(NumLit::i32(num_rep as i32), target.clone()),
                    DataType::U64 => Value::gen_num(NumLit::u64(num_rep as u64), target.clone()),
                    DataType::I64 => Value::gen_num(NumLit::i64(num_rep as i64), target.clone()),
                    DataType::F32 => Value::gen_num(NumLit::f32(num_rep as f32), target.clone()),
                    DataType::F64 => Value::gen_num(NumLit::f64(num_rep as f64), target.clone()),
                    _ => return Err(format!("{} to {}", self.ty(), target)),
                };
                Ok(())
            }
            Value::Tuple { .. } => {
                todo!()
            }
            _ => Err("non-numeric values".to_string()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub return_ty: Option<DataType>,
    pub loc: Option<Location>,
}
impl Block {
    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty()
    }
    pub fn loc(&self) -> &Option<Location> {
        &self.loc
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        self.loc().as_ref().map(|loc| loc.line_col.clone())
    }
}

// Statements
#[derive(Clone, Debug)]
pub enum Statement {
    Decl {
        ty: DataType,
        var_id: Expr, // should be VarId
        loc: Option<Location>,
    },

    Assign {
        var_id: Expr, // Should be VarId
        expr: Expr,
        loc: Option<Location>,
    },
    SetMap {
        map: Expr, // Should be VarId
        key: Expr,
        val: Expr,
        loc: Option<Location>,
    },
    Expr {
        expr: Expr,
        loc: Option<Location>,
    },
    Return {
        expr: Expr,
        loc: Option<Location>,
    },
    If {
        cond: Expr,
        conseq: Block,
        alt: Block,
        loc: Option<Location>,
    },
    // all report variables must be unshared,
    // but not all unshared variables must be reported
    UnsharedDecl {
        is_report: bool,
        decl: Box<Statement>,
        loc: Option<Location>,
    },
}
impl Statement {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Statement::Decl { loc, .. }
            | Statement::If { loc, .. }
            | Statement::Return { loc, .. }
            | Statement::Assign { loc, .. }
            | Statement::SetMap { loc, .. }
            | Statement::UnsharedDecl { loc, .. }
            | Statement::Expr { loc, .. } => loc,
        }
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        self.loc().as_ref().map(|loc| loc.line_col.clone())
    }
    pub fn dummy() -> Self {
        Self::Expr {
            expr: Expr::Primitive {
                val: Value::Number {
                    val: NumLit::u32(0),
                    ty: DataType::U32,
                    token: "0".to_string(),
                    fmt: NumFmt::Dec,
                },
                loc: None,
            },
            loc: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    UnOp {
        // Type is based on the outermost `op`
        op: UnOp,
        expr: Box<Expr>,

        done_on: DataType, // The type of data that this unary operation is performed on (populated by type checker)
        loc: Option<Location>,
    },
    Ternary {
        cond: Box<Expr>,
        conseq: Box<Expr>,
        alt: Box<Expr>,
        ty: DataType, // populated by the type-checker (for knowing the return_ty of the emitted blocks)
        loc: Option<Location>,
    },
    BinOp {
        // Type is based on the outermost `op` (if arithmetic op, also based on types of lhs/rhs due to doubles)
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,

        done_on: DataType, // The type of data that this binary operation is performed on (populated by type checker)
        loc: Option<Location>,
    },
    Call {
        // Type is fn_target.return_ty, should be VarId
        fn_target: Box<Expr>,
        args: Vec<Expr>,
        loc: Option<Location>,
    },
    VarId {
        definition: Definition,
        name: String,
        loc: Option<Location>,
    },
    Primitive {
        // Type is val.ty
        val: Value,
        loc: Option<Location>,
    },
    MapGet {
        map: Box<Expr>, //This should be a VarId
        key: Box<Expr>,
        loc: Option<Location>,
    },
}
impl Expr {
    pub fn one(line_col: LineColLocation) -> Self {
        Expr::Primitive {
            val: Value::Number {
                val: NumLit::u32(1),
                ty: DataType::U32,
                token: "1".to_string(),
                fmt: NumFmt::Dec,
            },
            loc: Some(Location {
                line_col,
                path: None,
            }),
        }
    }
    pub fn implicit_cast(&mut self, target: &DataType) -> Result<(), (String, bool)> {
        match self.internal_implicit_cast(target) {
            Err(msg) => Err((
                format!("CastError: Cannot implicitly cast {msg}. Please add an explicit cast."),
                false,
            )),
            _ => Ok(()),
        }
    }
    fn internal_implicit_cast(&mut self, target: &DataType) -> Result<(), String> {
        match self {
            Self::Primitive { val: value, .. } => match value.implicit_cast(target) {
                Ok(()) => Ok(()),
                Err(res) => Err(res),
            },
            Self::Ternary { conseq, alt, .. } => match conseq.internal_implicit_cast(target) {
                Ok(()) => match alt.internal_implicit_cast(target) {
                    Ok(()) => Ok(()),
                    Err(res) => Err(res),
                },
                Err(res) => Err(res),
            },
            _ => Err("expression".to_string()),
        }
    }

    pub fn loc(&self) -> &Option<Location> {
        match self {
            Expr::UnOp { loc, .. }
            | Expr::Ternary { loc, .. }
            | Expr::BinOp { loc, .. }
            | Expr::Call { loc, .. }
            | Expr::VarId { loc, .. }
            | Expr::MapGet { loc, .. }
            | Expr::Primitive { loc, .. } => loc,
        }
    }
}

// Functions

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FnId {
    pub name: String,
    pub loc: Option<Location>,
}

#[derive(Clone, Debug)]
pub struct Fn {
    pub(crate) def: Definition,
    pub(crate) name: FnId,
    pub(crate) params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) return_ty: DataType,
    pub(crate) body: Block,
}
impl Fn {
    pub fn print(&self, buffer: &mut Buffer) {
        green(true, self.name.name.to_string(), buffer);
        white(true, "(".to_string(), buffer);
        let mut is_first = true;
        for (param_name, param_ty) in self.params.iter() {
            if !is_first {
                white(true, ", ".to_string(), buffer);
            }
            if let Expr::VarId { name, .. } = param_name {
                green(true, name.to_string(), buffer);
                white(true, ": ".to_string(), buffer);
                param_ty.print(buffer);
            }
            is_first = false;
        }
        white(true, ")".to_string(), buffer);

        white(true, " -> ".to_string(), buffer);
        self.return_ty.print(buffer);
    }

    pub fn is_static(&self) -> bool {
        matches!(self.def, Definition::CompilerStatic)
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self.def, Definition::CompilerDynamic)
    }

    pub fn is_from_user(&self) -> bool {
        matches!(self.def, Definition::User)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Definition {
    User,
    CompilerStatic,
    CompilerDynamic,
}
impl Definition {
    pub fn is_comp_provided(&self) -> bool {
        matches!(self, Definition::CompilerStatic) || matches!(self, Definition::CompilerDynamic)
    }
}

#[derive(Clone, Debug)]
pub struct Global {
    pub def: Definition,
    pub report: bool,
    pub ty: DataType,
    pub var_name: Expr, // Should be VarId
    pub value: Option<Value>,
}
impl Global {
    pub fn print(&self, buffer: &mut Buffer) {
        if let Expr::VarId { name, .. } = &self.var_name {
            green(true, name.to_string(), buffer);
        }
        white(true, ": ".to_string(), buffer);
        self.ty.print(buffer);
    }

    pub fn is_static(&self) -> bool {
        matches!(self.def, Definition::CompilerStatic)
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self.def, Definition::CompilerDynamic)
    }

    pub fn is_from_user(&self) -> bool {
        matches!(self.def, Definition::User)
    }
}

pub(crate) fn print_global_vars(
    tabs: &mut usize,
    globals: &HashMap<String, ProvidedGlobal>,
    buffer: &mut Buffer,
) {
    if !globals.is_empty() {
        white(true, format!("{}GLOBALS:\n", " ".repeat(*tabs * 4)), buffer);
        *tabs += 1;
        for (.., ProvidedGlobal { docs, global, .. }) in globals.iter() {
            white(false, " ".repeat(*tabs * 4).to_string(), buffer);
            global.print(buffer);

            *tabs += 1;
            white(
                false,
                format!("\n{}{}\n", " ".repeat(*tabs * 4), docs),
                buffer,
            );
            *tabs -= 1;
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buffer);
    }
}

pub(crate) fn print_fns(tabs: &mut usize, functions: &[ProvidedFunction], buffer: &mut Buffer) {
    if !functions.is_empty() {
        white(
            true,
            format!("{}FUNCTIONS:\n", " ".repeat(*tabs * 4)),
            buffer,
        );
        *tabs += 1;
        for ProvidedFunction { docs, function, .. } in functions.iter() {
            green(true, " ".repeat(*tabs * 4).to_string(), buffer);
            function.print(buffer);
            green(true, "\n".to_string(), buffer);
            *tabs += 1;
            white(
                false,
                format!("{}{}\n", " ".repeat(*tabs * 4), docs),
                buffer,
            );
            *tabs -= 1;
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buffer);
    }
}

pub type ProvidedProbes = HashMap<
    String,
    (
        ProvidedFunctionality,
        HashMap<
            String,
            (
                ProvidedFunctionality,
                HashMap<String, (ProvidedFunctionality, Vec<(ProvidedFunctionality, String)>)>,
            ),
        >,
    ),
>;

#[derive(Default)]
pub struct Whamm {
    pub provided_probes: ProvidedProbes,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    pub scripts: Vec<Script>,
}
impl Whamm {
    pub fn new() -> Self {
        Whamm {
            provided_probes: HashMap::new(),
            fns: Whamm::get_provided_fns(),
            globals: Whamm::get_provided_globals(),

            scripts: vec![],
        }
    }

    fn get_provided_fns() -> Vec<ProvidedFunction> {
        let strcmp_params = vec![
            (
                Expr::VarId {
                    definition: Definition::CompilerStatic,
                    name: "str_addr".to_string(),
                    loc: None,
                },
                DataType::Tuple {
                    ty_info: vec![Box::new(DataType::I32), Box::new(DataType::I32)],
                },
            ),
            (
                Expr::VarId {
                    definition: Definition::CompilerStatic,
                    name: "value".to_string(),
                    loc: None,
                },
                DataType::Str,
            ),
        ];

        let strcmp = ProvidedFunction::new(
            "strcmp".to_string(),
            "Compare two wasm strings and return whether they are equivalent.".to_string(),
            strcmp_params,
            DataType::Boolean,
            false,
        );

        vec![strcmp]
    }

    fn get_provided_globals() -> HashMap<String, ProvidedGlobal> {
        HashMap::new()
    }

    pub fn add_script(&mut self, mut script: Script) -> usize {
        let id = self.scripts.len();
        script.id = self.scripts.len() as u8;
        self.scripts.push(script);

        id
    }
}

/// RulePart are the probe ids in a probe rule
#[derive(Clone, Debug, Default)]
pub struct RulePart {
    pub name: String,
    pub ty_info: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub loc: Option<Location>,
}
impl RulePart {
    pub(crate) fn new(name: String, loc: Option<Location>) -> Self {
        Self {
            name,
            loc,
            ty_info: vec![],
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProbeRule {
    pub provider: Option<RulePart>,
    pub package: Option<RulePart>,
    pub event: Option<RulePart>,
    pub mode: Option<RulePart>,
}
impl ProbeRule {
    pub fn new() -> Self {
        Self {
            provider: None,
            package: None,
            event: None,
            mode: None,
        }
    }
    pub fn full_name(&self) -> String {
        let provider = if let Some(name) = &self.provider {
            name.name.clone()
        } else {
            "<none>".to_string()
        };
        let package = if let Some(name) = &self.package {
            name.name.clone()
        } else {
            "<none>".to_string()
        };
        let event = if let Some(name) = &self.event {
            name.name.clone()
        } else {
            "<none>".to_string()
        };
        let mode = if let Some(name) = &self.mode {
            name.name.clone()
        } else {
            "<none>".to_string()
        };
        format!("{provider}:{package}:{event}:{mode}")
    }
    pub fn add_rule_def(&mut self, part: RulePart) {
        if self.provider.is_none() {
            self.provider = Some(part);
            return;
        }
        if self.package.is_none() {
            self.package = Some(part);
            return;
        }
        if self.event.is_none() {
            self.event = Some(part);
            return;
        }
        if self.mode.is_none() {
            self.mode = Some(part);
        }
    }

    pub fn print_bold_provider(&self, buffer: &mut Buffer) {
        magenta(
            true,
            self.provider.as_ref().unwrap().name.to_string(),
            buffer,
        );
        if let Some(package_patt) = &self.package {
            white(true, format!(":{}", &package_patt.name), buffer);
            self.print_event(buffer);
        }
        white(true, "\n".to_string(), buffer);
        grey_italics(true, "matches the following rules:\n\n".to_string(), buffer);
    }

    pub fn print_bold_package(&self, buffer: &mut Buffer) {
        white(
            true,
            format!("{}:", self.provider.as_ref().unwrap().name),
            buffer,
        );
        magenta(
            true,
            self.package.as_ref().unwrap().name.to_string(),
            buffer,
        );
        self.print_event(buffer);
        white(true, "\n".to_string(), buffer);
        grey_italics(
            true,
            "matches the following packages:\n\n".to_string(),
            buffer,
        );
    }
    fn print_event(&self, buffer: &mut Buffer) {
        if let Some(event_patt) = &self.event {
            white(true, format!(":{}", &event_patt.name), buffer);
            if let Some(mode_patt) = &self.mode {
                white(true, format!(":{}", &mode_patt.name), buffer);
            }
        }
    }

    pub fn print_bold_event(&self, buffer: &mut Buffer) {
        white(
            true,
            format!(
                "{}:{}:",
                self.provider.as_ref().unwrap().name,
                self.package.as_ref().unwrap().name
            ),
            buffer,
        );
        magenta(true, self.event.as_ref().unwrap().name.to_string(), buffer);
        if let Some(mode_patt) = &self.mode {
            white(true, format!(":{}", &mode_patt.name), buffer);
        }
        white(true, "\n".to_string(), buffer);
        grey_italics(
            true,
            "matches the following events:\n\n".to_string(),
            buffer,
        );
    }

    pub fn print_bold_mode(&self, buffer: &mut Buffer) {
        white(
            true,
            format!(
                "    {}:{}:{}:",
                self.provider.as_ref().unwrap().name,
                self.package.as_ref().unwrap().name,
                self.event.as_ref().unwrap().name
            ),
            buffer,
        );
        magenta(
            true,
            format!("{}\n", self.mode.as_ref().unwrap().name),
            buffer,
        );
        grey_italics(
            true,
            "    matches the following modes for the parent event:\n\n".to_string(),
            buffer,
        );
    }
}

#[derive(Default)]
pub struct Script {
    pub id: u8,
    /// The rules of the probes that have been used in the Script.
    pub providers: HashMap<String, Box<dyn Provider>>,
    pub fns: Vec<Fn>,                     // User-provided
    pub globals: HashMap<String, Global>, // User-provided, should be VarId
    pub global_stmts: Vec<Statement>,

    // track the number of probes that have been added to this script
    // (for ID bookkeeping)
    pub num_probes: u32,
}
impl Script {
    pub fn new() -> Self {
        Script {
            id: u8::MAX,
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new(),
            global_stmts: vec![],
            num_probes: 0,
        }
    }

    pub fn print_info(
        &mut self,
        probe_rule: &ProbeRule,
        print_globals: bool,
        print_functions: bool,
    ) -> Result<(), Box<WhammError>> {
        let writer = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = writer.buffer();

        // Print `whamm` info
        let mut tabs = 0;
        if print_globals || print_functions {
            white(true, "\nCORE ".to_string(), &mut buffer);
            magenta(true, "`whamm`".to_string(), &mut buffer);
            white(true, " FUNCTIONALITY\n\n".to_string(), &mut buffer);

            // Print the globals
            if print_globals {
                let globals = Whamm::get_provided_globals();
                print_global_vars(&mut tabs, &globals, &mut buffer);
            }

            // Print the functions
            if print_functions {
                let functions = Whamm::get_provided_fns();
                print_fns(&mut tabs, &functions, &mut buffer);
            }
        }

        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        let mut providers: HashMap<String, Box<dyn Provider>> = HashMap::new();
        let (matched_providers, matched_packages, matched_events, _matched_modes) =
            provider_factory::<WhammProvider>(
                &mut providers,
                &mut self.num_probes,
                probe_rule,
                None,
                None,
                None,
                true,
            )?;

        // Print the matched provider information
        if matched_providers {
            probe_rule.print_bold_provider(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            print_provider_docs(
                provider,
                print_globals,
                print_functions,
                &mut tabs,
                &mut buffer,
            );
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print the matched package information
        if matched_packages {
            probe_rule.print_bold_package(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            provider.print_package_docs(print_globals, print_functions, &mut tabs, &mut buffer);
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print the matched event information
        if matched_events {
            probe_rule.print_bold_event(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            provider.print_event_and_mode_docs(
                probe_rule,
                print_globals,
                print_functions,
                &mut tabs,
                &mut buffer,
            );
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        writer
            .print(&buffer)
            .expect("Uh oh, something went wrong while printing to terminal");
        buffer
            .reset()
            .expect("Uh oh, something went wrong while printing to terminal");

        Ok(())
    }

    pub fn add_global_stmts(&mut self, global_statements: Vec<Statement>) {
        for stmt in global_statements.iter() {
            self.global_stmts.push(stmt.clone());
        }
    }

    /// Iterates over all the matched rules, packages, events, and probe mode names
    /// to add a copy of the user-defined Probe for each of them.
    pub fn add_probe(
        &mut self,
        probe_rule: &ProbeRule,
        predicate: Option<Expr>,
        body: Option<Block>,
    ) -> Result<(), Box<WhammError>> {
        let (matched_providers, matched_packages, matched_events, matched_modes): (
            bool,
            bool,
            bool,
            bool,
        ) = provider_factory::<WhammProvider>(
            &mut self.providers,
            &mut self.num_probes,
            probe_rule,
            None,
            predicate,
            body,
            false,
        )?;

        if !matched_providers {
            return if let Some(prov_patt) = &probe_rule.provider {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified provider pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find a provider matching pattern!"
                    )),
                    None,
                )))
            };
        }

        if !matched_packages {
            return if let Some(prov_patt) = &probe_rule.package {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified package pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find a package matching pattern!"
                    )),
                    None,
                )))
            };
        }

        if !matched_events {
            return if let Some(prov_patt) = &probe_rule.event {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified event pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find an event matching pattern!"
                    )),
                    None,
                )))
            };
        }

        if !matched_modes {
            return if let Some(prov_patt) = &probe_rule.mode {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified mode pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find a mode matching pattern for {}!",
                        probe_rule.full_name()
                    )),
                    None,
                )))
            };
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProvidedFunctionality {
    pub name: String,
    pub docs: String,
}
#[derive(Clone, Debug)]
pub struct ProvidedGlobal {
    pub name: String,
    pub docs: String,
    pub global: Global,
}
impl ProvidedGlobal {
    pub fn new(name: String, docs: String, ty: DataType, is_static: bool) -> Self {
        Self {
            name: name.clone(),
            docs,
            global: Global {
                def: if is_static {
                    Definition::CompilerStatic
                } else {
                    Definition::CompilerDynamic
                },
                report: false,
                ty,
                var_name: Expr::VarId {
                    definition: if is_static {
                        Definition::CompilerStatic
                    } else {
                        Definition::CompilerDynamic
                    },
                    name,
                    loc: None,
                },
                value: None,
            },
        }
    }
}
#[derive(Clone, Debug)]
pub struct ProvidedFunction {
    pub name: String,
    pub docs: String,
    pub function: Fn,
}
impl ProvidedFunction {
    pub fn new(
        name: String,
        docs: String,
        params: Vec<(Expr, DataType)>,
        return_ty: DataType,
        is_static: bool,
    ) -> Self {
        Self {
            name: name.clone(),
            docs,
            function: Fn {
                def: if is_static {
                    Definition::CompilerStatic
                } else {
                    Definition::CompilerDynamic
                },
                name: FnId { name, loc: None },
                params,
                return_ty,
                body: Block {
                    stmts: vec![],
                    return_ty: None,
                    loc: None,
                },
            },
        }
    }
}

// =====================
// ---- Expressions ----
// =====================

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnOp {
    Cast { target: DataType },
    Not,
    BitwiseNot,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BinOp {
    // Logical operators
    And,
    Or,

    // Relational operators
    EQ,
    NE,
    GE,
    GT,
    LE,
    LT,

    // Highest precedence arithmetic operators
    Add,
    Subtract,

    // Next highest precedence arithmetic operators
    Multiply,
    Divide,
    Modulo,

    // Bitwise operators
    LShift,
    RShift,
    BitAnd,
    BitOr,
    BitXor,
}

// =================
// ==== Visitor ====
// =================

// TODO add a default visit implementation
// (take a look at the behavior tree visit trait) that would be good to add to
// the AST visitor as well to make the visit ordering/conventions less annoying.
pub trait WhammVisitor<T> {
    fn visit_whamm(&mut self, whamm: &Whamm) -> T;
    fn visit_script(&mut self, script: &Script) -> T;
    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> T;
    fn visit_package(&mut self, package: &dyn Package) -> T;
    fn visit_event(&mut self, event: &dyn Event) -> T;
    fn visit_probe(&mut self, probe: &Box<dyn Probe>) -> T;
    // fn visit_predicate(&mut self, predicate: &Expr) -> T;
    fn visit_fn(&mut self, f: &Fn) -> T;
    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_stmt(&mut self, stmt: &Statement) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_unop(&mut self, unop: &UnOp) -> T;
    fn visit_binop(&mut self, binop: &BinOp) -> T;
    fn visit_datatype(&mut self, datatype: &DataType) -> T;
    fn visit_value(&mut self, val: &Value) -> T;
}

/// To support setting constant-provided global vars
pub trait WhammVisitorMut<T> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> T;
    fn visit_script(&mut self, script: &mut Script) -> T;
    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) -> T;
    fn visit_package(&mut self, package: &mut dyn Package) -> T;
    fn visit_event(&mut self, event: &mut dyn Event) -> T;
    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> T;
    fn visit_fn(&mut self, f: &mut Fn) -> T;
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &mut Block) -> T;
    fn visit_stmt(&mut self, stmt: &mut Statement) -> T;
    fn visit_expr(&mut self, expr: &mut Expr) -> T;
    fn visit_unop(&mut self, unop: &mut UnOp) -> T;
    fn visit_binop(&mut self, op: &mut BinOp) -> T;
    fn visit_datatype(&mut self, datatype: &mut DataType) -> T;
    fn visit_value(&mut self, val: &mut Value) -> T;
}

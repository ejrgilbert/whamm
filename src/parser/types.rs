#![allow(clippy::borrowed_box)]

use pest::error::LineColLocation;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use termcolor::Buffer;

use crate::common::error::ErrorGen;
use crate::common::terminal::{green, grey_italics, magenta, white, yellow};
use crate::generator::ast::StackReq;
use crate::parser::provider_handler::{
    BoundVar, Event, Package, Probe, Provider, ProviderDef, get_matches,
};
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use wasmtime::Val;
use wirm::ir::types::DataType as WirmType;

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
            LineColLocation::Pos(pos1) => pos1,
            LineColLocation::Span(.., span1) => span1,
        };

        // make sure pos0 < pos1
        let ((l0, c0), (l1, c1)) = (pos0, pos1);
        if l0 > l1 || (l0 == l1 && c0 > c1) {
            panic!(
                "loc0 comes after loc1...something's gone horribly wrong! loc0: {:?}, loc1: {:?}",
                pos0, pos1
            );
        }

        Location {
            line_col: LineColLocation::Span(*pos0, *pos1),
            path,
        }
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
        ty_info: Vec<DataType>,
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
                let res = ty_info0.len() == ty_info1.len()
                    && ty_info0
                        .iter()
                        .zip(ty_info1.iter())
                        .all(|(ty0, ty1)| ty0 == ty1);
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
        self.to_wasm_type() == vec![WirmType::I32]
    }
    fn as_i64_in_wasm(&self) -> bool {
        self.to_wasm_type() == vec![WirmType::I64]
    }
    pub fn to_wasm_type(&self) -> Vec<WirmType> {
        match self {
            DataType::U8
            | DataType::I8
            | DataType::U16
            | DataType::I16
            | DataType::I32
            | DataType::U32
            | DataType::Boolean => vec![WirmType::I32],
            DataType::F32 => vec![WirmType::F32],
            DataType::I64 | DataType::U64 => vec![WirmType::I64],
            DataType::F64 => vec![WirmType::F64],
            // the ID used to track this var in the lib
            DataType::Map { .. } => vec![WirmType::I32],
            DataType::Null => unimplemented!(),
            DataType::Str => vec![WirmType::I32, WirmType::I32],
            DataType::Tuple { .. } => unimplemented!(),
            DataType::Unknown => unimplemented!(),
            DataType::AssumeGood => unimplemented!(),
        }
    }
    pub fn from_wasm_type(ty: &WirmType) -> Self {
        match ty {
            WirmType::I8 => DataType::I8,
            WirmType::I16 => DataType::I16,
            WirmType::I32 => DataType::I32,
            WirmType::I64 => DataType::I64,
            WirmType::F32 => DataType::F32,
            WirmType::F64 => DataType::F64,
            WirmType::FuncRef
            | WirmType::FuncRefNull
            | WirmType::Cont
            | WirmType::NoCont
            | WirmType::ExternRef
            | WirmType::ExternRefNull
            | WirmType::Any
            | WirmType::AnyNull
            | WirmType::None
            | WirmType::NoneNull
            | WirmType::NoExtern
            | WirmType::NoExternNull
            | WirmType::NoFunc
            | WirmType::NoFuncNull
            | WirmType::Eq
            | WirmType::EqNull
            | WirmType::Struct
            | WirmType::StructNull
            | WirmType::Array
            | WirmType::ArrayNull
            | WirmType::I31
            | WirmType::I31Null
            | WirmType::Exn
            | WirmType::NoExn
            | WirmType::Module { .. }
            | WirmType::RecGroup(_)
            | WirmType::CoreTypeId(_)
            | WirmType::V128 => unimplemented!(),
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
                white(true, ", ".to_string(), buffer);
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
    pub fn to_default_values(&self) -> Vec<Val> {
        match self {
            DataType::Boolean |
            DataType::Map { .. } | // this uses a map_id which is an i32
            DataType::U8 |
            DataType::I8 |
            DataType::U16 |
            DataType::I16 |
            DataType::U32 |
            DataType::I32 => vec![Val::I32(0)],
            DataType::F32 => vec![Val::F32(0)],
            DataType::U64 |
            DataType::I64 => vec![Val::I64(0)],
            DataType::F64 => vec![Val::F64(0)],
            DataType::Str => vec![Val::I32(0), Val::I32(0)], // (addr, len)
            DataType::Tuple { ty_info } => {
                let mut res = vec![];
                for ty in ty_info.iter() {
                    res.extend(ty.to_default_values());
                }
                res
            }
            DataType::Null |
            DataType::Unknown |
            DataType::AssumeGood => unreachable!()
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
impl Display for NumLit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumLit::I8 { val } => write!(f, "{val}"),
            NumLit::U8 { val } => write!(f, "{val}"),
            NumLit::I16 { val } => write!(f, "{val}"),
            NumLit::U16 { val } => write!(f, "{val}"),
            NumLit::I32 { val } => write!(f, "{val}"),
            NumLit::U32 { val } => write!(f, "{val}"),
            NumLit::I64 { val } => write!(f, "{val}"),
            NumLit::U64 { val } => write!(f, "{val}"),
            NumLit::F32 { val } => write!(f, "{val}"),
            NumLit::F64 { val } => write!(f, "{val}"),
        }
    }
}
impl NumLit {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::I8 { val } => val.to_le_bytes().to_vec(),
            Self::U8 { val } => val.to_le_bytes().to_vec(),
            Self::I16 { val } => val.to_le_bytes().to_vec(),
            Self::U16 { val } => val.to_le_bytes().to_vec(),
            Self::I32 { val } => val.to_le_bytes().to_vec(),
            Self::U32 { val } => val.to_le_bytes().to_vec(),
            Self::I64 { val } => val.to_le_bytes().to_vec(),
            Self::U64 { val } => val.to_le_bytes().to_vec(),
            Self::F32 { val } => val.to_le_bytes().to_vec(),
            Self::F64 { val } => val.to_le_bytes().to_vec(),
        }
    }
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
        val: HashMap<u32, u32>,
    },
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number { val, .. } => write!(f, "{val}"),
            Value::Boolean { val } => write!(f, "{val}"),
            Value::Str { val } => write!(f, "\"{val}\""),
            Value::Tuple { vals, .. } => {
                let mut vals_str = "".to_string();
                for val in vals.iter() {
                    vals_str = format!("{vals_str}, {val}");
                }
                write!(f, "({vals_str})")
            }
            Value::U32U32Map { .. } => write!(f, "U32U32Map {{..}}"),
        }
    }
}
impl Value {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Self::Number { val, .. } => val.encode(),
            Self::Boolean { .. }
            | Self::Str { .. }
            | Self::Tuple { .. }
            | Self::U32U32Map { .. } => {
                todo!()
            }
        }
    }
    pub fn gen_empty_tuple() -> Self {
        Self::Tuple {
            ty: DataType::Tuple { ty_info: vec![] },
            vals: Vec::new(),
        }
    }
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
                    if !success { Err(msg) } else { Ok(()) }
                } else {
                    Err(format!("{ty} to {target}"))
                }
            }
            _ => Err("non-numeric values".to_string()),
        }
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
impl From<&Val> for Value {
    fn from(val: &Val) -> Self {
        match val {
            Val::I32(val) => Self::gen_i32(*val),
            Val::I64(val) => Self::gen_i64(*val),
            Val::F32(val) => Self::gen_f32(f32::from_bits(*val)),
            Val::F64(val) => Self::gen_f64(f64::from_bits(*val)),
            Val::V128(_)
            | Val::FuncRef(_)
            | Val::ExternRef(_)
            | Val::AnyRef(_)
            | Val::ExnRef(_)
            | Val::ContRef(_) => todo!(),
        }
    }
}
pub(crate) fn whamm_value_to_wasm_val(v: &Value) -> Option<Val> {
    match v {
        Value::Number { val, .. } => match val {
            NumLit::I8 { val } => Some(Val::I32(*val as i32)),
            NumLit::U8 { val } => Some(Val::I32(*val as i32)),
            NumLit::I16 { val } => Some(Val::I32(*val as i32)),
            NumLit::U16 { val } => Some(Val::I32(*val as i32)),
            NumLit::I32 { val } => Some(Val::I32(*val)),
            NumLit::U32 { val } => Some(Val::I32(*val as i32)),
            NumLit::I64 { val } => Some(Val::I64(*val)),
            NumLit::U64 { val } => Some(Val::I64(*val as i64)),
            NumLit::F32 { val } => Some(Val::F32(val.to_bits())),
            NumLit::F64 { val } => Some(Val::F64(val.to_bits())),
        },
        Value::Boolean { val } => {
            if *val {
                Some(Val::I32(1))
            } else {
                Some(Val::I32(0))
            }
        }
        Value::Str { .. } | Value::Tuple { .. } | Value::U32U32Map { .. } => None,
    }
}

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub results: Option<DataType>,
    pub loc: Option<Location>,
}
impl Block {
    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty()
    }
    pub fn loc(&self) -> &Option<Location> {
        &self.loc
    }
    pub fn extend(&mut self, other: Block) {
        self.stmts.extend(other.stmts);
        assert_eq!(self.results, other.results);
        if self.loc.is_none() {
            self.loc = other.loc.clone();
        }
    }
}
impl From<&Statement> for Block {
    fn from(stmt: &Statement) -> Self {
        Self {
            stmts: vec![stmt.clone()],
            ..Default::default()
        }
    }
}
impl From<Vec<Statement>> for Block {
    fn from(stmts: Vec<Statement>) -> Self {
        Self {
            stmts,
            ..Default::default()
        }
    }
}

// Statements
#[derive(Clone, Debug)]
pub enum Statement {
    LibImport {
        lib_name: String,
        loc: Option<Location>,
    },
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
    // an unshared variable that has a special initialization
    UnsharedDeclInit {
        decl: Box<Statement>,
        init: Box<Statement>,
        loc: Option<Location>,
    },
}
impl Statement {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Statement::LibImport { loc, .. }
            | Statement::Decl { loc, .. }
            | Statement::If { loc, .. }
            | Statement::Return { loc, .. }
            | Statement::Assign { loc, .. }
            | Statement::SetMap { loc, .. }
            | Statement::UnsharedDecl { loc, .. }
            | Statement::UnsharedDeclInit { loc, .. }
            | Statement::Expr { loc, .. } => loc,
        }
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        self.loc().as_ref().map(|loc| loc.line_col.clone())
    }
}

#[derive(Clone, Debug)]
pub enum Annotation {
    Static,
}
impl Annotation {
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static)
    }
}
impl TryFrom<&str> for Annotation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "static" => Ok(Self::Static),
            _ => Err(format!("`@{}` is not a valid annotation", value)),
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
    LibCall {
        annotation: Option<Annotation>,
        lib_name: String,
        call: Box<Expr>,           // should be Expr::Call
        results: Option<DataType>, // set by the type checker!
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
    pub fn empty_tuple(loc: &Option<Location>) -> Self {
        Expr::Primitive {
            val: Value::gen_empty_tuple(),
            loc: loc.clone(),
        }
    }
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
    pub fn implicit_cast(&mut self, target: &DataType) -> Result<(), String> {
        match self.internal_implicit_cast(target) {
            Err(msg) => Err(format!(
                "CastError: Cannot implicitly cast {msg}. Please add an explicit cast."
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
            | Expr::LibCall { loc, .. }
            | Expr::Call { loc, .. }
            | Expr::VarId { loc, .. }
            | Expr::MapGet { loc, .. }
            | Expr::Primitive { loc, .. } => loc,
        }
    }
}
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::UnOp { op, expr, .. } => match op {
                UnOp::Cast { target } => write!(f, "{} as {}", expr, target),
                UnOp::Not => write!(f, "!{}", expr),
                UnOp::BitwiseNot => write!(f, "~{}", expr),
            },
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                write!(f, "{} ? {} : {}", cond, conseq, alt)
            }
            Expr::BinOp { lhs, op, rhs, .. } => {
                write!(f, "{} {} {}", lhs, op, rhs)
            }
            Expr::Call {
                fn_target, args, ..
            } => {
                let mut args_str = "".to_string();
                for arg in args.iter() {
                    args_str = format!("{args_str}. {arg}");
                }
                write!(f, "{fn_target}({args_str})")
            }
            Expr::LibCall { lib_name, call, .. } => {
                write!(f, "{lib_name}.{}", call)
            }
            Expr::VarId { name, .. } => write!(f, "{name}"),
            Expr::Primitive { val, .. } => write!(f, "{val}"),
            Expr::MapGet { map, key, .. } => write!(f, "{map}.{}", key),
        }
    }
}
pub(crate) fn expr_to_val(expr: &Expr) -> Option<Val> {
    match expr {
        Expr::Primitive { val, .. } => whamm_value_to_wasm_val(val),
        _ => None,
    }
}

// Functions

// TODO -- get rid of FnId?
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
    pub(crate) results: DataType,
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
        self.results.print(buffer);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Definition {
    User,
    CompilerStatic,
    CompilerDynamic,
    CompilerDerived, // TODO -- can I remove this variant?
}
impl Definition {
    pub fn is_comp_defined(&self) -> bool {
        matches!(self, Definition::CompilerStatic)
            || matches!(self, Definition::CompilerDynamic)
            || matches!(self, Definition::CompilerDerived)
    }
}
impl From<&str> for Definition {
    fn from(value: &str) -> Self {
        match value {
            "user" => Self::User,
            "static" => Self::CompilerStatic,
            "dynamic" => Self::CompilerDynamic,
            "derived" => Self::CompilerDerived,
            _ => panic!("Invalid definition string: {value}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Global {
    pub def: Definition,
    pub report: bool,
    pub ty: DataType,
    pub value: Option<Value>,
}
impl Global {
    pub fn is_from_user(&self) -> bool {
        matches!(self.def, Definition::User)
    }
}

pub(crate) fn print_bound_vars(tabs: &mut usize, vars: &[BoundVar], buffer: &mut Buffer) {
    if !vars.is_empty() {
        white(
            true,
            format!("{}VARIABLES:\n", " ".repeat(*tabs * 4)),
            buffer,
        );
        *tabs += 1;
        for var in vars.iter() {
            var.print_info(buffer, tabs);
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buffer);
    }
}

pub(crate) fn print_fns(tabs: &mut usize, functions: &[BoundFunction], buffer: &mut Buffer) {
    if !functions.is_empty() {
        white(
            true,
            format!("{}FUNCTIONS:\n", " ".repeat(*tabs * 4)),
            buffer,
        );
        *tabs += 1;
        for BoundFunction { docs, function, .. } in functions.iter() {
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

#[derive(Default)]
pub struct Whamm {
    pub fns: Vec<BoundFunction>,   // Comp-provided
    pub bound_vars: Vec<BoundVar>, // Comp-provided

    pub scripts: Vec<Script>,
}
impl Whamm {
    pub fn new() -> Self {
        Whamm {
            fns: Whamm::get_bound_fns(),
            bound_vars: Whamm::get_bound_vars(),

            scripts: vec![],
        }
    }

    pub(crate) fn get_bound_fns() -> Vec<BoundFunction> {
        let strcmp_params = vec![
            (
                Expr::VarId {
                    definition: Definition::CompilerStatic,
                    name: "str_addr".to_string(),
                    loc: None,
                },
                DataType::Tuple {
                    ty_info: vec![DataType::I32, DataType::I32],
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

        let strcmp = BoundFunction::new(
            "strcmp".to_string(),
            "Compare two wasm strings and return whether they are equivalent.".to_string(),
            strcmp_params,
            DataType::Boolean,
            false,
            StackReq::None,
        );

        vec![strcmp]
    }

    pub(crate) fn get_bound_vars() -> Vec<BoundVar> {
        vec![]
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
impl From<&crate::generator::ast::ProbeRule> for ProbeRule {
    fn from(value: &crate::generator::ast::ProbeRule) -> Self {
        Self {
            provider: Some(value.provider.clone()),
            package: Some(value.package.clone()),
            event: Some(value.event.clone()),
            mode: Some(value.mode.clone()),
        }
    }
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
        if self.provider.is_none() {
            return;
        }
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
        if self.provider.is_none() || self.package.is_none() {
            return;
        }
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
        if self.provider.is_none() || self.package.is_none() || self.event.is_none() {
            return;
        }
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
        if self.provider.is_none()
            || self.package.is_none()
            || self.event.is_none() | self.mode.is_none()
        {
            return;
        }
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
    pub providers: HashMap<String, Provider>,
    pub fns: Vec<Fn>,                     // User-provided
    pub globals: HashMap<String, Global>, // User-provided, should be VarId
    pub global_stmts: Vec<Statement>,
}
impl Script {
    pub fn new() -> Self {
        Script {
            id: u8::MAX,
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new(),
            global_stmts: vec![],
        }
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
        def: &[ProviderDef],
        predicate: Option<Expr>,
        body: Option<Block>,
        err: &mut ErrorGen,
    ) {
        let matches = get_matches(probe_rule, def, err);
        if matches.is_empty() {
            assert!(err.has_errors);
        }

        // create the location for the entire probe
        let loc_start = get_loc_with_priority(
            &probe_rule.provider,
            &probe_rule.package,
            &probe_rule.event,
            &probe_rule.mode,
            "start",
        );

        let loc_end = if let Some(body) = &body {
            if let Some(loc) = body.loc.as_ref() {
                loc.clone()
            } else if let Some(predicate) = &predicate {
                if let Some(loc) = predicate.loc().as_ref() {
                    loc.clone()
                } else {
                    get_loc_with_priority(
                        &probe_rule.mode,
                        &probe_rule.event,
                        &probe_rule.package,
                        &probe_rule.provider,
                        "start",
                    )
                }
            } else {
                get_loc_with_priority(
                    &probe_rule.mode,
                    &probe_rule.event,
                    &probe_rule.package,
                    &probe_rule.provider,
                    "start",
                )
            }
        } else {
            get_loc_with_priority(
                &probe_rule.mode,
                &probe_rule.event,
                &probe_rule.package,
                &probe_rule.provider,
                "start",
            )
        };

        let loc = Location::from(
            &loc_start.line_col,
            &loc_end.line_col,
            loc_start.path.clone(),
        );

        for prov_match in matches.iter() {
            let provider = self
                .providers
                .entry(prov_match.def.name.clone())
                .or_insert(Provider::new(prov_match.def.clone(), probe_rule));

            provider.add_probes(
                loc.clone(),
                &prov_match.packages,
                probe_rule,
                predicate.clone(),
                body.clone(),
            );
        }
    }
}

fn get_loc_with_priority(
    p0: &Option<RulePart>,
    p1: &Option<RulePart>,
    p2: &Option<RulePart>,
    p3: &Option<RulePart>,
    err: &str,
) -> Location {
    if let Some(p0) = get_loc(p0) {
        p0
    } else if let Some(p1) = get_loc(p1) {
        p1
    } else if let Some(p2) = get_loc(p2) {
        p2
    } else if let Some(p3) = get_loc(p3) {
        p3
    } else {
        panic!("Could not find a {err} for the probe's location!")
    }
}

fn get_loc(rule_part: &Option<RulePart>) -> Option<Location> {
    if let Some(part) = &rule_part {
        if let Some(loc) = part.loc.as_ref() {
            return Some(loc.clone());
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct BoundFunction {
    pub docs: String,
    pub function: Fn,
    pub req_args: StackReq,
}
impl BoundFunction {
    pub fn new(
        name: String,
        docs: String,
        params: Vec<(Expr, DataType)>,
        return_ty: DataType,
        is_static: bool,
        req_args: StackReq,
    ) -> Self {
        Self {
            docs,
            function: Fn {
                def: if is_static {
                    Definition::CompilerStatic
                } else {
                    Definition::CompilerDynamic
                },
                name: FnId { name, loc: None },
                params,
                results: return_ty,
                body: Block {
                    stmts: vec![],
                    results: None,
                    loc: None,
                },
            },
            req_args,
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
impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
            BinOp::EQ => write!(f, "=="),
            BinOp::NE => write!(f, "!="),
            BinOp::GE => write!(f, ">="),
            BinOp::GT => write!(f, ">"),
            BinOp::LE => write!(f, "<="),
            BinOp::LT => write!(f, "<"),
            BinOp::Add => write!(f, "+"),
            BinOp::Subtract => write!(f, "-"),
            BinOp::Multiply => write!(f, "*"),
            BinOp::Divide => write!(f, "/"),
            BinOp::Modulo => write!(f, "%"),
            BinOp::LShift => write!(f, "<<"),
            BinOp::RShift => write!(f, ">>"),
            BinOp::BitAnd => write!(f, "&"),
            BinOp::BitOr => write!(f, "|"),
            BinOp::BitXor => write!(f, "^"),
        }
    }
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
    fn visit_provider(&mut self, provider: &Provider) -> T;
    fn visit_package(&mut self, package: &Package) -> T;
    fn visit_event(&mut self, event: &Event) -> T;
    fn visit_probe(&mut self, probe: &Probe) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
}

/// To support setting constant bound vars
pub trait WhammVisitorMut<T> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> T;
    fn visit_script(&mut self, script: &mut Script) -> T;
    fn visit_provider(&mut self, provider: &mut Provider) -> T;
    fn visit_package(&mut self, package: &mut Package) -> T;
    fn visit_event(&mut self, event: &mut Event) -> T;
    fn visit_probe(&mut self, probe: &mut Probe) -> T;
    fn visit_fn(&mut self, f: &mut Fn) -> T;
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &mut Block) -> T;
    fn visit_stmt(&mut self, stmt: &mut Statement) -> T;
    fn visit_stmt_global(&mut self, stmt: &mut Statement) -> T;
    fn visit_expr(&mut self, expr: &mut Expr) -> T;
    fn visit_value(&mut self, val: &mut Value) -> T;
}

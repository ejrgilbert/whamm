use glob::Pattern;
use pest::error::LineColLocation;
use std::collections::HashMap;
use termcolor::{Buffer, ColorChoice, WriteColor};

use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{
    green, grey_italics, long_line, magenta, magenta_italics, white, yellow,
};
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use termcolor::BufferWriter;
use walrus::DataId;

#[derive(Parser)]
#[grammar = "./parser/whamm.pest"] // Path relative to base `src` dir
pub struct WhammParser;

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(and, Left) | Op::infix(or, Left)) // LOGOP
            .op(Op::infix(eq, Left)                         // RELOP
                | Op::infix(ne, Left)
                | Op::infix(ge, Left)
                | Op::infix(gt, Left)
                | Op::infix(le, Left)
                | Op::infix(lt, Left)
            ).op(Op::infix(add, Left) | Op::infix(subtract, Left)) // SUMOP
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left)) // MULOP
            .op(Op::prefix(neg))
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DataType {
    I32,
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
}
impl DataType {
    pub fn print(&self, buffer: &mut Buffer) {
        match self {
            DataType::I32 => {
                yellow(true, "int".to_string(), buffer);
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
        }
    }
}

// Values
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    Integer {
        ty: DataType,
        val: i32,
    },
    Str {
        ty: DataType,
        val: String,

        // Used by emitter to store this string's address/len in Wasm memory
        // DataId: Walrus ID to reference data segment
        // u32: address of data in memory
        // usize:  the length of the string in memory
        addr: Option<(DataId, u32, usize)>,
    },
    Tuple {
        ty: DataType,
        vals: Vec<Expr>,
    },
    Boolean {
        ty: DataType,
        val: bool,
    },
}
#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub loc: Option<Location>,
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

    Expr {
        expr: Expr,
        loc: Option<Location>,
    },
    Return {
        expr: Expr,
        loc: Option<Location>,
    },
}
impl Statement {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Statement::Decl { loc, .. }
            | Statement::Return { loc, .. }
            | Statement::Assign { loc, .. }
            | Statement::Expr { loc, .. } => loc,
        }
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        self.loc().as_ref().map(|loc| loc.line_col.clone())
    }
    pub fn dummy() -> Self {
        Self::Expr {
            expr: Expr::Primitive {
                val: Value::Integer {
                    ty: DataType::I32,
                    val: 0,
                },
                loc: None,
            },
            loc: None,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    UnOp {
        // Type is based on the outermost `op`
        op: UnOp,
        expr: Box<Expr>,
        loc: Option<Location>,
    },
    Ternary {
        cond: Box<Expr>,
        conseq: Box<Expr>,
        alt: Box<Expr>,
        loc: Option<Location>,
    },
    BinOp {
        // Type is based on the outermost `op` (if arithmetic op, also based on types of lhs/rhs due to doubles)
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
        loc: Option<Location>,
    },
    Call {
        // Type is fn_target.return_ty, should be VarId
        fn_target: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        loc: Option<Location>,
    },
    VarId {
        is_comp_provided: bool, // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!
        name: String,
        loc: Option<Location>,
    },
    Primitive {
        // Type is val.ty
        val: Value,
        loc: Option<Location>,
    },
}
impl Expr {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Expr::UnOp { loc, .. }
            | Expr::Ternary { loc, .. }
            | Expr::BinOp { loc, .. }
            | Expr::Call { loc, .. }
            | Expr::VarId { loc, .. }
            | Expr::Primitive { loc, .. } => loc,
        }
    }
}

// Functions

#[derive(Clone, Debug)]
pub struct FnId {
    pub name: String,
    pub loc: Option<Location>,
}

#[derive(Clone, Debug)]
pub struct Fn {
    pub(crate) is_comp_provided: bool,
    pub(crate) name: FnId,
    pub(crate) params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) return_ty: Option<DataType>,
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

        if let Some(return_ty) = &self.return_ty {
            white(true, " -> ".to_string(), buffer);
            return_ty.print(buffer);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Global {
    pub is_comp_provided: bool,

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
}

fn print_global_vars(
    tabs: &mut usize,
    globals: &HashMap<String, (ProvidedFunctionality, Global)>,
    buffer: &mut Buffer,
) {
    if !globals.is_empty() {
        white(true, format!("{}GLOBALS:\n", " ".repeat(*tabs * 4)), buffer);
        *tabs += 1;
        for (.., (info, global)) in globals.iter() {
            white(false, " ".repeat(*tabs * 4).to_string(), buffer);
            global.print(buffer);

            *tabs += 1;
            white(
                false,
                format!("\n{}{}\n", " ".repeat(*tabs * 4), info.docs),
                buffer,
            );
            *tabs -= 1;
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buffer);
    }
}

fn print_fns(tabs: &mut usize, functions: &[(ProvidedFunctionality, Fn)], buffer: &mut Buffer) {
    if !functions.is_empty() {
        white(
            true,
            format!("{}FUNCTIONS:\n", " ".repeat(*tabs * 4)),
            buffer,
        );
        *tabs += 1;
        for (info, f) in functions.iter() {
            green(true, " ".repeat(*tabs * 4).to_string(), buffer);
            f.print(buffer);
            green(true, "\n".to_string(), buffer);
            *tabs += 1;
            white(
                false,
                format!("{}{}\n", " ".repeat(*tabs * 4), info.docs),
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

pub struct Whamm {
    pub provided_probes: ProvidedProbes,
    pub fns: Vec<(ProvidedFunctionality, Fn)>, // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    pub scripts: Vec<Script>,
}
impl Default for Whamm {
    fn default() -> Self {
        Self::new()
    }
}
impl Whamm {
    pub fn new() -> Self {
        let mut whamm = Whamm {
            provided_probes: HashMap::new(),
            fns: Whamm::get_provided_fns(),
            globals: Whamm::get_provided_globals(),

            scripts: vec![],
        };
        whamm.init_provided_probes();
        whamm
    }

    fn get_provided_fns() -> Vec<(ProvidedFunctionality, Fn)> {
        let params = vec![
            (
                Expr::VarId {
                    is_comp_provided: true,
                    name: "str_addr".to_string(),
                    loc: None,
                },
                DataType::Tuple {
                    ty_info: (vec![Box::new(DataType::I32), Box::new(DataType::I32)]),
                },
            ),
            (
                Expr::VarId {
                    is_comp_provided: true,
                    name: "value".to_string(),
                    loc: None,
                },
                DataType::Str,
            ),
        ];
        let strcmp_fn = Fn {
            is_comp_provided: true,
            name: FnId {
                name: "strcmp".to_string(),
                loc: None,
            },
            params,
            return_ty: Some(DataType::Boolean),
            body: Block {
                stmts: vec![],
                loc: None,
            },
        };
        let docs = ProvidedFunctionality {
            name: "strcmp".to_string(),
            docs: "Compare two wasm strings and return whether they are equivalent.".to_string(),
        };

        vec![(docs, strcmp_fn)]
    }

    fn get_provided_globals() -> HashMap<String, (ProvidedFunctionality, Global)> {
        HashMap::new()
    }

    fn init_provided_probes(&mut self) {
        // A giant data structure to encode the available `providers->packages->events->probe_types`
        self.init_core_probes();
        self.init_wasm_probes();
    }

    fn init_core_probes(&mut self) {
        // Not really any packages or events for a whamm core probe...just two types!
        self.provided_probes.insert(
            "whamm".to_string(),
            (
                ProvidedFunctionality {
                    name: "whamm".to_string(),
                    docs: "Provides the core probe definitions of `whamm`.".to_string(),
                },
                HashMap::from([(
                    "".to_string(),
                    (
                        ProvidedFunctionality {
                            name: "".to_string(),
                            docs: "".to_string(),
                        },
                        HashMap::from([(
                            "".to_string(),
                            (
                                ProvidedFunctionality {
                                    name: "".to_string(),
                                    docs: "".to_string(),
                                },
                                vec![
                                    (
                                        ProvidedFunctionality {
                                            name: "begin".to_string(),
                                            docs: "Run this logic on application startup."
                                                .to_string(),
                                        },
                                        "begin".to_string(),
                                    ),
                                    (
                                        ProvidedFunctionality {
                                            name: "end".to_string(),
                                            docs: "Run this logic when the application exits."
                                                .to_string(),
                                        },
                                        "end".to_string(),
                                    ),
                                ],
                            ),
                        )]),
                    ),
                )]),
            ),
        );
        self.provided_probes.insert(
            "end".to_string(),
            (
                ProvidedFunctionality {
                    name: "end".to_string(),
                    docs: "Run this logic when the application exits.".to_string(),
                },
                HashMap::new(),
            ),
        );
    }

    fn init_wasm_probes(&mut self) {
        // This list of events matches up with bytecodes supported by Walrus.
        // See: https://docs.rs/walrus/latest/walrus/ir/
        let wasm_bytecode_events = vec![
            (
                ProvidedFunctionality {
                    name: "block".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/block".to_string(),
                },
                "block".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "loop".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/loop".to_string(),
                },
                "loop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "call".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                },
                "call".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "call_indirect".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                },
                "call_indirect".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "local_get".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_get".to_string(),
                },
                "local_get".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "local_set".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_set".to_string(),
                },
                "local_set".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "local_tee".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_tee".to_string(),
                },
                "local_tee".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "global_get".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_get".to_string(),
                },
                "global_get".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "global_set".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_set".to_string(),
                },
                "global_set".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "const".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const".to_string(),
                },
                "const".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "binop".to_string(),
                    docs: "Consume two operands and produce one result of the respective type. \
                    The types of binary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric".to_string(),
                },
                "binop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "unop".to_string(),
                    docs: "Consume one operand and produce one result of the respective type. \
                    The types of unary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric".to_string(),
                },
                "unop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "select".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Select".to_string(),
                },
                "select".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "unreachable".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/unreachable".to_string(),
                },
                "unreachable".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "br".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                },
                "br".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "br_if".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                },
                "br_if".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "if_else".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else".to_string(),
                },
                "if_else".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "br_table".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                },
                "br_table".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "drop".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Drop".to_string(),
                },
                "drop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "return".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/return".to_string(),
                },
                "return".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "memory_size".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Size".to_string(),
                },
                "memory_size".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "memory_grow".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Grow".to_string(),
                },
                "memory_grow".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "memory_init".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                },
                "memory_init".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "data_drop".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                },
                "data_drop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "memory_copy".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy".to_string(),
                },
                "memory_copy".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "memory_fill".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Fill".to_string(),
                },
                "memory_fill".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "load".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load".to_string(),
                },
                "load".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "store".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store".to_string(),
                },
                "store".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "atomic_rmw".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#read-modify-write".to_string(),
                },
                "atomic_rmw".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "cmpxchg".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#compare-exchange".to_string(),
                },
                "cmpxchg".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "atomic_notify".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                },
                "atomic_notify".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "atomic_wait".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                },
                "atomic_wait".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "atomic_fence".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#fence-operator".to_string(),
                },
                "atomic_fence".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "table_get".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "table_get".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "table_set".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "table_set".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "table_grow".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "table_grow".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "table_size".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "table_size".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "table_fill".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "table_fill".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "ref_null".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                },
                "ref_null".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "ref_is_null".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                },
                "ref_is_null".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "ref_func".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                },
                "ref_func".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "v128_bitselect".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "v128_bitselect".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "i8x16_swizzle".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "i8x16_swizzle".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "i8x16_shuffle".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "i8x16_shuffle".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "load_simd".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "load_simd".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "table_init".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "table_init".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "elem_drop".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "elem_drop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "table_copy".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "table_copy".to_string()
            ),
        ];
        let wasm_bytecode_probe_modes =
            vec![
            (
                ProvidedFunctionality {
                    name: "before".to_string(),
                    docs: "This mode will cause the instrumentation logic to run *before* the \
                    probed event (if the predicate evaluates to `true`).".to_string(),
                },
                "before".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "after".to_string(),
                    docs: "This mode will cause the instrumentation logic to run *after* the \
                    probed event (if the predicate evaluates to `true`).".to_string(),
                },
                "after".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "alt".to_string(),
                    docs: "This mode will cause the instrumentation logic to run *instead of* the \
                    probed event (if the predicate evaluates to `true`).".to_string(),
                },
                "alt".to_string()
            )
        ];
        let mut wasm_bytecode_map = HashMap::new();

        // Build out the wasm_bytecode_map
        for (info, name) in wasm_bytecode_events {
            wasm_bytecode_map.insert(name, (info.clone(), wasm_bytecode_probe_modes.clone()));
        }

        self.provided_probes.insert(
            "wasm".to_string(),
            (
                ProvidedFunctionality {
                    name: "wasm".to_string(),
                    docs: "This provides various events to instrument that are specific \
                to WebAssembly."
                        .to_string(),
                },
                HashMap::from([(
                    "bytecode".to_string(),
                    (
                        ProvidedFunctionality {
                            name: "bytecode".to_string(),
                            docs: "This package within the wasm provider contains enables the \
                    instrumentation of WebAssembly bytecode instructions."
                                .to_string(),
                        },
                        wasm_bytecode_map,
                    ),
                )]),
            ),
        );
    }
    pub fn add_script(&mut self, mut script: Script) -> usize {
        let id = self.scripts.len();
        script.name = format!("script{}", id);
        self.scripts.push(script);

        id
    }
}

pub struct SpecPart {
    pub name: String,
    pub loc: Option<Location>,
}

pub struct ProbeSpec {
    pub provider: Option<SpecPart>,
    pub package: Option<SpecPart>,
    pub event: Option<SpecPart>,
    pub mode: Option<SpecPart>,
}
impl Default for ProbeSpec {
    fn default() -> Self {
        Self::new()
    }
}
impl ProbeSpec {
    pub fn new() -> Self {
        Self {
            provider: None,
            package: None,
            event: None,
            mode: None,
        }
    }
    pub fn add_spec_def(&mut self, part: SpecPart) {
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
}

pub struct Script {
    pub name: String,
    /// The providers of the probes that have been used in the Script.
    pub providers: HashMap<String, Provider>,
    pub fns: Vec<Fn>,                     // User-provided
    pub globals: HashMap<String, Global>, // User-provided, should be VarId
    pub global_stmts: Vec<Statement>,
}
impl Default for Script {
    fn default() -> Self {
        Self::new()
    }
}
impl Script {
    pub fn new() -> Self {
        Script {
            name: "".to_string(),
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new(),
            global_stmts: vec![],
        }
    }

    fn get_provider_info(
        provided_probes: &ProvidedProbes,
        probe_spec: &ProbeSpec,
    ) -> Result<Vec<(ProvidedFunctionality, String)>, Box<WhammError>> {
        let (prov_matches, prov_loc) = if let Some(prov_patt) = &probe_spec.provider {
            (
                Provider::get_matches(provided_probes, &prov_patt.name),
                prov_patt.loc.clone(),
            )
        } else {
            (vec![], None)
        };

        if prov_matches.is_empty() {
            let loc = prov_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the provider pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }

        Ok(prov_matches)
    }

    #[allow(clippy::type_complexity)]
    fn get_package_info(
        provided_probes: &ProvidedProbes,
        provider_matches: &[(ProvidedFunctionality, String)],
        probe_spec: &ProbeSpec,
    ) -> Result<HashMap<String, Vec<(ProvidedFunctionality, String)>>, Box<WhammError>> {
        let (package_matches, package_loc) = if let Some(package_patt) = &probe_spec.package {
            let mut matches = HashMap::new();
            for (.., provider) in provider_matches.iter() {
                let next = Package::get_matches(provided_probes, provider, &package_patt.name);
                matches.insert(provider.clone(), next);
            }

            (matches, package_patt.loc.clone())
        } else {
            (HashMap::new(), None)
        };

        if package_matches.is_empty() {
            let loc = package_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the package pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        Ok(package_matches)
    }

    #[allow(clippy::type_complexity)]
    fn get_event_info(
        provided_probes: &ProvidedProbes,
        package_matches: &HashMap<String, Vec<(ProvidedFunctionality, String)>>,
        probe_spec: &ProbeSpec,
    ) -> Result<
        HashMap<String, HashMap<String, Vec<(ProvidedFunctionality, String)>>>,
        Box<WhammError>,
    > {
        let (event_matches, event_loc) = if let Some(event_patt) = &probe_spec.event {
            let mut event_matches = HashMap::new();
            for (provider_name, packages) in package_matches.iter() {
                let mut package = HashMap::new();
                for (.., package_name) in packages.iter() {
                    let next = Event::get_matches(
                        provided_probes,
                        provider_name,
                        package_name,
                        &event_patt.name,
                    );
                    package.insert(package_name.clone(), next);
                }
                event_matches.insert(provider_name.clone(), package);
            }

            (event_matches, event_patt.loc.clone())
        } else {
            (HashMap::new(), None)
        };

        if package_matches.is_empty() {
            let loc = event_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the event pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        Ok(event_matches)
    }

    #[allow(clippy::type_complexity)]
    fn get_mode_info(
        provided_probes: &ProvidedProbes,
        matches: &HashMap<String, HashMap<String, Vec<(ProvidedFunctionality, String)>>>,
        probe_spec: &ProbeSpec,
    ) -> Result<
        HashMap<String, HashMap<String, HashMap<String, Vec<(ProvidedFunctionality, String)>>>>,
        Box<WhammError>,
    > {
        let (mode_matches, mode_loc) = if let Some(mode_patt) = &probe_spec.mode {
            let mut mode_matches = HashMap::new();
            for (provider_name, package_matches) in matches.iter() {
                let mut package = HashMap::new();
                for (package_name, event_matches) in package_matches.iter() {
                    let mut modes = HashMap::new();
                    for (.., event_name) in event_matches.iter() {
                        let next = Probe::get_matches(
                            provided_probes,
                            provider_name,
                            package_name,
                            event_name,
                            &mode_patt.name,
                        );
                        modes.insert(package_name.clone(), next);
                    }
                    package.insert(package_name.clone(), modes);
                }
                mode_matches.insert(provider_name.clone(), package);
            }

            (mode_matches, mode_patt.loc.clone())
        } else {
            (HashMap::new(), None)
        };

        if mode_matches.is_empty() {
            let loc = mode_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the mode pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        Ok(mode_matches)
    }

    pub fn print_info(
        &mut self,
        provided_probes: &ProvidedProbes,
        probe_spec: &ProbeSpec,
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

        let prov_info = if probe_spec.provider.is_some() {
            Self::get_provider_info(provided_probes, probe_spec)?
        } else {
            vec![]
        };
        let pkg_info = if probe_spec.package.is_some() {
            Self::get_package_info(provided_probes, &prov_info, probe_spec)?
        } else {
            HashMap::new()
        };
        let event_info = if probe_spec.event.is_some() {
            Self::get_event_info(provided_probes, &pkg_info, probe_spec)?
        } else {
            HashMap::new()
        };
        let mode_info = if probe_spec.mode.is_some() {
            Self::get_mode_info(provided_probes, &event_info, probe_spec)?
        } else {
            HashMap::new()
        };

        // Print matched provider introduction
        if !prov_info.is_empty() {
            magenta(
                true,
                probe_spec.provider.as_ref().unwrap().name.to_string(),
                &mut buffer,
            );
            if let Some(package_patt) = &probe_spec.package {
                white(true, format!(":{}", &package_patt.name), &mut buffer);
                if let Some(event_patt) = &probe_spec.event {
                    white(true, format!(":{}", &event_patt.name), &mut buffer);
                    if let Some(mode_patt) = &probe_spec.mode {
                        white(true, format!(":{}", &mode_patt.name), &mut buffer);
                    }
                }
            }
            white(true, "\n".to_string(), &mut buffer);
            grey_italics(
                true,
                "matches the following providers:\n\n".to_string(),
                &mut buffer,
            );
        }

        // Print the matched provider information
        for (provider_info, provider_str) in prov_info.iter() {
            if provider_str.is_empty() {
                continue;
            }
            magenta_italics(true, provider_str.clone(), &mut buffer);
            white(true, " provider\n".to_string(), &mut buffer);

            // Print the provider description
            tabs += 1;
            white(
                false,
                format!("{}{}\n\n", " ".repeat(tabs * 4), provider_info.docs),
                &mut buffer,
            );

            // Print the globals
            if print_globals {
                let globals = Provider::get_provided_globals(provider_str);
                print_global_vars(&mut tabs, &globals, &mut buffer);
            }

            // Print the functions
            if print_functions {
                let functions = Provider::get_provided_fns(provider_str);
                print_fns(&mut tabs, &functions, &mut buffer);
            }
            tabs -= 1;
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print matched package introduction
        if !pkg_info.is_empty() {
            white(
                true,
                format!("{}:", &probe_spec.provider.as_ref().unwrap().name),
                &mut buffer,
            );
            magenta(
                true,
                probe_spec.package.as_ref().unwrap().name.to_string(),
                &mut buffer,
            );
            if let Some(event_patt) = &probe_spec.event {
                white(true, format!(":{}", &event_patt.name), &mut buffer);
                if let Some(mode_patt) = &probe_spec.mode {
                    white(true, format!(":{}", &mode_patt.name), &mut buffer);
                }
            }
            white(true, "\n".to_string(), &mut buffer);
            grey_italics(
                true,
                "matches the following packages:\n\n".to_string(),
                &mut buffer,
            );
        }

        // Print the matched package information
        let mut tabs = 0;
        for (_prov_str, package_list) in pkg_info.iter() {
            for (package_info, package_str) in package_list {
                if package_str.is_empty() {
                    continue;
                }
                magenta_italics(true, package_str.clone(), &mut buffer);
                white(true, " package\n".to_string(), &mut buffer);

                // Print the package description
                tabs += 1;
                white(
                    false,
                    format!("{}{}\n\n", " ".repeat(tabs * 4), package_info.docs),
                    &mut buffer,
                );

                // Print the globals
                if print_globals {
                    let globals = Package::get_provided_globals(package_str);
                    print_global_vars(&mut tabs, &globals, &mut buffer);
                }

                // Print the functions
                if print_functions {
                    let functions = Package::get_provided_fns(package_str);
                    print_fns(&mut tabs, &functions, &mut buffer);
                }
                tabs -= 1;
            }
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print matched event introduction
        if !event_info.is_empty() {
            white(
                true,
                format!(
                    "{}:{}:",
                    &probe_spec.provider.as_ref().unwrap().name,
                    &probe_spec.package.as_ref().unwrap().name
                ),
                &mut buffer,
            );
            magenta(
                true,
                probe_spec.event.as_ref().unwrap().name.to_string(),
                &mut buffer,
            );
            if let Some(mode_patt) = &probe_spec.mode {
                white(true, format!(":{}", &mode_patt.name), &mut buffer);
            }
            white(true, "\n".to_string(), &mut buffer);
            grey_italics(
                true,
                "matches the following events:\n\n".to_string(),
                &mut buffer,
            );
        }

        // Print the matched event information
        let mut tabs = 0;
        for (_prov_str, package_map) in event_info.iter() {
            for (_package_str, event_list) in package_map.iter() {
                for (event_info, event_str) in event_list {
                    if event_str.is_empty() {
                        continue;
                    }
                    magenta_italics(true, event_str.clone(), &mut buffer);
                    white(true, " event\n".to_string(), &mut buffer);

                    // Print the event description
                    tabs += 1;
                    white(
                        false,
                        format!("{}{}\n\n", " ".repeat(tabs * 4), event_info.docs),
                        &mut buffer,
                    );

                    // Print the globals
                    if print_globals {
                        let globals = Event::get_provided_globals(event_str);
                        print_global_vars(&mut tabs, &globals, &mut buffer);
                    }

                    // Print the functions
                    if print_functions {
                        let functions = Event::get_provided_fns(event_str);
                        print_fns(&mut tabs, &functions, &mut buffer);
                    }
                    tabs -= 1;
                }
            }
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print matched mode introduction
        if !mode_info.is_empty() {
            white(
                true,
                format!(
                    "{}:{}:{}:",
                    &probe_spec.provider.as_ref().unwrap().name,
                    &probe_spec.package.as_ref().unwrap().name,
                    &probe_spec.event.as_ref().unwrap().name
                ),
                &mut buffer,
            );
            magenta(
                true,
                format!("{}\n", &probe_spec.mode.as_ref().unwrap().name),
                &mut buffer,
            );
            grey_italics(
                true,
                "matches the following modes:\n\n".to_string(),
                &mut buffer,
            );
        }

        // Print the matched mode information
        let mut tabs = 0;
        for (_prov_str, package_map) in mode_info.iter() {
            for event_list in package_map.values() {
                for mode_list in event_list.values() {
                    for (mode_info, mode_str) in mode_list {
                        if mode_str.is_empty() {
                            continue;
                        }
                        magenta_italics(true, mode_str.clone(), &mut buffer);
                        white(true, " mode\n".to_string(), &mut buffer);

                        // Print the mode description
                        tabs += 1;
                        white(
                            false,
                            format!("{}{}\n\n", " ".repeat(tabs * 4), mode_info.docs),
                            &mut buffer,
                        );

                        // Print the globals
                        if print_globals {
                            let globals = Probe::get_provided_globals(mode_str);
                            print_global_vars(&mut tabs, &globals, &mut buffer);
                        }

                        // Print the functions
                        if print_functions {
                            let functions = Probe::get_provided_fns(mode_str);
                            print_fns(&mut tabs, &functions, &mut buffer);
                        }
                        tabs -= 1;
                    }
                }
            }
        }

        writer
            .print(&buffer)
            .expect("Uh oh, something went wrong while printing to terminal");
        buffer
            .reset()
            .expect("Uh oh, something went wrong while printing to terminal");

        Ok(())
    }

    pub fn add_global_stmts(&mut self, global_statements: Vec<Statement>) {
        self.global_stmts = global_statements;
    }

    /// Iterates over all the matched providers, packages, events, and probe mode names
    /// to add a copy of the user-defined Probe for each of them.
    pub fn add_probe(
        &mut self,
        provided_probes: &ProvidedProbes,
        probe_spec: &ProbeSpec,
        predicate: Option<Expr>,
        body: Option<Vec<Statement>>,
    ) -> Result<(), Box<WhammError>> {
        let mut reason = &probe_spec.provider;
        if let Some(prov_patt) = &probe_spec.provider {
            let matches = Provider::get_matches(provided_probes, &prov_patt.name);
            if matches.is_empty() {
                return Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified provider pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )));
            }

            for (.., provider_str) in matches.iter() {
                let mut is_empty = true;
                // Does provider exist yet?
                let provider = match self.providers.get_mut(provider_str) {
                    Some(prov) => prov,
                    None => {
                        // add the provider!
                        let new_prov = Provider::new(
                            provider_str.to_lowercase().to_string(),
                            prov_patt.loc.clone(),
                        );
                        self.providers
                            .insert(provider_str.to_lowercase().to_string(), new_prov);
                        self.providers
                            .get_mut(&provider_str.to_lowercase())
                            .unwrap()
                    }
                };

                if provider_str.to_uppercase() == "BEGIN" || provider_str.to_uppercase() == "END" {
                    // special case, just stop here
                    return Ok(());
                }

                if let Some(package_patt) = &probe_spec.package {
                    let matches =
                        Package::get_matches(provided_probes, provider_str, &package_patt.name);
                    if matches.is_empty() {
                        reason = &probe_spec.package;
                    }
                    for (.., package_str) in matches.iter() {
                        // Does package exist yet?
                        let package = match provider.packages.get_mut(package_str) {
                            Some(m) => m,
                            None => {
                                // add the package!
                                let new_mod = Package::new(
                                    package_str.to_lowercase().to_string(),
                                    package_patt.loc.clone(),
                                );
                                provider
                                    .packages
                                    .insert(package_str.to_lowercase().to_string(), new_mod);
                                provider
                                    .packages
                                    .get_mut(&package_str.to_lowercase())
                                    .unwrap()
                            }
                        };
                        if let Some(event_patt) = &probe_spec.event {
                            let matches = Event::get_matches(
                                provided_probes,
                                provider_str,
                                package_str,
                                &event_patt.name,
                            );
                            if matches.is_empty() {
                                reason = &probe_spec.event;
                            }
                            for (.., event_str) in matches.iter() {
                                // Does event exist yet?
                                let event = match package.events.get_mut(event_str) {
                                    Some(f) => f,
                                    None => {
                                        // add the event!
                                        let new_event = Event::new(
                                            event_str.to_lowercase().to_string(),
                                            event_patt.loc.clone(),
                                        );
                                        package.events.insert(
                                            event_str.to_lowercase().to_string(),
                                            new_event,
                                        );
                                        package.events.get_mut(&event_str.to_lowercase()).unwrap()
                                    }
                                };
                                if let Some(mode_patt) = &probe_spec.mode {
                                    let matches = Probe::get_matches(
                                        provided_probes,
                                        provider_str,
                                        package_str,
                                        event_str,
                                        &mode_patt.name,
                                    );
                                    if matches.is_empty() {
                                        reason = &probe_spec.mode;
                                    }

                                    for (.., name_str) in matches.iter() {
                                        event.insert_probe(
                                            name_str.to_string(),
                                            Probe::new(
                                                mode_patt.name.to_string(),
                                                mode_patt.loc.clone(),
                                                predicate.clone(),
                                                body.clone(),
                                            ),
                                        );
                                        is_empty = false;
                                    }
                                }
                            }
                        } else {
                            return Err(Box::new(ErrorGen::get_unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} Could not find an event matching pattern!")), None)));
                        }
                    }
                } else {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} Could not find a package matching pattern!"
                        )),
                        None,
                    )));
                }
                if is_empty {
                    // Never found a match under this provider, removing
                    self.providers.remove(provider_str);
                }
            }
        } else {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Could not find a provider matching pattern!"
                )),
                None,
            )));
        }
        if self.providers.is_empty() {
            if let Some(r) = reason {
                if let Some(mode_loc) = &r.loc {
                    return Err(Box::new(ErrorGen::get_parse_error(
                        true,
                        Some("Could not find any matches for this pattern".to_string()),
                        Some(mode_loc.line_col.clone()),
                        vec![],
                        vec![],
                    )));
                }
            }
        }
        Ok(())
    }
}

fn matches_globs(s: &str, globs: &[Pattern]) -> bool {
    for glob in globs.iter() {
        if glob.matches(s) {
            return true;
        }
    }
    false
}

fn get_globs(patt: &str) -> Vec<Pattern> {
    let mut globs = vec![];
    for p in patt.split('|') {
        globs.push(Pattern::new(p).unwrap());
    }

    globs
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProvidedFunctionality {
    pub name: String,
    pub docs: String,
}

pub struct Provider {
    pub name: String,
    pub fns: Vec<(ProvidedFunctionality, Fn)>, // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    /// The packages of the probes that have been used in the Script.
    /// These will be sub-packages of this Provider.
    pub packages: HashMap<String, Package>,
    pub loc: Option<Location>,
}
impl Provider {
    pub fn new(name: String, loc: Option<Location>) -> Self {
        let fns = Provider::get_provided_fns(&name);
        let globals = Provider::get_provided_globals(&name);
        Provider {
            name,
            fns,
            globals,
            packages: HashMap::new(),
            loc,
        }
    }

    fn get_provided_fns(_name: &str) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(_name: &str) -> HashMap<String, (ProvidedFunctionality, Global)> {
        HashMap::new()
    }

    /// Get the provider names that match the passed glob pattern
    pub fn get_matches(
        provided_probes: &ProvidedProbes,
        prov_patt: &str,
    ) -> Vec<(ProvidedFunctionality, String)> {
        let globs = get_globs(&prov_patt.to_lowercase());

        let mut matches = vec![];
        for (provider_name, (info, _provider)) in provided_probes.iter() {
            if matches_globs(&provider_name.to_lowercase(), &globs) {
                matches.push((info.clone(), provider_name.clone()));
            }
        }

        matches
    }
}

pub struct Package {
    pub name: String,
    pub fns: Vec<(ProvidedFunctionality, Fn)>, // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    /// The events of the probes that have been used in the Script.
    /// These will be sub-events of this Package.
    pub events: HashMap<String, Event>,
    pub loc: Option<Location>,
}
impl Package {
    pub fn new(name: String, loc: Option<Location>) -> Self {
        let fns = Package::get_provided_fns(&name);
        let globals = Package::get_provided_globals(&name);
        Package {
            name,
            fns,
            globals,
            events: HashMap::new(),
            loc,
        }
    }

    fn get_provided_fns(_name: &str) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(name: &str) -> HashMap<String, (ProvidedFunctionality, Global)> {
        let mut globals = HashMap::new();
        if name.to_lowercase() == "bytecode" {
            // Add in provided globals for the "call" event
            globals.insert(
                "tos".to_string(),
                (
                    ProvidedFunctionality {
                        name: "tos".to_string(),
                        docs: "To get the value on top of the Wasm stack.".to_string(),
                    },
                    Global {
                        is_comp_provided: true,
                        ty: DataType::I32,
                        var_name: Expr::VarId {
                            is_comp_provided: true,
                            name: "tos".to_string(),
                            loc: None,
                        },
                        value: None,
                    },
                ),
            );
            globals.insert(
                "wasm_bytecode_loc".to_string(),
                (
                    ProvidedFunctionality {
                        name: "wasm_bytecode_loc".to_string(),
                        docs:
                            "A unique identifier tied to the probe's location in the Wasm bytecode."
                                .to_string(),
                    },
                    Global {
                        is_comp_provided: true,
                        ty: DataType::I32,
                        var_name: Expr::VarId {
                            is_comp_provided: true,
                            name: "wasm_bytecode_loc".to_string(),
                            loc: None,
                        },
                        value: None,
                    },
                ),
            );
        }

        globals
    }

    /// Get the Package names that match the passed glob pattern
    pub fn get_matches(
        provided_probes: &ProvidedProbes,
        provider: &str,
        mod_patt: &str,
    ) -> Vec<(ProvidedFunctionality, String)> {
        let globs = get_globs(&mod_patt.to_lowercase());

        let mut matches = vec![];

        for (mod_name, (info, _package)) in provided_probes.get(provider).unwrap().1.iter() {
            if matches_globs(&mod_name.to_lowercase(), &globs) {
                matches.push((info.clone(), mod_name.clone()));
            }
        }

        matches
    }
}

pub struct Event {
    pub name: String,
    pub fns: Vec<(ProvidedFunctionality, Fn)>, // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided
    pub probe_map: HashMap<String, Vec<Probe>>,
    pub loc: Option<Location>,
}
impl Event {
    pub fn new(name: String, loc: Option<Location>) -> Self {
        let fns = Event::get_provided_fns(&name);
        let globals = Event::get_provided_globals(&name);
        Event {
            name,
            fns,
            globals,
            probe_map: HashMap::new(),
            loc,
        }
    }

    fn get_provided_fns(_name: &str) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(name: &str) -> HashMap<String, (ProvidedFunctionality, Global)> {
        let mut globals = HashMap::new();
        if name.to_lowercase() == "call" {
            // Add in provided globals for the "call" event
            globals.insert("target_fn_type".to_string(),(
                ProvidedFunctionality {
                    name: "target_fn_type".to_string(),
                    docs: "The type of function being called at this call site. This constant will \
                    evaluate to either `local` or `import`.".to_string()
                },
                Global {
                    is_comp_provided: true,
                    ty: DataType::Str,
                    var_name: Expr::VarId {
                        is_comp_provided: true,
                        name: "target_fn_type".to_string(),
                        loc: None
                    },
                    value: None
                }));
            globals.insert(
                "target_imp_module".to_string(),
                (
                    ProvidedFunctionality {
                        name: "target_imp_module".to_string(),
                        docs: "The name of the module that the imported function comes from. \
                    To improve performance, pair with `target_fn_type == \"import\"` \
                    for faster short-circuiting."
                            .to_string(),
                    },
                    Global {
                        is_comp_provided: true,
                        ty: DataType::Str,
                        var_name: Expr::VarId {
                            is_comp_provided: true,
                            name: "target_imp_module".to_string(),
                            loc: None,
                        },
                        value: None,
                    },
                ),
            );
            globals.insert(
                "target_imp_name".to_string(),
                (
                    ProvidedFunctionality {
                        name: "target_imp_name".to_string(),
                        docs: "The name of the imported function. \
                    To improve performance, pair with `target_fn_type == \"import\"` \
                    for faster short-circuiting."
                            .to_string(),
                    },
                    Global {
                        is_comp_provided: true,
                        ty: DataType::Str,
                        var_name: Expr::VarId {
                            is_comp_provided: true,
                            name: "target_imp_name".to_string(),
                            loc: None,
                        },
                        value: None,
                    },
                ),
            );
            globals.insert("new_target_fn_name".to_string(),(
                ProvidedFunctionality {
                    name: "new_target_fn_name".to_string(),
                    docs: "(DEPRECATED) The name of the target function to call instead of the original.".to_string()
                },
                Global {
                    is_comp_provided: true,
                    ty: DataType::Str,
                    var_name: Expr::VarId {
                        is_comp_provided: true,
                        name: "new_target_fn_name".to_string(),
                        loc: None
                    },
                    value: None
                }));
        }

        globals
    }

    /// Get the Event names that match the passed glob pattern
    pub fn get_matches(
        provided_probes: &ProvidedProbes,
        provider: &str,
        package: &str,
        func_patt: &str,
    ) -> Vec<(ProvidedFunctionality, String)> {
        let globs = get_globs(&func_patt.to_lowercase());

        let mut matches = vec![];

        for (fn_name, (info, _package)) in provided_probes
            .get(provider)
            .unwrap()
            .1
            .get(package)
            .unwrap()
            .1
            .iter()
        {
            if matches_globs(&fn_name.to_lowercase(), &globs) {
                matches.push((info.clone(), fn_name.clone()));
            }
        }

        matches
    }

    pub fn insert_probe(&mut self, name: String, probe: Probe) {
        // Does name exist yet?
        match self.probe_map.get_mut(&name) {
            Some(probes) => {
                // Add probe to list
                probes.push(probe);
            }
            None => {
                self.probe_map.insert(name, vec![probe]);
            }
        };
    }
}

#[derive(Clone, Debug)]
pub struct Probe {
    pub mode: String,
    pub loc: Option<Location>,
    pub fns: Vec<(ProvidedFunctionality, Fn)>, // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    pub predicate: Option<Expr>,
    pub body: Option<Vec<Statement>>,
}
impl Probe {
    pub fn new(
        mode: String,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Vec<Statement>>,
    ) -> Self {
        let fns = Probe::get_provided_fns(&mode);
        let globals = Probe::get_provided_globals(&mode);
        Probe {
            mode,
            loc,
            fns,
            globals,

            predicate,
            body,
        }
    }

    fn get_provided_fns(_mode: &str) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(_mode: &str) -> HashMap<String, (ProvidedFunctionality, Global)> {
        HashMap::new()
    }

    /// Get the Probe modes that match the passed glob pattern
    pub fn get_matches(
        provided_probes: &ProvidedProbes,
        provider: &str,
        package: &str,
        event: &str,
        mode_patt: &str,
    ) -> Vec<(ProvidedFunctionality, String)> {
        let globs = get_globs(&mode_patt.to_lowercase());

        let mut matches = vec![];

        for (info, m_name) in provided_probes
            .get(provider)
            .unwrap()
            .1
            .get(package)
            .unwrap()
            .1
            .get(event)
            .unwrap()
            .1
            .iter()
        {
            if matches_globs(&m_name.to_lowercase(), &globs) {
                matches.push((info.clone(), m_name.clone()));
            }
        }

        matches
    }
}

// =====================
// ---- Expressions ----
// =====================

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnOp {
    Not,
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
}

// =================
// ==== Visitor ====
// =================

pub trait WhammVisitor<T> {
    fn visit_whamm(&mut self, whamm: &Whamm) -> T;
    fn visit_script(&mut self, script: &Script) -> T;
    fn visit_provider(&mut self, provider: &Provider) -> T;
    fn visit_package(&mut self, package: &Package) -> T;
    fn visit_event(&mut self, event: &Event) -> T;
    fn visit_probe(&mut self, probe: &Probe) -> T;
    // fn visit_predicate(&mut self, predicate: &Expr) -> T;
    fn visit_fn(&mut self, f: &Fn) -> T;
    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> T;
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
    fn visit_provider(&mut self, provider: &mut Provider) -> T;
    fn visit_package(&mut self, package: &mut Package) -> T;
    fn visit_event(&mut self, event: &mut Event) -> T;
    fn visit_probe(&mut self, probe: &mut Probe) -> T;
    // fn visit_predicate(&mut self, predicate: &mut Expr) -> T;
    fn visit_fn(&mut self, f: &mut Fn) -> T;
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> T;
    fn visit_stmt(&mut self, stmt: &mut Statement) -> T;
    fn visit_expr(&mut self, expr: &mut Expr) -> T;
    fn visit_unop(&mut self, unop: &mut UnOp) -> T;
    fn visit_binop(&mut self, op: &mut BinOp) -> T;
    fn visit_datatype(&mut self, datatype: &mut DataType) -> T;
    fn visit_value(&mut self, val: &mut Value) -> T;
}

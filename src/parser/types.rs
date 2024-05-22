use std::collections::HashMap;
use termcolor::{Buffer, ColorChoice, WriteColor};
use glob::Pattern;
use pest::error::LineColLocation;

use pest_derive::Parser;
use pest::pratt_parser::PrattParser;
use termcolor::BufferWriter;
use walrus::DataId;
use crate::common::terminal::{green, long_line, magenta, magenta_italics, white, white_italics, yellow};
use crate::common::error::{ErrorGen, WhammError};

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
    };
}

const UNEXPECTED_ERR_MSG: &str = "WhammParser: Looks like you've found a bug...please report this behavior! Exiting now...";

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
            LineColLocation::Span(span0, ..) => span0
        };

        let pos1 = match loc1 {
            LineColLocation::Pos(pos0) => pos0,
            LineColLocation::Span(.., span1) => span1
        };

        Location {
            line_col: LineColLocation::Span(pos0.clone(), pos1.clone()),
            path
        }
    }

    pub fn span_between(loc0: &Location, loc1: &Location) -> LineColLocation {
        let pos0 = match &loc0.line_col {
            LineColLocation::Pos(pos0) |
            LineColLocation::Span(pos0, ..) => pos0.clone()
        };

        let pos1 = match &loc1.line_col {
            LineColLocation::Pos(end1) |
            LineColLocation::Span(.., end1) => end1.clone()
        };

        return LineColLocation::Span(pos0, pos1);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DataType {
    Integer,
    Boolean,
    Null,
    Str,
    Tuple {
        ty_info: Option<Vec<Box<DataType>>>
    }
}
impl DataType {
    pub fn print(&self, buffer: &mut Buffer) {
        match self {
            DataType::Integer => {
                yellow(true, "int".to_string(), buffer);
            },
            DataType::Boolean => {
                yellow(true, "bool".to_string(), buffer);
            },
            DataType::Null => {
                yellow(true, "null".to_string(), buffer);
            },
            DataType::Str => {
                yellow(true, "str".to_string(), buffer);
            },
            DataType::Tuple {ty_info} => {
                white(true, "(".to_string(), buffer);
                let mut is_first = true;
                if let Some(types) = ty_info {
                    for ty in types {
                        if !is_first {
                            white(true, ", ".to_string(), buffer);
                        }
                        ty.print(buffer);
                        is_first = false;
                    }
                }
                white(true, ")".to_string(), buffer);
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
        addr: Option<(DataId, u32, usize)>
    },
    Tuple {
        ty: DataType,
        vals: Vec<Expr>,
    },
    Boolean {
        ty: DataType,
        val: bool
    }
}


// Statements
#[derive(Clone, Debug)]
pub enum Statement {
    Assign {
        var_id: Expr, // Should be VarId
        expr: Expr,
        loc: Option<Location>
    },

    /// Standalone `Expr` statement, which means we can write programs like this:
    /// int main() {
    ///   2 + 2;
    ///   return 0;
    /// }
    Expr {
        expr: Expr,
        loc: Option<Location>
    }
}
impl Statement {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Statement::Assign {loc, ..} |
            Statement::Expr {loc, ..} => {
                loc
            }
        }
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        return match self.loc() {
            Some(loc) => Some(loc.line_col.clone()),
            None => None
        }
    }
    pub fn dummy() -> Self {
        Self::Expr {
            expr: Expr::Primitive {
                val: Value::Integer {
                    ty: DataType::Integer,
                    val: 0,
                },
                loc: None
            },
            loc: None
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    BinOp {     // Type is based on the outermost `op` (if arithmetic op, also based on types of lhs/rhs due to doubles)
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
        loc: Option<Location>
    },
    Call {      // Type is fn_target.return_ty, should be VarId
        fn_target: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        loc: Option<Location>
    },
    VarId {
        // is_comp_provided: bool, // TODO -- do I need this?
        name: String,
        loc: Option<Location>
    },
    Primitive { // Type is val.ty
        val: Value,
        loc: Option<Location>
    }
}
impl Expr {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Expr::BinOp {loc, ..} |
            Expr::Call {loc, ..} |
            Expr::VarId {loc, ..} |
            Expr::Primitive {loc, ..} => {
                loc
            }
        }
    }
}

// Functions

#[derive(Clone, Debug)]
pub struct FnId {
    pub name: String,
    pub loc: Option<Location>
}

#[derive(Clone, Debug)]
pub struct Fn {
    pub(crate) is_comp_provided: bool,
    pub(crate) name: FnId,
    pub(crate) params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) return_ty: Option<DataType>,
    pub(crate) body: Option<Vec<Statement>>
}
impl Fn {
    pub fn print(&self, buffer: &mut Buffer) {
        green(true, format!("{}", self.name.name), buffer);
        white(true, "(".to_string(), buffer);
        let mut is_first = true;
        for (param_name, param_ty) in self.params.iter() {
            if !is_first {
                white(true, ", ".to_string(), buffer);
            }
            if let Expr::VarId {name, ..} = param_name {
                green(true, format!("{name}"), buffer);
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
    pub value: Option<Value>
}
impl Global {
    pub fn print(&self, buffer: &mut Buffer) {
        if let Expr::VarId {name, ..} = &self.var_name {
            green(true, format!("{name}"), buffer);
        }
        white(true, ": ".to_string(), buffer);
        self.ty.print(buffer);
    }
}

fn print_global_vars(tabs: &mut usize, globals: &HashMap<String, (ProvidedFunctionality, Global)>, buffer: &mut Buffer) {
    if !globals.is_empty() {
        white(true, format!("{}GLOBALS:\n", " ".repeat(*tabs * 4)), buffer);
        *tabs += 1;
        for (.., (info, global)) in globals.iter() {
            white(false, format!("{}", " ".repeat(*tabs * 4)), buffer);
            global.print(buffer);

            *tabs += 1;
            white(false, format!("\n{}{}\n", " ".repeat(*tabs * 4), info.docs), buffer);
            *tabs -= 1;
        }
        *tabs -= 1;
        white(false, format!("\n"), buffer);
    }
}

fn print_fns(tabs: &mut usize, functions: &Vec<(ProvidedFunctionality, Fn)>, buffer: &mut Buffer) {
    if !functions.is_empty() {
        white(true, format!("{}FUNCTIONS:\n", " ".repeat(*tabs * 4)), buffer);
        *tabs += 1;
        for (info, f) in functions.iter() {
            green(true, format!("{}", " ".repeat(*tabs * 4)), buffer);
            f.print(buffer);
            green(true, format!("\n"), buffer);
            *tabs += 1;
            white(false, format!("{}{}\n", " ".repeat(*tabs * 4), info.docs), buffer);
            *tabs -= 1;
        }
        *tabs -= 1;
        white(false, format!("\n"), buffer);
    }
}

pub type ProvidedProbes = HashMap<String, (
    ProvidedFunctionality,
    HashMap<String, (
        ProvidedFunctionality,
        HashMap<String, (
            ProvidedFunctionality,
            Vec<(ProvidedFunctionality, String)>
        )>
    )>
)>;

pub struct Whamm {
    pub provided_probes: ProvidedProbes,
    pub fns: Vec<(ProvidedFunctionality, Fn)>,                     // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    pub whammys: Vec<Whammy>
}
impl Whamm {
    pub fn new() -> Self {
        let mut whamm = Whamm {
            provided_probes: HashMap::new(),
            fns: Whamm::get_provided_fns(),
            globals: Whamm::get_provided_globals(),

            whammys: vec![]
        };
        whamm.init_provided_probes();
        whamm
    }

    fn get_provided_fns() -> Vec<(ProvidedFunctionality, Fn)> {
        let params = vec![
            (
                Expr::VarId {
                    name: "str_addr".to_string(),
                    loc: None
                },
                DataType::Tuple {
                    ty_info: Some(vec![
                        Box::new(DataType::Integer),
                        Box::new(DataType::Integer)
                    ]),
                }
            ),
            (
                Expr::VarId {
                    name: "value".to_string(),
                    loc: None
                },
                DataType::Str
            )
        ];
        let strcmp_fn = Fn {
            is_comp_provided: true,
            name: FnId {
                name: "strcmp".to_string(),
                loc: None
            },
            params,
            return_ty: Some(DataType::Boolean),
            body: None
        };
        let docs = ProvidedFunctionality {
            name: "strcmp".to_string(),
            docs: "Compare two wasm strings and return whether they are equivalent.".to_string()
        };

        vec![ (docs, strcmp_fn) ]
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
        // Not really any packages or events for a core probe...just two types!
        self.provided_probes.insert("begin".to_string(), (
                ProvidedFunctionality {
                    name: "begin".to_string(),
                    docs: "Run this logic on application startup.".to_string(),
                },
                HashMap::new()
            ));
        self.provided_probes.insert("end".to_string(), (
            ProvidedFunctionality {
                name: "end".to_string(),
                docs: "Run this logic when the application exits.".to_string(),
            },
            HashMap::new()
        ));
    }

    fn init_wasm_probes(&mut self) {
        // This list of events matches up with bytecodes supported by Walrus.
        // See: https://docs.rs/walrus/latest/walrus/ir/
        let wasm_bytecode_events = vec![
            (
                ProvidedFunctionality {
                    name: "Block".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/block".to_string(),
                },
                "Block".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Loop".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/loop".to_string(),
                },
                "Loop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Call".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                },
                "Call".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "CallIndirect".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                },
                "CallIndirect".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "LocalGet".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_get".to_string(),
                },
                "LocalGet".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "LocalSet".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_set".to_string(),
                },
                "LocalSet".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "LocalTee".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_tee".to_string(),
                },
                "LocalTee".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "GlobalGet".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_get".to_string(),
                },
                "GlobalGet".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "GlobalSet".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_set".to_string(),
                },
                "GlobalSet".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Const".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const".to_string(),
                },
                "Const".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Binop".to_string(),
                    docs: "Consume two operands and produce one result of the respective type. \
                    The types of binary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric".to_string(),
                },
                "Binop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Unop".to_string(),
                    docs: "Consume one operand and produce one result of the respective type. \
                    The types of unary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric".to_string(),
                },
                "Unop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Select".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Select".to_string(),
                },
                "Select".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Unreachable".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/unreachable".to_string(),
                },
                "Unreachable".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Br".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                },
                "Br".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "BrIf".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                },
                "BrIf".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "IfElse".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else".to_string(),
                },
                "IfElse".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "BrTable".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                },
                "BrTable".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Drop".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Drop".to_string(),
                },
                "Drop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Return".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/return".to_string(),
                },
                "Return".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "MemorySize".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Size".to_string(),
                },
                "MemorySize".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "MemoryGrow".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Grow".to_string(),
                },
                "MemoryGrow".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "MemoryInit".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                },
                "MemoryInit".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "DataDrop".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                },
                "DataDrop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "MemoryCopy".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy".to_string(),
                },
                "MemoryCopy".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "MemoryFill".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Fill".to_string(),
                },
                "MemoryFill".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Load".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load".to_string(),
                },
                "Load".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Store".to_string(),
                    docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store".to_string(),
                },
                "Store".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "AtomicRmw".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#read-modify-write".to_string(),
                },
                "AtomicRmw".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "Cmpxchg".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#compare-exchange".to_string(),
                },
                "Cmpxchg".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "AtomicNotify".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                },
                "AtomicNotify".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "AtomicWait".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                },
                "AtomicWait".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "AtomicFence".to_string(),
                    docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#fence-operator".to_string(),
                },
                "AtomicFence".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "TableGet".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "TableGet".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "TableSet".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "TableSet".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "TableGrow".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "TableGrow".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "TableSize".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "TableSize".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "TableFill".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "TableFill".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "RefNull".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                },
                "RefNull".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "RefIsNull".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                },
                "RefIsNull".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "RefFunc".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                },
                "RefFunc".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "V128Bitselect".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "V128Bitselect".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "I8x16Swizzle".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "I8x16Swizzle".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "I8x16Shuffle".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "I8x16Shuffle".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "LoadSimd".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                },
                "LoadSimd".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "TableInit".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "TableInit".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "ElemDrop".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "ElemDrop".to_string()
            ),
            (
                ProvidedFunctionality {
                    name: "TableCopy".to_string(),
                    docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                },
                "TableCopy".to_string()
            ),
        ];
        let wasm_bytecode_probe_types = vec![
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
            wasm_bytecode_map.insert(name, (info.clone(), wasm_bytecode_probe_types.clone()));
        }

        self.provided_probes.insert("wasm".to_string(), (
            ProvidedFunctionality {
                name: "wasm".to_string(),
                docs: "This provides various events to instrument that are specific \
                to WebAssembly.".to_string(),
            },
            HashMap::from([("bytecode".to_string(), (
                ProvidedFunctionality {
                    name: "bytecode".to_string(),
                    docs: "This package within the wasm provider contains enables the \
                    instrumentation of WebAssembly bytecode instructions.".to_string(),
                },
                wasm_bytecode_map
            ))])));
    }
    pub fn add_whammy(&mut self, mut whammy: Whammy) -> usize {
        let id = self.whammys.len();
        whammy.name = format!("whammy{}", id);
        self.whammys.push(whammy);

        id
    }
}

pub struct SpecPart {
    pub name: String,
    pub loc: Option<Location>
}

pub struct ProbeSpec {
    pub provider: Option<SpecPart>,
    pub package: Option<SpecPart>,
    pub event: Option<SpecPart>,
    pub mode: Option<SpecPart>
}
impl ProbeSpec {
    pub fn new() -> Self {
        Self {
            provider: None,
            package: None,
            event: None,
            mode: None
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
            return;
        }
    }
}

pub struct Whammy {
    pub name: String,
    /// The providers of the probes that have been used in the Whammy.
    pub providers: HashMap<String, Provider>,
    pub fns: Vec<Fn>,                     // User-provided
    pub globals: HashMap<String, Global>, // User-provided, should be VarId
}
impl Whammy {
    pub fn new() -> Self {
        Whammy {
            name: "".to_string(),
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new()
        }
    }

    fn get_provider_info(provided_probes: &ProvidedProbes, probe_spec: &ProbeSpec) -> Result<Vec<(ProvidedFunctionality, String)>, WhammError> {
        let (prov_matches, prov_loc) = if let Some(prov_patt) = &probe_spec.provider {
            (Provider::get_matches(provided_probes, &prov_patt.name), prov_patt.loc.clone())
        } else {
            (vec![], None)
        };

        if prov_matches.is_empty() {
            let loc = if let Some(loc) = &prov_loc {
                Some(loc.line_col.clone())
            } else {
                None
            };
            return Err(ErrorGen::get_parse_error(true,
                 Some(format!("Could not find any matches for the provider pattern")),
                 loc, vec![], vec![]));
        }

        Ok(prov_matches)
    }

    fn get_package_info(provided_probes: &ProvidedProbes, provider_matches: &Vec<(ProvidedFunctionality, String)>, probe_spec: &ProbeSpec) -> Result<HashMap<String, Vec<(ProvidedFunctionality, String)>>, WhammError> {
        let (package_matches, package_loc) = if let Some(package_patt) = &probe_spec.package {
            let mut matches = HashMap::new();
            for (.., provider) in provider_matches.iter() {
                let next = Package::get_matches(provided_probes, provider, &package_patt.name);
                matches.insert(provider.clone(),next);
            }

            (matches, package_patt.loc.clone())
        } else {
            (HashMap::new(), None)
        };

        if package_matches.is_empty() {
            let loc = if let Some(loc) = &package_loc {
                Some(loc.line_col.clone())
            } else {
                None
            };
            return Err(ErrorGen::get_parse_error(true,
             Some(format!("Could not find any matches for the package pattern")),
             loc, vec![], vec![]));
        }
        Ok(package_matches)
    }

    fn get_event_info(provided_probes: &ProvidedProbes, package_matches: &HashMap<String, Vec<(ProvidedFunctionality, String)>>, probe_spec: &ProbeSpec) -> Result<HashMap<String, HashMap<String, Vec<(ProvidedFunctionality, String)>>>, WhammError> {
        let (event_matches, event_loc) = if let Some(event_patt) = &probe_spec.event {
            let mut event_matches = HashMap::new();
            for (provider_name, packages) in package_matches.iter() {
                let mut package = HashMap::new();
                for (.., package_name) in packages.iter() {
                    let next = Event::get_matches(provided_probes, provider_name, package_name, &event_patt.name);
                    package.insert(package_name.clone(), next);
                }
                event_matches.insert(provider_name.clone(), package);
            }

            (event_matches, event_patt.loc.clone())
        } else {
            (HashMap::new(), None)
        };

        if package_matches.is_empty() {
            let loc = if let Some(loc) = &event_loc {
                Some(loc.line_col.clone())
            } else {
                None
            };
            return Err(ErrorGen::get_parse_error(true,
                                                 Some(format!("Could not find any matches for the event pattern")),
                                                 loc, vec![], vec![]));
        }
        Ok(event_matches)
    }

    fn get_mode_info(provided_probes: &ProvidedProbes, matches: &HashMap<String, HashMap<String, Vec<(ProvidedFunctionality, String)>>>, probe_spec: &ProbeSpec) -> Result<HashMap<String, HashMap<String, HashMap<String, Vec<(ProvidedFunctionality, String)>>>>, WhammError> {
        let (mode_matches, mode_loc) = if let Some(mode_patt) = &probe_spec.mode {
            let mut mode_matches = HashMap::new();
            for (provider_name, package_matches) in matches.iter() {
                let mut package = HashMap::new();
                for (package_name, event_matches) in package_matches.iter() {
                    let mut modes = HashMap::new();
                    for (.., event_name) in event_matches.iter() {
                        let next = Probe::get_matches(provided_probes, provider_name, package_name, event_name, &mode_patt.name);
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
            let loc = if let Some(loc) = &mode_loc {
                Some(loc.line_col.clone())
            } else {
                None
            };
            return Err(ErrorGen::get_parse_error(true,
                                                 Some(format!("Could not find any matches for the mode pattern")),
                                                 loc, vec![], vec![]));
        }
        Ok(mode_matches)
    }

    pub fn print_info(&mut self, provided_probes: &ProvidedProbes, probe_spec: &ProbeSpec,
                      print_globals: bool, print_functions: bool) -> Result<(), WhammError> {
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
            magenta(true, format!("{}", &probe_spec.provider.as_ref().unwrap().name), &mut buffer);
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
            white_italics(true, "matches the following providers:\n\n".to_string(), &mut buffer);
        }

        // Print the matched provider information
        for (provider_info, provider_str) in prov_info.iter() {
            magenta_italics(true, provider_str.clone(), &mut buffer);
            white(true, format!(" provider\n"), &mut buffer);

            // Print the provider description
            tabs += 1;
            white(false, format!("{}{}\n\n", " ".repeat(tabs * 4), provider_info.docs), &mut buffer);

            // Print the globals
            if print_globals {
                let globals = Provider::get_provided_globals(&provider_str);
                print_global_vars(&mut tabs, &globals, &mut buffer);
            }

            // Print the functions
            if print_functions {
                let functions = Provider::get_provided_fns(&provider_str);
                print_fns(&mut tabs, &functions, &mut buffer);
            }
            tabs -= 1;
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print matched package introduction
        if !pkg_info.is_empty() {
            white(true, format!("{}:", &probe_spec.provider.as_ref().unwrap().name), &mut buffer);
            magenta(true, format!("{}", &probe_spec.package.as_ref().unwrap().name), &mut buffer);
            if let Some(event_patt) = &probe_spec.event {
                white(true, format!(":{}", &event_patt.name), &mut buffer);
                if let Some(mode_patt) = &probe_spec.mode {
                    white(true, format!(":{}", &mode_patt.name), &mut buffer);
                }
            }
            white(true, "\n".to_string(), &mut buffer);
            white_italics(true, "matches the following packages:\n\n".to_string(), &mut buffer);
        }

        // Print the matched package information
        let mut tabs = 0;
        for (_prov_str, package_list) in pkg_info.iter() {
            for (package_info, package_str) in package_list {
                magenta_italics(true, package_str.clone(), &mut buffer);
                white(true, format!(" package\n"), &mut buffer);

                // Print the package description
                tabs += 1;
                white(false, format!("{}{}\n\n", " ".repeat(tabs * 4), package_info.docs), &mut buffer);

                // Print the globals
                if print_globals {
                    let globals = Package::get_provided_globals(&package_str);
                    print_global_vars(&mut tabs, &globals, &mut buffer);
                }

                // Print the functions
                if print_functions {
                    let functions = Package::get_provided_fns(&package_str);
                    print_fns(&mut tabs, &functions, &mut buffer);
                }
                tabs -= 1;
            }
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print matched event introduction
        if !pkg_info.is_empty() {
            white(true, format!("{}:{}:", &probe_spec.provider.as_ref().unwrap().name, &probe_spec.package.as_ref().unwrap().name), &mut buffer);
            magenta(true, format!("{}", &probe_spec.event.as_ref().unwrap().name), &mut buffer);
            if let Some(mode_patt) = &probe_spec.mode {
                white(true, format!(":{}", &mode_patt.name), &mut buffer);
            }
            white(true, "\n".to_string(), &mut buffer);
            white_italics(true, "matches the following events:\n\n".to_string(), &mut buffer);
        }

        // Print the matched event information
        let mut tabs = 0;
        for (_prov_str, package_map) in event_info.iter() {
            for (_package_str, event_list) in package_map {
                for (event_info, event_str) in event_list {
                    magenta_italics(true, event_str.clone(), &mut buffer);
                    white(true, format!(" event\n"), &mut buffer);

                    // Print the event description
                    tabs += 1;
                    white(false, format!("{}{}\n\n", " ".repeat(tabs * 4), event_info.docs), &mut buffer);

                    // Print the globals
                    if print_globals {
                        let globals = Event::get_provided_globals(&event_str);
                        print_global_vars(&mut tabs, &globals, &mut buffer);
                    }

                    // Print the functions
                    if print_functions {
                        let functions = Event::get_provided_fns(&event_str);
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
            white(true, format!("{}:{}:{}:", &probe_spec.provider.as_ref().unwrap().name,
                                &probe_spec.package.as_ref().unwrap().name,
                                &probe_spec.event.as_ref().unwrap().name), &mut buffer);
            magenta(true, format!("{}\n", &probe_spec.mode.as_ref().unwrap().name), &mut buffer);
            white_italics(true, "matches the following modes:\n\n".to_string(), &mut buffer);
        }

        // Print the matched mode information
        let mut tabs = 0;
        for (_prov_str, package_map) in mode_info.iter() {
            for (_package_str, event_list) in package_map {
                for (_event_str, mode_list) in event_list {
                    for (mode_info, mode_str) in mode_list {
                        magenta_italics(true, mode_str.clone(), &mut buffer);
                        white(true, format!(" mode\n"), &mut buffer);

                        // Print the mode description
                        tabs += 1;
                        white(false, format!("{}{}\n\n", " ".repeat(tabs * 4), mode_info.docs), &mut buffer);

                        // Print the globals
                        if print_globals {
                            let globals = Probe::get_provided_globals(&mode_str);
                            print_global_vars(&mut tabs, &globals, &mut buffer);
                        }

                        // Print the functions
                        if print_functions {
                            let functions = Probe::get_provided_fns(&mode_str);
                            print_fns(&mut tabs, &functions, &mut buffer);
                        }
                        tabs -= 1;
                    }
                }
            }
        }

        writer.print(&buffer).expect("Uh oh, something went wrong while printing to terminal");
        buffer.reset().expect("Uh oh, something went wrong while printing to terminal");

        return Ok(());
    }

    /// Iterates over all of the matched providers, packages, events, and probe names
    /// to add a copy of the user-defined Probe for each of them.
    pub fn add_probe(&mut self, provided_probes: &ProvidedProbes,
                     probe_spec: &ProbeSpec, predicate: Option<Expr>, body: Option<Vec<Statement>>) -> Result<(), WhammError> {
        let mut reason = &probe_spec.provider;
        if let Some(prov_patt) = &probe_spec.provider {

            let matches = Provider::get_matches(provided_probes, &prov_patt.name);
            if matches.is_empty() {
                return Err(ErrorGen::get_parse_error(true,
                    Some(format!("Could not find any matches for the specified provider pattern: {}", prov_patt.name)),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()), vec![], vec![]));
            }

            for (.., provider_str) in matches.iter() {
                let mut is_empty = true;
                // Does provider exist yet?
                let provider = match self.providers.get_mut(provider_str) {
                    Some(prov) => prov,
                    None => {
                        // add the provider!
                        let new_prov = Provider::new(provider_str.to_lowercase().to_string(), prov_patt.loc.clone());
                        self.providers.insert(provider_str.to_lowercase().to_string(), new_prov);
                        self.providers.get_mut(&provider_str.to_lowercase()).unwrap()
                    }
                };

                if provider_str.to_uppercase() == "BEGIN" || provider_str.to_uppercase() == "END" {
                    // special case, just stop here
                    return Ok(());
                }

                if let Some(package_patt) = &probe_spec.package {
                    let matches = Package::get_matches(provided_probes, provider_str, &package_patt.name);
                    if matches.is_empty() {
                        reason = &probe_spec.package;
                    }
                    for (.., package_str) in matches.iter() {
                        // Does package exist yet?
                        let package = match provider.packages.get_mut(package_str) {
                            Some(m) => m,
                            None => {
                                // add the package!
                                let new_mod = Package::new(package_str.to_lowercase().to_string(), package_patt.loc.clone());
                                provider.packages.insert(package_str.to_lowercase().to_string(), new_mod);
                                provider.packages.get_mut(&package_str.to_lowercase()).unwrap()
                            }
                        };
                        if let Some(event_patt) = &probe_spec.event {
                            let matches = Event::get_matches(provided_probes, provider_str, package_str, &event_patt.name);
                            if matches.is_empty() {
                                reason = &probe_spec.event;
                            }
                            for (.., event_str) in matches.iter() {
                                // Does event exist yet?
                                let event = match package.events.get_mut(event_str) {
                                    Some(f) => f,
                                    None => {
                                        // add the package!
                                        let new_fn = Event::new(event_str.to_lowercase().to_string(), event_patt.loc.clone());
                                        package.events.insert(event_str.to_lowercase().to_string(), new_fn);
                                        package.events.get_mut(&event_str.to_lowercase()).unwrap()
                                    }
                                };
                                if let Some(mode_patt) = &probe_spec.mode {
                                    let matches = Probe::get_matches(provided_probes, provider_str, package_str, event_str, &mode_patt.name);
                                    if matches.is_empty() {
                                        reason = &probe_spec.mode;
                                    }

                                    for (.., name_str) in matches.iter() {
                                        event.insert_probe(name_str.to_string(), Probe::new(mode_patt.name.to_string(), mode_patt.loc.clone(), predicate.clone(), body.clone()));
                                        is_empty = false;
                                    }
                                }
                            }
                        } else {
                            return Err(ErrorGen::get_unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} Could not find an event matching pattern!")), None));
                        }
                    }
                } else {
                    return Err(ErrorGen::get_unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} Could not find a package matching pattern!")), None));
                }
                if is_empty {
                    // Never found a match under this provider, removing
                    self.providers.remove(provider_str);
                }
            }
        } else {
            return Err(ErrorGen::get_unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} Could not find a provider matching pattern!")), None));
        }
        if self.providers.is_empty() {
            if let Some(r) = reason {
                if let Some(mode_loc) = &r.loc {
                    return Err(ErrorGen::get_parse_error(true,
                         Some(format!("Could not find any matches for this pattern")),
                         Some(mode_loc.line_col.clone()), vec![], vec![]));
                }
            }
        }
        return Ok(());
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProvidedFunctionality {
    pub name: String,
    pub docs: String
}

pub struct Provider {
    pub name: String,
    pub fns: Vec<(ProvidedFunctionality, Fn)>,                     // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    /// The packages of the probes that have been used in the Whammy.
    /// These will be sub-packages of this Provider.
    pub packages: HashMap<String, Package>,
    pub loc: Option<Location>
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
            loc
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(_name: &String) -> HashMap<String, (ProvidedFunctionality, Global)> {
        HashMap::new()
    }

    /// Get the provider names that match the passed glob pattern
    pub fn get_matches(provided_probes: &ProvidedProbes, prov_patt: &str) -> Vec<(ProvidedFunctionality, String)> {
        let glob = Pattern::new(&prov_patt.to_lowercase()).unwrap();

        let mut matches = vec![];
        for (provider_name, (info, _provider)) in provided_probes.into_iter() {
            if glob.matches(&provider_name.to_lowercase()) {
                matches.push((info.clone(), provider_name.clone()));
            }
        }

        matches
    }
}

pub struct Package {
    pub name: String,
    pub fns: Vec<(ProvidedFunctionality, Fn)>,                     // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    /// The events of the probes that have been used in the Whammy.
    /// These will be sub-events of this Package.
    pub events: HashMap<String, Event>,
    pub loc: Option<Location>
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
            loc
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(_name: &String) -> HashMap<String, (ProvidedFunctionality, Global)> {
        HashMap::new()
    }

    /// Get the Package names that match the passed glob pattern
    pub fn get_matches(provided_probes: &ProvidedProbes, provider: &str, mod_patt: &str) -> Vec<(ProvidedFunctionality, String)> {
        let glob = Pattern::new(&mod_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for (mod_name, (info, _package)) in provided_probes.get(provider).unwrap().1.iter() {
            if glob.matches(&mod_name.to_lowercase()) {
                matches.push((info.clone(), mod_name.clone()));
            }
        }

        matches
    }
}

pub struct Event {
    pub name: String,
    pub fns: Vec<(ProvidedFunctionality, Fn)>,                     // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided
    pub probe_map: HashMap<String, Vec<Probe>>,
    pub loc: Option<Location>
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
            loc
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(name: &String) -> HashMap<String, (ProvidedFunctionality, Global)> {
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
                        name: "target_fn_type".to_string(),
                        loc: None
                    },
                    value: None
                }));
            globals.insert("target_imp_module".to_string(),(
                ProvidedFunctionality {
                    name: "target_imp_module".to_string(),
                    docs: "The name of the module that the imported function comes from. \
                    To improve performance, pair with `target_fn_type == \"import\"` \
                    for faster short-circuiting.".to_string()
                },
                Global {
                    is_comp_provided: true,
                    ty: DataType::Str,
                    var_name: Expr::VarId {
                        name: "target_imp_module".to_string(),
                        loc: None
                    },
                    value: None
                }));
            globals.insert("target_imp_name".to_string(),(
                ProvidedFunctionality {
                    name: "target_imp_name".to_string(),
                    docs: "The name of the imported function. \
                    To improve performance, pair with `target_fn_type == \"import\"` \
                    for faster short-circuiting.".to_string()
                },
                Global {
                    is_comp_provided: true,
                    ty: DataType::Str,
                    var_name: Expr::VarId {
                        name: "target_imp_name".to_string(),
                        loc: None
                    },
                    value: None
                }));
            globals.insert("new_target_fn_name".to_string(),(
                ProvidedFunctionality {
                    name: "new_target_fn_name".to_string(),
                    docs: "(DEPRECATED) The name of the target function to call instead of the original.".to_string()
                },
                Global {
                    is_comp_provided: true,
                    ty: DataType::Str,
                    var_name: Expr::VarId {
                        name: "new_target_fn_name".to_string(),
                        loc: None
                    },
                    value: None
                }));
        }

        globals
    }

    /// Get the Event names that match the passed glob pattern
    pub fn get_matches(provided_probes: &ProvidedProbes, provider: &str, package: &str, func_patt: &str) -> Vec<(ProvidedFunctionality, String)> {
        let glob = Pattern::new(&func_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for (fn_name, (info, _package)) in provided_probes.get(provider).unwrap().1.get(package).unwrap().1.iter() {
            if glob.matches(&fn_name.to_lowercase()) {
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
            },
            None => {
                self.probe_map.insert(name, vec![ probe ]);
            }
        };
    }
}

#[derive(Clone, Debug)]
pub struct Probe {
    pub name: String,
    pub loc: Option<Location>,
    pub fns: Vec<(ProvidedFunctionality, Fn)>,                     // Comp-provided
    pub globals: HashMap<String, (ProvidedFunctionality, Global)>, // Comp-provided

    pub predicate: Option<Expr>,
    pub body: Option<Vec<Statement>>
}
impl Probe {
    pub fn new(name: String, loc: Option<Location>, predicate: Option<Expr>, body: Option<Vec<Statement>>) -> Self {
        let fns = Probe::get_provided_fns(&name);
        let globals = Probe::get_provided_globals(&name);
        Probe {
            name,
            loc,
            fns,
            globals,

            predicate,
            body
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<(ProvidedFunctionality, Fn)> {
        vec![]
    }

    fn get_provided_globals(_name: &String) -> HashMap<String, (ProvidedFunctionality, Global)> {
        HashMap::new()
    }

    /// Get the Probe names that match the passed glob pattern
    pub fn get_matches(provided_probes: &ProvidedProbes, provider: &str, package: &str, event: &str, probe_patt: &str) -> Vec<(ProvidedFunctionality, String)> {
        let glob = Pattern::new(&probe_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for (info, p_name) in provided_probes.get(provider).unwrap().1.get(package).unwrap().1.get(event).unwrap().1.iter() {
            if glob.matches(&p_name.to_lowercase()) {
                matches.push((info.clone(), p_name.clone()));
            }
        }

        matches
    }
}

// =====================
// ---- Expressions ----
// =====================

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Op {
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
    fn visit_whammy(&mut self, whammy: &Whammy) -> T;
    fn visit_provider(&mut self, provider: &Provider) -> T;
    fn visit_package(&mut self, package: &Package) -> T;
    fn visit_event(&mut self, event: &Event) -> T;
    fn visit_probe(&mut self, probe: &Probe) -> T;
    // fn visit_predicate(&mut self, predicate: &Expr) -> T;
    fn visit_fn(&mut self, f: &Fn) -> T;
    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> T;
    fn visit_stmt(&mut self, stmt: &Statement) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_op(&mut self, op: &Op) -> T;
    fn visit_datatype(&mut self, datatype: &DataType) -> T;
    fn visit_value(&mut self, val: &Value) -> T;
}

/// To support setting constant-provided global vars
pub trait WhammVisitorMut<T> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> T;
    fn visit_whammy(&mut self, whammy: &mut Whammy) -> T;
    fn visit_provider(&mut self, provider: &mut Provider) -> T;
    fn visit_package(&mut self, package: &mut Package) -> T;
    fn visit_event(&mut self, event: &mut Event) -> T;
    fn visit_probe(&mut self, probe: &mut Probe) -> T;
    // fn visit_predicate(&mut self, predicate: &mut Expr) -> T;
    fn visit_fn(&mut self, f: &mut Fn) -> T;
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> T;
    fn visit_stmt(&mut self, stmt: &mut Statement) -> T;
    fn visit_expr(&mut self, expr: &mut Expr) -> T;
    fn visit_op(&mut self, op: &mut Op) -> T;
    fn visit_datatype(&mut self, datatype: &mut DataType) -> T;
    fn visit_value(&mut self, val: &mut Value) -> T;
}
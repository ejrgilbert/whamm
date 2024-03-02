use std::str::FromStr;

use pest_derive::Parser;
// use pest::error::Error;
// use pest::Parser;
// use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;

#[derive(Parser)]
#[grammar = "./parser/dtrace.pest"] // Path relative to base `src` dir
pub struct DtraceParser;

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

#[derive(Debug, Clone)]
pub enum AstNode {
    // IDs
    VarId {
        name: String,
    },
    ProbeId {
        name: String,
    },

    // Values
    Integer {
        val: i32,
    },
    Str {
        val: String,
    },

    // Expressions
    // Rust doesn't allow unboxed recursive types -- https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
    BinOp {
        lhs: Box<AstNode>, // Should be INT, ID, STR, or BINOP
        op: Op,
        rhs: Box<AstNode>, // Should be INT, ID, STR, or BINOP
    },

    // Probes
    DfinityProbe {
        module: String,
        function: String,
        name: DfinityProbeName,
        predicate: Option<Box<AstNode>>,
        body: Option<Vec<Box<AstNode>>>
    },
    CoreProbe {
        name: CoreProbeName,
        body: Option<Vec<Box<AstNode>>>
    },

    Spec {
        provider: Box<AstNode>, // Should be ProbeIds
        module: Box<AstNode>,
        function: Box<AstNode>,
        name: Box<AstNode>
    },

    // Dscript
    Dscript {
        probes: Vec<Box<AstNode>>
    },

    // EOI because it's an easier workaround than hiding the dscript rule
    EOI,
}

// =============
// = Providers =
// =============

// ** Dfinity Provider **

#[derive(Clone, Debug, Eq, Hash)]
pub enum DfinityProbeName {
    Before,
    After,
    Alt
}

impl PartialEq for DfinityProbeName {
    #[inline]
    fn eq(&self, other: &DfinityProbeName) -> bool {
        match *self {
            DfinityProbeName::Before => match other {
                DfinityProbeName::Before => true,
                _ => false,
            },
            DfinityProbeName::After => match other {
                DfinityProbeName::After => true,
                _ => false,
            },
            DfinityProbeName::Alt => match other {
                DfinityProbeName::Alt => true,
                _ => false,
            },
        }
    }

    #[inline]
    fn ne(&self, other: &DfinityProbeName) -> bool {
        match *self {
            DfinityProbeName::Before => match other {
                DfinityProbeName::Before => false,
                _ => true,
            },
            DfinityProbeName::After => match other {
                DfinityProbeName::After => false,
                _ => true,
            },
            DfinityProbeName::Alt => match other {
                DfinityProbeName::Alt => false,
                _ => true,
            },
        }
    }
}

impl FromStr for DfinityProbeName {
    type Err = ();

    fn from_str(input: &str) -> Result<DfinityProbeName, ()> {
        match input.to_uppercase().as_str() {
            "BEFORE" => Ok(DfinityProbeName::Before),
            "AFTER" => Ok(DfinityProbeName::After),
            "ALT" => Ok(DfinityProbeName::Alt),
            _ => Err(()),
        }
    }
}

impl ToString for DfinityProbeName {
    fn to_string(&self) -> String {
        match *self {
            DfinityProbeName::Before => "Before".to_string(),
            DfinityProbeName::After => "After".to_string(),
            DfinityProbeName::Alt => "Alt".to_string(),
        }
    }
}

// ** Core Provider **

#[derive(Clone, Debug, Eq, Hash)]
pub enum CoreProbeName {
    Begin,
    End
}

impl PartialEq for CoreProbeName {
    #[inline]
    fn eq(&self, other: &CoreProbeName) -> bool {
        match *self {
            CoreProbeName::Begin => match other {
                CoreProbeName::Begin => true,
                _ => false,
            },
            CoreProbeName::End => match other {
                CoreProbeName::End => true,
                _ => false,
            }
        }
    }

    #[inline]
    fn ne(&self, other: &CoreProbeName) -> bool {
        match *self {
            CoreProbeName::Begin => match other {
                CoreProbeName::Begin => false,
                _ => true,
            },
            CoreProbeName::End => match other {
                CoreProbeName::End => false,
                _ => true,
            }
        }
    }
}

impl FromStr for CoreProbeName {
    type Err = ();

    fn from_str(input: &str) -> Result<CoreProbeName, ()> {
        match input.to_uppercase().as_str() {
            "BEGIN" => Ok(CoreProbeName::Begin),
            "END" => Ok(CoreProbeName::End),
            _ => Err(()),
        }
    }
}

impl ToString for CoreProbeName {
    fn to_string(&self) -> String {
        match *self {
            CoreProbeName::Begin => "Begin".to_string(),
            CoreProbeName::End => "End".to_string(),
        }
    }
}

// =====================
// ---- Expressions ----
// =====================

#[derive(Debug, Clone)]
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

impl Op {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Op::And => "and, &&",
            Op::Or => "or, ||",
            Op::EQ => "eq, ==",
            Op::NE => "ne, !=",
            Op::GE => "ge, >=",
            Op::GT => "gt, >",
            Op::LE => "le, <=",
            Op::LT => "lt, <",
            Op::Add => "add, +",
            Op::Subtract => "subtract, -",
            Op::Multiply => "multiply, *",
            Op::Divide => "divide, /",
            Op::Modulo => "modulo, %",
        }
    }
}

use pest_derive::Parser;
use pest::error::Error;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;

use log::{debug, error, trace};
use std::cmp;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "./parser/dtrace.pest"] // Path relative to base `src` dir
struct DtraceParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
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
    fn as_str(&self) -> &'static str {
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

// ====================
// = AST Constructors =
// ====================

fn get_ast_from_expr(pairs: Pairs<Rule>) -> AstNode {
    PRATT_PARSER
        .map_primary(|primary| {
            get_ast_from_pair(primary)
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                // Logical operators
                Rule::and => Op::And,
                Rule::or => Op::Or,

                // Relational operators
                Rule::eq => Op::EQ,
                Rule::ne => Op::NE,
                Rule::ge => Op::GE,
                Rule::gt => Op::GT,
                Rule::le => Op::LE,
                Rule::lt => Op::LT,

                // Highest precedence arithmetic operators
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,

                // Next highest precedence arithmetic operators
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            AstNode::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

fn get_ast_from_pair(pair: Pair<Rule>) -> AstNode {
    // TODO -- implement some type of logging config (these println's should be debugs)
    trace!("Entered get_ast_from_pair");
    match pair.as_rule() {
        Rule::dscript => {
            trace!("Entering dscript");
            let probes = pair.into_inner().map(get_ast_from_pair).filter(|res| match res {
                AstNode::EOI => false,
                _ => true,
            }).map(|res| {
                Box::new(res)
            }).collect();

            trace!("Exiting dscript");
            AstNode::Dscript {
                probes
            }
        }
        Rule::probe_def => {
            trace!("Entering probe_def");
            let mut pair = pair.into_inner();
            let spec = pair.next().unwrap();
            let mut base_probe = get_ast_from_pair(spec);

            let next = pair.next().unwrap();
            let (this_predicate, mut this_body) = match next.as_rule() {
                Rule::predicate => (Some(Box::new(get_ast_from_expr(next.into_inner()))), None),
                Rule::statement => (None, Some(next.into_inner().map(get_ast_from_pair).map(|res| {
                    Box::new(res)
                }).collect())),
                _ => { (None, None) },
            };

            if this_body.is_none() {
                this_body = match pair.next() {
                    Some(b) => {
                        Some(b.into_inner().map(get_ast_from_pair).map(|res| {
                            Box::new(res)
                        }).collect())
                    },
                    None => None
                };
            }

            if let AstNode::CoreProbe{name: _, ref mut body} = base_probe {
                if !this_predicate.is_none() {
                    error!("Core probe should not have a predicate, ignoring.");
                }
                *body = this_body;
            } else if let AstNode::DfinityProbe{module: _, function: _, name: _, ref mut predicate, ref mut body} = base_probe {
                *predicate = this_predicate;
                *body = this_body;
            } else {
                error!("Expected Core or Dfinity probe, received: {:?}", base_probe)
            }

            trace!("Exiting probe_def");
            base_probe
        },
        Rule::spec => {
            trace!("Entering spec");
            let res = get_ast_from_pair(pair.into_inner().next().unwrap());
            trace!("Entering spec");
            res
        }
        Rule::predicate => {
            trace!("Entering predicate");
            let mut pair = pair.into_inner();
            let expr = pair.next().unwrap();

            trace!("Exiting predicate");
            get_ast_from_pair(expr)
        },
        Rule::statement => {
            trace!("Entering statement");
            let res = get_ast_from_expr(pair.into_inner());

            trace!("Exiting statement");
            res
        },
        Rule::expr => {
            trace!("Entering expr");
            let res = get_ast_from_expr(pair.into_inner());

            trace!("Exiting expr");
            res
        },
        Rule::operand => {
            trace!("Entering operand");
            let res = get_ast_from_expr(pair.into_inner());

            trace!("Exiting operand");
            res
        },
        Rule::ID => {
            trace!("Entering ID");

            trace!("Exiting ID");
            AstNode::VarId {
                name: pair.as_str().parse().unwrap()
            }
        },
        Rule::PROBE_ID => {
            trace!("Entering PROBE_ID");
            let name: String = pair.as_str().parse().unwrap();

            // Special BEGIN/END case
            if name.to_uppercase().as_str() == "BEGIN" || name.to_uppercase().as_str() == "END" {
                return AstNode::CoreProbe {
                    name: CoreProbeName::from_str(&*name).unwrap(),
                    body: None,
                }
            }

            trace!("Exiting PROBE_ID");
            AstNode::ProbeId {
                name
            }
        },
        Rule::PROBE_SPEC => {
            trace!("Entering PROBE_SPEC");
            let mut parts = pair.into_inner();
            let mut spec_as_str = parts.as_str();

            let mut contents: Vec<String> = vec![];

            for _ in 0..4 {
                let res = match parts.next() {
                    Some(part) => {
                        if let AstNode::ProbeId {ref name} = get_ast_from_pair(part) {
                            spec_as_str = spec_as_str.strip_prefix(name).unwrap();
                            name.to_string()
                        } else {
                            "*".to_string()
                        }
                    }
                    None => {
                        break;
                    }
                };
                contents.push(res);
                while spec_as_str.starts_with("::") {
                    contents.push("*".to_string());
                    spec_as_str = spec_as_str.strip_prefix(":").unwrap();
                }
                if spec_as_str.starts_with(":") {
                    spec_as_str = spec_as_str.strip_prefix(":").unwrap();
                }
            }

            while contents.len() < 4 {
                contents.insert(0, "*".to_string());
            }

            let this_name = contents.pop().unwrap();
            let this_function = contents.pop().unwrap();
            let this_module = contents.pop().unwrap();
            let this_provider = contents.pop().unwrap();

            let base_probe = match this_provider.to_uppercase().as_str() {
                "DFINITY" => AstNode::DfinityProbe {
                    module: this_module,
                    function: this_function,
                    name: DfinityProbeName::from_str(&this_name).unwrap(),
                    predicate: None,
                    body: None,
                },
                "CORE" => AstNode::CoreProbe {
                    name: CoreProbeName::from_str(&this_name).unwrap(),
                    body: None,
                } ,
                n => unreachable!("Only dfinity and core providers are supported, received: {:?}", n)
            };

            trace!("Exiting PROBE_SPEC");
            base_probe
        },
        Rule::INT => {
            trace!("Entering INT");

            trace!("Exiting INT");
            AstNode::Integer {
                val: pair.as_str().parse::<i32>().unwrap()
            }
        },
        Rule::STRING => {
            trace!("Entering STRING");
            let mut val: String = pair.as_str().parse().unwrap();
            if val.starts_with("\"") {
                val = val.strip_prefix("\"").expect("Should never get here...").to_string();
            }
            if val.ends_with("\"") {
                val = val.strip_suffix("\"").expect("Should never get here...").to_string();
            }

            trace!("Exiting STRING");
            AstNode::Str {
                val
            }
        },
        Rule::EOI => {
            AstNode::EOI
        }
        rule => unreachable!("Unexpected rule, found {:?}", rule)
    }
}

pub fn to_ast(pair: Pair<Rule>) -> Result<Vec<AstNode>, Error<Rule>> {
    trace!("Entered to_ast");
    let mut ast = vec![];
    match pair.as_rule() {
        Rule::dscript => {
            trace!("Starting Rule::dscript");
            match get_ast_from_pair(pair) {
                AstNode::EOI => {}
                res => {
                    ast.push(res);
                }
            }
            trace!("Ending Rule::dscript");
        }
        rule => unreachable!("Expected dscript, found {:?}", rule)
    }

    Ok(ast)
}

// ================
// = AST Visitors =
// ================

fn increase_indent(i: i32) -> i32 {
    i + 1
}

fn decrease_indent(i: i32) -> i32 {
    i - 1
}

fn get_indent(i: i32) -> String {
    "--".repeat(cmp::max(0, i as usize))
}

fn dump(node: AstNode, mut indent: i32) -> (String, i32) {
    let nl: &str = "\n";

    match node {
        AstNode::VarId { name } => {
            let mut s = get_indent(indent);
            s += &*format!("VarId: {name}{nl}");
            (s, indent)
        }
        AstNode::ProbeId { name } => {
            let mut s = get_indent(indent);
            s += &*format!("ProbeId: {name}{nl}");
            (s, indent)
        }
        AstNode::Integer { val } => {
            let mut s = get_indent(indent);
            s += &*format!("Int: {val}{nl}");
            (s, indent)
        }
        AstNode::Str { val } => {
            let mut s = get_indent(indent);
            s += &*format!("Str: {val}{nl}");
            (s, indent)
        }
        AstNode::BinOp { lhs, op, rhs } => {
            let mut s = get_indent(indent);

            s += &*("left:".to_owned() + &*nl);
            indent = increase_indent(indent);
            s += &*dump(*lhs, indent).0;
            indent = decrease_indent(indent);

            s += &*(get_indent(indent) + "operator: " + op.as_str() + &*nl);

            s += &*(get_indent(indent) + "right:" + &*nl);
            indent = increase_indent(indent);
            s += &*dump(*rhs, indent).0;
            indent = decrease_indent(indent);
            (s, indent)
        }
        AstNode::Spec { provider, module, function, name } => {
            let mut s = get_indent(indent) + "ProbeSpec:" + &*nl;
            indent = increase_indent(indent);

            // provider
            s += &*(get_indent(indent) + "provider:" + &*nl);
            indent = increase_indent(indent);
            s += &*dump(*provider, indent).0;
            indent = decrease_indent(indent);

            // module
            s += &*(get_indent(indent) + "module:" + &*nl);
            indent = increase_indent(indent);
            s += &*dump(*module, indent).0;
            indent = decrease_indent(indent);

            // function
            s += &*(get_indent(indent) + "function:" + &*nl);
            indent = increase_indent(indent);
            s += &*dump(*function, indent).0;
            indent = decrease_indent(indent);

            // name
            s += &*(get_indent(indent) + "name:" + &*nl);
            indent = increase_indent(indent);
            s += &*dump(*name, indent).0;
            indent = decrease_indent(indent);

            indent = decrease_indent(indent);
            (s, indent)
        }
        AstNode::DfinityProbe {module, function, name, predicate, body} => {
            let mut s = get_indent(indent) + "DfinityProbe:" + &*nl;
            indent = increase_indent(indent);

            // spec
            s += &*(get_indent(indent) + "module: " + &*module + &*nl);
            s += &*(get_indent(indent) + "function: " + &*function + &*nl);
            s += &*(get_indent(indent) + "name: " + &*name.to_string() + &*nl);

            // predicate
            match predicate {
                Some(pred) => {
                    s += &*(get_indent(indent) + "predicate:" + &*nl);
                    indent = increase_indent(indent);
                    s += &*dump(*pred, indent).0;
                    indent = decrease_indent(indent);
                }
                None => {}
            };

            // body
            match body {
                Some(b) => {
                    s += &*(get_indent(indent) + "body:" + &*nl);
                    indent = increase_indent(indent);
                    for stmt in b {
                        s += &*dump(*stmt, indent).0;
                    }
                    indent = decrease_indent(indent);
                }
                None => {}
            }

            indent = decrease_indent(indent);
            (s, indent)
        },
        AstNode::CoreProbe {name, body} => {

            let mut s = get_indent(indent) + "CoreProbe:" + &*nl;
            indent = increase_indent(indent);

            // spec
            s += &*(get_indent(indent) + "name: " + &*name.to_string() + &*nl);

            // body
            match body {
                Some(b) => {
                    s += &*(get_indent(indent) + "body:" + &*nl);
                    indent = increase_indent(indent);
                    for stmt in b {
                        s += &*dump(*stmt, indent).0;
                    }
                    indent = decrease_indent(indent);
                }
                None => {}
            }

            indent = decrease_indent(indent);
            (s, indent)
        }
        AstNode::Dscript { probes } => {
            let mut s = "".to_string();
            s += &*nl;
            for probe in probes {
                indent = increase_indent(indent);
                s += &*("ProbeDef".to_owned() + &*nl);
                s += &*dump(*probe, indent).0;
                indent = decrease_indent(indent);
            }
            (s, indent)
        }
        AstNode::EOI => ("".to_string(), indent)
    }
}

pub fn dump_ast(ast: Vec<AstNode>) {
    let indent = 0;
    for node in ast {
        let (res, _indent) = dump(node, indent);
        println!("{}", res);
    }
}

// ==========
// = Parser =
// ==========

pub fn parse_script(script: String) -> Result<Vec<AstNode>, String> {
    trace!("Entered parse_script");

    match DtraceParser::parse(Rule::dscript, &*script) {
        Ok(mut pairs) => {
            let res = to_ast(
                // inner of script
                pairs.next().unwrap()
            );
            debug!("Parsed: {:#?}", res);

            match res {
                Ok(ast) => Ok(ast),
                Err(e) => Err(e.to_string()),
            }
        },
        Err(e) => {
            Err(e.to_string())
        },
    }
}


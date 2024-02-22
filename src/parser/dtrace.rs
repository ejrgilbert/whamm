use pest_derive::Parser;
use pest::error::Error;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;

use std::cmp;
use std::os::unix::raw::gid_t;
use std::string::ToString;

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
            ).op(Op::infix(add, Left) | Op::infix(subtract, Left)) //SUMOP
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left)) // MULOP
    };
}

#[derive(Debug)]
pub enum AstNode {
    // IDs
    // TODO -- verify these with regex
    VarId{
        name: String,
    },
    ProbeId{
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
    Spec {
        provider: String,
        module: String,
        function: String,
        name: String
    },
    Probe {
        spec: Box<AstNode>,
        predicate: Option<Box<AstNode>>,
        body: Option<Vec<Box<AstNode>>>
    },

    // Dscript
    Dscript {
        probes: Vec<Box<AstNode>>
    },

    // TODO -- add '_' to 'dscript' rule to not need this EOI variant
    // EOI because I don't know what to do
    // Don't add this to the AST
    EOI,
}

// =====================
// ---- Expressions ----
// =====================

#[derive(Debug)]
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
            // AstNode::BinOp {
            //     lhs: Box::new(AstNode::Integer {
            //         val: 0
            //     }),
            //     op,
            //     rhs: Box::new(AstNode::Integer {
            //         val: 0
            //     }),
            // }
        })
        .parse(pairs)
}

fn get_ast_from_pair(pair: Pair<Rule>) -> AstNode {
    println!("Entered get_ast_from_pair");
    match pair.as_rule() {
        Rule::dscript => {
            println!("Entering dscript");
            let probes = pair.into_inner().map(get_ast_from_pair).filter(|res| match res {
                AstNode::EOI => false,
                _ => true,
            }).map(|res| {

                Box::new(res)
            }).collect();

            println!("Exiting dscript");
            AstNode::Dscript {
                probes
            }
        }
        Rule::probe_def => {
            println!("Entering probe_def");
            let mut pair = pair.into_inner();
            let spec = pair.next().unwrap();
            let spec = Box::new(get_ast_from_pair(spec));

            let predicate = match pair.next() {
                Some(pred) => {
                    Some(Box::new(get_ast_from_expr(pred.into_inner())))
                },
                None => None,
            };

            let body = match pair.next() {
                Some(b) => {
                    Some(b.into_inner().map(get_ast_from_pair).map(|res| {
                        Box::new(res)
                    }).collect())
                },
                None => None
            };

            println!("Exiting probe_def");
            AstNode::Probe {
                spec,
                predicate,
                body
            }
        },
        Rule::predicate => {
            println!("Entering predicate");
            let mut pair = pair.into_inner();
            let expr = pair.next().unwrap();

            println!("Exiting predicate");
            get_ast_from_pair(expr)
        },
        Rule::spec => {
            // TODO
            println!("Entering spec");

            println!("Exiting spec");
            AstNode::Integer {
                val: 0
            }
        },
        Rule::statement => {
            println!("Entering statement");
            let res = get_ast_from_expr(pair.into_inner());

            println!("Exiting statement");
            res
        },
        Rule::expr => {
            println!("Entering expr");
            let res = get_ast_from_expr(pair.into_inner());

            println!("Exiting expr");
            res
        },
        Rule::operand => {
            println!("Entering operand");
            let res = get_ast_from_expr(pair.into_inner());

            println!("Exiting operand");
            res
        },
        Rule::ID => {
            println!("Entering ID");

            println!("Exiting ID");
            AstNode::VarId {
                name: pair.as_str().parse().unwrap()
            } // TODO -- this could need to be a ProbeId since `ID` occurs inside `spec`
        },
        Rule::PROBE_ID => {
            println!("Entering PROBE_ID");

            println!("Exiting PROBE_ID");
            AstNode::ProbeId {
                name: pair.as_str().parse().unwrap()
            }
        },
        Rule::PROBE_SPEC => {
            // TODO
            println!("Entering PROBE_SPEC");

            println!("Exiting PROBE_SPEC");
            AstNode::Integer {
                val: 0
            }
        },
        Rule::INT => {
            println!("Entering INT");

            println!("Exiting INT");
            AstNode::Integer {
                val: pair.as_str().parse::<i32>().unwrap()
            }
        },
        Rule::STRING => {
            println!("Entering STRING");

            println!("Exiting STRING");
            AstNode::Str {
                val: pair.as_str().parse().unwrap()
            }
        },
        Rule::EOI => {
            AstNode::EOI
        }
        rule => unreachable!("Unexpected rule, found {:?}", rule)
    }
}

pub fn to_ast(pair: Pair<Rule>) -> Result<Vec<AstNode>, Error<Rule>> {
    println!("Entered to_ast");
    let mut ast = vec![];
    match pair.as_rule() {
        Rule::dscript => {
            println!("Starting Rule::dscript");
            match get_ast_from_pair(pair) {
                AstNode::EOI => {}
                res => {
                    ast.push(res);
                }
            }
            println!("Ending Rule::dscript");
        }
        rule => unreachable!("Expected dscript, found {:?}", rule)
    }

    // for pair in pairs {
    //
    // }

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
            s += &*provider;
            indent = decrease_indent(indent);

            // module
            s += &*(get_indent(indent) + "module:" + &*nl);
            indent = increase_indent(indent);
            s += &*module;
            indent = decrease_indent(indent);

            // function
            s += &*(get_indent(indent) + "function:" + &*nl);
            indent = increase_indent(indent);
            s += &*function;
            indent = decrease_indent(indent);

            // name
            s += &*(get_indent(indent) + "name:" + &*nl);
            indent = increase_indent(indent);
            s += &*name;
            indent = decrease_indent(indent);

            indent = decrease_indent(indent);
            (s, indent)
        }
        AstNode::Probe { spec, predicate, body } => {
            let mut s = get_indent(indent) + "Probe:" + &*nl;
            indent = increase_indent(indent);

            // spec
            s += &*(get_indent(indent) + "spec:" + &*nl);
            indent = increase_indent(indent);
            s += &*dump(*spec, indent).0;
            indent = decrease_indent(indent);

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

pub fn parse_script(script: &str) -> Result<Vec<AstNode>, String> {
    println!("Entered parse_script");

    match DtraceParser::parse(Rule::dscript, script) {
        Ok(mut pairs) => {
            let res = to_ast(
                // inner of script
                pairs.next().unwrap()
            );
            println!("Parsed: {:#?}", res);

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


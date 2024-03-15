use crate::parser::types;
use types::{AstNode, CoreProbeName, DtraceParser, Op, PRATT_PARSER, Rule};

use pest::error::Error;
use pest::Parser;
use pest::iterators::{Pair, Pairs};

use log::{debug, error, trace};
use std::cmp;
use std::str::FromStr;
use crate::parser::types::WasmProbeName;

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
                probes,
                // id: None
            }
        }
        Rule::probe_def => {
            trace!("Entering probe_def");
            let mut pair = pair.into_inner();
            let spec = pair.next().unwrap();
            let mut base_probe = get_ast_from_pair(spec);

            let next = pair.next();
            let (this_predicate, this_body) = match next {
                Some(n) => {
                    let (this_predicate, mut this_body) = match n.as_rule() {
                        Rule::predicate => (Some(Box::new(get_ast_from_expr(n.into_inner()))), None),
                        Rule::statement => (None, Some(n.into_inner().map(get_ast_from_pair).map(|res| {
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

                    (this_predicate, this_body)
                },
                None => (None, None)
            };

            if let AstNode::CoreProbe{name: _, ref mut body} = base_probe {
                if !this_predicate.is_none() {
                    error!("Core probe should not have a predicate, ignoring.");
                }
                *body = this_body;
            } else if let AstNode::WasmProbe{module: _, function: _, name: _, ref mut predicate, ref mut body} = base_probe {
                *predicate = this_predicate;
                *body = this_body;
            }else {
                error!("Expected Core or Wasm probe, received: {:?}", base_probe)
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
        Rule::assignment => {
            trace!("Entering assignment");
            let mut pair = pair.into_inner();
            let var_id_rule = pair.next().unwrap();
            let expr_rule = pair.next().unwrap().into_inner();

            let var_id = Box::new(get_ast_from_pair(var_id_rule));
            let expr = Box::new(get_ast_from_expr(expr_rule));
            trace!("Exiting assignment");

            AstNode::Assign {
                var_id,
                expr,
            }
        },
        Rule::fn_call => {
            trace!("Entering fn_call");
            let mut pair = pair.into_inner();
            
            // handle fn target
            let fn_rule = pair.next().unwrap();
            let fn_target = Box::new(get_ast_from_pair(fn_rule));
            
            // handle args
            let mut next = pair.next();
            let mut init = vec!();
            while next.is_some() {
                let mut others = next.unwrap().into_inner().map(get_ast_from_pair).map(|res| {
                    Box::new(res)
                }).collect();
                init.append(&mut others);
                next = pair.next();
            };
            let args = if init.len() > 0 {
                Some(init)
            } else {
                None
            };

            trace!("Exiting fn_call");
            AstNode::Call {
                fn_target,
                args
            }
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
        Rule::tuple => {
            trace!("Entering tuple");
            let val_rules = pair.into_inner();

            // handle vals
            let vals = val_rules.map(get_ast_from_pair).map(|res| {
                Box::new(res)
            }).collect();

            trace!("Exiting tuple");
            AstNode::Tuple {
                vals
            }
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
                "WASM" => AstNode::WasmProbe {
                    module: this_module,
                    function: this_function,
                    name: WasmProbeName::from_str(&this_name).unwrap(),
                    predicate: None,
                    body: None,
                },
                "CORE" => AstNode::CoreProbe {
                    name: CoreProbeName::from_str(&this_name).unwrap(),
                    body: None,
                } ,
                n => unreachable!("Only wasm and core providers are supported, received: {:?}", n)
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
        AstNode::Integer { val } => {
            let mut s = get_indent(indent);
            s += &format!("Int: {val}{nl}");
            (s, indent)
        }
        AstNode::Str { val } => {
            let mut s = get_indent(indent);
            s += &format!("Str: {val}{nl}");
            (s, indent)
        }
        AstNode::Tuple { vals } => {
            let mut s = get_indent(indent);
            s += &format!("Tuple: ({nl}");
            indent = increase_indent(indent);
            for val in vals {
                s += &format!("{}", &*dump(*val, indent).0);
            }
            indent = decrease_indent(indent);
            s += &format!("{}){nl}", get_indent(indent));
            (s, indent)
        }
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
        AstNode::Assign { var_id, expr } => {
            let mut s = get_indent(indent);
            s += &*format!("Assign:{nl}");
            indent = increase_indent(indent);
            s += &format!("{}var_id: {nl}{}", get_indent(indent), &*dump(*var_id, increase_indent(indent)).0);
            s += &format!("{}expr:{nl}{}", get_indent(indent), &*dump(*expr, increase_indent(indent)).0);
            indent = decrease_indent(indent);
            (s, indent)
        }
        AstNode::Call { fn_target, args } => {
            let mut s = get_indent(indent);
            s += &format!("Call:{nl}");
            indent = increase_indent(indent);

            s += &format!("{}function target:{nl}", get_indent(indent));
            s += &format!("{}", &*dump(*fn_target, increase_indent(indent)).0);

            if args.is_some() {
                let mut i = 0;
                for arg in args.unwrap() {
                    s += &format!("{}arg{i}:{nl}", get_indent(indent));
                    s += &format!("{}", &*dump(*arg, increase_indent(indent)).0);
                    i += 1;
                }
            }

            indent = decrease_indent(indent);
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
        },
        AstNode::WasmProbe {module, function, name, predicate, body} => {
            let mut s = get_indent(indent) + "WasmProbe:" + &*nl;
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


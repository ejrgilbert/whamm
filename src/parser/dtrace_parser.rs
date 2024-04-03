use crate::parser::types;
use types::{DtraceParser, Op, PRATT_PARSER, Rule};

use pest::error::Error;
use pest::Parser;
use pest::iterators::{Pair, Pairs};

use log::{trace};
use crate::parser::types::{DataType, Dscript, Dtrace, Expr, Statement, Value};

// ====================
// = AST Constructors =
// ====================

pub fn to_ast(pair: Pair<Rule>) -> Result<Dtrace, Error<Rule>> {
    trace!("Entered to_ast");

    // Create initial AST with Dtrace node
    let mut dtrace = Dtrace::new();
    let dscript_count = 0;

    match pair.as_rule() {
        Rule::dscript => {
            process_pair(&mut dtrace, dscript_count, pair);
        }
        rule => unreachable!("Expected dscript, found {:?}", rule)
    }

    Ok(dtrace)
}

fn process_pair(dtrace: &mut Dtrace, dscript_count: usize, pair: Pair<Rule>) {
    trace!("Entered process_pair");
    match pair.as_rule() {
        Rule::dscript => {
            trace!("Entering dscript");
            let base_dscript = Dscript::new();
            let id = dtrace.add_dscript(base_dscript);
            pair.into_inner().for_each(| p | {
                process_pair(dtrace, id, p);
            });
            trace!("Exiting dscript");
        }
        Rule::probe_def => {
            trace!("Entering probe_def");
            let mut pair = pair.into_inner();
            let spec_rule = pair.next().unwrap();
            let spec = probe_spec_from_rule(spec_rule);
            let mut spec_split = spec.split(":");

            // Get out the spec info
            let provider = spec_split.next().unwrap();
            let module = spec_split.next().unwrap();
            let function = spec_split.next().unwrap();
            let name = spec_split.next().unwrap();

            // Get out the probe predicate/body contents
            let next = pair.next();
            let (this_predicate, this_body) = match next {
                Some(n) => {
                    let (this_predicate, mut this_body) = match n.as_rule() {
                        Rule::predicate => (Some(expr_from_pairs(n.into_inner())), None),
                        Rule::statement => {
                            let mut stmts = vec![];
                            n.into_inner().for_each(|p| {
                                stmts.push(stmt_from_rule(p));
                            });
                            (None, Some(stmts))
                        },
                        _ => { (None, None) },
                    };

                    if this_body.is_none() {
                        this_body = match pair.next() {
                            Some(b) => {
                                let mut stmts = vec![];

                                b.into_inner().for_each(|p| {
                                    stmts.push(stmt_from_rule(p));
                                });
                                Some(stmts)
                            },
                            None => None
                        };
                    }

                    (this_predicate, this_body)
                },
                None => (None, None)
            };

            // Add probe definition to the dscript
            let dscript: &mut Dscript = dtrace.dscripts.get_mut(dscript_count).unwrap();
            dscript.add_probe(&dtrace.provided_probes, provider, module, function, name, this_predicate, this_body);

            trace!("Exiting probe_def");
        },
        Rule::EOI => {},
        rule => unreachable!("Unexpected rule in process_pair, found {:?}", rule)
    }
}

fn fn_call_from_rule(pair: Pair<Rule>) -> Expr {
    trace!("Entering fn_call");
    // This has to be duplicated due to the Expression/Statement masking as the function return type
    let mut pair = pair.into_inner();

    // handle fn target
    let fn_rule = pair.next().unwrap();
    let fn_target = Expr::VarId {
        name: fn_rule.as_str().parse().unwrap()
    };

    // handle args
    let mut next = pair.next();
    let mut init = vec!();
    while next.is_some() {
        let mut others = vec!();
        others.push(Box::new(expr_from_pairs(next.unwrap().into_inner())));
        init.append(&mut others);
        next = pair.next();
    };
    let args = if init.len() > 0 {
        Some(init)
    } else {
        None
    };

    trace!("Exiting fn_call");

    Expr::Call {
        fn_target: Box::new(fn_target),
        args
    }
}

fn stmt_from_rule(pair: Pair<Rule>) -> Statement {
    trace!("Entered stmt_from_rule");
    match pair.as_rule() {
        Rule::statement => {
            trace!("Entering statement");
            let res = stmt_from_rule(pair);

            trace!("Exiting statement");
            trace!("Exiting stmt_from_rule");
            return res;
        },
        Rule::assignment => {
            trace!("Entering assignment");
            let mut pair = pair.into_inner();
            let var_id_rule = pair.next().unwrap();
            let expr_rule = pair.next().unwrap().into_inner();

            let var_id = Expr::VarId {
                name: var_id_rule.as_str().parse().unwrap()
            };
            let expr = expr_from_pairs(expr_rule);
            trace!("Exiting assignment");
            trace!("Exiting stmt_from_rule");

            return Statement::Assign {
                var_id,
                expr,
            };
        },
        Rule::fn_call => {
            let call = fn_call_from_rule(pair);
            trace!("Exiting stmt_from_rule");

            return Statement::Expr {
                expr: call
            };
        },
        rule => unreachable!("Expected statement, assignment, or fn_call, found {:?}", rule)
    }
}

fn probe_spec_from_rule(pair: Pair<Rule>) -> String {
    trace!("Entered probe_spec_from_rule");
    match pair.as_rule() {
        Rule::PROBE_ID => {
            trace!("Entering PROBE_ID");
            let name: String = pair.as_str().parse().unwrap();
            trace!("Exiting PROBE_ID");

            trace!("Exiting probe_spec_from_rule");
            return name
        },
        Rule::PROBE_SPEC => {
            trace!("Entering PROBE_SPEC");
            let spec_as_str = pair.as_str();
            let mut parts = pair.into_inner();

            let str_parts = spec_as_str.split(":");

            let mut contents: Vec<String> = vec![];
            for s in str_parts {
                if s == "" {
                    contents.push("*".to_string());
                    continue;
                }
                let next = parts.next();

                let res = match next {
                    Some(part) => {
                        match part.as_rule() {
                            Rule::PROBE_ID => {
                                probe_spec_from_rule(part)
                            },
                            _ => "*".to_string()
                        }
                    }
                    None => {
                        break;
                    }
                };
                contents.push(res);
            }
            trace!("Exiting PROBE_SPEC");
            trace!("Exiting probe_spec_from_rule");
            if contents.len() == 1 {
                // This is a BEGIN or END probe! Special case
                contents.insert(0, "*".to_string());
                contents.insert(0, "*".to_string());
                contents.insert(0, "core".to_string());
            }

            return contents.join(":")
        },
        rule => unreachable!("Expected spec, PROBE_SPEC, or PROBE_ID, found {:?}", rule)
    }
}

fn expr_primary(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::fn_call => {
            let call = fn_call_from_rule(pair);
            return call;
        },
        Rule::ID => {
            return Expr::VarId {
                name: pair.as_str().parse().unwrap()
            };
        },
        Rule::tuple => {
            trace!("Entering tuple");
            // handle contents
            let vals = pair.into_inner().map(expr_primary).collect();

            trace!("Exiting tuple");
            return Expr::Primitive {
                val: Value::Tuple {
                    ty: DataType::Tuple {ty_info: None},
                    vals
                }
            };
        },
        Rule::INT => {
            trace!("Entering INT");
            let val = pair.as_str().parse::<i32>().unwrap();

            trace!("Exiting INT");
            return Expr::Primitive {
                val: Value::Integer {
                    ty: DataType::Integer,
                    val
                }
            };
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
            return Expr::Primitive {
                val: Value::Str {
                    ty: DataType::Str,
                    val
                }
            };
        },
        _ => expr_from_pairs(pair.into_inner())
    }
}

fn expr_from_pairs(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| -> Expr {
            expr_primary(primary)
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
            return Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        })
        .parse(pairs)
}

// ==========
// = Parser =
// ==========

pub fn parse_script(script: String) -> Result<Dtrace, String> {
    trace!("Entered parse_script");

    match DtraceParser::parse(Rule::dscript, &*script) {
        Ok(mut pairs) => {
            let res = to_ast(
                // inner of script
                pairs.next().unwrap()
            );
            // debug!("Parsed: {:#?}", res);

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


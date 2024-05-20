use std::process::exit;
use crate::parser::types;
use types::{WhammParser, Op, PRATT_PARSER, Rule};

use pest::error::{Error, LineColLocation};
use pest::Parser;
use pest::iterators::{Pair, Pairs};

use log::{trace};
use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::{DataType, Whammy, Whamm, Expr, Statement, Value, Location, ProbeSpec, SpecPart};

const UNEXPECTED_ERR_MSG: &str = "WhammParser: Looks like you've found a bug...please report this behavior! Exiting now...";

pub fn parse_script(script: &String, err: &mut ErrorGen) -> Option<Whamm> {
    trace!("Entered parse_script");
    err.set_script_text(script.to_owned());

    let res = WhammParser::parse(Rule::whammy, &*script);
    match res {
        Ok(mut pairs) => {
            let res = to_ast(
                // inner of script
                pairs.next().unwrap(),
                err
            );

            match res {
                Ok(ast) => {
                    Some(ast)
                },
                Err(e) => {
                    err.pest_err(e);
                    None
                },
            }
        },
        Err(e) => {
            err.pest_err(e);
            None
        },
    }
}

// ====================
// = AST Constructors =
// ====================

pub fn to_ast(pair: Pair<Rule>, err: &mut ErrorGen) -> Result<Whamm, Error<Rule>> {
    trace!("Entered to_ast");

    // Create initial AST with Whamm node
    let mut whamm = Whamm::new();
    let whammy_count = 0;

    match pair.as_rule() {
        Rule::whammy => {
            process_pair(&mut whamm, whammy_count, pair, err);
        }
        rule => {
            err.parse_error(true,
                Some(UNEXPECTED_ERR_MSG.to_string()),
                LineColLocation::from(pair.as_span()),
                vec![Rule::whammy], vec![rule]);
            // should have exited above (since it's a fatal error)
            unreachable!()
        }
    }

    Ok(whamm)
}


// ================
// = Parser Logic =
// ================

pub fn process_pair(whamm: &mut Whamm, whammy_count: usize, pair: Pair<Rule>, err: &mut ErrorGen) {
    trace!("Entered process_pair");
    match pair.as_rule() {
        Rule::whammy => {
            trace!("Entering whammy");
            let base_whammy = Whammy::new();
            let id = whamm.add_whammy(base_whammy);
            pair.into_inner().for_each(| p | {
                process_pair(whamm, id, p, err);
            });
            trace!("Exiting whammy");
        }
        Rule::probe_def => {
            trace!("Entering probe_def");
            let mut pair = pair.into_inner();
            let spec_rule = pair.next().unwrap();
            // Get out the spec info
            let probe_spec = probe_spec_from_rule(spec_rule, err);

            // Get out the probe predicate/body contents
            let next = pair.next();
            let (this_predicate, this_body) = match next {
                Some(n) => {
                    let (this_predicate, mut this_body) = match n.as_rule() {
                        Rule::predicate => {
                            match expr_from_pairs(n.into_inner()) {
                                Ok(res) => (Some(res), None),
                                Err(errors) => {
                                    err.add_errors(errors);
                                    // ignore predicate due to errors
                                    (None, None)
                                }
                            }
                        },
                        Rule::statement => {
                            let mut stmts = vec![];
                            n.into_inner().for_each(|p| {
                                stmts.push(stmt_from_rule(p, err));
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
                                    stmts.push(stmt_from_rule(p, err));
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

            // Add probe definition to the whammy
            let whammy: &mut Whammy = whamm.whammys.get_mut(whammy_count).unwrap();
            match whammy.add_probe(&whamm.provided_probes, &probe_spec, this_predicate, this_body) {
                Err(e) => {
                    err.add_error(e);
                },
                _ => {}
            }

            trace!("Exiting probe_def");
        },
        Rule::EOI => {},
        rule => {
            err.parse_error(true,
                            Some(UNEXPECTED_ERR_MSG.to_string()),
                            LineColLocation::from(pair.as_span()),
                            vec![Rule::whammy, Rule::probe_def, Rule::EOI], vec![rule]);
            // should have exited above (since it's a fatal error)
            unreachable!()
        }
    }
}

fn fn_call_from_rule(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    trace!("Entering fn_call");
    // This has to be duplicated due to the Expression/Statement masking as the function return type
    let mut pair = pair.into_inner();

    // handle fn target
    let fn_rule = pair.next().unwrap();

    let fn_target_line_col = LineColLocation::from(fn_rule.as_span());
    let fn_target = Expr::VarId {
        name: fn_rule.as_str().parse().unwrap(),
        loc: Some(Location {
            line_col: fn_target_line_col.clone(),
            path: None
        })
    };

    // handle args
    let mut next = pair.next();
    let mut init = vec!();
    let mut errors = vec![];
    while next.is_some() {
        let mut others = vec!();
        match expr_from_pairs(next.unwrap().into_inner()) {
            Ok(expr) => {
                others.push(Box::new(expr));
                init.append(&mut others);
            },
            Err(err) => errors.extend(err)
        }

        next = pair.next();
    };
    let args = if init.len() > 0 {
        Some(init)
    } else {
        None
    };

    trace!("Exiting fn_call");
    if !errors.is_empty() {
        return Err(errors);
    }

    let last_arg_loc = if let Some(args) = &args {
        if let Some(last_arg) = args.last() {
            if let Some(last_arg_loc) = last_arg.loc() {
                Some(LineColLocation::from(last_arg_loc.line_col.clone()))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(last_arg_loc) = last_arg_loc {
        Ok(Expr::Call {
            fn_target: Box::new(fn_target),
            args,
            loc: Some(Location::from(&fn_target_line_col, &last_arg_loc, None))
        })
    } else {
        Ok(Expr::Call {
            fn_target: Box::new(fn_target),
            args,
            loc: Some(Location {
                line_col: fn_target_line_col.clone(),
                path: None
            })
        })
    }

}

fn stmt_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> Statement {
    trace!("Entered stmt_from_rule");
    match pair.as_rule() {
        Rule::statement => {
            trace!("Entering statement");
            let res = stmt_from_rule(pair, err);

            trace!("Exiting statement");
            trace!("Exiting stmt_from_rule");
            return res;
        },
        Rule::assignment => {
            trace!("Entering assignment");
            let mut pair = pair.into_inner();
            let var_id_rule = pair.next().unwrap();
            let expr_rule = pair.next().unwrap().into_inner();

            let var_id_line_col = LineColLocation::from(var_id_rule.as_span());

            let var_id = Expr::VarId {
                name: var_id_rule.as_str().parse().unwrap(),
                loc: Some(Location {
                    line_col: var_id_line_col.clone(),
                    path: None
                })
            };

            return match expr_from_pairs(expr_rule) {
                Err(errors) => {
                    err.add_errors(errors);
                    Statement::dummy()
                },
                Ok(expr) => {
                    trace!("Exiting assignment");
                    trace!("Exiting stmt_from_rule");

                    let expr_line_col = if let Some(expr_loc) = expr.loc() {
                        LineColLocation::from(expr_loc.line_col.clone())
                    } else {
                        exit(1);
                    };

                    Statement::Assign {
                        var_id,
                        expr,
                        loc: Some(Location::from(&var_id_line_col, &expr_line_col, None))
                    }
                }
            }
        },
        Rule::fn_call => {
            return match fn_call_from_rule(pair) {
                Err(errors) => {
                    err.add_errors(errors);
                    Statement::dummy()
                },
                Ok(call) => {
                    let call_loc = call.loc().clone();
                    trace!("Exiting stmt_from_rule");

                    Statement::Expr {
                        expr: call,
                        loc: call_loc
                    }
                }
            }
        },
        rule => {
            err.parse_error(true,
                            Some(UNEXPECTED_ERR_MSG.to_string()),
                            LineColLocation::from(pair.as_span()),
                            vec![Rule::statement, Rule::assignment, Rule::fn_call], vec![rule]);
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    }
}

fn probe_spec_part_from_rule(pair: Pair<Rule>, err: &mut ErrorGen)  -> SpecPart {
    trace!("Entered probe_spec_part_from_rule");
    match pair.as_rule() {
        Rule::PROBE_ID => {
            trace!("Entering PROBE_ID");
            let name: String = pair.as_str().parse().unwrap();
            let id_line_col =  LineColLocation::from(pair.as_span());

            let part = SpecPart {
                name,
                loc: Some(Location {
                    line_col: id_line_col,
                    path: None
                })
            };
            trace!("Exiting PROBE_ID");

            trace!("Exiting probe_spec_part_from_rule");
            return part
        },
        rule => {
            err.parse_error(true,
                            Some(UNEXPECTED_ERR_MSG.to_string()),
                            LineColLocation::from(pair.as_span()),
                            vec![Rule::PROBE_ID, Rule::PROBE_ID], vec![rule]);
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    }
}

fn probe_spec_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> ProbeSpec {
    trace!("Entered probe_spec_from_rule");
    match pair.as_rule() {
        Rule::PROBE_SPEC => {
            trace!("Entering PROBE_SPEC");
            let spec_as_str = pair.as_str();
            let mut parts = pair.into_inner();

            if spec_as_str.to_uppercase() == "BEGIN" || spec_as_str.to_uppercase() == "END" {
                // This is a BEGIN or END probe! Special case
                let loc = if let Some(rule) = parts.next() {
                    let id_line_col = LineColLocation::from(rule.as_span());
                    Some(Location {
                        line_col: id_line_col,
                        path: None
                    })
                } else {
                    None
                };

                return ProbeSpec {
                    provider: Some(SpecPart {
                        name: "core".to_string(),
                        loc: loc.clone()
                    }),
                    package: Some(SpecPart {
                        name: "*".to_string(),
                        loc: loc.clone()
                    }),
                    event: Some(SpecPart {
                        name: "*".to_string(),
                        loc: loc.clone()
                    }),
                    mode: Some(SpecPart {
                        name: "BEGIN".to_string(),
                        loc
                    }),
                }
            }

            let str_parts = spec_as_str.split(":");

            let mut probe_spec = ProbeSpec::new();
            let mut contents: Vec<String> = vec![];
            for s in str_parts {
                if s == "" {
                    probe_spec.add_spec_def(SpecPart {
                        name: "*".to_string(),
                        loc: None
                    });
                    contents.push("*".to_string());
                    continue;
                }
                let next = parts.next();

                match next {
                    Some(part) => {
                        match part.as_rule() {
                            Rule::PROBE_ID => {
                                let n = probe_spec_part_from_rule(part, err);
                                probe_spec.add_spec_def(n);

                            },
                            _ => {
                                probe_spec.add_spec_def(SpecPart {
                                    name: "*".to_string(),
                                    loc: None
                                });
                            }
                        }
                    }
                    None => {
                        break;
                    }
                };
            }
            trace!("Exiting PROBE_SPEC");
            trace!("Exiting probe_spec_from_rule");

            return probe_spec
        },
        rule => {
            err.parse_error(true,
                            Some(UNEXPECTED_ERR_MSG.to_string()),
                            LineColLocation::from(pair.as_span()),
                            vec![Rule::PROBE_SPEC], vec![rule]);
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    }
}

fn expr_primary(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    match pair.as_rule() {
        Rule::fn_call => {
            let call = fn_call_from_rule(pair);
            return call;
        },
        Rule::ID => {
            return Ok(Expr::VarId {
                name: pair.as_str().parse().unwrap(),
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None
                })
            });
        },
        Rule::tuple => {
            trace!("Entering tuple");
            // handle contents
            let pair_line_col = LineColLocation::from(pair.as_span());
            let mut vals = vec![];

            for inner in pair.into_inner() {
                match expr_primary(inner) {
                    Ok(expr) => vals.push(expr),
                    other => {
                        return other;
                    }
                }
            }

            trace!("Exiting tuple");
            return Ok(Expr::Primitive {
                val: Value::Tuple {
                    ty: DataType::Tuple {ty_info: None},
                    vals
                },
                loc: Some(Location {
                    line_col: pair_line_col,
                    path: None
                })
            });
        },
        Rule::INT => {
            trace!("Entering INT");
            let val = pair.as_str().parse::<i32>().unwrap();

            trace!("Exiting INT");
            return Ok(Expr::Primitive {
                val: Value::Integer {
                    ty: DataType::Integer,
                    val
                },
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None
                })
            });
        },
        Rule::BOOL => {
            trace!("Entering BOOL");
            let val = pair.as_str().parse::<bool>().unwrap();

            trace!("Exiting BOOL");
            return Ok(Expr::Primitive {
                val: Value::Boolean {
                    ty: DataType::Boolean,
                    val
                },
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None
                })
            });
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
            return Ok(Expr::Primitive {
                val: Value::Str {
                    ty: DataType::Str,
                    val,
                    addr: None
                },
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None
                })
            });
        },
        _ => expr_from_pairs(pair.into_inner())
    }
}

fn expr_from_pairs(pairs: Pairs<Rule>) -> Result<Expr, Vec<WhammError>> {
    PRATT_PARSER
        .map_primary(|primary| -> Result<Expr, Vec<WhammError>> {
            expr_primary(primary)
        })
        .map_infix(|lhs, op, rhs| -> Result<Expr, Vec<WhammError>> {
            return match (lhs, rhs) {
                (Ok(lhs), Ok(rhs)) => {
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
                        rule => {
                            return Err(vec![ErrorGen::get_parse_error(
                                true,
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                LineColLocation::from(op.as_span()),
                                vec![Rule::and, Rule::or, Rule::eq, Rule::ne, Rule::ge, Rule::gt, Rule::le, Rule::lt,
                                        Rule::add, Rule::subtract, Rule::multiply, Rule::divide, Rule::modulo],
                                vec![rule])]);
                        },
                    };

                    let lhs_line_col = if let Some(lhs_loc) = lhs.loc() {
                        LineColLocation::from(lhs_loc.line_col.clone())
                    } else {
                        exit(1);
                    };

                    let rhs_line_col = if let Some(rhs_loc) = rhs.loc() {
                        LineColLocation::from(rhs_loc.line_col.clone())
                    } else {
                        exit(1);
                    };

                    Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                        loc: Some(Location::from(&lhs_line_col, &rhs_line_col, None))
                    })
                },
                (lhs, rhs) => {
                    let mut errors = vec![];
                    if let Err(lhs_err) = lhs {
                        errors.extend(lhs_err);
                    }
                    if let Err(rhs_err) = rhs {
                        errors.extend(rhs_err);
                    }

                    Err(errors)
                }
            };
        })
        .parse(pairs)
}
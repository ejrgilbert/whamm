use crate::parser::types;
use types::{BinOp, Block, FnId, Rule, UnOp, WhammParser, PRATT_PARSER};

use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::{
    DataType, Expr, Location, ProbeSpec, Script, SpecPart, Statement, Value, Whamm,
};
use log::trace;
use pest::error::{Error, LineColLocation};
use pest::iterators::Pair;
use pest::Parser;

const UNEXPECTED_ERR_MSG: &str =
    "WhammParser: Looks like you've found a bug...please report this behavior! Exiting now...";

pub fn print_info(spec: String, print_globals: bool, print_functions: bool, err: &mut ErrorGen) {
    trace!("Entered print_info");
    err.set_script_text(spec.to_owned());

    let res = WhammParser::parse(Rule::PROBE_SPEC, &spec);
    match res {
        Ok(mut pairs) => {
            // Create the probe specification from the input string
            let probe_spec = probe_spec_from_rule(
                // inner of script
                pairs.next().unwrap(),
                err,
            );

            // Print the information for the passed probe specification
            let mut whamm = Whamm::new();
            let id = whamm.add_script(Script::new());
            let script: &mut Script = whamm.scripts.get_mut(id).unwrap();
            if let Err(e) = script.print_info(
                &whamm.provided_probes,
                &probe_spec,
                print_globals,
                print_functions,
            ) {
                err.add_error(*e);
            }
        }
        Err(e) => {
            err.pest_err(e);
        }
    }
}

pub fn parse_script(script: &String, err: &mut ErrorGen) -> Option<Whamm> {
    trace!("Entered parse_script");
    err.set_script_text(script.to_owned());

    let res = WhammParser::parse(Rule::script, script);
    match res {
        Ok(mut pairs) => {
            let res = to_ast(
                // inner of script
                pairs.next().unwrap(),
                err,
            );

            match res {
                Ok(ast) => Some(ast),
                Err(e) => {
                    err.pest_err(*e);
                    None
                }
            }
        }
        Err(e) => {
            err.pest_err(e);
            None
        }
    }
}

// ====================
// = AST Constructors =
// ====================

pub fn to_ast(pair: Pair<Rule>, err: &mut ErrorGen) -> Result<Whamm, Box<Error<Rule>>> {
    trace!("Entered to_ast");

    // Create initial AST with Whamm node
    let mut whamm = Whamm::new();
    let script_count = 0;

    match pair.as_rule() {
        Rule::script => {
            process_pair(&mut whamm, script_count, pair, err);
        }
        rule => {
            err.parse_error(
                true,
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::script],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!()
        }
    }

    Ok(whamm)
}

// ================
// = Parser Logic =
// ================

pub fn process_pair(whamm: &mut Whamm, script_count: usize, pair: Pair<Rule>, err: &mut ErrorGen) {
    trace!("Entered process_pair");
    match pair.as_rule() {
        Rule::script => {
            trace!("Entering script");
            let base_script = Script::new();
            let id = whamm.add_script(base_script);
            pair.into_inner().for_each(|p| {
                process_pair(whamm, id, p, err);
            });
            trace!("Exiting script");
        }
        Rule::statement => {
            trace!("Entering statement");

            // let mut pair = pair.into_inner();
            // let stmt_rules = pair.next().unwrap();

            let mut global_stmts = vec![];
            pair.into_inner().for_each(|p| {
                for stmt in stmt_from_rule(p, err) {
                    global_stmts.push(stmt);
                }
            });

            // Add global statements to the script
            let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
            script.add_global_stmts(global_stmts);

            trace!("Exiting statement");
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
                            match expr_from_pair(n.into_inner().next().unwrap()) {
                                Ok(res) => (Some(res), None),
                                Err(errors) => {
                                    err.add_errors(errors);
                                    // ignore predicate due to errors
                                    (None, None)
                                }
                            }
                        }
                        Rule::statement => {
                            let mut stmts = vec![];
                            n.into_inner().for_each(|p| {
                                stmts.push(stmt_from_rule(p, err));
                            });
                            (None, Some(stmts))
                        }
                        _ => (None, None),
                    };

                    if this_body.is_none() {
                        this_body = match pair.next() {
                            Some(b) => {
                                let mut stmts = vec![];

                                b.into_inner().for_each(|p| {
                                    stmts.push(stmt_from_rule(p, err));
                                });
                                Some(stmts)
                            }
                            None => None,
                        };
                    }

                    (this_predicate, this_body)
                }
                None => (None, None),
            };

            let this_body: Option<Vec<Statement>> = this_body.map(|b| {
                let mut stmts = vec![];
                //each stmt_from_rule returns a vector
                for stmt in b {
                    for s in stmt {
                        stmts.push(s);
                    }
                }
                stmts
            });

            // Add probe definition to the script
            let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
            if let Err(e) = script.add_probe(
                &whamm.provided_probes,
                &probe_spec,
                this_predicate,
                this_body,
            ) {
                err.add_error(*e);
            }

            trace!("Exiting probe_def");
        }
        Rule::EOI => {}
        //Rule looks like this: fn_def = { ID ~ "(" ~ ( param ) ? ~ ("," ~ param )* ~ ")" ~ "->" ~ TYPE ~ block }
        Rule::fn_def => {
            trace!("Entering fn_def");
            let mut pair = pair.into_inner();
            let fn_name_rule: Pair<Rule> = pair.next().unwrap();
            let fn_name_line_col = LineColLocation::from(fn_name_rule.as_span());
            let mut args = vec![];
            let mut return_ty = DataType::Tuple { ty_info: vec![] };
            let mut body = Block {
                stmts: vec![],
                loc: Some(Location {
                    line_col: fn_name_line_col.clone(),
                    path: None,
                }),
            };
            //pair now holds the list of tokens in the fn_def rule
            for p in pair {
                //iterate over each p in the pair and match based on the rule
                match p.as_rule() {
                    //make a set of rules for each possible type
                    Rule::param => {
                        //go into the param rule and add the output to args
                        let p_clone = p.clone();
                        let mut type_local = DataType::I32;
                        let mut arg_name = "".to_string();
                        for inner_p in p.into_inner() {
                            match inner_p.as_rule() {
                                Rule::ID => {
                                    arg_name = inner_p.as_str().parse().unwrap();
                                }
                                _ => {
                                    type_local = type_from_rule(inner_p, err);
                                }
                            }
                        }
                        let param_id_local = Expr::VarId {
                            is_comp_provided: false,
                            name: arg_name,
                            loc: Some(Location::from(
                                &LineColLocation::from(p_clone.as_span()),
                                &LineColLocation::from(p_clone.as_span()),
                                None,
                            )),
                        };
                        //arg holds Vec<(VarId, DataType)>
                        args.push((param_id_local, type_local));
                    }
                    Rule::block => {
                        body = block_from_rule(p, err);
                    }
                    _ => {
                        return_ty = type_from_rule(p, err);
                    }
                }
            }
            //convert fn_name_rule from Pair<_, Rule> to FnId
            let fn_id = FnId {
                name: fn_name_rule.as_str().parse().unwrap(),
                loc: Some(Location::from(&fn_name_line_col, &fn_name_line_col, None)),
            };
            //before creating the output, check if there is a return statement in the body
            //if there is not, add a return empty tuple
            let mut has_return = false;
            for stmt in &body.stmts {
                #[allow(clippy::single_match)]
                match stmt {
                    //ADD SUPPORT FOR IF/ELSE/LOOPS WHEN IMPLEMENTED
                    Statement::Return { .. } => {
                        has_return = true;
                        break;
                    }
                    _ => {}
                }
            }
            if !has_return {
                //add an empty tuple return statement to body.stmts
                let empty_tuple = Expr::Primitive {
                    val: Value::Tuple {
                        ty: DataType::Tuple { ty_info: vec![] },
                        vals: vec![],
                    },
                    loc: None,
                };
                let return_stmt = Statement::Return {
                    expr: empty_tuple,
                    loc: None,
                };
                body.stmts.push(return_stmt);
            }
            let output = types::Fn {
                is_comp_provided: false,
                name: fn_id,
                params: args,
                body,
                return_ty: Some(return_ty),
            };
            let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
            script.fns.push(output);
            trace!("Exiting fn_def");
        }
        rule => {
            err.parse_error(
                true,
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::script, Rule::probe_def, Rule::EOI],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!()
        }
    }
}

pub fn block_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> Block {
    trace!("Entered parse_block");
    let fn_name_line_col = LineColLocation::from(pair.as_span());
    let mut body_vec = vec![];
    pair.into_inner().for_each(|p| {
        for stmt in stmt_from_rule(p, err) {
            body_vec.push(stmt);
        }
    });

    //create the block object and return it in the wrapper with result
    Block {
        stmts: body_vec,
        loc: Some(Location::from(&fn_name_line_col, &fn_name_line_col, None)),
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
        is_comp_provided: false,
        name: fn_rule.as_str().parse().unwrap(),
        loc: Some(Location {
            line_col: fn_target_line_col.clone(),
            path: None,
        }),
    };

    // handle args
    let mut next = pair.next();
    let mut init = vec![];
    let mut errors = vec![];
    while next.is_some() {
        let mut others = vec![];
        match expr_from_pair(next.unwrap()) {
            Ok(expr) => {
                others.push(Box::new(expr));
                init.append(&mut others);
            }
            Err(err) => errors.extend(err),
        }

        next = pair.next();
    }
    let args = if !init.is_empty() { Some(init) } else { None };

    trace!("Exiting fn_call");
    if !errors.is_empty() {
        return Err(errors);
    }

    let last_arg_loc = if let Some(args) = &args {
        if let Some(last_arg) = args.last() {
            last_arg
                .loc()
                .as_ref()
                .map(|last_arg_loc| last_arg_loc.line_col.clone())
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
            loc: Some(Location::from(&fn_target_line_col, &last_arg_loc, None)),
        })
    } else {
        Ok(Expr::Call {
            fn_target: Box::new(fn_target),
            args,
            loc: Some(Location {
                line_col: fn_target_line_col.clone(),
                path: None,
            }),
        })
    }
}

fn stmt_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    trace!("Entered stmt_from_rule");
    match pair.as_rule() {
        Rule::statement => {
            trace!("Entering statement");
            let stmt = pair.into_inner().next().unwrap();
            let res = stmt_from_rule(stmt, err);
            trace!("Exiting statement");

            trace!("Exiting stmt_from_rule");
            res
        }
        Rule::declaration => {
            trace!("Entering declaration");
            // declaration = { TYPE ~ ID }
            let mut pair = pair.into_inner();
            let type_rule = pair.next().unwrap();
            let type_line_col = LineColLocation::from(type_rule.as_span());
            let ty = type_from_rule(type_rule, err);

            let var_id_rule = pair.next().unwrap();
            let var_id_line_col = LineColLocation::from(var_id_rule.as_span());
            let var_id = Expr::VarId {
                is_comp_provided: false,
                name: var_id_rule.as_str().parse().unwrap(),
                loc: Some(Location {
                    line_col: var_id_line_col.clone(),
                    path: None,
                }),
            };
            trace!("Exiting declaration");
            let output = vec![Statement::Decl {
                ty,
                var_id,
                loc: Some(Location::from(&type_line_col, &var_id_line_col, None)),
            }];
            output
        }
        Rule::assignment => {
            trace!("Entering assignment");
            let mut pair = pair.into_inner();
            let var_id_rule = pair.next().unwrap();
            let expr_rule = pair.next().unwrap();

            let var_id_line_col = LineColLocation::from(var_id_rule.as_span());

            let var_id = Expr::VarId {
                is_comp_provided: false,
                name: var_id_rule.as_str().parse().unwrap(),
                loc: Some(Location {
                    line_col: var_id_line_col.clone(),
                    path: None,
                }),
            };
            let mut output = vec![];
            return match expr_from_pair(expr_rule) {
                Err(errors) => {
                    err.add_errors(errors);

                    output
                }
                Ok(expr) => {
                    trace!("Exiting assignment");
                    trace!("Exiting stmt_from_rule");

                    let expr_line_col = if let Some(expr_loc) = expr.loc() {
                        expr_loc.line_col.clone()
                    } else {
                        err.add_error(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!(
                                "{}{}",
                                UNEXPECTED_ERR_MSG, "could not get location"
                            )),
                            None,
                        ));
                        return output;
                    };
                    output.push(Statement::Assign {
                        var_id,
                        expr,
                        loc: Some(Location::from(&var_id_line_col, &expr_line_col, None)),
                    });
                    output
                }
            };
        }
        Rule::fn_call => {
            let mut output: Vec<Statement> = vec![];
            return match fn_call_from_rule(pair) {
                Err(errors) => {
                    err.add_errors(errors);
                    output
                }
                Ok(call) => {
                    let call_loc = call.loc().clone();
                    trace!("Exiting stmt_from_rule");
                    output.push(Statement::Expr {
                        expr: call,
                        loc: call_loc,
                    });
                    output
                }
            };
        }
        Rule::incrementor => {
            trace!("Entering incrementor");
            let mut output: Vec<Statement> = vec![];
            let mut pair = pair.into_inner();
            let var_id_rule = pair.next().unwrap();
            let var_id_line_col = LineColLocation::from(var_id_rule.as_span());
            let var_id = Expr::VarId {
                is_comp_provided: false,
                name: var_id_rule.as_str().parse().unwrap(),
                loc: Some(Location {
                    line_col: var_id_line_col.clone(),
                    path: None,
                }),
            };
            let val = Value::Integer {
                ty: DataType::I32,
                val: 1,
            };
            let rhs = Expr::Primitive {
                val,
                loc: Some(Location::from(&var_id_line_col, &var_id_line_col, None)),
            };
            let expr = Expr::BinOp {
                lhs: Box::new(Expr::VarId {
                    is_comp_provided: false,
                    name: var_id_rule.as_str().parse().unwrap(),
                    loc: Some(Location {
                        line_col: var_id_line_col.clone(),
                        path: None,
                    }),
                }),
                op: BinOp::Add,
                rhs: Box::new(rhs),
                loc: Some(Location::from(&var_id_line_col, &var_id_line_col, None)),
            };
            trace!("Exiting incrementor");
            output.push(Statement::Assign {
                var_id,
                expr,
                loc: Some(Location::from(&var_id_line_col, &var_id_line_col, None)),
            });
            output
        }
        Rule::decrementor => {
            trace!("Entering decrementor");
            let mut output: Vec<Statement> = vec![];
            let mut pair = pair.into_inner();
            let var_id_rule = pair.next().unwrap();
            let var_id_line_col = LineColLocation::from(var_id_rule.as_span());
            let var_id = Expr::VarId {
                is_comp_provided: false,
                name: var_id_rule.as_str().parse().unwrap(),
                loc: Some(Location {
                    line_col: var_id_line_col.clone(),
                    path: None,
                }),
            };
            let val = Value::Integer {
                ty: DataType::I32,
                val: 1,
            };
            let rhs = Expr::Primitive {
                val,
                loc: Some(Location::from(&var_id_line_col, &var_id_line_col, None)),
            };
            let expr = Expr::BinOp {
                lhs: Box::new(Expr::VarId {
                    is_comp_provided: false,
                    name: var_id_rule.as_str().parse().unwrap(),
                    loc: Some(Location {
                        line_col: var_id_line_col.clone(),
                        path: None,
                    }),
                }),
                op: BinOp::Subtract,
                rhs: Box::new(rhs),
                loc: Some(Location::from(&var_id_line_col, &var_id_line_col, None)),
            };
            trace!("Exiting decrementor");
            output.push(Statement::Assign {
                var_id,
                expr,
                loc: Some(Location::from(&var_id_line_col, &var_id_line_col, None)),
            });
            output
        }
        Rule::ret => {
            trace!("Entering return_stmt");
            let mut output: Vec<Statement> = vec![];
            let ret_statement_line_col = LineColLocation::from(pair.as_span());
            let expr_rule;
            let mut pair = pair.into_inner();
            let next_pair = pair.next();
            match next_pair {
                None => {
                    output.push(Statement::Return {
                        expr: Expr::Primitive {
                            val: Value::Tuple {
                                ty: DataType::Tuple { ty_info: vec![] },
                                vals: vec![],
                            },
                            loc: Some(Location::from(
                                &ret_statement_line_col,
                                &ret_statement_line_col,
                                None,
                            )),
                        },
                        loc: Some(Location::from(
                            &ret_statement_line_col,
                            &ret_statement_line_col,
                            None,
                        )),
                    });
                    output
                }
                Some(_) => {
                    expr_rule = next_pair.unwrap();
                    return match expr_from_pair(expr_rule) {
                        Err(errors) => {
                            err.add_errors(errors);
                            output
                        }
                        Ok(expr) => {
                            trace!("Exiting return_stmt");
                            trace!("Exiting stmt_from_rule");

                            let expr_line_col = if let Some(expr_loc) = expr.loc() {
                                expr_loc.line_col.clone()
                            } else {
                                err.add_error(ErrorGen::get_unexpected_error(
                                    true,
                                    Some(format!(
                                        "{}{}",
                                        UNEXPECTED_ERR_MSG, "could not get location"
                                    )),
                                    None,
                                ));
                                return output;
                            };
                            output.push(Statement::Return {
                                expr,
                                loc: Some(Location::from(&expr_line_col, &expr_line_col, None)),
                            });
                            output
                        }
                    };
                }
            }
        }
        rule => {
            err.parse_error(
                true,
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::statement, Rule::assignment, Rule::fn_call],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    }
}

fn type_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> DataType {
    trace!("Entering type_from_rule");
    // TYPE = _{ TY_I32 | TY_BOOL | TY_STRING | TY_TUPLE | TY_MAP }
    return match pair.as_rule() {
        Rule::TY_I32 => DataType::I32,
        Rule::TY_BOOL => DataType::Boolean,
        Rule::TY_STRING => DataType::Str,
        Rule::TY_TUPLE => {
            let mut tuple_content_types = vec![];
            pair.into_inner().for_each(|p| {
                tuple_content_types.push(Box::new(type_from_rule(p, err)));
            });
            return if tuple_content_types.is_empty() {
                DataType::Tuple { ty_info: vec![] }
            } else {
                DataType::Tuple {
                    ty_info: tuple_content_types,
                }
            };
        }
        Rule::TY_MAP => {
            let mut pair = pair.into_inner();
            let key_ty_rule = pair.next().unwrap();
            let val_ty_rule = pair.next().unwrap();

            let key_ty = type_from_rule(key_ty_rule, err);
            let val_ty = type_from_rule(val_ty_rule, err);

            return DataType::Map {
                key_ty: Box::new(key_ty),
                val_ty: Box::new(val_ty),
            };
        }
        rule => {
            err.parse_error(
                true,
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::TY_I32,
                    Rule::TY_BOOL,
                    Rule::TY_STRING,
                    Rule::TY_TUPLE,
                    Rule::TY_MAP,
                ],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    };
}

fn probe_spec_part_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> SpecPart {
    trace!("Entered probe_spec_part_from_rule");
    match pair.as_rule() {
        Rule::PROBE_ID => {
            trace!("Entering PROBE_ID");
            let name: String = pair.as_str().parse().unwrap();
            let id_line_col = LineColLocation::from(pair.as_span());

            let part = SpecPart {
                name,
                loc: Some(Location {
                    line_col: id_line_col,
                    path: None,
                }),
            };
            trace!("Exiting PROBE_ID");

            trace!("Exiting probe_spec_part_from_rule");
            part
        }
        rule => {
            err.parse_error(
                true,
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::PROBE_ID, Rule::PROBE_ID],
                vec![rule],
            );
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
                        path: None,
                    })
                } else {
                    None
                };

                return ProbeSpec {
                    provider: Some(SpecPart {
                        name: "whamm".to_string(),
                        loc: loc.clone(),
                    }),
                    package: Some(SpecPart {
                        name: "*".to_string(),
                        loc: loc.clone(),
                    }),
                    event: Some(SpecPart {
                        name: "*".to_string(),
                        loc: loc.clone(),
                    }),
                    mode: Some(SpecPart {
                        name: "BEGIN".to_string(),
                        loc,
                    }),
                };
            }

            let str_parts = spec_as_str.split(':');

            let mut probe_spec = ProbeSpec::new();
            let mut contents: Vec<String> = vec![];
            for s in str_parts {
                if s.is_empty() {
                    probe_spec.add_spec_def(SpecPart {
                        name: "*".to_string(),
                        loc: None,
                    });
                    contents.push("*".to_string());
                    continue;
                }
                let next = parts.next();

                match next {
                    Some(part) => match part.as_rule() {
                        Rule::PROBE_ID => {
                            let n = probe_spec_part_from_rule(part, err);
                            probe_spec.add_spec_def(n);
                        }
                        _ => {
                            probe_spec.add_spec_def(SpecPart {
                                name: "*".to_string(),
                                loc: None,
                            });
                        }
                    },
                    None => {
                        break;
                    }
                };
            }
            trace!("Exiting PROBE_SPEC");
            trace!("Exiting probe_spec_from_rule");

            probe_spec
        }
        rule => {
            err.parse_error(
                true,
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::PROBE_SPEC],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    }
}

fn expr_primary(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    match pair.as_rule() {
        Rule::fn_call => fn_call_from_rule(pair),
        Rule::ID => {
            return Ok(Expr::VarId {
                is_comp_provided: false,
                name: pair.as_str().parse().unwrap(),
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None,
                }),
            });
        }
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
            Ok(Expr::Primitive {
                val: Value::Tuple {
                    ty: DataType::Tuple { ty_info: vec![] },
                    vals,
                },
                loc: Some(Location {
                    line_col: pair_line_col,
                    path: None,
                }),
            })
        }
        Rule::I32 => {
            trace!("Entering I32");
            let val = pair.as_str().parse::<i32>().unwrap();

            trace!("Exiting I32");
            return Ok(Expr::Primitive {
                val: Value::Integer {
                    ty: DataType::I32,
                    val,
                },
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None,
                }),
            });
        }
        Rule::BOOL => {
            trace!("Entering BOOL");
            let val = pair.as_str().parse::<bool>().unwrap();

            trace!("Exiting BOOL");
            return Ok(Expr::Primitive {
                val: Value::Boolean {
                    ty: DataType::Boolean,
                    val,
                },
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None,
                }),
            });
        }
        Rule::STRING => {
            trace!("Entering STRING");
            let mut val: String = pair.as_str().parse().unwrap();
            if val.starts_with('\"') {
                val = val
                    .strip_prefix('\"')
                    .expect("Should never get here...")
                    .to_string();
            }
            if val.ends_with('\"') {
                val = val
                    .strip_suffix('\"')
                    .expect("Should never get here...")
                    .to_string();
            }

            trace!("Exiting STRING");
            return Ok(Expr::Primitive {
                val: Value::Str {
                    ty: DataType::Str,
                    val,
                    addr: None,
                },
                loc: Some(Location {
                    line_col: LineColLocation::from(pair.as_span()),
                    path: None,
                }),
            });
        }
        _ => expr_from_pair(pair),
    }
}

fn expr_from_pair(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    return match pair.as_rule() {
        Rule::ternary => {
            // handle contents
            let pair_loc = LineColLocation::from(pair.as_span());
            let mut pairs = pair.into_inner();

            let cond_rule = pairs.next().unwrap();
            let cond = match expr_from_pair(cond_rule) {
                Ok(expr) => expr,
                other => {
                    return other;
                }
            };

            let conseq_rule = pairs.next().unwrap();
            let conseq = match expr_from_pair(conseq_rule) {
                Ok(expr) => expr,
                other => {
                    return other;
                }
            };

            let alt_rule = pairs.next().unwrap();
            let alt = match expr_from_pair(alt_rule) {
                Ok(expr) => expr,
                other => {
                    return other;
                }
            };

            Ok(Expr::Ternary {
                cond: Box::new(cond),
                conseq: Box::new(conseq),
                alt: Box::new(alt),
                loc: Some(Location {
                    line_col: pair_loc,
                    path: None,
                }),
            })
        }
        Rule::arg => {
            let mut pairs = pair.into_inner();
            let arg = pairs.next().unwrap();
            match arg.as_rule() {
                Rule::expr => expr_from_pair(arg),
                _ => expr_primary(arg),
            }
        }
        Rule::expr => {
            let pairs = pair.into_inner();
            // TODO -- try boxing ErrorGen so you can put it in both closures?
            PRATT_PARSER
                .map_primary(|primary| -> Result<Expr, Vec<WhammError>> { expr_primary(primary) })
                .map_prefix(|op, rhs| -> Result<Expr, Vec<WhammError>> {
                    return match rhs {
                        Ok(rhs) => {
                            let op = match op.as_rule() {
                                Rule::neg => UnOp::Not,
                                rule => {
                                    return Err(vec![ErrorGen::get_parse_error(
                                        true,
                                        Some(UNEXPECTED_ERR_MSG.to_string()),
                                        Some(LineColLocation::from(op.as_span())),
                                        vec![Rule::prefix],
                                        vec![rule],
                                    )]);
                                }
                            };

                            let rhs_line_col = if let Some(rhs_loc) = rhs.loc() {
                                rhs_loc.line_col.clone()
                            } else {
                                return Err(vec![ErrorGen::get_unexpected_error(
                                    true,
                                    Some(format!(
                                        "{}{}",
                                        UNEXPECTED_ERR_MSG, "could not get location"
                                    )),
                                    None,
                                )]);
                            };

                            Ok(Expr::UnOp {
                                op,
                                expr: Box::new(rhs),
                                loc: Some(Location::from(&rhs_line_col, &rhs_line_col, None)),
                            })
                        }
                        Err(errors) => Err(errors),
                    };
                })
                .map_infix(|lhs, op, rhs| -> Result<Expr, Vec<WhammError>> {
                    return match (lhs, rhs) {
                        (Ok(lhs), Ok(rhs)) => {
                            let op = match op.as_rule() {
                                // Logical operators
                                Rule::and => BinOp::And,
                                Rule::or => BinOp::Or,

                                // Relational operators
                                Rule::eq => BinOp::EQ,
                                Rule::ne => BinOp::NE,
                                Rule::ge => BinOp::GE,
                                Rule::gt => BinOp::GT,
                                Rule::le => BinOp::LE,
                                Rule::lt => BinOp::LT,

                                // Highest precedence arithmetic operators
                                Rule::add => BinOp::Add,
                                Rule::subtract => BinOp::Subtract,

                                // Next highest precedence arithmetic operators
                                Rule::multiply => BinOp::Multiply,
                                Rule::divide => BinOp::Divide,
                                Rule::modulo => BinOp::Modulo,
                                rule => {
                                    return Err(vec![ErrorGen::get_parse_error(
                                        true,
                                        Some(UNEXPECTED_ERR_MSG.to_string()),
                                        Some(LineColLocation::from(op.as_span())),
                                        vec![
                                            Rule::and,
                                            Rule::or,
                                            Rule::eq,
                                            Rule::ne,
                                            Rule::ge,
                                            Rule::gt,
                                            Rule::le,
                                            Rule::lt,
                                            Rule::add,
                                            Rule::subtract,
                                            Rule::multiply,
                                            Rule::divide,
                                            Rule::modulo,
                                        ],
                                        vec![rule],
                                    )]);
                                }
                            };

                            let lhs_line_col = if let Some(lhs_loc) = lhs.loc() {
                                lhs_loc.line_col.clone()
                            } else {
                                return Err(vec![ErrorGen::get_unexpected_error(
                                    true,
                                    Some(format!(
                                        "{}{}",
                                        UNEXPECTED_ERR_MSG, "could not get location"
                                    )),
                                    None,
                                )]);
                            };

                            let rhs_line_col = if let Some(rhs_loc) = rhs.loc() {
                                rhs_loc.line_col.clone()
                            } else {
                                return Err(vec![ErrorGen::get_unexpected_error(
                                    true,
                                    Some(format!(
                                        "{}{}",
                                        UNEXPECTED_ERR_MSG, "could not get location"
                                    )),
                                    None,
                                )]);
                            };

                            Ok(Expr::BinOp {
                                lhs: Box::new(lhs),
                                op,
                                rhs: Box::new(rhs),
                                loc: Some(Location::from(&lhs_line_col, &rhs_line_col, None)),
                            })
                        }
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
        rule => Err(vec![ErrorGen::get_parse_error(
            true,
            Some(UNEXPECTED_ERR_MSG.to_string()),
            Some(LineColLocation::from(pair.as_span())),
            vec![Rule::expr, Rule::ternary],
            vec![rule],
        )]),
    };
}

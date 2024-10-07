use crate::common::error::{ErrorGen, WhammError};
use crate::parser::print_visitor::AsStrVisitor;
use crate::parser::types;
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, FnId, Location, ProbeSpec, Rule, Script, SpecPart,
    Statement, UnOp, Value, Whamm, WhammParser, WhammVisitor, PRATT_PARSER,
};
use log::trace;
use pest::error::{Error, LineColLocation};
use pest::iterators::{Pair, Pairs};
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
            let probe_spec = handle_probe_spec(
                // inner of script
                pairs.next().unwrap(),
                err,
            );

            // Print the information for the passed probe specification
            let mut whamm = Whamm::new();
            let id = whamm.add_script(Script::new());
            let script: &mut Script = whamm.scripts.get_mut(id).unwrap();
            if let Err(e) = script.print_info(&probe_spec, print_globals, print_functions) {
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
            parser_entry_point(&mut whamm, script_count, pair, err);
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
    let mut visitor = AsStrVisitor { indent: 0 };
    let s = visitor.visit_whamm(&whamm);
    println!("{}", s);
    Ok(whamm)
}

// =======================
// = Pair Handling Logic =
// =======================

pub fn parser_entry_point(
    whamm: &mut Whamm,
    script_count: usize,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) {
    trace!("Enter process_pair");
    match pair.as_rule() {
        Rule::script => {
            trace!("Begin process script");
            handle_script(whamm, pair, err);
            trace!("End process script");
        }
        Rule::if_stmt => {
            trace!("Begin process statement");
            handle_global_if_stmt(whamm, script_count, pair, err);
            trace!("End process statement");
        }
        Rule::statement => {
            trace!("Begin process statement");
            handle_global_statements(whamm, script_count, pair, err);
            trace!("End process statement");
        }
        Rule::probe_def => {
            trace!("Begin process probe_def");
            handle_probe_def(whamm, script_count, pair, err);
            trace!("End process probe_def");
        }
        Rule::EOI => {}
        Rule::fn_def => {
            trace!("Begin process fn_def");
            handle_fn_def(whamm, script_count, pair, err);
            trace!("End process fn_def");
        }
        rule => {
            err.parse_error(
                true,
                None,
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::script,
                    Rule::statement,
                    Rule::probe_def,
                    Rule::EOI,
                    Rule::fn_def,
                ],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!()
        }
    }
    trace!("Exit process_pair");
}

pub fn handle_script(whamm: &mut Whamm, pair: Pair<Rule>, err: &mut ErrorGen) {
    let base_script = Script::new();
    let new_script_count = whamm.add_script(base_script);

    pair.into_inner().for_each(|p| {
        parser_entry_point(whamm, new_script_count, p, err);
    });
}

pub fn handle_global_if_stmt(
    whamm: &mut Whamm,
    script_count: usize,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) {
    // Add global if stmt to the script
    let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
    script.add_global_stmts(handle_if(pair, err));
}

pub fn handle_global_statements(
    whamm: &mut Whamm,
    script_count: usize,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) {
    // Add global statements to the script
    let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
    script.add_global_stmts(handle_stmts(&mut pair.into_inner(), err));
}

pub fn handle_probe_def(
    whamm: &mut Whamm,
    script_count: usize,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) {
    let mut pair = pair.into_inner();
    let spec_rule = pair.next().unwrap();
    // Get out the spec info
    let probe_spec = handle_probe_spec(spec_rule, err);

    // Get out the probe predicate/body contents
    let next = pair.next();
    let (this_predicate, this_body) = match next {
        Some(mut n) => {
            let this_predicate = if matches!(n.as_rule(), Rule::predicate) {
                let res = match handle_expr(n.into_inner().next().unwrap()) {
                    Ok(res) => Some(res),
                    Err(errors) => {
                        err.add_errors(errors);
                        // ignore predicate due to errors
                        None
                    }
                };
                n = pair.next().unwrap();
                res
            } else {
                None
            };

            let this_body = if matches!(n.as_rule(), Rule::block) {
                let loc = LineColLocation::from(n.as_span());
                let block = handle_body(&mut n.into_inner(), loc, err);
                if block.is_empty() {
                    None
                } else {
                    Some(block)
                }
            } else {
                None
            };

            (this_predicate, this_body)
        }
        None => (None, None),
    };

    // Add probe definition to the script
    let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
    if let Err(e) = script.add_probe(&probe_spec, this_predicate, this_body) {
        err.add_error(*e);
    }
}

pub fn handle_fn_def(whamm: &mut Whamm, script_count: usize, pair: Pair<Rule>, err: &mut ErrorGen) {
    let mut pairs = pair.into_inner();

    // Get the function name
    let fn_name: Pair<Rule> = pairs.next().unwrap();
    let fn_id = FnId {
        name: fn_name.as_str().parse().unwrap(),
        loc: Some(Location {
            line_col: LineColLocation::from(fn_name.as_span()),
            path: None,
        }),
    };

    // Get the parameters
    let mut params = vec![];
    let mut next = pairs.next();
    while let Some(n) = &next {
        if matches!(n.as_rule(), Rule::param) {
            if let Some(param) = handle_param(n.clone().into_inner(), err) {
                params.push(param)
            }
            next = pairs.next();
        } else {
            break;
        }
    }

    // Get the return type
    let return_ty = match next.clone() {
        Some(pair) => {
            if !matches!(pair.as_rule(), Rule::block) {
                next = pairs.next();
                type_from_rule(pair, err)
            } else {
                DataType::Tuple { ty_info: vec![] }
            }
        }
        None => DataType::Tuple { ty_info: vec![] },
    };

    // Get the function body
    let body = if let Some(n) = next {
        if matches!(n.as_rule(), Rule::block) {
            let loc = LineColLocation::from(n.as_span());
            let mut pairs = n.into_inner();
            handle_body(&mut pairs, loc, err)
        } else {
            // If didn't match, create empty body
            Block::default()
        }
    } else {
        // If didn't match, create empty body
        Block::default()
    };

    // Add the new function to the current script
    let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
    script.fns.push(types::Fn {
        def: Definition::User,
        name: fn_id,
        params,
        body,
        return_ty,
    });
}

fn handle_probe_spec(pair: Pair<Rule>, err: &mut ErrorGen) -> ProbeSpec {
    match pair.as_rule() {
        Rule::PROBE_SPEC => probe_spec_from_rule(pair, err),
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

fn expr_from_pair(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    match pair.as_rule() {
        Rule::ternary => handle_ternary(pair),
        Rule::arg => handle_arg(pair),
        Rule::expr => handle_expr(pair),
        rule => Err(vec![ErrorGen::get_parse_error(
            true,
            None,
            Some(LineColLocation::from(pair.as_span())),
            vec![Rule::expr, Rule::arg, Rule::ternary],
            vec![rule],
        )]),
    }
}

// ====================
// = HELPER FUNCTIONS =
// ====================

// STATEMENTS

fn handle_body(pairs: &mut Pairs<Rule>, line_col: LineColLocation, err: &mut ErrorGen) -> Block {
    let mut stmts = vec![];
    for pair in pairs {
        let new_stmts = if matches!(pair.as_rule(), Rule::if_stmt) {
            handle_if(pair, err)
        } else if matches!(pair.as_rule(), Rule::statement) {
            handle_stmts(&mut pair.into_inner(), err)
        } else {
            break;
        };
        stmts.extend(new_stmts);
    }
    Block {
        stmts,
        return_ty: None,
        loc: Some(Location {
            line_col,
            path: None,
        }),
    }
}

fn handle_stmts(pairs: &mut Pairs<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let mut stmts = vec![];
    pairs.for_each(|p| {
        for stmt in stmt_from_rule(p, err) {
            stmts.push(stmt);
        }
    });
    stmts
}

fn stmt_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    match pair.as_rule() {
        Rule::statement => {
            let stmt = pair.into_inner().next().unwrap();
            stmt_from_rule(stmt, err)
        }
        Rule::declaration => handle_decl(&mut pair.into_inner(), err),
        Rule::assignment => handle_assignment(pair, err),
        Rule::fn_call => handle_function_call_outer(pair, err),
        Rule::incrementor => handle_incrementor(pair, err),
        Rule::decrementor => handle_decrementor(pair, err),
        Rule::ret => handle_ret(pair, err),
        Rule::initialize => handle_initialize(pair, err),
        Rule::if_stmt => handle_if(pair, err),
        Rule::report_declaration => handle_report(pair, err),
        rule => {
            err.parse_error(
                true,
                None,
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::statement,
                    Rule::declaration,
                    Rule::assignment,
                    Rule::fn_call,
                    Rule::incrementor,
                    Rule::decrementor,
                    Rule::ret,
                    Rule::initialize,
                    Rule::if_stmt,
                    Rule::report_declaration,
                ],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    }
}

fn handle_decl(pair: &mut Pairs<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let type_rule = pair.next().unwrap();
    let type_line_col = LineColLocation::from(type_rule.as_span());
    let ty = type_from_rule(type_rule, err);

    let var_id_rule = pair.next().unwrap();
    let var_id_line_col = LineColLocation::from(var_id_rule.as_span());

    trace!("Exiting declaration");
    vec![Statement::Decl {
        ty,
        var_id: handle_id(var_id_rule),
        loc: Some(Location::from(&type_line_col, &var_id_line_col, None)),
    }]
}

fn handle_assignment(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let mut pairs = pair.into_inner();

    // get the target assignee
    let var_id_rule = pairs.next().unwrap();
    let var_id_line_col = LineColLocation::from(var_id_rule.as_span());
    let (var_id, key, is_map) = handle_lhs(var_id_rule, err);

    // get the value to assign
    let val_rule = pairs.next().unwrap();
    let val_line_col = LineColLocation::from(val_rule.as_span());
    let val = match expr_primary(val_rule) {
        Ok(expr) => expr,
        Err(errors) => {
            err.add_errors(errors);
            return vec![];
        }
    };

    // create the assignment statement
    if is_map {
        vec![Statement::SetMap {
            map: var_id,
            key: key.unwrap(),
            val,
            loc: Some(Location::from(&var_id_line_col, &val_line_col, None)),
        }]
    } else {
        vec![Statement::Assign {
            var_id,
            expr: val,
            loc: Some(Location::from(&var_id_line_col, &val_line_col, None)),
        }]
    }
}

fn handle_function_call_outer(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    match handle_fn_call(pair) {
        Ok(call) => {
            let call_loc = call.loc().clone();
            vec![Statement::Expr {
                expr: call,
                loc: call_loc,
            }]
        }
        Err(errors) => {
            err.add_errors(errors);
            vec![]
        }
    }
}
fn handle_incrementor(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    vec![handle_custom_binop(
        BinOp::Add,
        Expr::Primitive {
            val: Value::I32 {
                ty: DataType::I32,
                val: 1,
            },
            loc: Some(Location {
                line_col: LineColLocation::from(pair.as_span()),
                path: None,
            }),
        },
        pair,
        err,
    )]
}

fn handle_decrementor(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    vec![handle_custom_binop(
        BinOp::Subtract,
        Expr::Primitive {
            val: Value::I32 {
                ty: DataType::I32,
                val: 1,
            },
            loc: Some(Location {
                line_col: LineColLocation::from(pair.as_span()),
                path: None,
            }),
        },
        pair,
        err,
    )]
}

fn handle_lhs(pair: Pair<Rule>, err: &mut ErrorGen) -> (Expr, Option<Expr>, bool) {
    match expr_primary(pair) {
        Ok(expr) => match expr {
            Expr::MapGet { map, key, .. } => (*map, Some(*key), true),
            var_id => (var_id, None, false),
        },
        Err(errors) => {
            err.add_errors(errors);
            (
                Expr::VarId {
                    definition: Definition::User,
                    name: "placeholder".to_string(),
                    loc: None,
                },
                None,
                false,
            )
        }
    }
}

fn handle_custom_binop(op: BinOp, rhs: Expr, pair: Pair<Rule>, err: &mut ErrorGen) -> Statement {
    let full_loc = LineColLocation::from(pair.as_span());
    let mut pair = pair.into_inner();
    let var_id_rule = pair.next().unwrap();
    // get the increment target
    let (var_id, target_key, is_map) = handle_lhs(var_id_rule, err);

    let binop_lhs = if is_map {
        Expr::MapGet {
            map: Box::new(var_id.clone()),
            key: Box::new(target_key.clone().unwrap()),
            loc: var_id.loc().clone(),
        }
    } else {
        var_id.clone()
    };

    let val = Expr::BinOp {
        lhs: Box::new(binop_lhs),
        op,
        rhs: Box::new(rhs),
        loc: Some(Location {
            line_col: full_loc.clone(),
            path: None,
        }),
    };

    if is_map {
        Statement::SetMap {
            map: var_id,
            key: target_key.unwrap(),
            val,
            loc: Some(Location {
                line_col: full_loc,
                path: None,
            }),
        }
    } else {
        Statement::Assign {
            var_id,
            expr: val,
            loc: Some(Location {
                line_col: full_loc,
                path: None,
            }),
        }
    }
}

fn handle_ret(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let ret_statement_line_col: LineColLocation = LineColLocation::from(pair.as_span());

    match pair.into_inner().next() {
        None => {
            vec![Statement::Return {
                expr: Expr::Primitive {
                    val: Value::Tuple {
                        ty: DataType::Tuple { ty_info: vec![] },
                        vals: vec![],
                    },
                    loc: Some(Location {
                        line_col: ret_statement_line_col.clone(),
                        path: None,
                    }),
                },
                loc: Some(Location {
                    line_col: ret_statement_line_col.clone(),
                    path: None,
                }),
            }]
        }
        Some(val) => match expr_from_pair(val) {
            Err(errors) => {
                err.add_errors(errors);
                vec![]
            }
            Ok(expr) => {
                trace!("Exiting return_stmt");
                trace!("Exiting stmt_from_rule");

                vec![Statement::Return {
                    expr,
                    loc: Some(Location {
                        line_col: ret_statement_line_col.clone(),
                        path: None,
                    }),
                }]
            }
        },
    }
}

fn handle_initialize(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let mut pairs = pair.into_inner();
    // create the decl
    let decls = handle_decl(&mut pairs, err);
    let decl = decls.first().unwrap();
    let var_id = if let Statement::Decl { var_id, .. } = decl {
        var_id.clone()
    } else {
        return vec![];
    };

    // get the assignment
    let expr_rule = pairs.next().unwrap();
    let expr_line_col = LineColLocation::from(expr_rule.as_span());
    let assign = match expr_from_pair(expr_rule) {
        Ok(expr) => {
            let loc = decl
                .line_col()
                .map(|loc| Location::from(&loc.clone(), &expr_line_col, None));
            Statement::Assign {
                var_id: var_id.clone(),
                expr,
                loc,
            }
        }
        Err(errors) => {
            err.add_errors(errors);
            return vec![];
        }
    };

    vec![decl.to_owned(), assign]
}
fn handle_if(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let if_stmt_line_col: LineColLocation = LineColLocation::from(pair.as_span());
    let mut pairs = pair.into_inner();

    // get the conditional
    let cond = match expr_from_pair(pairs.next().unwrap()) {
        Ok(expr) => expr,
        Err(errors) => {
            err.add_errors(errors);
            return vec![];
        }
    };

    // get the consequent block
    let conseq_pair = pairs.next().unwrap();
    let line_col = LineColLocation::from(conseq_pair.as_span());
    let conseq = handle_body(&mut conseq_pair.into_inner(), line_col, err);

    // get the alternate block
    match pairs.next() {
        Some(inner) => {
            let alt = handle_alt(inner, err);
            vec![Statement::If {
                cond,
                conseq,
                alt,
                loc: Some(Location {
                    line_col: if_stmt_line_col.clone(),
                    path: None,
                }),
            }]
        }
        None => {
            vec![Statement::If {
                cond,
                conseq,
                alt: Block {
                    stmts: vec![],
                    return_ty: None,
                    loc: Some(Location {
                        line_col: if_stmt_line_col.clone(),
                        path: None,
                    }),
                },
                loc: Some(Location {
                    line_col: if_stmt_line_col.clone(),
                    path: None,
                }),
            }]
        }
    }
}

fn handle_alt(pair: Pair<Rule>, err: &mut ErrorGen) -> Block {
    let alt_loc = LineColLocation::from(pair.as_span());
    match pair.as_rule() {
        Rule::else_stmt => handle_else(pair, err),
        Rule::elif => handle_elif(pair, err),
        _ => {
            err.parse_error(
                true,
                Some("Error parsing if/else".to_string()),
                Some(alt_loc.clone()),
                vec![Rule::else_stmt, Rule::elif],
                vec![pair.as_rule()],
            );
            Block::default()
        }
    }
}

fn handle_else(pair: Pair<Rule>, err: &mut ErrorGen) -> Block {
    let line_col = LineColLocation::from(pair.as_span());
    let mut pairs = pair.into_inner();

    let block = pairs.next().unwrap();
    handle_body(&mut block.into_inner(), line_col, err)
}

fn handle_elif(pair: Pair<Rule>, err: &mut ErrorGen) -> Block {
    let alt_loc = LineColLocation::from(pair.as_span());

    // get the condition
    let mut pairs = pair.into_inner();
    let cond = match expr_from_pair(pairs.next().unwrap()) {
        Ok(expr) => expr,
        Err(errors) => {
            err.add_errors(errors);
            return Block::default();
        }
    };

    // get the consequent
    let inner_block_pair = pairs.next().unwrap();
    let inner_block_loc = LineColLocation::from(inner_block_pair.as_span());
    let inner_block = handle_body(&mut inner_block_pair.into_inner(), inner_block_loc, err);

    let next_pair = pairs.next();
    if next_pair.is_none() {
        // no more elifs, return
        return Block {
            stmts: vec![Statement::If {
                cond,
                conseq: inner_block,
                alt: Block {
                    stmts: vec![],
                    return_ty: None,
                    loc: Some(Location {
                        line_col: alt_loc.clone(),
                        path: None,
                    }),
                },
                loc: Some(Location {
                    line_col: alt_loc.clone(),
                    path: None,
                }),
            }],
            return_ty: None,
            loc: Some(Location {
                line_col: alt_loc.clone(),
                path: None,
            }),
        };
    }

    // keep going
    let alt = handle_alt(next_pair.unwrap(), err);
    Block {
        stmts: vec![Statement::If {
            cond,
            conseq: inner_block,
            alt,
            loc: Some(Location {
                line_col: alt_loc.clone(),
                path: None,
            }),
        }],
        return_ty: None,
        loc: Some(Location {
            line_col: alt_loc.clone(),
            path: None,
        }),
    }
}

fn handle_report(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let line_col = LineColLocation::from(pair.as_span());
    let mut pairs = pair.into_inner();

    let decl = stmt_from_rule(pairs.next().unwrap(), err);
    vec![Statement::ReportDecl {
        decl: Box::new(decl.first().unwrap().to_owned()),
        loc: Some(Location {
            line_col,
            path: None,
        }),
    }]
}
// EXPRESSIONS

fn handle_param(mut pairs: Pairs<Rule>, err: &mut ErrorGen) -> Option<(Expr, DataType)> {
    if let Some(param_rule) = pairs.next() {
        // process the type
        let ty = type_from_rule(param_rule, err);
        // process the name
        let id = handle_id(pairs.next().unwrap());
        Some((id, ty))
    } else {
        None
    }
}

fn handle_fn_call(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    // This has to be duplicated due to the Expression/Statement masking as the function return type
    let mut pairs = pair.into_inner();

    let fn_rule = pairs.next().unwrap();
    let fn_target_line_col = LineColLocation::from(fn_rule.as_span());
    let fn_target = handle_id(fn_rule);

    // handle args
    let mut args = vec![];
    let mut errors = vec![];
    for next in pairs {
        match handle_arg(next) {
            Ok(expr) => {
                args.push(expr);
            }
            Err(err) => {
                errors.extend(err);
            }
        };
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let last_arg_loc = if let Some(last_arg) = args.last() {
        last_arg
            .loc()
            .as_ref()
            .map(|last_arg_loc| last_arg_loc.line_col.clone())
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

fn handle_ternary(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
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
        ty: DataType::Null,
        loc: Some(Location {
            line_col: pair_loc,
            path: None,
        }),
    })
}

fn handle_arg(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let mut pairs = pair.into_inner();
    let arg = pairs.next().unwrap();
    match arg.as_rule() {
        Rule::expr => handle_expr(arg),
        _ => expr_primary(arg),
    }
}

/// TLDR; We cannot keep from passing up WhammErrors during parsing.
///
/// NOTE -- I've tried to refactor to push down using the ErrorGen into the PRATT_PARSER
/// logic, but it's not possible due to the following compilation error:
/// error[E0499]: cannot borrow `***err` as mutable more than once at a time
///    --> src/parser/whamm_parser.rs:904:20
///     |
/// 874 |         .map_primary(|primary| -> Expr { expr_primary(primary, err) })
///     |                      -----------------                         --- first borrow occurs due to use of `*err` in closure
///     |                      |
///     |                      first mutable borrow occurs here
/// ...
/// 904 |         .map_infix(|lhs, op, rhs| -> Expr {
///     |          --------- ^^^^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
///     |          |
///     |          first borrow later used by call
/// ...
/// 927 |                     err.parse_error(
///     |                     --- second borrow occurs due to use of `***err` in closure
///
/// Since there are multiple live mutable references to the ErrorGen, it won't compile.
/// I think this would be a problem even if we were to refactor the Parser into an object that
/// would hold the mutable reference to the error gen as a member field. This is because you'd call
/// self.<something> which would require 2 mutable references to self between the closures.
fn handle_expr(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let pairs = pair.into_inner();
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

                    let loc = if let Some(rhs_loc) = rhs.loc() {
                        let rhs_line_col = rhs_loc.line_col.clone();
                        Some(Location::from(&rhs_line_col, &rhs_line_col, None))
                    } else {
                        None
                    };

                    Ok(Expr::UnOp {
                        op,
                        expr: Box::new(rhs),
                        loc,
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
                                None,
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

                    let loc = if let (Some(lhs_loc), Some(rhs_loc)) = (lhs.loc(), rhs.loc()) {
                        let lhs_line_col = lhs_loc.line_col.clone();
                        let rhs_line_col = rhs_loc.line_col.clone();
                        Some(Location::from(&lhs_line_col, &rhs_line_col, None))
                    } else {
                        None
                    };

                    Ok(Expr::BinOp {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                        loc,
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

fn expr_primary(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    match pair.as_rule() {
        Rule::fn_call => handle_fn_call(pair),
        Rule::ID => Ok(handle_id(pair)),
        Rule::tuple => handle_tuple(pair),
        Rule::INT => handle_int(pair),
        Rule::DEC => handle_dec(pair),
        Rule::BOOL => handle_bool(pair),
        Rule::STRING => handle_string(pair),
        Rule::get_map => handle_map_get(pair),
        _ => expr_from_pair(pair),
    }
}

// ================================
// = LOWER-LEVEL HELPER FUNCTIONS =
// ================================

// PROBE SPECIFICATIONS (MATCH RULES)

fn probe_spec_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> ProbeSpec {
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
                name: "core".to_string(),
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
                name: spec_as_str.to_string(),
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

fn probe_spec_part_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> SpecPart {
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
                vec![Rule::PROBE_ID],
                vec![rule],
            );
            // should have exited above (since it's a fatal error)
            unreachable!();
        }
    }
}

// TYPES

fn type_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> DataType {
    trace!("Entering type_from_rule");
    // TYPE = _{ TY_I32 | TY_BOOL | TY_STRING | TY_TUPLE | TY_MAP }
    return match pair.as_rule() {
        Rule::TY_U32 => DataType::U32,
        Rule::TY_I32 => DataType::I32,
        Rule::TY_F32 => {
            err.parse_error(
                true,
                Some("f32 not supported yet, see Issue #29: https://github.com/ejrgilbert/whamm/issues/141".to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::TY_I32,
                    Rule::TY_BOOL,
                    Rule::TY_STRING,
                    Rule::TY_TUPLE,
                    Rule::TY_MAP,
                ],
                vec![pair.as_rule()],
            );
            DataType::F32
        }
        Rule::TY_U64 => {
            err.parse_error(
                true,
                Some("u64 not supported yet, see Issue #29: https://github.com/ejrgilbert/whamm/issues/141".to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::TY_I32,
                    Rule::TY_BOOL,
                    Rule::TY_STRING,
                    Rule::TY_TUPLE,
                    Rule::TY_MAP,
                ],
                vec![pair.as_rule()],
            );
            DataType::U64
        }
        Rule::TY_I64 => {
            err.parse_error(
                true,
                Some("i64 not supported yet, see Issue #29: https://github.com/ejrgilbert/whamm/issues/141".to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::TY_I32,
                    Rule::TY_BOOL,
                    Rule::TY_STRING,
                    Rule::TY_TUPLE,
                    Rule::TY_MAP,
                ],
                vec![pair.as_rule()],
            );
            DataType::I64
        }
        Rule::TY_F64 => {
            err.parse_error(
                true,
                Some("f64 not supported yet, see Issue #29: https://github.com/ejrgilbert/whamm/issues/141".to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::TY_I32,
                    Rule::TY_BOOL,
                    Rule::TY_STRING,
                    Rule::TY_TUPLE,
                    Rule::TY_MAP,
                ],
                vec![pair.as_rule()],
            );
            DataType::F64
        }
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
                    Rule::TY_U32,
                    Rule::TY_I32,
                    Rule::TY_F32,
                    Rule::TY_U64,
                    Rule::TY_I64,
                    Rule::TY_F64,
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

// EXPRESSIONS

fn handle_id(pair: Pair<Rule>) -> Expr {
    Expr::VarId {
        definition: Definition::User,
        name: pair.as_str().parse().unwrap(),
        loc: Some(Location {
            line_col: LineColLocation::from(pair.as_span()),
            path: None,
        }),
    }
}

fn handle_tuple(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
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

fn handle_int(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let val = pair.as_str().parse::<i32>().unwrap();
    Ok(Expr::Primitive {
        val: Value::I32 {
            ty: DataType::I32,
            val,
        },
        loc: Some(Location {
            line_col: LineColLocation::from(pair.as_span()),
            path: None,
        }),
    })
}

fn handle_dec(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let val = pair.as_str().parse::<f32>().unwrap();

    Ok(Expr::Primitive {
        val: Value::F32 {
            ty: DataType::F32,
            val,
        },
        loc: Some(Location {
            line_col: LineColLocation::from(pair.as_span()),
            path: None,
        }),
    })
}

fn handle_bool(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let val = pair.as_str().parse::<bool>().unwrap();

    Ok(Expr::Primitive {
        val: Value::Boolean {
            ty: DataType::Boolean,
            val,
        },
        loc: Some(Location {
            line_col: LineColLocation::from(pair.as_span()),
            path: None,
        }),
    })
}

fn handle_string(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
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

    Ok(Expr::Primitive {
        val: Value::Str {
            ty: DataType::Str,
            val,
        },
        loc: Some(Location {
            line_col: LineColLocation::from(pair.as_span()),
            path: None,
        }),
    })
}

fn handle_map_get(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let loc = LineColLocation::from(pair.clone().as_span());
    let mut pairs = pair.into_inner();
    let map = pairs.next().unwrap();
    let key = pairs.next().unwrap();

    //this SHOULD be a VarId
    let map_expr = match expr_primary(map) {
        Ok(expr) => expr,
        Err(errors) => {
            return Err(errors);
        }
    };
    let key_expr = match expr_primary(key) {
        Ok(expr) => expr,
        Err(errors) => {
            return Err(errors);
        }
    };
    Ok(Expr::MapGet {
        map: Box::new(map_expr),
        key: Box::new(key_expr),
        loc: Some(Location {
            line_col: loc,
            path: None,
        }),
    })
}

// STATEMENTS

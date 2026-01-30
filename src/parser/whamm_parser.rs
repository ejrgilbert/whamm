use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{long_line, magenta, white};
use crate::parser::provider_handler::{PrintInfo, ProviderDef, get_matches, yml_to_providers};
use crate::parser::types;
use crate::parser::types::Statement::LibImport;
use crate::parser::types::{
    Annotation, BinOp, Block, DataType, Definition, Expr, FnId, Location, NumFmt, NumLit,
    PRATT_PARSER, ProbeRule, Rule, RulePart, Script, Statement, UnOp, Value, Whamm, WhammParser,
    print_bound_vars, print_fns,
};
use log::trace;
use pest::Parser;
use pest::error::{Error, LineColLocation};
use pest::iterators::{Pair, Pairs};
use std::process::exit;
use std::str::FromStr;
use termcolor::{BufferWriter, ColorChoice, WriteColor};

const UNEXPECTED_ERR_MSG: &str =
    "WhammParser: Looks like you've found a bug...please report this behavior! Exiting now...";

pub fn print_info(
    rule: String,
    def_yamls: &[String],
    print_vars: bool,
    print_functions: bool,
    err: &mut ErrorGen,
) -> Result<(), Box<ErrorGen>> {
    let def = yml_to_providers(def_yamls)?;
    assert!(!def.is_empty());

    trace!("Entered print_info");
    err.set_script_text(rule.to_owned());

    let writer = BufferWriter::stdout(ColorChoice::Always);
    let mut whamm_buffer = writer.buffer();

    // Print `whamm` info
    let mut tabs = 0;
    if print_vars || print_functions {
        white(true, "\nCORE ".to_string(), &mut whamm_buffer);
        magenta(true, "`whamm`".to_string(), &mut whamm_buffer);
        white(true, " FUNCTIONALITY\n\n".to_string(), &mut whamm_buffer);

        // Print the vars
        if print_vars {
            let vars = Whamm::get_bound_vars();
            print_bound_vars(&mut tabs, &vars, &mut whamm_buffer);
        }

        // Print the functions
        if print_functions {
            let functions = Whamm::get_bound_fns();
            print_fns(&mut tabs, &functions, &mut whamm_buffer);
        }
    }

    long_line(&mut whamm_buffer);
    white(true, "\n\n".to_string(), &mut whamm_buffer);

    let mut prov_buff = writer.buffer();
    let mut pkg_buff = writer.buffer();
    let mut evt_buff = writer.buffer();

    let res = WhammParser::parse(Rule::PROBE_RULE, &rule);
    match res {
        Ok(mut pairs) => {
            // Create the probe rule from the input string
            let probe_rule = handle_probe_rule(pairs.next().unwrap(), err);

            // Print the information for the passed probe rule
            let matches = get_matches(&probe_rule, &def, err);
            let mut tabs = 0;
            if !matches.is_empty() {
                probe_rule.print_bold_provider(&mut prov_buff);
                for provider in matches.iter() {
                    provider.print_info(
                        &probe_rule,
                        print_vars,
                        print_functions,
                        &mut prov_buff,
                        &mut pkg_buff,
                        &mut evt_buff,
                        &mut tabs,
                    );
                }
            }
        }
        Err(e) => {
            err.pest_err(e);
        }
    }

    let mut buffs = [whamm_buffer, prov_buff, pkg_buff, evt_buff];

    for buff in buffs.iter_mut() {
        writer
            .print(buff)
            .expect("Uh oh, something went wrong while printing to terminal");
        buff.reset()
            .expect("Uh oh, something went wrong while printing to terminal");
    }
    Ok(())
}

pub fn parse_script(def_yamls: &Vec<String>, script: &String, err: &mut ErrorGen) -> Option<Whamm> {
    trace!("Entered parse_script");
    err.set_script_text(script.to_owned());

    let res = WhammParser::parse(Rule::script, script);
    match res {
        Ok(mut pairs) => {
            let res = to_ast(
                def_yamls,
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

fn to_ast(
    def_yamls: &Vec<String>,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) -> Result<Whamm, Box<Error<Rule>>> {
    trace!("Entered to_ast");

    // Create initial AST with Whamm node
    let mut whamm = Whamm::new();
    let script_count = 0;

    match pair.as_rule() {
        Rule::script => {
            if let Err(mut e) = parser_entry_point(def_yamls, &mut whamm, script_count, pair, err) {
                e.report();
                exit(1);
            }
        }
        rule => {
            err.parse_error(
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::script],
                vec![rule],
            );
        }
    }

    Ok(whamm)
}

// =======================
// = Pair Handling Logic =
// =======================

fn parser_entry_point(
    def_yamls: &Vec<String>,
    whamm: &mut Whamm,
    script_count: usize,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) -> Result<(), Box<ErrorGen>> {
    let def = yml_to_providers(def_yamls)?;
    assert!(!def.is_empty());

    trace!("Enter process_pair");
    match pair.as_rule() {
        Rule::script => {
            trace!("Begin process script");
            handle_script(def_yamls, whamm, pair, err)?;
            trace!("End process script");
        }
        Rule::lib_import => handle_lib_import(whamm, script_count, pair),
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
            handle_probe_def(whamm, &def, script_count, pair, err);
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
        }
    }
    trace!("Exit process_pair");
    Ok(())
}

pub fn handle_script(
    def_yamls: &Vec<String>,
    whamm: &mut Whamm,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) -> Result<(), Box<ErrorGen>> {
    let base_script = Script::new();
    let new_script_count = whamm.add_script(base_script);

    for p in pair.into_inner() {
        parser_entry_point(def_yamls, whamm, new_script_count, p, err)?;
    }
    Ok(())
}

pub fn handle_lib_import(whamm: &mut Whamm, script_count: usize, pair: Pair<Rule>) {
    // Add lib import to the script
    let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();

    let lib_name: Pair<Rule> = pair.into_inner().next().unwrap();
    let lib_import = LibImport {
        lib_name: lib_name.as_str().parse().unwrap(),
        loc: Some(Location {
            line_col: LineColLocation::from(lib_name.as_span()),
            path: None,
        }),
    };

    script.add_global_stmts(vec![lib_import]);
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
    prov_def: &[ProviderDef],
    script_count: usize,
    pair: Pair<Rule>,
    err: &mut ErrorGen,
) {
    let mut pair = pair.into_inner();
    let rule_rule = pair.next().unwrap();
    // Get out the rule info
    let probe_rule = handle_probe_rule(rule_rule, err);

    // Get out the probe predicate/body contents
    let next = pair.next();
    let (this_predicate, this_body) = match next {
        Some(mut n) => {
            let this_predicate = if matches!(n.as_rule(), Rule::predicate) {
                let res = match handle_expr(n.into_inner().next().unwrap()) {
                    Ok(res) => Some(res),
                    Err(errors) => {
                        let _ = err.add_errors(errors);
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
                if block.is_empty() { None } else { Some(block) }
            } else {
                None
            };

            (this_predicate, this_body)
        }
        None => (None, None),
    };

    // Add probe definition to the script
    let script: &mut Script = whamm.scripts.get_mut(script_count).unwrap();
    script.add_probe(&probe_rule, prov_def, this_predicate, this_body, err);
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
                type_from_rule_handler(pair, err)
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
        results: return_ty,
    });
}

fn handle_probe_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> ProbeRule {
    match pair.as_rule() {
        Rule::PROBE_RULE => probe_rule_from_rule(pair, err),
        rule => {
            err.parse_error(
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::PROBE_RULE],
                vec![rule],
            );
            ProbeRule::default()
        }
    }
}

pub fn expr_from_pair(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    match pair.as_rule() {
        Rule::assignment_rhs => expr_from_pair(pair.into_inner().next().unwrap()),
        Rule::ternary => handle_ternary(pair),
        Rule::arg => handle_arg(pair),
        Rule::expr => handle_expr(pair),
        rule => Err(vec![ErrorGen::get_parse_error(
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
        results: None,
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
        Rule::decl => handle_decl(&mut pair.into_inner(), err),
        Rule::assignment => handle_assignment(pair, err),
        Rule::lib_call => handle_lib_call_outer(pair, err),
        Rule::fn_call => handle_function_call_outer(pair, err),
        Rule::incrementor => handle_incrementor(pair, err),
        Rule::decrementor => handle_decrementor(pair, err),
        Rule::ret => handle_ret(pair, err),
        Rule::decl_init => handle_decl_init(pair, err),
        Rule::if_stmt => handle_if(pair, err),
        Rule::special_decl => handle_special_decl(pair, err),
        rule => {
            err.parse_error(
                None,
                Some(LineColLocation::from(pair.as_span())),
                vec![
                    Rule::statement,
                    Rule::decl,
                    Rule::assignment,
                    Rule::lib_call,
                    Rule::fn_call,
                    Rule::incrementor,
                    Rule::decrementor,
                    Rule::ret,
                    Rule::decl_init,
                    Rule::special_decl,
                ],
                vec![rule],
            );
            vec![]
        }
    }
}

fn handle_decl(pair: &mut Pairs<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let var_id_rule = pair.next().unwrap();
    let var_id_line_col = LineColLocation::from(var_id_rule.as_span());

    let type_rule = pair.next().unwrap();
    let type_line_col = LineColLocation::from(type_rule.as_span());
    let ty = type_from_rule_handler(type_rule, err);

    trace!("Exiting declaration");
    vec![Statement::Decl {
        ty,
        var_id: handle_id(var_id_rule),
        loc: Some(Location::from(&var_id_line_col, &type_line_col, None)),
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
            let _ = err.add_errors(errors);
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

fn handle_lib_call_outer(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    match handle_lib_call(pair) {
        Ok(call) => {
            let call_loc = call.loc().clone();
            vec![Statement::Expr {
                expr: call,
                loc: call_loc,
            }]
        }
        Err(errors) => {
            let _ = err.add_errors(errors);
            vec![]
        }
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
            let _ = err.add_errors(errors);
            vec![]
        }
    }
}
fn handle_incrementor(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    vec![handle_custom_binop(
        BinOp::Add,
        Expr::one(LineColLocation::from(pair.as_span())),
        pair,
        err,
    )]
}

fn handle_decrementor(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    vec![handle_custom_binop(
        BinOp::Subtract,
        Expr::one(LineColLocation::from(pair.as_span())),
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
            let _ = err.add_errors(errors);
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

        done_on: DataType::Unknown,
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
                let _ = err.add_errors(errors);
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

fn handle_decl_init(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let pair_loc = pair.as_span();
    let mut pairs = pair.into_inner();

    // create the decl
    let decl_pair = pairs.next().unwrap();
    let decls = match decl_pair.as_rule() {
        Rule::special_decl => handle_special_decl(decl_pair, err),
        Rule::decl => handle_decl(&mut decl_pair.into_inner(), err),
        rule => {
            err.parse_error(
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair_loc)),
                vec![Rule::special_decl, Rule::decl],
                vec![rule],
            );
            vec![]
        }
    };
    let decl = decls.first().unwrap();

    // get the assignment
    let expr_rule = pairs.next().unwrap();
    let expr_line_col = LineColLocation::from(expr_rule.as_span());
    match expr_from_pair(expr_rule) {
        Ok(expr) => {
            let loc = decl
                .line_col()
                .map(|loc| Location::from(&loc.clone(), &expr_line_col, None));

            match decl {
                Statement::Decl { var_id, .. } => {
                    vec![
                        decl.to_owned(),
                        Statement::Assign {
                            var_id: var_id.clone(),
                            expr,
                            loc,
                        },
                    ]
                }
                Statement::UnsharedDecl { decl: inner, .. } => {
                    if let Statement::Decl { var_id, .. } = &**inner {
                        let assign = Statement::Assign {
                            var_id: var_id.clone(),
                            expr,
                            loc: loc.clone(),
                        };
                        vec![Statement::UnsharedDeclInit {
                            decl: Box::new(decl.to_owned()),
                            init: Box::new(assign),
                            loc,
                        }]
                    } else {
                        vec![]
                    }
                }
                _ => vec![],
            }
        }
        Err(errors) => {
            let _ = err.add_errors(errors);
            vec![]
        }
    }
}
fn handle_if(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let if_stmt_line_col: LineColLocation = LineColLocation::from(pair.as_span());
    let mut pairs = pair.into_inner();

    // get the conditional
    let cond = match expr_from_pair(pairs.next().unwrap()) {
        Ok(expr) => expr,
        Err(errors) => {
            let _ = err.add_errors(errors);
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
                    results: None,
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
            let _ = err.add_errors(errors);
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
                    results: None,
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
            results: None,
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
        results: None,
        loc: Some(Location {
            line_col: alt_loc.clone(),
            path: None,
        }),
    }
}

fn handle_special_decl(pair: Pair<Rule>, err: &mut ErrorGen) -> Vec<Statement> {
    let line_col = LineColLocation::from(pair.as_span());
    let mut pairs = pair.into_inner();

    // handle var_decorator(s)
    let mut is_report = false;
    let mut is_unshared = false;
    let decorator_pairs = pairs.next().unwrap().into_inner();
    for pair in decorator_pairs {
        match pair.as_rule() {
            Rule::REPORT => {
                if is_report {
                    // cannot mark as report 2x, error
                    err.parse_error(
                        Some(
                            "Marked variable with 'report' multiple times, should only mark once."
                                .to_string(),
                        ),
                        Some(LineColLocation::from(pair.as_span())),
                        vec![],
                        vec![],
                    )
                }
                is_report = true
            }
            Rule::UNSHARED => {
                if is_unshared {
                    // cannot mark as unshared 2x, error
                    err.parse_error(
                        Some(
                            "Marked variable with 'unshared' multiple times, should only mark once."
                                .to_string(),
                        ),
                        Some(LineColLocation::from(pair.as_span())),
                        vec![],
                        vec![],
                    )
                }
                is_unshared = true
            }
            rule => {
                err.parse_error(
                    Some(UNEXPECTED_ERR_MSG.to_string()),
                    Some(LineColLocation::from(pair.as_span())),
                    vec![Rule::REPORT, Rule::UNSHARED],
                    vec![rule],
                );
            }
        }
    }

    // next should be the declaration
    let decl = stmt_from_rule(pairs.next().unwrap(), err);
    vec![Statement::UnsharedDecl {
        is_report,
        decl: Box::new(decl.first().unwrap().to_owned()),
        loc: Some(Location {
            line_col,
            path: None,
        }),
    }]
}
// EXPRESSIONS

pub fn handle_param(mut pairs: Pairs<Rule>, err: &mut ErrorGen) -> Option<(Expr, DataType)> {
    if let Some(id_rule) = pairs.next() {
        // process the name
        let id = handle_id(id_rule);
        // process the type
        let ty = type_from_rule_handler(pairs.next().unwrap(), err);
        Some((id, ty))
    } else {
        None
    }
}

fn handle_lib_call(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let loc = LineColLocation::from(pair.as_span());
    let mut pairs = pair.into_inner();

    // check if we have an annotation on this library call
    let s = pairs.next().unwrap();
    let (annotation, lib_name) = if s.as_str().starts_with("@") {
        let annotation = handle_annotation(s)?;
        let lib_name_rule = pairs.next().unwrap();
        (Some(annotation), lib_name_rule.as_str().to_string())
    } else {
        (None, s.as_str().to_string())
    };

    // handle lib func call
    let lib_func_call = handle_fn_call(pairs.next().unwrap())?;

    Ok(Expr::LibCall {
        annotation,
        lib_name,
        call: Box::new(lib_func_call),
        results: None,
        loc: Some(Location {
            line_col: loc,
            path: None,
        }),
    })
}

fn handle_annotation(pair: Pair<Rule>) -> Result<Annotation, Vec<WhammError>> {
    let mut pairs = pair.into_inner();
    let annotation_pair = pairs.next().unwrap();
    let annotation_string = annotation_pair.as_str();

    Annotation::try_from(annotation_string).map_err(|err| {
        vec![ErrorGen::get_parse_error(
            Some(err),
            Some(LineColLocation::from(annotation_pair.as_span())),
            vec![],
            vec![],
        )]
    })
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
pub fn handle_expr(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let pairs = pair.into_inner();
    PRATT_PARSER
        .map_primary(|primary| -> Result<Expr, Vec<WhammError>> { expr_primary(primary) })
        .map_prefix(|op, rhs| -> Result<Expr, Vec<WhammError>> {
            match rhs {
                Ok(rhs) => {
                    let op = match op.as_rule() {
                        Rule::neg => UnOp::Not,
                        Rule::binary_not => UnOp::BitwiseNot,
                        rule => {
                            return Err(vec![ErrorGen::get_parse_error(
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                Some(LineColLocation::from(op.as_span())),
                                vec![Rule::neg],
                                vec![rule],
                            )]);
                        }
                    };
                    let loc = rhs.loc().clone();

                    Ok(Expr::UnOp {
                        op,
                        expr: Box::new(rhs),
                        done_on: DataType::Unknown,
                        loc,
                    })
                }
                Err(errors) => Err(errors),
            }
        })
        .map_infix(|lhs, op, rhs| -> Result<Expr, Vec<WhammError>> {
            match (lhs, rhs) {
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

                        // Bitwise Operators
                        Rule::lshift => BinOp::LShift,
                        Rule::rshift => BinOp::RShift,
                        Rule::binary_and => BinOp::BitAnd,
                        Rule::binary_or => BinOp::BitOr,
                        Rule::binary_xor => BinOp::BitXor,

                        rule => {
                            return Err(vec![ErrorGen::get_parse_error(
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
                                    Rule::lshift,
                                    Rule::rshift,
                                    Rule::binary_and,
                                    Rule::binary_or,
                                    Rule::binary_xor,
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
                        done_on: DataType::Unknown,
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
            }
        })
        .map_postfix(|lhs, op| -> Result<Expr, Vec<WhammError>> {
            match lhs {
                Ok(lhs) => {
                    let op_rule = op.as_rule();
                    let op_span = op.as_span();
                    let target = type_from_rule(op.into_inner().next().unwrap())?;

                    let op = match op_rule {
                        Rule::cast => UnOp::Cast { target },
                        rule => {
                            return Err(vec![ErrorGen::get_parse_error(
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                Some(LineColLocation::from(op_span)),
                                vec![Rule::cast],
                                vec![rule],
                            )]);
                        }
                    };
                    let loc = lhs.loc().clone();

                    Ok(Expr::UnOp {
                        op,
                        expr: Box::new(lhs),
                        done_on: DataType::Unknown,
                        loc,
                    })
                }
                Err(errors) => Err(errors),
            }
        })
        .parse(pairs)
}

fn expr_primary(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    match pair.as_rule() {
        Rule::lib_call => handle_lib_call(pair),
        Rule::fn_call => handle_fn_call(pair),
        Rule::ID => Ok(handle_id(pair)),
        Rule::tuple => handle_tuple(pair),
        Rule::INT => handle_int(pair),
        Rule::FLOAT => handle_float(pair),
        Rule::BOOL => handle_bool(pair),
        Rule::STRING => handle_string(pair),
        Rule::get_map => handle_map_get(pair),
        _ => expr_from_pair(pair),
    }
}

// ================================
// = LOWER-LEVEL HELPER FUNCTIONS =
// ================================

// PROBE MATCH RULES

fn probe_rule_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> ProbeRule {
    let rule_as_str = pair.as_str();
    let mut parts = pair.into_inner();

    let simplified = if let Some((prefix, postfix)) = rule_as_str.split_once("(") {
        let (_, after) = postfix.split_once(")").unwrap();
        format!("{prefix}{after}")
    } else {
        rule_as_str.to_string()
    };

    let str_parts = simplified.split(':');

    let mut probe_rule = ProbeRule::new();
    let mut contents: Vec<String> = vec![];
    let mut next = parts.next();
    for s in str_parts {
        if s.trim().is_empty() {
            probe_rule.add_rule_def(RulePart::new("*".to_string(), None));
            contents.push("*".to_string());
            continue;
        }

        let mut rule_part = match next {
            Some(part) => match part.as_rule() {
                Rule::PROBE_ID => {
                    next = parts.next();
                    probe_rule_part_from_rule(part.clone(), err)
                }
                _ => {
                    next = parts.next();
                    RulePart::new("*".to_string(), None)
                }
            },
            None => {
                break;
            }
        };

        // check if there is type info associated with this probe part
        if let Some(n) = next.clone() {
            if matches!(n.as_rule(), Rule::TY_BOUNDS) {
                let mut param_pairs = n.into_inner();

                let mut params = vec![];
                let mut next_param = param_pairs.next();
                while let Some(n) = &next_param {
                    if matches!(n.as_rule(), Rule::param) {
                        if let Some(param) = handle_param(n.clone().into_inner(), err) {
                            params.push(param)
                        }
                        next_param = param_pairs.next();
                    } else {
                        break;
                    }
                }

                // let params = handle_params(&mut parts, err);
                next = parts.next();
                rule_part.ty_info = params;
            }
        }
        probe_rule.add_rule_def(rule_part);
    }
    trace!("Exiting probe_rule_from_rule");

    probe_rule
}

fn probe_rule_part_from_rule(pair: Pair<Rule>, err: &mut ErrorGen) -> RulePart {
    match pair.as_rule() {
        Rule::PROBE_ID => {
            trace!("Entering PROBE_ID");
            let name: String = pair.as_str().parse().unwrap();
            let id_line_col = LineColLocation::from(pair.as_span());

            let part = RulePart::new(
                name,
                Some(Location {
                    line_col: id_line_col,
                    path: None,
                }),
            );
            trace!("Exiting PROBE_ID");

            trace!("Exiting probe_rule_part_from_rule");
            part
        }
        rule => {
            err.parse_error(
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::PROBE_ID],
                vec![rule],
            );
            RulePart::default()
        }
    }
}

// TYPES
fn type_from_rule_handler(pair: Pair<Rule>, err: &mut ErrorGen) -> DataType {
    match type_from_rule(pair) {
        Ok(ty) => ty,
        Err(errors) => {
            let _ = err.add_errors(errors);
            DataType::Unknown
        }
    }
}

pub fn type_from_rule(pair: Pair<Rule>) -> Result<DataType, Vec<WhammError>> {
    trace!("Entering type_from_rule");
    match pair.as_rule() {
        Rule::TY_U8 => Ok(DataType::U8),
        Rule::TY_I8 => Ok(DataType::I8),
        Rule::TY_U16 => Ok(DataType::U16),
        Rule::TY_I16 => Ok(DataType::I16),
        Rule::TY_U32 => Ok(DataType::U32),
        Rule::TY_I32 => Ok(DataType::I32),
        Rule::TY_F32 => Ok(DataType::F32),
        Rule::TY_U64 => Ok(DataType::U64),
        Rule::TY_I64 => Ok(DataType::I64),
        Rule::TY_F64 => Ok(DataType::F64),
        Rule::TY_BOOL => Ok(DataType::Boolean),
        Rule::TY_STRING => Ok(DataType::Str),
        Rule::TY_UNKNOWN => Ok(DataType::Unknown),
        Rule::TY_TUPLE => {
            let mut tuple_content_types = vec![];
            for p in pair.into_inner() {
                match type_from_rule(p) {
                    Ok(res) => tuple_content_types.push(res),
                    Err(e) => return Err(e),
                }
            }
            if tuple_content_types.is_empty() {
                Ok(DataType::Tuple { ty_info: vec![] })
            } else {
                Ok(DataType::Tuple {
                    ty_info: tuple_content_types,
                })
            }
        }
        Rule::TY_MAP => {
            let mut pair = pair.into_inner();
            let key_ty_rule = pair.next().unwrap();
            let val_ty_rule = pair.next().unwrap();

            let key_ty = type_from_rule(key_ty_rule)?;
            let val_ty = type_from_rule(val_ty_rule)?;

            Ok(DataType::Map {
                key_ty: Box::new(key_ty),
                val_ty: Box::new(val_ty),
            })
        }
        rule => Err(vec![ErrorGen::get_parse_error(
            Some(UNEXPECTED_ERR_MSG.to_string()),
            Some(LineColLocation::from(pair.as_span())),
            vec![
                Rule::TY_U8,
                Rule::TY_I8,
                Rule::TY_U16,
                Rule::TY_I16,
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
        )]),
    }
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

pub fn handle_int(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let pair = pair.into_inner().next().unwrap();
    // make uppercase and remove all '_'
    let token = pair.as_str().to_uppercase().replace("_", "");
    let mut digits = token.len() as i32;
    let is_neg = if let Some(first) = token.chars().next() {
        if first == '-' {
            // remove '-' from digits count
            digits -= 1;
            true
        } else {
            false
        }
    } else {
        // should always be at least two chars to even make it past pest parsing!
        unreachable!()
    };

    let (to_parse, fmt) = match pair.as_rule() {
        Rule::int_hex => {
            // remove '0x' from token to parse (required by rust utils)
            // but still keep '-' if used
            let delim = "0X";
            digits -= delim.len() as i32;

            // number of binary digits per hex char
            digits *= 4;
            (token.strip_prefix(delim).unwrap().to_string(), NumFmt::Hex)
        }
        Rule::int_bin => {
            // remove '0b' from token to parse (required by rust utils)
            // but still keep '-' if used
            let delim = "0B";
            digits -= delim.len() as i32;

            (token.strip_prefix(delim).unwrap().to_string(), NumFmt::Bin)
        }
        Rule::int => {
            // number of binary digits required to represent is unknown
            digits = -1;
            (token.clone(), NumFmt::Dec)
        }
        rule => {
            return Err(vec![ErrorGen::get_parse_error(
                Some(UNEXPECTED_ERR_MSG.to_string()),
                Some(LineColLocation::from(pair.as_span())),
                vec![Rule::int_hex, Rule::int_bin, Rule::int],
                vec![rule],
            )]);
        }
    };

    let val = if is_neg || fmt == NumFmt::Bin || fmt == NumFmt::Hex {
        // By default, always parse hex and binary as signed
        if digits > 32 {
            if let Ok(val) = i64::from_str_radix(&to_parse, fmt.base()) {
                Ok(NumLit::i64(val))
            } else if fmt == NumFmt::Bin || fmt == NumFmt::Hex {
                if let Ok(val) = u64::from_str_radix(&to_parse, fmt.base()) {
                    // convert and allow wrapping
                    Ok(NumLit::i64(val as i64))
                } else {
                    Err("i32 OR u32")
                }
            } else {
                Err("i64")
            }
        } else if digits >= 0 {
            if let Ok(val) = i32::from_str_radix(&to_parse, fmt.base()) {
                Ok(NumLit::i32(val))
            } else if fmt == NumFmt::Bin || fmt == NumFmt::Hex {
                if let Ok(val) = u32::from_str_radix(&to_parse, fmt.base()) {
                    // convert and allow wrapping
                    Ok(NumLit::i32(val as i32))
                } else {
                    Err("i32 OR u32")
                }
            } else {
                Err("i32")
            }
        } else {
            // num digits required is unknown, figure it out!
            if let Ok(val) = i32::from_str_radix(&to_parse, fmt.base()) {
                Ok(NumLit::i32(val))
            } else if let Ok(val) = i64::from_str_radix(&to_parse, fmt.base()) {
                Ok(NumLit::i64(val))
            } else {
                Err("i32 OR i64")
            }
        }
    } else if digits >= 32 {
        if let Ok(val) = u64::from_str_radix(&to_parse, fmt.base()) {
            Ok(NumLit::u64(val))
        } else {
            Err("u64")
        }
    } else if digits >= 0 {
        if let Ok(val) = u32::from_str_radix(&to_parse, fmt.base()) {
            Ok(NumLit::u32(val))
        } else {
            Err("u32")
        }
    } else {
        // num digits required is unknown, figure it out!
        if let Ok(val) = u32::from_str_radix(&to_parse, fmt.base()) {
            Ok(NumLit::u32(val))
        } else if let Ok(val) = u64::from_str_radix(&to_parse, fmt.base()) {
            Ok(NumLit::u64(val))
        } else {
            Err("u32 OR u64")
        }
    };

    match val {
        Ok(val) => Ok(Expr::Primitive {
            val: Value::Number {
                val,
                ty: DataType::U32,
                token: token.to_string(),
                fmt,
            },
            loc: Some(Location {
                line_col: LineColLocation::from(pair.as_span()),
                path: None,
            }),
        }),
        Err(ty) => Err(vec![ErrorGen::get_parse_error(
            Some(format!("Failed to parse value into {ty}: {token}")),
            Some(LineColLocation::from(pair.as_span())),
            vec![],
            vec![],
        )]),
    }
}

pub fn handle_float(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let pair = pair.into_inner().next().unwrap();
    // make lowercase and remove all '_'
    let token = pair.as_str().to_lowercase().replace("_", "");

    // num digits required is unknown, figure it out!
    let (val, ty) = if let Ok(val) = f32::from_str(&token) {
        let mut res = (NumLit::f32(val), DataType::F32);
        if val.is_infinite() && !token.contains("inf") {
            // try to parse as f64
            if let Ok(new_val) = f64::from_str(&token) {
                if !new_val.is_infinite() {
                    res = (NumLit::f64(new_val), DataType::F64)
                }
            }
        }
        res
    } else if let Ok(val) = f64::from_str(&token) {
        (NumLit::f64(val), DataType::F64)
    } else {
        return Err(vec![ErrorGen::get_parse_error(
            Some(format!("Failed to parse value into f32 OR f64: {token}")),
            Some(LineColLocation::from(pair.as_span())),
            vec![],
            vec![],
        )]);
    };

    Ok(Expr::Primitive {
        val: Value::Number {
            val,
            ty,
            token: token.to_string(),
            fmt: NumFmt::Dec,
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
        val: Value::Boolean { val },
        loc: Some(Location {
            line_col: LineColLocation::from(pair.as_span()),
            path: None,
        }),
    })
}

fn handle_string(pair: Pair<Rule>) -> Result<Expr, Vec<WhammError>> {
    let mut val: String = pair.as_str().to_string();
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
        val: Value::Str { val },
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

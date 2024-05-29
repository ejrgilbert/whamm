// =================
// = Setup Logging =
// =================

use std::process::exit;
use log::error;
use crate::common::error::ErrorGen;
use crate::generator::types::ExprFolder;
use crate::parser::tests;
use crate::parser::types::Expr::{BinOp, VarId};
use crate::parser::types::{DataType, Whamm, Expr, Op, Value};
use crate::verifier::types::{Record, ScopeType, SymbolTable};
use crate::verifier::verifier;

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_rec<'a>(table: &'a mut SymbolTable, name: &str) -> Option<&'a mut Record> {
    let var_rec_id = match table.lookup(&name.to_string()) {
        Some(id) => id.clone(),
        None => {
            error!("Variable symbol does not exist for name {}!", name);
            println!("{:#?}", table);
            exit(1);
        }
    };

    return match table.get_record_mut(&var_rec_id) {
        Some(rec) => {
            Some(rec)
        },
        _ => {
            error!("Variable symbol does not exist!");
            None
        }
    }
}

fn get_pred(whamm: &Whamm) -> &Expr {
    whamm.scripts.get(0).unwrap()
        .providers.get("wasm").unwrap()
        .packages.get("bytecode").unwrap()
        .events.get("call").unwrap()
        .probe_map.get("alt").unwrap()
        .get(0).unwrap().predicate.as_ref().unwrap()
}

fn move_through_scopes_til_match(desired_ty: ScopeType, table: &mut SymbolTable, err: &mut ErrorGen) {
    while table.get_curr_scope().unwrap().ty != desired_ty {
        match table.exit_scope() {
            Err(e) => {
                err.add_error(e);
                err.report();
            },
            _ => {}
        }
        match table.enter_scope() {
            Err(e) => {
                err.add_error(e);
                err.report();
            },
            _ => {}
        }
    }
}

fn hardcode_compiler_constants(table: &mut SymbolTable, err: &mut ErrorGen) {
    match table.enter_scope() {
        Err(e) => {
            err.add_error(e);
            err.report();
        },
        _ => {}
    }
    move_through_scopes_til_match(ScopeType::Script, table, err);
    println!("Scope name: {}", table.get_curr_scope().unwrap().name);
    // enter wasm scope
    match table.enter_scope() {
        Err(e) => {
            err.add_error(e);
            err.report();
        },
        _ => {}
    }
    move_through_scopes_til_match(ScopeType::Provider, table, err);
    println!("Scope name: {}", table.get_curr_scope().unwrap().name);
    // enter bytecode scope
    match table.enter_scope() {
        Err(e) => {
            err.add_error(e);
            err.report();
        },
        _ => {}
    }
    move_through_scopes_til_match(ScopeType::Package, table, err);
    println!("Scope name: {}", table.get_curr_scope().unwrap().name);
    // enter call scope
    match table.enter_scope() {
        Err(e) => {
            err.add_error(e);
            err.report();
        },
        _ => {}
    }
    while table.get_curr_scope().unwrap().ty != ScopeType::Event {
        match table.exit_scope() {
            Err(e) => {
                err.add_error(e);
                err.report();
            },
            _ => {}
        }
        match table.enter_scope() {
            Err(e) => {
                err.add_error(e);
                err.report();
            },
            _ => {}
        }
    }

    // define target_fn_type
    let target_fn_type = get_rec(table, "target_fn_type");
    if let Some(Record::Var {value, ..}) = target_fn_type {
        *value = Some(Value::Str {
            ty: DataType::Str,
            val: "import".to_string(),
            addr: None
        })
    } else {
        error!("Could not find symbol for `target_fn_type`");
        assert!(false);
    }

    // define target_imp_module
    let target_imp_module = get_rec(table, "target_imp_module");
    if let Some(Record::Var {value, ..}) = target_imp_module {
        *value = Some(Value::Str {
            ty: DataType::Str,
            val: "ic0".to_string(),
            addr: None
        })
    } else {
        error!("Could not find symbol for `target_imp_module`");
        assert!(false);
    }

    // define target_imp_name
    let target_imp_name = get_rec(table, "target_imp_name");
    if let Some(Record::Var {value, ..}) = target_imp_name {
        *value = Some(Value::Str {
            ty: DataType::Str,
            val: "call_new".to_string(),
            addr: None
        })
    } else {
        error!("Could not find symbol for `target_imp_name`");
        assert!(false);
    }
}

fn assert_simplified_predicate(pred: &Expr) {
    // ExprFolder should not be able to simplify the expression at all.
    if let VarId{ name, .. } = pred {
        assert_eq!("i", name);
    } else {
        // failed!
        error!("ExprFolder did not fold correctly...");
        print!("{:#?}\n", pred);
        assert!(false);
    }
}

fn basic_run(script: &str, err: &mut ErrorGen) {
    match tests::get_ast(script, err) {
        Some(whamm) => {
            let mut table = verifier::build_symbol_table(&whamm, err);
            table.reset();

            let pred = get_pred(&whamm);
            hardcode_compiler_constants(&mut table, err);

            let folded_expr = ExprFolder::fold_expr(pred, &table);
            assert_simplified_predicate(&folded_expr);
        },
        None => {
            error!("Could not get ast from script: {}", script);
            err.report();
            assert!(false);
        }
    };
}

#[test]
pub fn basic_test() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    basic_run("wasm::call:alt / i / {}", &mut err);
}

#[test]
pub fn single_prim() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    basic_run(r#"
wasm::call:alt /
    true && i
/ {}
    "#, &mut err);
}

#[test]
pub fn basic_with_compiler_vars() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    basic_run(r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new" &&
    i
/ {}
    "#, &mut err);
}

fn asserts_on_call(call: &Expr) {
    if let Expr::Call {
        fn_target,
        args,
        ..
    } = call {
        if let VarId { name , ..} = &**fn_target {
            assert_eq!("strcmp", name);
        } else {
            error!("ExprFolder did not fold correctly...");
            assert!(false);
        }

        let args = args.as_ref().unwrap();
        assert_eq!(2, args.len());

        let tuple = &**args.get(0).unwrap();
        if let Expr::Primitive { val: Value::Tuple {vals, ..}, ..} = tuple {
            assert_eq!(2, vals.len());
        } else {
            error!("ExprFolder did not fold correctly...");
            assert!(false);
        }
    }
}

#[test]
pub fn basic_with_fn_call() {
    setup_logger();
    let script =r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    match tests::get_ast(script, &mut err) {
        Some(whamm) => {
            let mut table = verifier::build_symbol_table(&whamm, &mut err);
            table.reset();

            let pred = get_pred(&whamm);
            hardcode_compiler_constants(&mut table, &mut err);

            let folded_expr = ExprFolder::fold_expr(pred, &table);
            println!("{:#?}", folded_expr);

            // ExprFolder should not be able to simplify the Call expressions at all.
            if let BinOp{
                lhs,
                op,
                rhs,
                ..
            } = pred {
                assert_eq!(*op, Op::And);
                asserts_on_call(&**lhs);
                asserts_on_call(&**rhs);
            } else {
                // failed!
                error!("ExprFolder did not fold correctly...");
                print!("{:#?}\n", folded_expr);
                assert!(false);
            }
        },
        None => {
            error!("Could not get ast from script: {}", script);
            assert!(false);
        }
    };
}

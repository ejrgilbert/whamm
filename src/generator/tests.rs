// =================
// = Setup Logging =
// =================

use std::collections::HashMap;
use crate::common::error::ErrorGen;
use crate::generator::folding::ExprFolder;
use crate::parser::tests;
use crate::parser::types::Expr::{BinOp as ExprBinOp, VarId};
use crate::parser::types::{BinOp, DataType, Expr, Value, Whamm};
use crate::verifier::types::{Record, ScopeType, SymbolTable};
use crate::verifier::verifier;
use log::{debug, error};

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_rec<'a>(table: &'a mut SymbolTable, name: &str) -> Option<&'a mut Record> {
    let var_rec_id = match table.lookup(name) {
        Some(id) => id,
        None => {
            error!("Variable symbol does not exist for name {}!", name);
            println!("{:#?}", table);
            panic!();
        }
    };

    return match table.get_record_mut(var_rec_id) {
        Some(rec) => Some(rec),
        _ => {
            error!("Variable symbol does not exist!");
            None
        }
    };
}

fn get_pred(whamm: &Whamm) -> &Expr {
    whamm
        .scripts
        .first()
        .unwrap()
        .providers
        .get("wasm")
        .unwrap()
        .packages()
        .next()
        .unwrap()
        .events()
        .next()
        .unwrap()
        .probes()
        .get("alt")
        .unwrap()
        .first()
        .unwrap()
        .predicate()
        .as_ref()
        .unwrap()
}

fn move_through_scopes_til_match(
    desired_ty: ScopeType,
    table: &mut SymbolTable,
    err: &mut ErrorGen,
) {
    while table.get_curr_scope().unwrap().ty != desired_ty {
        table.exit_scope(err);
        err.report();
        table.enter_scope(err);
        err.report();
    }
}

fn hardcode_compiler_constants(table: &mut SymbolTable, err: &mut ErrorGen) {
    table.enter_scope(err);
    err.report();
    move_through_scopes_til_match(ScopeType::Script, table, err);
    debug!("Scope name: {}", table.get_curr_scope().unwrap().name);
    // enter wasm scope
    table.enter_scope(err);
    err.report();
    move_through_scopes_til_match(ScopeType::Provider, table, err);
    debug!("Scope name: {}", table.get_curr_scope().unwrap().name);
    // enter opcode scope
    table.enter_scope(err);
    err.report();
    move_through_scopes_til_match(ScopeType::Package, table, err);
    debug!("Scope name: {}", table.get_curr_scope().unwrap().name);
    // enter call scope
    table.enter_scope(err);
    err.report();
    while table.get_curr_scope().unwrap().ty != ScopeType::Event {
        table.exit_scope(err);
        err.report();
        table.enter_scope(err);
        err.report();
    }

    // define target_fn_type
    let target_fn_type = get_rec(table, "target_fn_type");
    if let Some(Record::Var { value, .. }) = target_fn_type {
        *value = Some(Value::Str {
            val: "import".to_string(),
        })
    } else {
        error!("Could not find symbol for `target_fn_type`");
        panic!();
    }

    // define target_imp_module
    let target_imp_module = get_rec(table, "target_imp_module");
    if let Some(Record::Var { value, .. }) = target_imp_module {
        *value = Some(Value::Str {
            val: "ic0".to_string(),
        })
    } else {
        error!("Could not find symbol for `target_imp_module`");
        panic!();
    }

    // define target_fn_name
    let target_fn_name = get_rec(table, "target_fn_name");
    if let Some(Record::Var { value, .. }) = target_fn_name {
        *value = Some(Value::Str {
            val: "call_new".to_string(),
        })
    } else {
        error!("Could not find symbol for `target_fn_name`");
        panic!();
    }
}

fn assert_simplified_predicate(pred: &Expr) {
    // ExprFolder should not be able to simplify the expression at all.
    if let VarId { name, .. } = pred {
        assert_eq!("i", name);
    } else {
        // failed!
        error!("ExprFolder did not fold correctly...");
        println!("{:#?}", pred);
        panic!();
    }
}

fn basic_run(script: &str, err: &mut ErrorGen) {
    let mut whamm = tests::get_ast(script, err);
    let mut table = verifier::build_symbol_table(&mut whamm, HashMap::default(), err);
    table.reset();

    let pred = get_pred(&whamm);
    hardcode_compiler_constants(&mut table, err);

    let folded_expr = ExprFolder::fold_expr(pred, &table, err);
    assert_simplified_predicate(&folded_expr);
}

#[test]
pub fn basic_test() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    basic_run("wasm::call:alt / i / {}", &mut err);
}

fn fatal_fold(expr: &Expr) {
    let result = std::panic::catch_unwind(|| {
        let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
        ExprFolder::fold_expr(expr, &SymbolTable::new(), &mut err);
    });
    match result {
        Ok(_) => {
            panic!("Expected a fatal error, but got Ok");
        }
        Err(_) => {
            //this means the function properly exited with a fatal error
        }
    }
}
#[test]
pub fn div_by_zero() {
    // 1 / 0
    fatal_fold(&Expr::BinOp {
        lhs: Box::new(Expr::Primitive {
            val: Value::gen_i32(1),
            loc: None,
        }),
        op: BinOp::Divide,
        rhs: Box::new(Expr::Primitive {
            val: Value::gen_i32(0),
            loc: None,
        }),
        done_on: DataType::U8,
        loc: None,
    })
}

#[test]
pub fn mod_by_zero() {
    // 1 % 0
    fatal_fold(&Expr::BinOp {
        lhs: Box::new(Expr::Primitive {
            val: Value::gen_i32(1),
            loc: None,
        }),
        op: BinOp::Modulo,
        rhs: Box::new(Expr::Primitive {
            val: Value::gen_i32(0),
            loc: None,
        }),
        done_on: DataType::U8,
        loc: None,
    })
}

#[test]
pub fn single_prim() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    basic_run(
        r#"
wasm::call:alt /
    true && i
/ {}
    "#,
        &mut err,
    );
}

#[test]
pub fn basic_with_compiler_vars() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    basic_run(
        r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
    i
/ {}
    "#,
        &mut err,
    );
}

fn asserts_on_call(call: &Expr) {
    if let Expr::Call {
        fn_target, args, ..
    } = call
    {
        if let VarId { name, .. } = &**fn_target {
            assert_eq!("strcmp", name);
        } else {
            error!("ExprFolder did not fold correctly...");
            panic!();
        }

        assert_eq!(2, args.len());

        let tuple = args.first().unwrap();
        if let Expr::Primitive {
            val: Value::Tuple { vals, .. },
            ..
        } = tuple
        {
            assert_eq!(2, vals.len());
        } else {
            error!("ExprFolder did not fold correctly...");
            panic!();
        }
    }
}

#[test]
pub fn basic_with_fn_call() {
    setup_logger();
    let script = r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    let mut whamm = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut whamm, HashMap::default(), &mut err);
    table.reset();

    let pred = get_pred(&whamm);
    hardcode_compiler_constants(&mut table, &mut err);

    let folded_expr = ExprFolder::fold_expr(pred, &table, &mut err);
    debug!("{:#?}", folded_expr);

    // ExprFolder should not be able to simplify the Call expressions at all.
    if let ExprBinOp { lhs, op, rhs, .. } = pred {
        assert_eq!(*op, BinOp::And);
        asserts_on_call(lhs);
        asserts_on_call(rhs);
    } else {
        // failed!
        error!("ExprFolder did not fold correctly...");
        println!("{:#?}", folded_expr);
        panic!();
    }
}

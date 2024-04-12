// =================
// = Setup Logging =
// =================

use std::process::exit;
use log::error;
use crate::generator::types::ExprFolder;
use crate::parser::tests;
use crate::parser::types::Expr::VarId;
use crate::parser::types::{DataType, Dtrace, Expr, Value};
use crate::verifier::types::{Record, ScopeType, SymbolTable};
use crate::verifier::verifier;

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn get_rec<'a>(table: &'a mut SymbolTable, name: &str) -> Option<&'a mut Record> {
    let var_rec_id = match table.lookup(&name.to_string()) {
        Some(id) => id.clone(),
        None => {
            error!("Variable symbol does not exist for name {name}!");
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

fn get_pred(dtrace: &Dtrace) -> &Expr {
    dtrace.dscripts.get(0).unwrap()
        .providers.get("wasm").unwrap()
        .modules.get("bytecode").unwrap()
        .functions.get("call").unwrap()
        .probe_map.get("alt").unwrap()
        .get(0).unwrap().predicate.as_ref().unwrap()
}

fn hardcode_compiler_constants(table: &mut SymbolTable) {
    table.enter_scope();
    while table.get_curr_scope().unwrap().ty != ScopeType::Dscript {
        table.exit_scope();
        table.enter_scope()
    }
    println!("Scope name: {}", table.get_curr_scope().unwrap().name);
    table.enter_scope(); // enter wasm scope
    while table.get_curr_scope().unwrap().ty != ScopeType::Provider {
        table.exit_scope();
        table.enter_scope()
    }
    println!("Scope name: {}", table.get_curr_scope().unwrap().name);
    table.enter_scope(); // enter bytecode scope
    while table.get_curr_scope().unwrap().ty != ScopeType::Module {
        table.exit_scope();
        table.enter_scope()
    }
    println!("Scope name: {}", table.get_curr_scope().unwrap().name);
    table.enter_scope(); // enter call scope
    while table.get_curr_scope().unwrap().ty != ScopeType::Function {
        table.exit_scope();
        table.enter_scope()
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
    if let VarId{ name } = pred {
        assert_eq!("i", name);
    } else {
        // failed!
        error!("ExprFolder did not fold correctly...");
        print!("{:#?}\n", pred);
        assert!(false);
    }
}

fn basic_run(script: &str) {
    match tests::get_ast(script) {
        Some(dtrace) => {
            let mut table = verifier::verify(&dtrace);
            table.reset();

            let pred = get_pred(&dtrace);
            hardcode_compiler_constants(&mut table);

            let folded_expr = ExprFolder::fold_expr(pred, &table);
            assert_simplified_predicate(&folded_expr);
        },
        None => {
            error!("Could not get ast from script: {script}");
            assert!(false);
        }
    };
}

#[test]
pub fn basic_test() {
    setup_logger();
    basic_run("wasm::call:alt / i / {}");
}

#[test]
pub fn single_prim() {
    setup_logger();
    basic_run(r#"
wasm::call:alt /
    true && i
/ {}
    "#);
}

#[test]
pub fn complex_test() {
    setup_logger();
    basic_run(r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new" &&
    i
/ {}
    "#);
}

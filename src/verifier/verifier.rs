use crate::verifier::types as verifier_types;
// use crate::verifier::providers;
use crate::parser::types as parser_types;

// use providers::*;
use parser_types::AstNode;
use verifier_types::{AllWasmFnProbes, SymbolTable, CoreProbe, WasmProbe, unbox};

use log::{error, trace};
use std::cell::RefCell;
use std::process::exit;

thread_local! {
    static NUM_DSCRIPTS: RefCell<i32> = RefCell::new(0);
}

pub fn verify(ast: &Vec<AstNode>) -> (SymbolTable, Vec<CoreProbe>, AllWasmFnProbes) {
    // Build the initial symbol table from the passed AST
    // This function also updates the AST component IDs to point to their corresponding
    // scope in the symbol table
    let table = build_symbol_table(&ast);

    // Organize all the probes in the AST
    // TODO -- might not need the IDs in the AST nodes!
    let (core_probes, wasm_fn_probes) = organize_probes(&ast);
    // If step 2 produced either of those probes, get the corresponding providers to add
    //      their symbols to the symbol table
    // add_provider_defs(&mut table, &core_probes, &wasm_fn_probes);

    return (table, core_probes, wasm_fn_probes);
}

// ================
// = SYMBOL TABLE =
// ================

fn build_symbol_table(ast: &Vec<AstNode>) -> SymbolTable {
    let mut table = SymbolTable::new();
    for node in ast {
        build_table_from_node(&node, &mut table);
    }

    return table;
}

fn build_table_from_node(node: &AstNode, table: &mut SymbolTable) {
    match node {
        AstNode::Dscript { probes } => {
            trace!("Enter Dscript node");
            let i = NUM_DSCRIPTS.with(|num| {
                let orig = num.borrow().clone();
                *num.borrow_mut() = orig + 1; // increment Dscript count
                orig
            });
            table.add_dscript(format!("Dscript{}", i));

            // TODO -- add variables provided by Dscript
            for probe in probes {
                build_table_from_node(probe, table);
            }
            trace!("Exit Dscript node");
        },
        AstNode::WasmProbe { module, function, name,
            predicate, body} => {
            trace!("Enter WasmProbe node");
            // TODO -- adding "wasm" here is a hack to keep from adding some other code
            table.add_probe(format!("wasm:{module}:{function}:{}", name.to_string()));

            // TODO -- add variables provided by WasmProbe

            // visit predicate
            match predicate {
                Some(pred) => {
                    build_table_from_node(pred, table);
                }
                None => {}
            };

            // visit body
            match body {
                Some(body) => {
                    for stmt in body {
                        build_table_from_node(&(**stmt), table);
                    }
                }
                None => {}
            };

            table.exit_scope();
            trace!("Exit WasmProbe node");
        },
        AstNode::CoreProbe {name, body} => {
            trace!("Enter CoreProbe node");
            table.add_probe(name.to_string());

            // TODO -- add variables provided by CoreProbe

            // visit body
            match body {
                Some(body) => {
                    for stmt in body {
                        build_table_from_node(&(**stmt), table);
                    }
                }
                None => {}
            };

            table.exit_scope();
            trace!("Exit CoreProbe node");
        }
        other => {
            table.print();
            error!("Visited unexpected node: {other:?}");
        }
    }
}

// ======================
// = PROBE ORGANIZATION =
// ======================

fn organize_probes(ast: &Vec<AstNode>) -> (Vec<CoreProbe>, AllWasmFnProbes) {
    let mut core_probes = vec![];
    let mut wasm_fn_probes = AllWasmFnProbes::new();

    for node in ast {
        if let AstNode::Dscript { probes, .. } = node {
            let probes = unbox(probes);
            (core_probes, wasm_fn_probes) = organize_probes(&probes);
        } else if let AstNode::CoreProbe{ name, body } = node {
            let b = match body {
                Some(bd) => Some(unbox(bd)),
                None => None,
            };
            // let i = match id {
            //     Some(content) => content,
            //     None => {
            //         error!("No ID for CoreProbe, something went wrong!");
            //         exit(1);
            //     }
            // };
            core_probes.push(CoreProbe {
                // id: 0,
                name: name.clone(),
                body: b
            });
        } else if let AstNode::WasmProbe{ module, function, name, predicate, body } = node {
            let pred = match predicate {
                Some(pred) => Some((**pred).clone()),
                None => None
            };
            let b = match body {
                Some(inner) => Some(unbox(inner)),
                None => None
            };
            // let i = match id {
            //     Some(content) => content,
            //     None => {
            //         error!("No ID for WasmProbe, something went wrong!");
            //         exit(1);
            //     }
            // };

            wasm_fn_probes.add_probe(module, function, name, WasmProbe {
                // id: i.clone(),
                predicate: pred,
                body: b
            });
        } else {
            error!("Expected Core or Wasm probe, received: {:?}", node);
            exit(1);
        }
    }

    return (core_probes, wasm_fn_probes);
}

// ========================
// = PROVIDER DEFINITIONS =
// ========================

// Traverse the AST
// - Add in functions/variables built into DtraceCore and ProbeTypes
//   to the SymbolTable for lookup
//   - This structure keeps a logical separation between the provider
//     symbols and the user symbols
//   - These functions should have a boolean `used` that is `true` if it's
//     been used in the body/predicate to show that it should be injected
//     into the bytecode
// - Enter the probe bodies/predicates
//   - If field/function is used, mark it by manipulating the `used` boolean
//   - Add user-defined variables to symbol table
// fn add_provider_defs(table: &mut SymbolTable, core_probes: &Vec<CoreProbe>, wasm_probes: &AllWasmFnProbes) -> bool {
//     // If step 2 produced either of those probes, get the corresponding providers to add
//     //      their symbols to the symbol table
//
// }

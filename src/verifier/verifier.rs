use std::cell::RefCell;
use crate::verifier::types as verifier_types;
use crate::parser::types as parser_types;

use parser_types::AstNode;
use verifier_types::{ScopeType, SymbolTable};

use log::{error, trace};

thread_local! {
    static NUM_DSCRIPTS: RefCell<i32> = RefCell::new(0);
}

pub fn build_symbol_table(ast: Vec<AstNode>) -> SymbolTable {
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
            table.add_probe(format!("{module}:{function}:{}", name.to_string()));

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

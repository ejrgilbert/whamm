use crate::parser::dtrace_parser::AstNode;

use log::error;
use std::process::exit;

// ================
// = Target: Wasm =
// ================

fn core_emit_wasm(_ast: &Vec<AstNode>, _probe: &AstNode, _app_wasm: &Vec<u8>) -> bool {
    error!("Not yet implemented");
    false
}

fn dfinity_emit_wasm(_ast: &Vec<AstNode>, _probe: &AstNode, _app_wasm: &Vec<u8>) -> bool {
    error!("Not yet implemented");
    false
}

pub fn emit_wasm(ast: &Vec<AstNode>, app_wasm: &Vec<u8>) -> bool {
    let mut success = true;
    for node in ast {
        success &= if let AstNode::Dscript { probes } = node {
            let probes = probes.into_iter().map(|item| {
                *item.clone()
            }).collect();
            emit_wasm(&probes, app_wasm)
        } else if let AstNode::CoreProbe{ .. } = node {
            core_emit_wasm(&ast, node, app_wasm)
        } else if let AstNode::DfinityProbe{ .. } = node {
            dfinity_emit_wasm(&ast, node, app_wasm)
        } else {
            error!("Expected Core or Dfinity probe, received: {:?}", node);
            exit(1);
        }
    }

    // At this point `app_wasm` should now contain the instrumented variation of the app code.
    return success;
}

// ================
// = Target: Wasi =
// ================

pub fn _emit_wasi(_ast: Vec<AstNode>, _app_wasm: &[u8]) -> bool {
    error!("Not yet implemented");
    false
}

// ==================
// = Target: Virgil =
// ==================

pub fn _emit_virgil(_ast: Vec<AstNode>) -> String {
    todo!()
}

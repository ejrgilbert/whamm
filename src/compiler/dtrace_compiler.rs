use crate::parser::dtrace_parser::AstNode;

use log::*;
// trait Emit {
//     // Returns instrumented variation of app Wasm code, needs to be flushed to file
//     fn emit_wasm(&self, app_wasm: &[u8]) -> &[u8];
//     // Returns instrumented variation of app Wasm code, needs to be flushed to file
//     fn emit_wasi(&self, app_wasm: &[u8]) -> &[u8];
//     // Returns String representation of Virgil code, needs to be flushed to file
//     fn emit_virgil(&self) -> String;
// }

// ================
// = Target: Wasm =
// ================

fn core_emit_wasm(_ast: &Vec<AstNode>, _probe: &AstNode, _app_wasm: &Vec<u8>) {
    todo!()
}

fn dfinity_emit_wasm(_ast: &Vec<AstNode>, _probe: &AstNode, _app_wasm: &Vec<u8>) {
    todo!()
}

pub fn emit_wasm(ast: &Vec<AstNode>, app_wasm: &Vec<u8>) {
    for node in ast {
        if let AstNode::Dscript { probes } = node {
            let probes = probes.into_iter().map(|item| {
                *item.clone()
            }).collect();
            emit_wasm(&probes, app_wasm);
        } else if let AstNode::CoreProbe{ .. } = node {
            core_emit_wasm(&ast, node, app_wasm);
        } else if let AstNode::DfinityProbe{ .. } = node {
            dfinity_emit_wasm(&ast, node, app_wasm);
        } else {
            error!("Expected Core or Dfinity probe, received: {:?}", node)
        }
    }

    // At this point `app_wasm` should now contain the instrumented variation of the app code.
}

// ================
// = Target: Wasi =
// ================

pub fn _emit_wasi(_ast: Vec<AstNode>, _app_wasm: &[u8]) -> &[u8] {
    todo!()
}

// ==================
// = Target: Virgil =
// ==================

pub fn _emit_virgil(_ast: Vec<AstNode>) -> String {
    todo!()
}

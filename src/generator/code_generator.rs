// ================
// = Target: Wasm =
// ================

use crate::generator::emitters::Emitter;
use crate::parser::types::AstNode;
use crate::verifier::types::{AllWasmFnProbes, CoreProbe, SymbolTable};

pub fn emit(emitter: &dyn Emitter, symbol_table: &SymbolTable, core_probes: &Vec<CoreProbe>, wasm_fn_probes: &AllWasmFnProbes) -> bool {
    for probe in core_probes {
        success &= emitter.emit_core_probe(symbol_table, probe);
    }

    for probe in wasm_fn_probes.all_probes {
        success &= emitter.emit_wasm_probe(symbol_table, probe.1);
    }

    if success {
        // At this point `emitter.app_wasm` should now contain the instrumented variation of the app code.
        success &= emitter.dump_to_file();
    }

    return success;
}

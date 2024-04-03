use log::error;
use crate::parser::types::{DataType, Dscript, Dtrace, DtraceVisitor, Expr, Function, Module, Op, Probe, Provider, Statement, Value};
use crate::verifier::types::SymbolTable;

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait Emitter {
    fn emit(&mut self, dtrace: &Dtrace) -> bool;
    fn dump_to_file(&mut self) -> bool;
}

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

pub(crate) struct WasmRewritingEmitter<'a> {
    pub(crate) app_wasm: walrus::Module,
    pub(crate) output_wasm_path: String,

    pub(crate) table: &'a SymbolTable
}
/// Walrus Visitor over `app.wasm`
/// - as we get relevant info, lookup in SymbolTable for binding to globally set that value
/// - for each bytecode, do we have a probe?
///   - fold predicate with known globals. FALSE? Don't inject! NOT FALSE? inject (with remaining Expr, not folded parts)
///   - See fold Rust pattern: https://rust-unofficial.github.io/patterns/patterns/creational/fold.html
/// - now we have instrumented `app.wasm`
///   - write to app_instr.wasm
impl Emitter for WasmRewritingEmitter<'_> {
    fn emit(&mut self, dtrace: &Dtrace) -> bool {
        let mut is_success = false;
        is_success &= self.visit_dtrace(dtrace);

        is_success
    }

    fn dump_to_file(&mut self) -> bool {
        match self.app_wasm.emit_wasm_file(&self.output_wasm_path) {
            Ok(_ok) => {
                true
            },
            Err(err) => {
                error!("Failed to dump instrumented wasm to {} from error: {}", &self.output_wasm_path, err);
                false
            },
        }
    }
}
// TODO -- this might need to have a first and second pass traversal.
// 1. emit fns and globals
// 2. emit probes
impl DtraceVisitor<bool> for WasmRewritingEmitter<'_> {
    fn visit_dtrace(&mut self, dtrace: &Dtrace) -> bool {
        // TODO -- inject fns
        // TODO -- inject globals
        // TODO -- define any compiler constants
        todo!()
    }

    fn visit_dscript(&mut self, dscript: &Dscript) -> bool {
        // TODO -- inject fns
        // TODO -- inject globals
        // TODO -- define any compiler constants
        todo!()
    }

    fn visit_provider(&mut self, provider: &Provider) -> bool {
        // TODO -- inject fns
        // TODO -- inject globals
        // TODO -- define any compiler constants
        todo!()
    }

    fn visit_module(&mut self, module: &Module) -> bool {
        // TODO -- inject fns
        // TODO -- inject globals
        // TODO -- define any compiler constants
        // TODO -- set up `walrus::ir::VisitorMut`
        //         at each bytecode as traversing IR, do we have a `function` for the bytecode?
        //         If so, enter that function
        todo!()
    }

    fn visit_function(&mut self, function: &Function) -> bool {
        // TODO -- inject fns
        // TODO -- inject globals
        // TODO -- define any compiler constants
        // TODO -- inject probes (should be at this point in the `walrus::ir::VisitorMut` since visited from `visit_module` above
        todo!()
    }

    fn visit_probe(&mut self, probe: &Probe) -> bool {
        // TODO -- inject fns
        // TODO -- inject globals
        // TODO -- define any compiler constants
        todo!()
    }

    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> bool {
        todo!()
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> bool {
        todo!()
    }

    fn visit_stmt(&mut self, assign: &Statement) -> bool {
        todo!()
    }

    fn visit_expr(&mut self, call: &Expr) -> bool {
        todo!()
    }

    fn visit_op(&mut self, op: &Op) -> bool {
        todo!()
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> bool {
        todo!()
    }

    fn visit_value(&mut self, val: &Value) -> bool {
        todo!()
    }
}

// =====================
// ==== WasiEmitter ====
// =====================
// TODO

// =======================
// ==== VirgilEmitter ====
// =======================
// TODO

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
impl DtraceVisitor<bool> for WasmRewritingEmitter<'_> {
    fn visit_dtrace(&mut self, dtrace: &Dtrace) -> bool {
        todo!()
    }

    fn visit_dscript(&mut self, dscript: &Dscript) -> bool {
        todo!()
    }

    fn visit_provider(&mut self, provider: &Provider) -> bool {
        todo!()
    }

    fn visit_module(&mut self, module: &Module) -> bool {
        todo!()
    }

    fn visit_function(&mut self, function: &Function) -> bool {
        todo!()
    }

    fn visit_probe(&mut self, probe: &Probe) -> bool {
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

use log::error;
use crate::parser::types::{DataType, Dscript, Dtrace, Expr, Fn, Function, Module, Op, Probe, Provider, Statement, Value};
use crate::verifier::types::{Record, SymbolTable};

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait Emitter {
    fn enter_scope(&mut self);
    fn exit_scope(&mut self);

    fn emit_dtrace(&mut self, dtrace: &Dtrace) -> bool;
    fn emit_dscript(&mut self, dscript: &Dscript) -> bool;
    fn emit_provider(&mut self, provider: &Provider) -> bool;

    // TODO -- should emit module/function/probe be private?
    fn emit_module(&mut self, module: &Module) -> bool;
    fn emit_function(&mut self, function: &Function) -> bool;
    fn emit_probe(&mut self, probe: &Probe) -> bool;

    fn emit_fn(&mut self, f: &Fn) -> bool;
    fn emit_formal_param(&mut self, param: &(Expr, DataType)) -> bool;
    fn emit_global(&mut self, name: String, ty: DataType, val: &Option<Value>) -> bool;
    fn emit_stmt(&mut self, stmt: &Statement) -> bool;
    fn emit_expr(&mut self, expr: &Expr) -> bool;
    fn emit_op(&mut self, op: &Op) -> bool;
    fn emit_datatype(&mut self, datatype: &DataType) -> bool;
    fn emit_value(&mut self, val: &Value) -> bool;

    fn dump_to_file(&mut self, output_wasm_path: String) -> bool;
}

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

pub(crate) struct WasmRewritingEmitter {
    pub(crate) app_wasm: walrus::Module,
    pub(crate) table: SymbolTable
}
/// Walrus Visitor over `app.wasm`
/// - as we get relevant info, lookup in SymbolTable for binding to globally set that value
/// - for each bytecode, do we have a probe?
///   - fold predicate with known globals. FALSE? Don't inject! NOT FALSE? inject (with remaining Expr, not folded parts)
///   - See fold Rust pattern: https://rust-unofficial.github.io/patterns/patterns/creational/fold.html
/// - now we have instrumented `app.wasm`
///   - write to app_instr.wasm
impl Emitter for WasmRewritingEmitter {
    fn enter_scope(&mut self) {
        self.table.enter_scope();
    }
    fn exit_scope(&mut self) {
        self.table.exit_scope();
    }
    fn emit_dtrace(&mut self, _dtrace: &Dtrace) -> bool {
        // nothing to do here
        true
    }
    fn emit_dscript(&mut self, _dscript: &Dscript) -> bool {
        // nothing to do here
        true
    }
    fn emit_provider(&mut self, provider: &Provider) -> bool {
        let mut is_success = true;
        provider.modules.iter().for_each(|(_name, module)| {
            is_success &= self.emit_module(module);
        });
        is_success
    }
    fn emit_module(&mut self, _module: &Module) -> bool {
        // TODO -- define any compiler constants
        // TODO -- set up `walrus::ir::VisitorMut`
        //         at each bytecode as traversing IR, do we have a `function` for the bytecode?
        //         If so, enter that function
        todo!();
    }
    fn emit_function(&mut self, _function: &Function) -> bool {
        // TODO -- define any compiler constants
        // TODO -- inject probes (should be at this point in the `walrus::ir::VisitorMut` since visited from `visit_module` above
        todo!();
    }
    fn emit_probe(&mut self, _function: &Function) -> bool {
        // TODO -- define any compiler constants
        todo!();
    }
    fn emit_fn(&mut self, f: &Fn) -> bool {
        self.table.enter_scope();
        // TODO -- figure out if this is a provided fn.
        todo!();
    }
    fn emit_formal_param(&mut self, param: &(Expr, DataType)) -> bool {
        todo!();
    }
    fn emit_global(&mut self, name: String, ty: DataType, val: &Option<Value>) -> bool {
        return match self.table.lookup(&name) {
            Some(rec_id) => {
                let rec = self.table.get_record_mut(rec_id);
                match rec {
                    Some(Record::Var { addr, .. }) => {
                        // TODO -- emit global variable and set addr in symbol table
                    },
                    Some(ty) => {
                        error!("Incorrect global variable record, expected Record::Var, found: {:?}", ty);
                        return false;
                    },
                    None => {
                        error!("Global variable symbol does not exist!");
                        return false;
                    }
                }
                false
            },
            _ => {
                error!("Global variable symbol does not exist in this scope!");
                false
            } // Ignore, continue to emit
        };
    }

    fn emit_stmt(&mut self, stmt: &Statement) -> bool {
        todo!()
    }

    fn emit_expr(&mut self, expr: &Expr) -> bool {
        todo!()
    }

    fn emit_op(&mut self, op: &Op) -> bool {
        todo!()
    }

    fn emit_datatype(&mut self, datatype: &DataType) -> bool {
        todo!()
    }

    fn emit_value(&mut self, val: &Value) -> bool {
        todo!()
    }

    fn dump_to_file(&mut self, output_wasm_path: String) -> bool {
        match self.app_wasm.emit_wasm_file(&output_wasm_path) {
            Ok(_ok) => {
                true
            },
            Err(err) => {
                error!("Failed to dump instrumented wasm to {} from error: {}", &output_wasm_path, err);
                false
            },
        }
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

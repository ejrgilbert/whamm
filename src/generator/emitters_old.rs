use crate::parser::types::{AstNode, WasmProbeName};
use crate::verifier::types::{CoreProbe, SymbolTable, WasmFnProbes, WasmProbe};

use log::{ debug, error, info };
use std::path::PathBuf;
use std::process::exit;
use walrus::{ Import, MemoryId, Module, ModuleImports };
use crate::parser::types::AstNode::Call;

// =====================
// ==== WasmEmitter ====
// =====================

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WasmEmitter {
    app_wasm: Module,
    output_wasm_path: String
}

impl ProbeEmitter for WasmEmitter {
    fn emit_wasm_probe(&self, symbol_table: &SymbolTable, fn_probes: WasmFnProbes) -> bool {
        if fn_probes.function.to_lowercase() == "call" {
            // Handle the `call` function
            self.handle_call(symbol_table, fn_probes);
        } else {
            error!("Not yet implemented");
            false
        }
        return true;
    }
}

// See https://github.com/rustwasm/wasm-snip/blob/master/src/lib.rs#L236
struct ReplaceImportedFnCallsWithWrapper<'a> {
    imp_to_replace: &'a Import,
    wrapper_fn_id: walrus::FunctionId,
}

impl ReplaceImportedFnCallsWithWrapper<'_> {
    fn should_replace_call(&self, instr: &walrus::ir::Instr) -> bool {
        if let walrus::ir::Instr::Call(walrus::ir::Call { func }) = instr {
            if self.imp_to_replace.id().index().eq(&func.index()) {
                return true;
            }
        }
        false
    }
}

impl walrus::ir::VisitorMut for ReplaceImportedFnCallsWithWrapper<'_> {
    fn visit_instr_mut(&mut self, instr: &mut walrus::ir::Instr, _instr_loc: &mut walrus::ir::InstrLocId) {
        if self.should_replace_call(instr) {
            *instr = walrus::ir::Call {
                func : self.wrapper_fn_id
            }.into()
        }
    }
}

impl WasmEmitter {
    pub fn new(app_wasm_path: String, output_wasm_path: String) -> Self {
        WasmEmitter {
            app_wasm: get_walrus_module(PathBuf::from(app_wasm_path)),
            output_wasm_path
        }
    }

    fn handle_call(&self, symbol_table: &SymbolTable, fn_probes: WasmFnProbes) -> bool {
        // iterate over the fn_probes
        for probes in fn_probes.fn_probes.iter() {
            match probes.0 {
                WasmProbeName::Before => {
                    for probe in probes.1 {

                    }
                },
                WasmProbeName::Alt => {
                    for probe in probes.1 {

                    }
                },
                WasmProbeName::After => {
                    for probe in probes.1 {

                    }
                }
            }
        }

        //
        // TODO: BUG - if import not found, unwrap() is on a None value...BAD!
        let imp = self.get_import_for_probe(&app_wasm.imports, &fn_probes.module, &fn_probes.function).unwrap();

        // Create instrumented wrapper function for import
        let wrapper_fn_id = self.create_wrapper(get_memory_id(app_wasm), &mut app_wasm.types, &mut app_wasm.locals, &mut app_wasm.funcs, imp, fn_probes);
        redirect_to_wrapper(&mut app_wasm.funcs, &imp, wrapper_fn_id);
    }

    fn get_memory_id(m: &Module) -> MemoryId {
        m.memories
            .iter()
            .next()
            .expect("only single memory is supported")
            .id()
    }

    fn inject_call_by_name(wrapper_body: &mut walrus::InstrSeqBuilder, params: &[walrus::LocalId], funcs: &walrus::ModuleFunctions, fn_name: &String) {
        debug!("Injecting call by name");
        match funcs.by_name(fn_name) {
            None => {
                error!("Cannot inject call, function def not found: {}", fn_name);
            },
            Some(fid) => {
                // add new call to this function
                inject_call_by_id(wrapper_body, params, &fid);
            }
        }
    }

    fn inject_call_by_id(wrapper_body: &mut walrus::InstrSeqBuilder, params: &[walrus::LocalId], fid: &walrus::FunctionId) {
        debug!("Injecting call by ID");
        for p in params.iter() {
            wrapper_body.local_get(*p);
        }

        // add new call to this function
        wrapper_body.call(*fid);
    }

    fn get_import_for_probe<'a>(imports: &'a ModuleImports, module: &String, function: &String) -> Option<&'a walrus::Import> {
        for imp in imports.iter() {
            if imp.module.eq(module) && imp.name.eq(function) {
                // We want to instrument this imported function!
                return Some(imp);
            }
        }

        return None;
    }

    fn inject_probe(_memory: MemoryId, funcs: &mut walrus::ModuleFunctions, _locals: &mut walrus::ModuleLocals, _orig_call_id: &walrus::FunctionId, wrapper_body: &mut walrus::InstrSeqBuilder, params: &mut Vec<walrus::LocalId>,
                    probe: &WasmProbe) {
        if probe.predicate.is_some() {
            // TODO -- inject predicate
            error!("Not implemented - inject_probe for predicates");
            // inject_predicate(memory, funcs, locals, orig_call_id, wrapper_body, params, probe);
            exit(1);
        } else {
            // TODO -- eventually support more complex probe body...right now it only supports a single function call specified by the body content
            if let AstNode::VarId{name} = probe.body.as_ref().unwrap().get(0).unwrap() {
                inject_call_by_name(wrapper_body, params, funcs, name);
            }
        }
    }

    fn create_wrapper(memory: MemoryId, all_types: &mut walrus::ModuleTypes, locals: &mut walrus::ModuleLocals, funcs: &mut walrus::ModuleFunctions, import: &Import, fn_probes: WasmFnProbes) -> walrus::FunctionId {
        // Build the wrapper function
        info!("Instrumenting import: {}::{}", import.module, import.name);


        // Get the function representation of the import
        let import_id = match import.kind {
            walrus::ImportKind::Function(id) => id,
            _ => {
                error!("Import didn't wind up being a function: {}::{}", import.module, import.name);
                panic!();
            }
        };

        let import_fn = funcs.get(import_id);
        let ty = all_types.get_mut(import_fn.ty()).clone(); // Cannot have 2 overlapping mut refs to all_types content

        // Get the function type of the import
        let params = ty.params();
        let results = ty.results();

        let mut wrapper = walrus::FunctionBuilder::new(all_types, params, results);

        // Create params
        let wrapper_body = &mut wrapper.func_body();
        // wrapper_body.i32_const(1234);  // Uncomment if debugging, helpful to flag the generated methods (but will cause module to be invalid)
        let mut created_params = Vec::new();
        for param in params.iter() {
            let p = locals.add(*param);
            created_params.push(p);
        }

        // Inject before probes
        for before_probe in fn_probes.fn_probes.get(&WasmProbeName::Before).unwrap() {
            inject_probe(memory, funcs, locals, &import_id, wrapper_body, &mut created_params, before_probe)
        }

        // Inject alt probes
        let alt_probes = fn_probes.fn_probes.get(&WasmProbeName::Alt).unwrap();
        if alt_probes.len() > 0 {
            if alt_probes.len() > 1 {
                println!("WARN: Multiple ALT probes configured for function, defaulting to last in configuration.");
            }
            let alt_probe = alt_probes.last().unwrap();

            inject_probe(memory, funcs, locals, &import_id, wrapper_body, &mut created_params, alt_probe);
        } else {
            inject_call_by_id(wrapper_body, &created_params, &import_id);
        }

        // Inject after probes
        for after_probe in fn_probes.fn_probes.get(&WasmProbeName::After).unwrap() {
            inject_probe(memory, funcs, locals, &import_id, wrapper_body, &mut created_params, after_probe);
        }

        // Finish the builder, wrap it all up and insert it into the module's functions
        let wrapper_id = wrapper.finish(created_params, funcs);
        return wrapper_id.clone();
    }

    fn redirect_to_wrapper(funcs: &mut walrus::ModuleFunctions, imp_to_replace: &Import, wrapper_fn_id: walrus::FunctionId) {
        funcs.iter_local_mut().for_each(|(id, func)| {
            if id != wrapper_fn_id {
                let entry = func.entry_block();
                walrus::ir::dfs_pre_order_mut(&mut ReplaceImportedFnCallsWithWrapper {
                    imp_to_replace,
                    wrapper_fn_id
                }, func, entry)
            }
        });
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

use std::collections::HashMap;
use crate::parser::dtrace_parser::{AstNode, DfinityProbeName};
use crate::parser::dtrace_parser;

use log::{ error, info };
use std::path::PathBuf;
use std::process::exit;
use walrus::{ Import, MemoryId, Module, ModuleImports };


// =========
// = Types =
// =========

// Probes
fn organize_probes(ast: &Vec<AstNode>) -> (Vec<CoreProbe>, AllDfinityFnProbes) {
    let mut core_probes = vec![];
    let mut dfinity_fn_probes = AllDfinityFnProbes::new();

    for node in ast {
        if let AstNode::Dscript { probes } = node {
            let probes = unbox(probes);
            (core_probes, dfinity_fn_probes) = organize_probes(&probes);
        } else if let AstNode::CoreProbe{ name, body } = node {
            let b = match body {
                Some(bd) => Some(unbox(bd)),
                None => None,
            };
            core_probes.push(CoreProbe {
                name: name.clone(),
                body: b
            });
        } else if let AstNode::DfinityProbe{ module, function, name, predicate, body } = node {
            let pred = match predicate {
                Some(pred) => Some((**pred).clone()),
                None => None
            };
            let b = match body {
                Some(inner) => Some(unbox(inner)),
                None => None
            };

            dfinity_fn_probes.add_probe(module, function, name, DfinityProbe {
                predicate: pred,
                body: b
            });
        } else {
            error!("Expected Core or Dfinity probe, received: {:?}", node);
            exit(1);
        }
    }

    return (core_probes, dfinity_fn_probes);
}

struct AllDfinityFnProbes {
    all_probes: HashMap<String, DfinityFnProbes>
}

impl AllDfinityFnProbes {
    pub fn new() -> Self {
        return AllDfinityFnProbes {
            all_probes: HashMap::new()
        }
    }

    fn add_or_append(&mut self, module: &String, function: &String, probe_type: &DfinityProbeName, probe: DfinityProbe) {
        let mut new_fn_probes = DfinityFnProbes::new(module.clone(), function.clone()); // might not be used

        self.all_probes.entry(format!("{module}.{function}"))
            .and_modify(|fn_probes| fn_probes.add_probe(module, function, probe_type, probe.clone()))
            .or_insert_with(|| {
                new_fn_probes.add_probe(module, function, probe_type, probe);
                new_fn_probes
            });
    }

    pub fn add_probe(&mut self, module: &String, function: &String, probe_type: &DfinityProbeName, probe: DfinityProbe) {
        self.add_or_append(module, function, probe_type, probe);
    }
}

struct DfinityFnProbes {
    module: String,
    function: String,
    fn_probes: HashMap<DfinityProbeName, Vec<DfinityProbe>>
}

impl DfinityFnProbes {
    pub fn new(module: String, function: String) -> Self {
        let mut fps = HashMap::new();
        fps.insert(DfinityProbeName::Before, Vec::new());
        fps.insert(DfinityProbeName::After, Vec::new());
        fps.insert(DfinityProbeName::Alt, Vec::new());

        return DfinityFnProbes {
            module,
            function,
            fn_probes: fps
        }
    }

    pub fn add_probe(&mut self, module: &String, function: &String, probe_type: &DfinityProbeName, probe: DfinityProbe) {
        if self.module != *module && self.function != *function {
            println!("ERROR: trying to add probe with mismatching clause. Expected: {}:{}. Actual: {module}:{function}.", self.module, self.function);
            return;
        }

        self.fn_probes.get_mut(&probe_type).unwrap().push(probe);
    }
}

trait Probe {}
#[derive(Clone, Debug)]
struct DfinityProbe {
    predicate: Option<AstNode>,
    body: Option<Vec<AstNode>>
}
impl Probe for DfinityProbe {}

// TODO -- CoreProbe compilation
#[derive(Clone, Debug)]
struct CoreProbe {
    name: dtrace_parser::CoreProbeName,
    body: Option<Vec<AstNode>>
}
impl Probe for CoreProbe {}

// ==================
// = Helper Methods =
// ==================

fn unbox(contents: &Vec<Box<AstNode>>) -> Vec<AstNode> {
    contents.into_iter().map(|item| {
        *item.clone()
    }).collect()
}

// ==================
// = Walrus Helpers =
// ==================

// GENERAL HELPERS

fn get_walrus_module(app_wasm_path: &PathBuf) -> Module {
    // Read app Wasm into Walrus module
    let _config =  walrus::ModuleConfig::new();
    Module::from_file(&app_wasm_path).unwrap()
}

fn get_memory_id(m: &Module) -> MemoryId {
    m.memories
        .iter()
        .next()
        .expect("only single memory is supported")
        .id()
}

fn inject_call_by_name(body: &mut walrus::InstrSeqBuilder, params: &[walrus::LocalId], funcs: &walrus::ModuleFunctions, fn_name: &String) {
    match funcs.by_name(fn_name) {
        None => {
            error!("ERROR: Cannot inject call, function def not found: {}", fn_name);
        },
        Some(fid) => {
            // add new call to this function
            inject_call_by_id(body, params, &fid);
        }
    }
}

fn inject_call_by_id(body: &mut walrus::InstrSeqBuilder, params: &[walrus::LocalId], fid: &walrus::FunctionId) {
    for p in params.iter() {
        // body = body.local_get(*p);
        body.local_get(*p);
    }

    // add new call to this function
    body.call(*fid);
}

// PROBE HELPERS

fn get_import_for_probe<'a>(imports: &'a ModuleImports, module: &String, function: &String) -> Option<&'a walrus::Import> {
    for imp in imports.iter() {
        if imp.module.eq(module) && imp.name.eq(function) {
            // We want to instrument this imported function!
            return Some(imp);
        }
    }

    return None;
}

fn inject_probe(_memory: MemoryId, funcs: &mut walrus::ModuleFunctions, _locals: &mut walrus::ModuleLocals, _orig_call_id: &walrus::FunctionId, fn_body: &mut walrus::InstrSeqBuilder, params: &mut Vec<walrus::LocalId>,
                probe: &DfinityProbe) {
    if probe.predicate.is_some() {
        // TODO -- inject predicate
        error!("Not implemented - inject_probe for predicates");
        // inject_predicate(memory, funcs, locals, orig_call_id, fn_body, params, probe);
        exit(1);
    } else {
        // TODO -- eventually support more complex probe body...right now it only supports a single function call specified by the body content
        if let AstNode::ProbeId{name} = probe.body.as_ref().unwrap().get(0).unwrap() {
            inject_call_by_name(fn_body, params, funcs, name);
        }
    }
}

fn create_wrapper(memory: MemoryId, all_types: &mut walrus::ModuleTypes, locals: &mut walrus::ModuleLocals, funcs: &mut walrus::ModuleFunctions, import: &Import, fn_probes: DfinityFnProbes) -> walrus::FunctionId {
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
    let fn_body = &mut wrapper.func_body();
    // body = body.i32_const(1234);  // Uncomment if debugging, helpful to flag the generated methods (but will cause module to be invalid)
    let mut created_params = Vec::new();
    for param in params.iter() {
        let p = locals.add(*param);
        created_params.push(p);
    }

    // Inject before probes
    for before_probe in fn_probes.fn_probes.get(&DfinityProbeName::Before).unwrap() {
        inject_probe(memory, funcs, locals, &import_id, fn_body, &mut created_params, before_probe)
    }

    // Inject alt probes
    let alt_probes = fn_probes.fn_probes.get(&DfinityProbeName::Alt).unwrap();
    if alt_probes.len() > 0 {
        if alt_probes.len() > 1 {
            println!("WARN: Multiple ALT probes configured for function, defaulting to last in configuration.");
        }
        let alt_probe = alt_probes.last().unwrap();

        inject_probe(memory, funcs, locals, &import_id, fn_body, &mut created_params, alt_probe);
    } else {
        inject_call_by_id(fn_body, &created_params, &import_id);
    }

    // Inject after probes
    for after_probe in fn_probes.fn_probes.get(&DfinityProbeName::After).unwrap() {
        inject_probe(memory, funcs, locals, &import_id, fn_body, &mut created_params, after_probe);
    }

    // Finish the builder, wrap it all up and insert it into the module's functions
    let wrapper_id = wrapper.finish(created_params, funcs);
    return wrapper_id.clone();
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

// ================
// = Target: Wasm =
// ================

fn core_emit_wasm(_probe: CoreProbe, _app_wasm: &mut Module) -> bool {
    error!("Not yet implemented");
    false
}

fn dfinity_emit_wasm(fn_probes: DfinityFnProbes, app_wasm: &mut Module) -> bool {
    let imp = get_import_for_probe(&app_wasm.imports, &fn_probes.module, &fn_probes.function).unwrap();

    // Create instrumented wrapper function for import
    let wrapper_fn_id = create_wrapper(get_memory_id(app_wasm), &mut app_wasm.types, &mut app_wasm.locals, &mut app_wasm.funcs, imp, fn_probes);
    redirect_to_wrapper(&mut app_wasm.funcs, &imp, wrapper_fn_id);

    return true;
}

pub fn emit_wasm(ast: &Vec<AstNode>, app_wasm_path: &PathBuf) -> bool {
    let mut success = true;
    let (core_probes, dfinity_fn_probes) = organize_probes(ast);

    let mut wasm = get_walrus_module(app_wasm_path);
    for probe in core_probes {
        success &= core_emit_wasm(probe, &mut wasm);
    }

    for probe in dfinity_fn_probes.all_probes {
        success &= dfinity_emit_wasm(probe.1, &mut wasm);
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

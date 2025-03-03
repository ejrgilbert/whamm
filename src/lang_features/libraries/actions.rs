use crate::common::error::ErrorGen;
use crate::common::instr::LibraryLinkStrategy;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::generator::ast::Script;
use crate::lang_features::libraries::core::LibPackage;
use crate::verifier::types::SymbolTable;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::Module;
use std::collections::{HashMap, HashSet};

pub fn link_core_lib(
    method: LibraryLinkStrategy,
    ast: &[Script],
    app_wasm: &mut Module,
    core_wasm_path: &str,
    mem_allocator: &mut MemoryAllocator,
    packages: &mut [&mut dyn LibPackage],
    err: &mut ErrorGen,
) -> Vec<FunctionID> {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::lang_features::libraries::linking::import_lib::link_core_lib(
                ast,
                app_wasm,
                core_wasm_path,
                mem_allocator,
                packages,
                err,
            )
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging core library code.");
        }
    }
}

pub fn link_user_libs(
    method: LibraryLinkStrategy,
    app_wasm: &mut Module,
    used_user_lib_fns: &HashSet<(String, String)>,
    user_lib_modules: &HashMap<String, Module>,
    table: &mut SymbolTable,
    err: &mut ErrorGen,
) -> Vec<FunctionID> {
    match method {
        LibraryLinkStrategy::Imported => {
            // Link the user libs
            let mut used_fns_per_lib: HashMap<String, HashSet<String>> = HashMap::default();
            for (used_lib, used_fn) in used_user_lib_fns.iter() {
                used_fns_per_lib
                    .entry(used_lib.clone())
                    .and_modify(|set| {
                        set.insert(used_fn.clone());
                    })
                    .or_insert(HashSet::from_iter([used_fn.clone()].iter().cloned()));
            }

            let mut added_funcs = vec![];
            for (used_lib, lib_fns) in used_fns_per_lib.iter() {
                let Some(lib_wasm) = user_lib_modules.get(used_lib) else {
                    panic!("Could not find wasm module for library '{used_lib}'");
                };
                added_funcs.extend(
                    crate::lang_features::libraries::linking::import_lib::link_user_lib(
                        app_wasm,
                        lib_wasm,
                        used_lib.clone(),
                        lib_fns,
                        table,
                        err,
                    ),
                );
            }
            added_funcs
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging user library code.");
        }
    }
}

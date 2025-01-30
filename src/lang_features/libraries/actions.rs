use crate::common::error::ErrorGen;
use crate::common::instr::LibraryLinkStrategy;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::lang_features::libraries::core::LibPackage;
use crate::parser::types::Whamm;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::Module;

pub fn link_core_lib(
    method: &LibraryLinkStrategy,
    ast: &Whamm,
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

pub fn link_user_lib(
    method: LibraryLinkStrategy,
    ast: &Whamm,
    app_wasm: &mut Module,
    lib_wasm: &Module,
    err: &mut ErrorGen,
) {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::lang_features::libraries::linking::import_lib::link_user_lib(
                ast, app_wasm, lib_wasm, err,
            );
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging user library code.");
        }
    }
}

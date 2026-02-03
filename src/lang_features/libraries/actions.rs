#![allow(clippy::too_many_arguments)]

use crate::api::instrument::LibraryLinkStrategy;
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::generator::ast::Script;
use crate::lang_features::libraries::core::utils::UtilsPackage;
use crate::lang_features::libraries::core::LibPackage;
use wirm::ir::id::FunctionID;
use wirm::Module;

pub fn link_core_lib(
    method: LibraryLinkStrategy,
    ast: &[Script],
    app_wasm: &mut Module,
    core_lib: &[u8],
    mem_allocator: &mut MemoryAllocator,
    utils: &mut UtilsPackage,
    packages: &mut [&mut dyn LibPackage],
    err: &mut ErrorGen,
) -> Vec<FunctionID> {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::lang_features::libraries::linking::import_lib::link_core_lib(
                ast,
                app_wasm,
                core_lib,
                mem_allocator,
                utils,
                packages,
                err,
            )
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging core library code.");
        }
    }
}

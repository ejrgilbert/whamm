#![allow(clippy::too_many_arguments)]
use crate::api::instrument::LibraryLinkStrategy;
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::generator::ast::Script;
use crate::lang_features::libraries::core::LibPackage;
use std::collections::HashMap;
use wirm::ir::id::FunctionID;
use wirm::{Component, Module};

pub fn link_core_lib(
    method: LibraryLinkStrategy,
    ast: &[Script],
    app_wasm: &mut Module,
    core_lib: &[u8],
    libs_as_components: bool,
    mem_allocator: &mut MemoryAllocator,
    packages: &mut [&mut dyn LibPackage],
    err: &mut ErrorGen,
) -> Vec<FunctionID> {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::lang_features::libraries::linking::import_lib::link_core_lib(
                ast,
                app_wasm,
                core_lib,
                libs_as_components,
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

pub fn configure_component_libraries(
    _component: &mut Component,
    _core_lib: &[u8],
    _user_libs: &HashMap<String, &[u8]>,
) {
    // TODO: add libraries as *core module*s in the top-level of the component
    // TODO: add *core instance*s in the top-level of the component for each of the added library modules
    //       Should list out what's exported in the library in the instantiation too
    // TODO: modify the main's *core instance*s with clauses to import the added library's contents
    // let module_index = component.add_module((*core_lib).to_owned());
    //
    // // TODO: Look for the instance that provides "wasi_snapshot_preview1"
    // component.instances.push(Instance::Instantiate {
    //     module_index: *module_index,
    //     args: Box::new([]),
    // })
    todo!()
}

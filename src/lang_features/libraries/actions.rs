#![allow(clippy::too_many_arguments)]
use crate::api::instrument::LibraryLinkStrategy;
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::generator::ast::Script;
use crate::lang_features::libraries::core::{LibPackage, WHAMM_CORE_LIB_NAME};
use std::collections::HashMap;
use wasmparser::{CanonicalFunction, ComponentAlias, ComponentExport, ComponentExternalKind, ComponentImport, ComponentImportName, ComponentType, ComponentTypeRef, Export, ExternalKind, Instance, InstanceTypeDeclaration, InstantiationArg, InstantiationArgKind};
use wirm::ir::id::{ComponentExportId, FunctionID};
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

pub fn configure_component_libraries<'a>(
    target_module_id: u32,
    component: &mut Component<'a>,
    core_lib: &'a [u8],
    user_libs: &'a HashMap<String, &'a[u8]>,
) {
    // find "wasi_snapshot_preview1" instance
    let mut wasi_instance = None;
    let wasi_exports = ["fd_write", "environ_get", "environ_sizes_get", "proc_exit"];
    for (i, inst) in component.instances.iter().enumerate() {
        if let Instance::FromExports(exports) = inst {
            let mut found_count = 0;
            for export in exports.iter() {
                if wasi_exports.contains(&export.name) {
                    found_count += 1;
                }
            }

            if found_count == wasi_exports.len() {
                wasi_instance = Some(i);
                break;
            }
        }
    }
    if let Some(wasi_instance) = wasi_instance {
        let mut has_whamm_core = false;
        for (name, bytes) in user_libs.iter() {
            if name == WHAMM_CORE_LIB_NAME {
                has_whamm_core = true;
            }
            configure_lib(target_module_id, component, wasi_instance, name, bytes);
        }
        if !has_whamm_core {
            configure_lib(target_module_id, component, wasi_instance, WHAMM_CORE_LIB_NAME, core_lib);
        }
    } else {
        panic!("Target component does not already import wasi_snapshot_preview1, not supported yet.")
    }

    fn configure_lib<'a>(target_module_id: u32, wasm: &mut Component<'a>, wasi_instance_loc: usize, lib_name: &'a str, lib_bytes: &'a [u8]) {
        let wasi_name = "wasi_snapshot_preview1";
        let lib_wasm = Component::parse(lib_bytes, false, true).unwrap();
        // TODO: add libraries as *core module*s in the top-level of the component
        // TODO: add *core instance*s in the top-level of the component for each of the added library modules
        //       Should list out what's exported in the library in the instantiation too
        // TODO: modify the main's *core instance*s with clauses to import the added library's contents
        // let module_index = component.add_module((*core_lib).to_owned());
        //
        // Create an instance type that defines the library
        let mut decls = vec![];
        let mut num_exported_fns = 0;
        let mut curr_ty_id = 0;
        for (i, export) in lib_wasm.exports.iter().enumerate() {
            println!("[configure_lib] component export: {}", export.name.0);
            if !matches!(export.kind, ComponentExternalKind::Func) {
                println!("  --> skipped");
                continue;
            }
            let comp_ty = lib_wasm.get_type_of_exported_lift_func(ComponentExportId(i as u32));
            // let comp_ty = get_fn_type_from_component_export(&lib_wasm, num_exported_fns, export);
            if let Some(ComponentType::Func(ty)) = comp_ty {
                println!("  --> used");
                decls.push(InstanceTypeDeclaration::Type(comp_ty.unwrap().clone()));
                decls.push(InstanceTypeDeclaration::Export { name: export.name, ty: ComponentTypeRef::Func(curr_ty_id)});
                curr_ty_id += 1;
            } else {
                println!("  --> skipped, {:?}", comp_ty);
            }
            num_exported_fns += 1;
        }
        let (inst_ty_id, ..) = wasm.add_type_instance(decls);

        // Import the library from an external provider
        // TODO -- switch to general case! (convert to kebab case)
        let inst_id = wasm.add_import(ComponentImport { name: ComponentImportName("whamm-core"), ty: ComponentTypeRef::Instance(*inst_ty_id)});

        // Lower the exported functions using aliases
        let mut exports = vec![];
        for ComponentExport {name, kind, ..} in lib_wasm.exports.iter() {
            let (alias_func_id, ..) = wasm.add_alias_func(ComponentAlias::InstanceExport {name: name.0, kind: kind.clone(), instance_index: inst_id});
            let canon_id = wasm.add_canon_func(CanonicalFunction::Lower {func_index: *alias_func_id, options: vec![].into_boxed_slice()});

            exports.push(Export {name: name.0, kind: ExternalKind::Func, index: *canon_id});
        }

        // Create a core instance from the library
        let lib_inst_id = wasm.add_core_instance(Instance::FromExports(exports.into_boxed_slice()));

        // Edit the instantiation of the instrumented module to include the added library
        for inst in wasm.instances.iter_mut() {
            if let Instance::Instantiate {module_index, args} = inst {
                if target_module_id == *module_index {
                    let mut uses_wasi = false;
                    let mut new_args = vec![];
                    for arg in args.iter() {
                        if arg.name == wasi_name {
                            uses_wasi = true;
                        }
                        new_args.push(arg.clone());
                    }
                    assert!(uses_wasi, "Target module does not already import wasi_snapshot_preview1, not supported yet.");

                    new_args.push(InstantiationArg {name: lib_name, kind: InstantiationArgKind::Instance, index: *lib_inst_id});

                    *args = new_args.into_boxed_slice();
                }
            }
        }
    }
}

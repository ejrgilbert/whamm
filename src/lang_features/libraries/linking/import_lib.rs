#![allow(clippy::too_many_arguments)]

use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::tag_handler::get_tag_for;
use crate::generator::ast::Script;
use crate::lang_features::libraries::core::{
    LibPackage, WHAMM_CORE_LIB_MEM_NAME, WHAMM_CORE_LIB_NAME,
};
use crate::parser::types::Location;
use crate::verifier::types::{Record, SymbolTable};
use log::trace;
use std::collections::HashSet;
use wasmparser::{
    ComponentExternalKind, ComponentType, ComponentValType, ExternalKind, MemoryType,
    PrimitiveValType,
};
use wirm::ir::id::{ComponentExportId, FunctionID};
use wirm::{Component, DataType, Module};

// Some documentation on why it's difficult to only import the *used* functions.
//
// TLDR; Rust ownership.
// If I pass in a reference to both the application module (to conditionally import
// if not already done) AND a function modifier to the library adapter, I'll have
// two mutable references to the app module.
// This means that the caller will have to check for needed imports BEFORE actually
// delegating to the adapter...which means that I'd have to break the practice of
// information hiding. Or, I could create a 'check_OPERATION()' per 'OPERATION()',
// which would just be more clunky.
//
// So for now, we'll do this, but it can be changed later if I get a better idea.

pub fn link_core_lib(
    ast: &[Script],
    app_wasm: &mut Module,
    core_lib: &[u8],
    libs_as_components: bool,
    mem_allocator: &mut MemoryAllocator,
    packages: &mut [&mut dyn LibPackage],
    err: &mut ErrorGen,
) -> Vec<FunctionID> {
    let mut injected_funcs = vec![];
    for package in packages.iter_mut() {
        package.visit_ast(ast);
        package.set_adapter_usage(package.is_used());
        package.set_global_adapter_usage(package.is_used_in_global_scope());
        if package.is_used() {
            if package.import_memory() {
                let lib_mem_id =
                    import_lib_memory(app_wasm, &None, WHAMM_CORE_LIB_NAME.to_string());
                package.set_lib_mem_id(lib_mem_id);
            }
            package.set_instr_mem_id(mem_allocator.mem_id as i32);
            injected_funcs.extend(import_lib_package(
                app_wasm,
                &None,
                WHAMM_CORE_LIB_NAME.to_string(),
                &None,
                core_lib,
                libs_as_components,
                *package,
                err,
            ));
        }
    }
    injected_funcs
}

pub fn link_user_lib(
    app_wasm: &mut Module,
    loc: &Option<Location>,
    lib_bytes: &[u8],
    libs_as_components: bool,
    lib_name: String,
    lib_name_import_override: &Option<String>,
    used_lib_fns: &HashSet<String>,
    table: &mut SymbolTable,
) -> Vec<FunctionID> {
    let added = if libs_as_components {
        let lib_wasm = Component::parse(lib_bytes, false, true).unwrap();

        import_lib_fn_names_from_component(
            app_wasm,
            loc,
            lib_name,
            &lib_wasm,
            used_lib_fns,
            Some(table),
        )
    } else {
        let lib_wasm = Module::parse(lib_bytes, false, true).unwrap();
        import_lib_fn_names_from_module(
            app_wasm,
            loc,
            lib_name,
            lib_name_import_override,
            &lib_wasm,
            used_lib_fns,
            Some(table),
        )
    };

    let mut injected_funcs = vec![];
    for (_, fid) in added.iter() {
        injected_funcs.push(FunctionID(*fid));
    }

    injected_funcs
}

fn import_lib_memory(app_wasm: &mut Module, loc: &Option<Location>, lib_name: String) -> i32 {
    trace!("Enter import_lib_memory");
    let mem_id = import_memory(
        lib_name.as_str(),
        WHAMM_CORE_LIB_MEM_NAME,
        "lib_mem",
        loc,
        app_wasm,
    );

    trace!("Exit import_lib");
    mem_id as i32
}

fn import_lib_package(
    app_wasm: &mut Module,
    loc: &Option<Location>,
    lib_name: String,
    lib_name_import_override: &Option<String>,
    lib_bytes: &[u8],
    libs_as_components: bool,
    package: &mut dyn LibPackage,
    err: &mut ErrorGen,
) -> Vec<FunctionID> {
    trace!("Enter import_lib");

    // should only import the EXPORTED contents of the lib_wasm
    let added = if libs_as_components {
        println!("HEY!!!");
        let lib_wasm = Component::parse(lib_bytes, false, true).unwrap();

        println!("HEY!!! -- end");
        import_lib_fn_names_from_component(
            app_wasm,
            loc,
            lib_name,
            &lib_wasm,
            &HashSet::from_iter(package.get_fn_names().iter().cloned()),
            None,
        )
    } else {
        let lib_wasm = Module::parse(lib_bytes, false, true).unwrap();
        import_lib_fn_names_from_module(
            app_wasm,
            loc,
            lib_name,
            lib_name_import_override,
            &lib_wasm,
            &HashSet::from_iter(package.get_fn_names().iter().cloned()),
            None,
        )
    };

    for (name, fid) in added.iter() {
        // save the FID
        package.add_fid_to_adapter(name.as_str(), *fid);
    }

    // enable the library to define in-module helper functions
    let injected_funcs = package.define_helper_funcs(app_wasm, err);

    trace!("Exit import_lib");
    injected_funcs
}
fn canon_lower(ty: &ComponentValType) -> DataType {
    match ty {
        ComponentValType::Primitive(pty) => match pty {
            PrimitiveValType::Bool
            | PrimitiveValType::S8
            | PrimitiveValType::U8
            | PrimitiveValType::S16
            | PrimitiveValType::U16
            | PrimitiveValType::S32
            | PrimitiveValType::U32 => DataType::I32,
            PrimitiveValType::S64 | PrimitiveValType::U64 => DataType::I64,
            PrimitiveValType::F32 => DataType::F32,
            PrimitiveValType::F64 => DataType::F64,
            PrimitiveValType::Char | PrimitiveValType::String | PrimitiveValType::ErrorContext => {
                todo!()
            }
        },
        ComponentValType::Type(_) => todo!(),
    }
}

fn import_lib_fn_names_from_component(
    app_wasm: &mut Module,
    loc: &Option<Location>,
    lib_name: String,
    lib_wasm: &Component,
    lib_fns: &HashSet<String>,
    mut table: Option<&mut SymbolTable>,
) -> Vec<(String, u32)> {
    let mut injected_fns = vec![];
    // let mut num_exported_fns = 0;
    for (i, export) in lib_wasm.exports.iter().enumerate() {
        if !matches!(export.kind, ComponentExternalKind::Func) {
            // we don't care about non-function exports
            continue;
        }

        let fn_name = export.name.0;
        if lib_fns.contains(fn_name) {
            let ty = lib_wasm.get_type_of_exported_lift_func(ComponentExportId(i as u32));
            if let Some(ComponentType::Func(ty)) = ty {
                let mut params = vec![];
                for (_, pty) in ty.params.iter() {
                    params.push(canon_lower(pty))
                }
                let results = if let Some(rty) = &ty.result {
                    vec![canon_lower(rty)]
                } else {
                    vec![]
                };

                let fid = import_func(
                    lib_name.as_str(),
                    fn_name,
                    params.as_slice(),
                    results.as_slice(),
                    loc,
                    app_wasm,
                );
                // save the FID to the symbol table
                if let Some(table) = table.as_mut() {
                    let Some(Record::LibFn { addr, .. }) =
                        table.lookup_lib_fn_mut(&lib_name, fn_name)
                    else {
                        panic!("unexpected type");
                    };

                    *addr = Some(fid);
                }

                // save the FID as an injected function
                injected_fns.push((export.name.0.to_string(), fid));
            } else {
                panic!(
                    "ImportLib::component: Could not add function \"{}\" as application import, looking at ID {}, actual type: {:?}. Full export: {:?}",
                    export.name.0,
                    export.index,
                    export.ty,
                    export
                );
            }
        }
        // num_exported_fns += 1;
    }
    injected_fns
}

fn import_lib_fn_names_from_module(
    app_wasm: &mut Module,
    loc: &Option<Location>,
    lib_name: String,
    lib_name_import_override: &Option<String>,
    lib_wasm: &Module,
    lib_fns: &HashSet<String>,
    mut table: Option<&mut SymbolTable>,
) -> Vec<(String, u32)> {
    let mut injected_fns = vec![];
    for export in lib_wasm.exports.iter() {
        // we don't care about non-function exports
        if let ExternalKind::Func = export.kind {
            if lib_fns.contains(&export.name) {
                let func = lib_wasm.functions.get(FunctionID(export.index));
                if let Some(ty) = lib_wasm.types.get(func.get_type_id()) {
                    let import_name = if let Some(name_override) = lib_name_import_override {
                        name_override.as_str()
                    } else {
                        lib_name.as_str()
                    };
                    let fn_name = export.name.as_str();

                    let fid = import_func(
                        import_name,
                        fn_name,
                        &ty.params().clone(),
                        &ty.results().clone(),
                        loc,
                        app_wasm,
                    );
                    // save the FID to the symbol table
                    if let Some(table) = table.as_mut() {
                        let Some(Record::LibFn { addr, .. }) =
                            table.lookup_lib_fn_mut(&lib_name, fn_name)
                        else {
                            panic!("unexpected type");
                        };

                        *addr = Some(fid);
                    }

                    // save the FID as an injected function
                    injected_fns.push((export.name.clone(), fid));
                } else {
                    unreachable!(
                        "ImportLib::module: Could not add function \"{}\" as application import",
                        export.name
                    );
                }
            }
        }
    }
    injected_fns
}

fn import_memory(
    module_name: &str,
    mem_name: &str,
    use_name: &str,
    loc: &Option<Location>,
    app_wasm: &mut Module,
) -> u32 {
    let (mem_id, imp_id) = app_wasm.add_import_memory_with_tag(
        module_name.to_string(),
        mem_name.to_string(),
        MemoryType {
            memory64: false,
            shared: false,
            initial: 0,
            maximum: None,
            page_size_log2: None,
        },
        get_tag_for(loc),
    );
    app_wasm.imports.set_name(use_name.to_string(), imp_id);

    *mem_id
}

pub fn import_func(
    module_name: &str,
    fname: &str,
    params: &[DataType],
    results: &[DataType],
    loc: &Option<Location>,
    app_wasm: &mut Module,
) -> u32 {
    let ty_id = app_wasm
        .types
        .add_func_type_with_tag(params, results, get_tag_for(loc));
    let (fid, imp_id) = app_wasm.add_import_func_with_tag(
        module_name.to_string(),
        fname.to_string(),
        ty_id,
        get_tag_for(loc),
    );
    app_wasm.imports.set_name(fname.to_string(), imp_id);

    *fid
}

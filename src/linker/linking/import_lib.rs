use crate::common::error::ErrorGen;
use crate::linker::core::LibPackage;
use crate::linker::WHAMM_CORE_LIB_NAME;
use crate::parser::types::Whamm;
use crate::verifier::types::{Record, SymbolTable};
use log::trace;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::Module;
use std::collections::HashMap;
use wasmparser::ExternalKind;

pub fn link_core_lib(
    ast: &Whamm,
    table: &mut SymbolTable,
    app_wasm: &mut Module,
    core_wasm_path: &str,
    packages: &mut [Box<dyn LibPackage>],
    err: &mut ErrorGen,
) {
    for package in packages.iter_mut() {
        package.visit_whamm(ast);
        if package.is_used() {
            // Read core library Wasm into Orca module
            let buff = std::fs::read(core_wasm_path).unwrap();
            let core_lib = Module::parse(&buff, false).unwrap();
            import_lib(
                table,
                app_wasm,
                WHAMM_CORE_LIB_NAME.to_string(),
                true,
                &core_lib,
                &**package,
                err,
            );
        }
    }
}

pub fn link_user_lib(
    _ast: &Whamm,
    _table: &mut SymbolTable,
    _app_wasm: &mut Module,
    _lib_wasm: &Module,
    _err: &mut ErrorGen,
) {
    // should only import ALL EXPORTED contents of the lib_wasm
    unimplemented!("Have not added support for user libraries...yet!")
}

fn import_lib(
    table: &mut SymbolTable,
    app_wasm: &mut Module,
    lib_name: String,
    is_comp_provided: bool,
    lib_wasm: &Module,
    package: &dyn LibPackage,
    err: &mut ErrorGen,
) {
    trace!("Enter import_lib");
    table.reset();

    // should only import the EXPORTED contents of the lib_wasm
    let package_fn_names = package.get_fn_names();
    let mut lib_funcs = HashMap::new();
    for export in lib_wasm.exports.iter() {
        // we don't care about non-function exports
        if let ExternalKind::Func = export.kind {
            if package_fn_names.contains(&export.name) {
                let func = lib_wasm.functions.get(FunctionID(export.index));
                if let Some(ty) = lib_wasm.types.get(func.get_type_id()) {
                    let ty_id = app_wasm.types.add(&ty.params.clone(), &ty.results.clone());
                    let (fid, imp_id) =
                        app_wasm.add_import_func(lib_name.clone(), export.name.clone(), ty_id);
                    app_wasm.imports.set_name(export.name.clone(), imp_id);

                    // add to symbol table!
                    lib_funcs.insert(export.name.clone(), *fid);
                } else {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "ImportLib: Could not add function \"{}\" as application import",
                            export.name
                        )),
                        None,
                    );
                }
            }
        }
    }

    match table.lookup_core_lib_mut(&None, err) {
        Some(Record::Library { fns, .. }) => {
            fns.extend(lib_funcs);
        }
        None => {
            // library record doesn't exist yet!
            let library_rec = Record::Library {
                name: lib_name.clone(),
                is_comp_provided,
                fns: lib_funcs,
            };
            table.put(lib_name, library_rec);
        }
        Some(_) => {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
        }
    }

    trace!("Exit import_lib");
}

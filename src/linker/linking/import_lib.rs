use log::trace;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::Module;
use wasmparser::ExternalKind;
use crate::common::error::ErrorGen;
use crate::linker::core::LibPackage;
use crate::parser::types::Whamm;
use crate::verifier::types::{Record, ScopeType, SymbolTable};

pub fn link_core_lib(ast: &Whamm, table: &mut SymbolTable, app_wasm: &mut Module, lib_wasm: &Module, packages: &mut Vec<Box<dyn LibPackage>>, err: &mut ErrorGen) {
    for package in packages.iter_mut() {
        package.visit_whamm(ast);
        if package.is_used() {
            import_lib(table, app_wasm, "whamm_core".to_string(), lib_wasm, package, err);
        }
    }
}

pub fn link_user_lib(_ast: &Whamm, _table: &mut SymbolTable, _app_wasm: &mut Module, _lib_wasm: &Module, _err: &mut ErrorGen) {
    // should only import ALL EXPORTED contents of the lib_wasm
    unimplemented!("Have not added support for user libraries...yet!")
}

fn import_lib(table: &mut SymbolTable, app_wasm: &mut Module, lib_name: String, lib_wasm: &Module, package: &Box<dyn LibPackage>, err: &mut ErrorGen) {
    trace!("Enter import_lib");
    table.enter_scope(err);
    table.set_curr_scope_info(lib_name.clone(), ScopeType::Library);

    // should only import the EXPORTED contents of the lib_wasm
    let package_fn_names = package.get_fn_names();
    for export in lib_wasm.exports.iter() {
        match export.kind {
            ExternalKind::Func => {
                if package_fn_names.contains(&export.name) {
                    let func = lib_wasm.functions.get(FunctionID(export.index));
                    if let Some(ty) = lib_wasm.types.get(func.get_type_id()) {
                        let ty_id = app_wasm.types.add(&ty.params.clone(), &ty.results.clone());
                        let (fid, ..) = app_wasm.add_import_func(lib_name.clone(), export.name.clone(), ty_id);

                        // add to symbol table!
                        table.put(export.name.clone(), Record::LibFn {
                            name: export.name.clone(),
                            fn_id: fid.0
                        });
                    } else {
                        err.unexpected_error(true, Some(format!("ImportLib: Could not add function \"{}\" as application import", export.name)), None);
                    }
                }
            },
            _ => {} // we don't care about non-function exports
        }
    }
    table.exit_scope(err);
    trace!("Exit import_lib");
}

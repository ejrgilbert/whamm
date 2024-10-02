use log::trace;
use crate::common::error::ErrorGen;
use crate::parser::types::Whamm;
use orca_wasm::{DataType, Module};
use orca_wasm::ir::id::FunctionID;
use wasmparser::ExternalKind;
use crate::libraries::core::{LibPackage, WHAMM_CORE_LIB_NAME};

/// Some documentation on why it's difficult to only import the *used* functions.
///
/// TLDR; Rust ownership.
/// If I pass in a reference to both the application module (to conditionally import
/// if not already done) AND a function modifier to the library adapter, I'll have
/// two mutable references to the app module.
/// This means that the caller will have to check for needed imports BEFORE actually
/// delegating to the adapter...which means that I'd have to break the practice of
/// information hiding. Or, I could create a 'check_OPERATION()' per 'OPERATION()',
/// which would just be more clunky.
///
/// So for now, we'll do this, but it can be changed later if I get a better idea.

pub fn link_core_lib(
    ast: &Whamm,
    app_wasm: &mut Module,
    core_wasm_path: &str,
    packages: &mut [Box<&mut dyn LibPackage>],
    err: &mut ErrorGen,
) {
    for package in packages.iter_mut() {
        package.visit_whamm(ast);
        if package.is_used() {
            // Read core library Wasm into Orca module
            let buff = std::fs::read(core_wasm_path).unwrap();
            let core_lib = Module::parse(&buff, false).unwrap();
            import_lib(
                app_wasm,
                WHAMM_CORE_LIB_NAME.to_string(),
                &core_lib,
                **package,
                err,
            );
        }
    }
}

pub fn link_user_lib(
    _ast: &Whamm,
    _app_wasm: &mut Module,
    _lib_wasm: &Module,
    _err: &mut ErrorGen,
) {
    // should only import ALL EXPORTED contents of the lib_wasm
    unimplemented!("Have not added support for user libraries...yet!")
}

fn import_lib(
    app_wasm: &mut Module,
    lib_name: String,
    lib_wasm: &Module,
    package: &mut dyn LibPackage,
    err: &mut ErrorGen,
) {
    trace!("Enter import_lib");

    // should only import the EXPORTED contents of the lib_wasm
    let package_fn_names = package.get_fn_names();
    for export in lib_wasm.exports.iter() {
        // we don't care about non-function exports
        if let ExternalKind::Func = export.kind {
            if package_fn_names.contains(&export.name) {
                let func = lib_wasm.functions.get(FunctionID(export.index));
                if let Some(ty) = lib_wasm.types.get(func.get_type_id()) {
                    let fid = import_func(lib_name.as_str(), export.name.as_str(), &ty.params.clone(), &ty.results.clone(), app_wasm);
                    // save the FID
                    package.add_fid_to_adapter(export.name.as_str(), fid);
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

    trace!("Exit import_lib");
}

pub fn import_func(module_name: &str, fname: &str, params: &[DataType], results: &[DataType], app_wasm: &mut Module) -> u32 {
    let ty_id = app_wasm.types.add(params, results);
    let (fid, imp_id) =
        app_wasm.add_import_func(module_name.to_string(), fname.to_string(), ty_id);
    app_wasm.imports.set_name(fname.to_string(), imp_id);

    *fid
}

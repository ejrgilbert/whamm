use orca_wasm::Module;
use crate::linker::core::LibPackage;
use crate::parser::types::Whamm;
use crate::verifier::types::SymbolTable;

pub fn link_core_lib(ast: &Whamm, table: &mut SymbolTable, app_wasm: &mut Module, lib_wasm: &Module, packages: &mut Vec<Box<dyn LibPackage>>) {
    for package in packages.iter_mut() {
        package.visit_whamm(ast);
        if package.is_used() {
            import_lib(table, app_wasm, lib_wasm);
        }
    }
}

pub fn link_user_lib(_ast: &Whamm, _table: &mut SymbolTable, _app_wasm: &mut Module, _lib_wasm: &Module) {
    // should only import ALL EXPORTED contents of the lib_wasm
    unimplemented!("Have not added support for user libraries...yet!")
}

fn import_lib(_table: &mut SymbolTable, _app_wasm: &mut Module, _lib_wasm: &Module) {
    // should only import the EXPORTED contents of the lib_wasm
    todo!()
}

use crate::common::error::ErrorGen;
use crate::common::instr::LibraryLinkStrategy;
use crate::linker::core::LibPackage;
use crate::parser::types::Whamm;
use crate::verifier::types::SymbolTable;
use orca_wasm::Module;

pub fn link_core_lib(
    method: LibraryLinkStrategy,
    ast: &Whamm,
    table: &mut SymbolTable,
    app_wasm: &mut Module,
    core_wasm_path: &str,
    packages: &mut [Box<dyn LibPackage>],
    err: &mut ErrorGen,
) {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::linker::linking::import_lib::link_core_lib(
                ast,
                table,
                app_wasm,
                core_wasm_path,
                packages,
                err,
            );
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging core library code.");
        }
    }
}

pub fn link_user_lib(
    method: LibraryLinkStrategy,
    ast: &Whamm,
    table: &mut SymbolTable,
    app_wasm: &mut Module,
    lib_wasm: &Module,
    err: &mut ErrorGen,
) {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::linker::linking::import_lib::link_user_lib(ast, table, app_wasm, lib_wasm, err);
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging user library code.");
        }
    }
}
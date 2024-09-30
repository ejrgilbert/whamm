use orca_wasm::Module;
use crate::common::instr::LibraryLinkStrategy;
use crate::linker::core::LibPackage;
use crate::parser::types::Whamm;
use crate::verifier::types::SymbolTable;

pub fn link_core_lib(method: LibraryLinkStrategy, ast: &Whamm, table: &mut SymbolTable, app_wasm: &mut Module, lib_wasm: &Module, packages: &mut Vec<Box<dyn LibPackage>>) {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::linker::linking::import_lib::link_core_lib(ast, table, app_wasm, lib_wasm, packages);
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging core library code.");
        }
    }
}

pub fn link_user_lib(method: LibraryLinkStrategy, ast: &Whamm, table: &mut SymbolTable, app_wasm: &mut Module, lib_wasm: &Module) {
    match method {
        LibraryLinkStrategy::Imported => {
            crate::linker::linking::import_lib::link_user_lib(ast, table, app_wasm, lib_wasm);
        }
        LibraryLinkStrategy::Merged => {
            unimplemented!("Have not implemented support for merging user library code.");
        }
    }
}

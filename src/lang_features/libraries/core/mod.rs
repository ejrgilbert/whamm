pub mod io;
pub mod maps;

use crate::generator::ast::AstVisitor;
use std::collections::HashMap;
use wirm::ir::id::FunctionID;
use wirm::Module;

pub const WHAMM_CORE_LIB_NAME: &str = "whamm_core";
pub const WHAMM_CORE_LIB_MEM_NAME: &str = "memory";
const UNEXPECTED_ERR_MSG: &str =
    "Adapter: Looks like you've found a bug...please report this behavior! Exiting now...";

// A lib package needs to be able to visit the AST and determine if it's needed (should be linked)
pub trait LibPackage: AstVisitor<bool> {
    fn is_used(&self) -> bool;
    fn is_used_in_global_scope(&self) -> bool;
    fn import_memory(&self) -> bool;
    fn set_lib_mem_id(&mut self, mem_id: i32);
    fn set_instr_mem_id(&mut self, mem_id: i32);
    fn get_fn_names(&self) -> Vec<String>;
    fn add_fid_to_adapter(&mut self, fname: &str, fid: u32);
    fn set_adapter_usage(&mut self, is_used: bool);
    fn set_global_adapter_usage(&mut self, is_used: bool);
    fn define_helper_funcs(&mut self, app_wasm: &mut Module)
        -> Vec<FunctionID>;
}
pub trait LibAdapter {
    fn get_funcs(&self) -> &HashMap<String, u32>;
    fn get_funcs_mut(&mut self) -> &mut HashMap<String, u32>;
    fn define_helper_funcs(&mut self, app_wasm: &mut Module)
        -> Vec<FunctionID>;
    fn get_fn_names(&self) -> Vec<String> {
        self.get_funcs().keys().cloned().collect()
    }
    fn get_fid(&self, fname: &str) -> u32 {
        if let Some(fid) = self.get_funcs().get(fname) {
            *fid
        } else {
            unreachable!(
                    "{} Could not find expected configured library function: {}", UNEXPECTED_ERR_MSG,fname);
        }
    }

    fn add_fid(&mut self, fname: &str, fid: u32) {
        self.get_funcs_mut().insert(fname.to_string(), fid);
    }
}

pub mod maps;
pub mod io;

use crate::parser::types::WhammVisitor;
use std::collections::HashMap;
use crate::common::error::ErrorGen;

pub const WHAMM_CORE_LIB_NAME: &str = "whamm_core";
const UNEXPECTED_ERR_MSG: &str =
    "Adapter: Looks like you've found a bug...please report this behavior! Exiting now...";

// A lib package needs to be able to visit the AST and determine if it's needed (should be linked)
pub trait LibPackage: WhammVisitor<bool> {
    fn is_used(&self) -> bool;
    fn get_fn_names(&self) -> Vec<String>;
    fn add_fid_to_adapter(&mut self, fname: &str, fid: u32);
}
pub trait LibAdapter {
    fn get_funcs(&self) -> &HashMap<String, u32>;
    fn get_funcs_mut(&mut self) -> &mut HashMap<String, u32>;
    fn get_fn_names(&self) -> Vec<String> {
        self.get_funcs().keys().cloned().collect()
    }
    fn get_fid(&self, fname: &str, err: &mut ErrorGen) -> u32 {
        if let Some(fid) = self.get_funcs().get(fname) {
            *fid
        } else {
            err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Could not find expected configured library function: {fname}"
                )),
                None,
            );
            0
        }
    }

    fn add_fid(&mut self, fname: &str, fid: u32) {
        self.get_funcs_mut().insert(fname.to_string(), fid);
    }
}

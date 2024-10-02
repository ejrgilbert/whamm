use std::collections::HashMap;
use orca_wasm::ir::function::FunctionModifier;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::{DataType, Module, Opcode};
use crate::common::error::ErrorGen;
use crate::libraries::core::WHAMM_CORE_LIB_NAME;
use crate::libraries::linking::import_lib::import_func;

const UNEXPECTED_ERR_MSG: &str =
    "IOAdapter: Looks like you've found a bug...please report this behavior! Exiting now...";

// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
pub struct IOAdapter {
    // func_name -> fid (if used)
    funcs: HashMap<String, Option<u32>>
}
impl Default for IOAdapter {
    fn default() -> Self {
        Self::new()
    }
}
impl IOAdapter {
    pub fn new() -> Self {
        let funcs = HashMap::from([
            ("putc".to_string(), None),
            ("puti".to_string(), None)
        ]);
        //Reserve map 0 for the var metadata map and map 1 for the map metadata map
        IOAdapter {
            funcs
        }
    }

    pub fn putsln(&mut self, s: String, func: &mut FunctionModifier, err: &mut ErrorGen) {
        // s -> [u8] (no need for data segment!)
        // iterate over and call putc
        self.puts(s, func, err);
        self.putln(func, err);
    }

    pub fn puts(&mut self, s: String, func: &mut FunctionModifier, err: &mut ErrorGen) {
        // s -> [u8] (no need for data segment!)
        // iterate over and call putc
        let data = s.as_bytes();
        for c in data.iter() {
            self.putc(*c, func, err);
        }
    }

    pub fn putln(&mut self, func: &mut FunctionModifier, err: &mut ErrorGen) {
        self.puts("\n".to_string(), func, err)
    }

    pub fn call_puti(&mut self, func: &mut FunctionModifier, err: &mut ErrorGen) {
        self.call("puti", func, err);
    }

    fn putc(&mut self, c: u8, func: &mut FunctionModifier, err: &mut ErrorGen) {
        func.i32_const(c as i32);
        self.call_putc(func, err);
    }

    pub fn call_putc(&mut self, func: &mut FunctionModifier, err: &mut ErrorGen) {
        self.call("putc", func, err)
    }

    // INNER HELPERS
    fn get_fid(&self, fname: &str, err: &mut ErrorGen) -> &Option<u32> {
        if let Some(fid) = self.funcs.get(fname) {
            fid
        } else {
            err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Could not find expected configured library function: {fname}"
                )),
                None,
            );
            &None
        }
    }

    fn add_fid(&mut self, fname: &str, fid: u32) {
        self.funcs.insert(fname.to_string(), Some(fid));
    }

    fn call(&mut self, fname: &str, func: &mut FunctionModifier, err: &mut ErrorGen) {
        if let Some(fid) = self.get_fid(fname, err) {
            func.call(FunctionID(*fid));
        } else {
            err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Library function is not imported yet, but it should be: {fname}"
                )),
                None,
            );
        }
    }

    pub fn check_deps(&mut self, fnames: Vec<String>, err: &mut ErrorGen) -> Vec<String> {
        let mut missing = vec![];
        for fname in fnames.iter() {
            if self.get_fid(fname, err).is_none() {
                // return the missing fname
                missing.push(fname.clone());
            }
        }
        missing
    }

    pub fn fix_import(&mut self, fname: &str, app_wasm: &mut Module, err: &mut ErrorGen) {
        // import the missing function!
        let fid = import_func(WHAMM_CORE_LIB_NAME, fname, &[DataType::I32], &[], app_wasm, err);
        self.add_fid(fname, fid);
    }
}

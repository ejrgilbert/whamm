use crate::common::error::ErrorGen;
use crate::libraries::core::LibAdapter;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::Opcode;
use std::collections::HashMap;

pub const PUTC: &str = "putc";
pub const PUTI: &str = "puti";

// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
pub struct IOAdapter {
    pub is_used: bool,
    // func_name -> fid
    funcs: HashMap<String, u32>,
}
impl LibAdapter for IOAdapter {
    fn get_funcs(&self) -> &HashMap<String, u32> {
        &self.funcs
    }
    fn get_funcs_mut(&mut self) -> &mut HashMap<String, u32> {
        &mut self.funcs
    }
}
impl Default for IOAdapter {
    fn default() -> Self {
        Self::new()
    }
}
impl IOAdapter {
    pub fn new() -> Self {
        let funcs = HashMap::from([(PUTC.to_string(), 0), (PUTI.to_string(), 0)]);
        //Reserve map 0 for the var metadata map and map 1 for the map metadata map
        IOAdapter {
            is_used: false,
            funcs,
        }
    }

    pub fn putsln<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        s: String,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        // s -> [u8] (no need for data segment!)
        // iterate over and call putc
        self.puts(s, func, err);
        self.putln(func, err);
    }

    pub fn puts<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        s: String,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        // s -> [u8] (no need for data segment!)
        // iterate over and call putc
        let data = s.as_bytes();
        for c in data.iter() {
            self.putc(*c, func, err);
        }
    }

    pub fn putln<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.puts("\n".to_string(), func, err)
    }

    pub fn call_puti<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTI, func, err);
    }

    fn putc<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        c: u8,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        func.i32_const(c as i32);
        self.call_putc(func, err);
    }

    pub fn call_putc<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTC, func, err)
    }

    fn call<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        fname: &str,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        let fid = self.get_fid(fname, err);
        func.call(FunctionID(fid));
    }
}

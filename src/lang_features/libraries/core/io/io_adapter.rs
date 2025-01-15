use crate::common::error::ErrorGen;
use crate::lang_features::libraries::core::LibAdapter;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, LocalID};
use orca_wasm::ir::types::{BlockType, DataType as OrcaType};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::{Module, Opcode};
use std::collections::HashMap;

pub const PUTS: &str = "puts";
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
    fn define_helper_funcs(
        &mut self,
        app_wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        self.emit_helper_funcs(app_wasm, err)
    }
}
impl Default for IOAdapter {
    fn default() -> Self {
        Self::new()
    }
}
impl IOAdapter {
    pub fn new() -> Self {
        let funcs = HashMap::from([
            (PUTC.to_string(), 0),
            (PUTI.to_string(), 0),
            (PUTS.to_string(), 0),
        ]);
        //Reserve map 0 for the var metadata map and map 1 for the map metadata map
        IOAdapter {
            is_used: false,
            funcs,
        }
    }

    fn emit_helper_funcs(&mut self, app_wasm: &mut Module, err: &mut ErrorGen) -> Vec<FunctionID> {
        vec![self.emit_puts(app_wasm, err)]
    }
    fn emit_puts(&mut self, app_wasm: &mut Module, err: &mut ErrorGen) -> FunctionID {
        let start_addr = LocalID(0);
        let len = LocalID(1);
        let mut puts = FunctionBuilder::new(&[OrcaType::I32, OrcaType::I32], &[]);

        let i = puts.add_local(OrcaType::I32);

        #[rustfmt::skip]
        puts.loop_stmt(BlockType::Empty)
            // Check if we've reached the end of the string
            .local_get(i)
            .local_get(len)
            .i32_lt_unsigned()
            .i32_eqz()
            .br_if(1)

            // get next char
            .local_get(start_addr)
            .local_get(i)
            .i32_add()
            // load a byte from memory
            .i32_load8_u(
                wasmparser::MemArg {
                    align: 0,
                    max_align: 0,
                    offset: 0,
                    memory: 0 // app memory!
                }
            );

        self.call_putc(&mut puts, err);

        // Increment i and continue loop
        puts.local_get(i)
            .i32_const(1)
            .i32_add()
            .local_set(i)
            .br(0) // (;3;)
            .end();

        let puts_fid = puts.finish_module(app_wasm);
        app_wasm.set_fn_name(puts_fid, "puts".to_string());
        self.add_fid(PUTS, *puts_fid);

        puts_fid
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

    pub fn call_puts<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTS, func, err);
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

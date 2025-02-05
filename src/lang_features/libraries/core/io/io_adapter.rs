// may use some of this code in the future (intrusive_puts)
#![allow(dead_code)]
use crate::common::error::ErrorGen;
use crate::lang_features::libraries::core::LibAdapter;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::types::{BlockType, DataType as OrcaType};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::{Module, Opcode};
use std::collections::HashMap;

// FROM LIB

pub const PUTS: &str = "puts";
pub const PUTC: &str = "putc";
pub const PUTU8: &str = "putu8";
pub const PUTI8: &str = "puti8";
pub const PUTU16: &str = "putu16";
pub const PUTI16: &str = "puti16";
pub const PUTU32: &str = "putu32";
pub const PUTI32: &str = "puti32";
pub const PUTU64: &str = "putu64";
pub const PUTI64: &str = "puti64";
pub const PUTF32: &str = "putf32";
pub const PUTF64: &str = "putf64";

// HELPER FUNCTIONS

pub const PUTS_INTERNAL: &str = "safe_puts";
pub const INTRUSIVE_PUTS: &str = "intrusive_puts";
pub const INTRUSIVE_PUTS_MAX: u32 = 100;

// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
pub struct IOAdapter {
    pub is_used: bool,
    // func_name -> fid
    funcs: HashMap<String, u32>,

    pub(crate) app_mem: i32,
    pub(crate) lib_mem: i32,
    mem_tracker_global: u32,
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
impl IOAdapter {
    pub fn new(mem_tracker_global: u32) -> Self {
        let funcs = HashMap::from([
            (PUTC.to_string(), 0),
            (PUTU8.to_string(), 0),
            (PUTI8.to_string(), 0),
            (PUTU16.to_string(), 0),
            (PUTI16.to_string(), 0),
            (PUTU32.to_string(), 0),
            (PUTI32.to_string(), 0),
            (PUTU64.to_string(), 0),
            (PUTI64.to_string(), 0),
            (PUTF32.to_string(), 0),
            (PUTF64.to_string(), 0),
            (PUTS.to_string(), 0),
        ]);
        //Reserve map 0 for the var metadata map and map 1 for the map metadata map
        IOAdapter {
            is_used: false,
            funcs,
            app_mem: -1,
            lib_mem: -1,
            mem_tracker_global,
        }
    }

    fn emit_helper_funcs(&mut self, app_wasm: &mut Module, err: &mut ErrorGen) -> Vec<FunctionID> {
        vec![
            self.emit_puts_internal(app_wasm, err),
        ]
    }
    fn emit_puts_internal(&mut self, app_wasm: &mut Module, err: &mut ErrorGen) -> FunctionID {
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
                    memory: self.app_mem as u32
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
        app_wasm.set_fn_name(puts_fid, PUTS_INTERNAL.to_string());
        self.add_fid(PUTS_INTERNAL, *puts_fid);

        puts_fid
    }
    fn emit_intrusive_puts(&mut self, app_wasm: &mut Module, err: &mut ErrorGen) -> FunctionID {
        let str_addr = LocalID(0);
        let len = LocalID(1);
        let mut puts = FunctionBuilder::new(&[OrcaType::I32, OrcaType::I32], &[]);

        let i = puts.add_local(OrcaType::I32);
        let tmp = puts.add_local(OrcaType::I32);

        let my_mem = wasmparser::MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: self.app_mem as u32,
        };
        let lib_mem = wasmparser::MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: self.lib_mem as u32,
        };
        let mem_tracker = GlobalID(self.mem_tracker_global);

        #[rustfmt::skip]
        puts.loop_stmt(BlockType::Empty)
            // save old data
            .local_get(str_addr)
            .local_get(i)
            .i32_add()         // mem pointer
            .i32_load8_u(lib_mem) // load old char
            .local_set(tmp)

            .global_get(mem_tracker)
            .local_get(i)
            .i32_add()
            .local_get(tmp)
            .i32_store8(my_mem) // store old char

            // write new data
            .local_get(str_addr)
            .local_get(i)
            .i32_add()
            .i32_load8_u(my_mem) // load new char
            .local_set(tmp)
            .local_get(str_addr)
            .local_get(i)
            .i32_add()
            .local_get(tmp)
            .i32_store8(lib_mem) // store new char


            // update i
            .i32_const(1)
            .local_get(i)
            .i32_add()
            .local_set(i)

            // continue loop if we're still less than the length of the string
            .local_get(i)
            .local_get(len)
            .i32_lt_signed()
            .br_if(0)
        .end();

        puts.local_get(str_addr).local_get(len);

        self.call_puts(&mut puts, err);

        // write back old data
        puts.i32_const(0)
            .local_set(i)
            .loop_stmt(BlockType::Empty)
            // load old data
            .global_get(mem_tracker)
            .local_get(i)
            .i32_add() // mem pointer
            .i32_load8_u(my_mem)
            .local_set(tmp)
            // write back old data
            .local_get(str_addr)
            .local_get(i)
            .i32_add()
            .local_get(tmp)
            .i32_store8(lib_mem) // store old char
            // update i
            .i32_const(1)
            .local_get(i)
            .i32_add()
            .local_set(i)
            // continue loop if we're still less than the length of the string
            .local_get(i)
            .local_get(len)
            .i32_lt_signed()
            .br_if(0)
            .end();

        let puts_fid = puts.finish_module(app_wasm);
        app_wasm.set_fn_name(puts_fid, INTRUSIVE_PUTS.to_string());
        self.add_fid(INTRUSIVE_PUTS, *puts_fid);

        puts_fid
    }

    pub fn puts<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        start_addr: u32,
        len: u32,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.puts_internal(start_addr, len, func, err);
    }

    pub fn putsln<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        start_addr: u32,
        len: u32,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.puts(start_addr, len, func, err);
        self.putln(func, err);
    }

    pub fn putln<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.putc(b'\n', func, err)
    }

    fn puts_internal<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        start_addr: u32,
        len: u32,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        func.u32_const(start_addr).u32_const(len);
        self.call_puts_internal(func, err);
    }

    pub(crate) fn call_puts_internal<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTS_INTERNAL, func, err);
    }

    pub fn call_puts<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTS, func, err);
    }

    pub fn call_putu8<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTU8, func, err);
    }

    pub fn call_puti8<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTI8, func, err);
    }

    pub fn call_putu16<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTU16, func, err);
    }

    pub fn call_puti16<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTI16, func, err);
    }

    pub fn call_putu32<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTU32, func, err);
    }

    pub fn call_puti32<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTI32, func, err);
    }

    pub fn call_putu64<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTU64, func, err);
    }

    pub fn call_puti64<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTI64, func, err);
    }

    pub fn call_putf32<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTF32, func, err);
    }

    pub fn call_putf64<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PUTF64, func, err);
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

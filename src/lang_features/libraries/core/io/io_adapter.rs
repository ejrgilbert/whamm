// may use some of this code in the future (intrusive_puts)
#![allow(dead_code)]
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::tag_handler::get_tag_for;
use crate::lang_features::libraries::core::LibAdapter;
use std::collections::HashMap;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, LocalID};
use wirm::ir::types::{BlockType, DataType as WirmType};
use wirm::module_builder::AddLocal;
use wirm::opcode::MacroOpcode;
use wirm::{Module, Opcode};

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
pub const PUTBOOL: &str = "putbool";

// HELPER FUNCTIONS

pub const PUTS_INTERNAL: &str = "puts_internal";
pub const INTRUSIVE_PUTS: &str = "intrusive_puts";
pub const INTRUSIVE_PUTS_MAX: u32 = 100;

// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
pub struct IOAdapter {
    pub is_used: bool,
    // func_name -> fid
    funcs: HashMap<String, u32>,

    pub(crate) instr_mem: i32,
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
    fn define_helper_funcs(&mut self, app_wasm: &mut Module) -> Vec<FunctionID> {
        self.emit_helper_funcs(app_wasm)
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
            (PUTBOOL.to_string(), 0),
            (PUTS.to_string(), 0),
        ]);
        //Reserve map 0 for the var metadata map and map 1 for the map metadata map
        IOAdapter {
            is_used: false,
            funcs,
            instr_mem: -1,
            lib_mem: -1,
            mem_tracker_global,
        }
    }

    fn emit_helper_funcs(&mut self, app_wasm: &mut Module) -> Vec<FunctionID> {
        vec![self.emit_puts_internal(app_wasm)]
    }
    fn emit_puts_internal(&mut self, app_wasm: &mut Module) -> FunctionID {
        let start_addr = LocalID(0);
        let len = LocalID(1);
        let mut puts = FunctionBuilder::new(&[WirmType::I32, WirmType::I32], &[]);

        let i = puts.add_local(WirmType::I32);

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
                    memory: self.instr_mem as u32
                }
            );

        self.call_putc(&mut puts);

        // Increment i and continue loop
        puts.local_get(i)
            .i32_const(1)
            .i32_add()
            .local_set(i)
            .br(0) // (;3;)
            .end();

        let puts_fid = puts.finish_module_with_tag(app_wasm, get_tag_for(&None));
        app_wasm.set_fn_name(puts_fid, PUTS_INTERNAL.to_string());
        self.add_fid(PUTS_INTERNAL, *puts_fid);

        puts_fid
    }
    fn emit_intrusive_puts(
        &mut self,
        mem_allocator: &mut MemoryAllocator,
        app_wasm: &mut Module,
    ) -> FunctionID {
        let str_addr = LocalID(0);
        let len = LocalID(1);
        let mut puts = FunctionBuilder::new(&[WirmType::I32, WirmType::I32], &[]);

        mem_allocator.copy_to_mem_and_save(
            self.instr_mem as u32,
            str_addr,
            len,
            self.lib_mem as u32,
            0,
            &mut puts,
        );

        puts.local_get(str_addr).local_get(len);
        self.call_puts(&mut puts);

        mem_allocator.copy_back_saved_mem(len, self.lib_mem as u32, 0, &mut puts);

        let puts_fid = puts.finish_module_with_tag(app_wasm, get_tag_for(&None));
        app_wasm.set_fn_name(puts_fid, INTRUSIVE_PUTS.to_string());
        self.add_fid(INTRUSIVE_PUTS, *puts_fid);

        puts_fid
    }

    pub fn puts<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        start_addr: u32,
        len: u32,
        func: &mut T,
    ) {
        self.puts_internal(start_addr, len, func);
    }

    pub fn putsln<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        start_addr: u32,
        len: u32,
        func: &mut T,
    ) {
        self.puts(start_addr, len, func);
        self.putln(func);
    }

    pub fn putln<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.putc(b'\n', func)
    }

    fn puts_internal<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        start_addr: u32,
        len: u32,
        func: &mut T,
    ) {
        func.u32_const(start_addr).u32_const(len);
        self.call_puts_internal(func);
    }

    pub(crate) fn call_puts_internal<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
    ) {
        self.call(PUTS_INTERNAL, func);
    }

    pub fn call_puts<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTS, func);
    }

    pub fn call_putu8<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTU8, func);
    }

    pub fn call_puti8<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTI8, func);
    }

    pub fn call_putu16<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTU16, func);
    }

    pub fn call_puti16<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTI16, func);
    }

    pub fn call_putu32<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTU32, func);
    }

    pub fn call_puti32<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTI32, func);
    }

    pub fn call_putu64<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTU64, func);
    }

    pub fn call_puti64<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTI64, func);
    }

    pub fn call_putf32<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTF32, func);
    }

    pub fn call_putf64<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTF64, func);
    }

    pub fn call_putbool<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTBOOL, func);
    }

    fn putc<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, c: u8, func: &mut T) {
        func.i32_const(c as i32);
        self.call_putc(func);
    }

    pub fn call_putc<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, func: &mut T) {
        self.call(PUTC, func)
    }

    fn call<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(&mut self, fname: &str, func: &mut T) {
        let fid = self.get_fid(fname);
        func.call(FunctionID(fid));
    }
}

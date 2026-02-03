use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::lang_features::libraries::core::LibAdapter;
use std::collections::HashMap;
use wirm::ir::id::{FunctionID, LocalID};
use wirm::module_builder::AddLocal;
use wirm::opcode::MacroOpcode;
use wirm::{Module, Opcode};

pub const MEM_ALLOC: &str = "mem_alloc";
pub const MEM_FREE: &str = "mem_free";

// this is the code that knows how to invoke the utils mod
// in the whamm_core library
pub struct UtilsAdapter {
    pub is_used: bool,
    funcs: HashMap<String, u32>,
}
impl LibAdapter for UtilsAdapter {
    fn get_funcs(&self) -> &HashMap<String, u32> {
        &self.funcs
    }
    fn get_funcs_mut(&mut self) -> &mut HashMap<String, u32> {
        &mut self.funcs
    }
    fn define_helper_funcs(
        &mut self,
        _: &UtilsAdapter,
        _: &mut MemoryAllocator,
        app_wasm: &mut Module,
        _: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        self.emit_helper_funcs(app_wasm)
    }
}
impl UtilsAdapter {
    pub fn new() -> Self {
        let funcs = HashMap::from([(MEM_ALLOC.to_string(), 0), (MEM_FREE.to_string(), 0)]);
        Self {
            is_used: false,
            funcs,
        }
    }
    pub fn emit_helper_funcs(&mut self, _: &mut Module) -> Vec<FunctionID> {
        // (nothing to do)
        vec![]
    }

    pub fn mem_alloc<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        len: i32,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        func.i32_const(len);
        self.call_mem_alloc(func, err);
    }

    pub fn mem_alloc_from_local<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        local: LocalID,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        func.local_get(local);
        self.call_mem_alloc(func, err);
    }

    fn call_mem_alloc<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(MEM_ALLOC, func, err)
    }

    pub fn mem_free<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        ptr: i32,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        func.i32_const(ptr);
        self.call_mem_free(func, err);
    }

    pub fn mem_free_from_local<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        local: LocalID,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        func.local_get(local);
        self.call_mem_free(func, err);
    }

    fn call_mem_free<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(MEM_FREE, func, err)
    }

    fn call<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        fname: &str,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        let fid = self.get_fid(fname, err);
        func.call(FunctionID(fid));
    }
}

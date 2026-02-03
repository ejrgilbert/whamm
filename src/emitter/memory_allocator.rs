use crate::common::error::ErrorGen;
use crate::common::instr::ENGINE_BUFFER_MAX_SIZE;
use crate::emitter::tag_handler::get_tag_for;
use crate::parser::types::DataType;
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use std::collections::HashMap;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, GlobalID, LocalID, MemoryID};
use wirm::ir::types::DataType as WirmType;
use wirm::ir::types::{BlockType, InitExpr, Value as WirmValue};
use wirm::module_builder::AddLocal;
use wirm::opcode::MacroOpcode;
use wirm::wasmparser::MemArg;
use wirm::{DataSegment, DataSegmentKind, InitInstr, Module, Opcode};

pub const WASM_PAGE_SIZE: u32 = 65_536;
pub const VAR_BLOCK_BASE_VAR: &str = "var_block_base_offset";

const UNEXPECTED_ERR_MSG: &str =
    "MemoryAllocator: Looks like you've found a bug...please report this behavior! Exiting now...";

pub struct MemoryAllocator {
    pub mem_id: u32,
    curr_mem_offset: usize,
    pub mem_tracker_global: GlobalID,

    pub alloc_var_mem_id: Option<u32>,
    pub alloc_var_mem_tracker_global: Option<GlobalID>,

    // used to save off data transmitted from the engine (e.g. fname)
    pub engine_mem_id: Option<u32>,
    pub engine_mem_start_global: Option<GlobalID>,
    // Constant pool for strings emitted thus far
    pub emitted_strings: HashMap<String, StringAddr>,

    // The Wasm func ID for a function that can be called to check
    // the used memory (mem_tracker_global value) vs. the current memory size.
    // It will grow the memory if necessary.
    pub base_mem_checker_fid: Option<u32>,
    pub alloc_mem_checker_fid: Option<u32>,
}
impl MemoryAllocator {
    pub fn new(
        mem_id: u32,
        mem_tracker_global: GlobalID,
        alloc_var_mem_id: Option<u32>,
        alloc_var_mem_tracker_global: Option<GlobalID>,
        engine_mem_id: Option<u32>,
        engine_mem_start_global: Option<GlobalID>,
    ) -> Self {
        Self {
            mem_id,
            mem_tracker_global,
            alloc_var_mem_id,
            alloc_var_mem_tracker_global,
            engine_mem_id,
            engine_mem_start_global,
            curr_mem_offset: 0,
            emitted_strings: HashMap::new(),
            base_mem_checker_fid: None,
            alloc_mem_checker_fid: None,
        }
    }

    // ===================
    // ==== Get / Set ====
    // ===================

    pub(crate) fn emit_addr<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        table: &SymbolTable,
        injector: &mut T,
    ) {
        // get the var block base offset variable
        let Some(Record::Var {
            addr: Some(addrs), ..
        }) = table.lookup_var(VAR_BLOCK_BASE_VAR, true)
        else {
            unreachable!("unexpected type");
        };

        let VarAddr::Local {
            addr: var_block_start,
        } = addrs.first().unwrap()
        else {
            assert_eq!(addrs.len(), 1);
            panic!("memory address should be represented with a single address")
        };
        injector.local_get(LocalID(*var_block_start));
    }

    pub fn get_from_mem<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        mem_id: u32,
        ty: &DataType,
        var_offset: u32,
        table: &SymbolTable,
        injector: &mut T,
    ) {
        self.emit_addr(table, injector);

        // perform the correct load based on the type of data at this memory location
        match ty {
            DataType::U8 => injector.i32_load8_u(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::I8 => injector.i32_load8_s(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::U16 => injector.i32_load16_u(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::I16 => injector.i32_load16_s(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::U32 | DataType::I32 | DataType::Boolean => injector.i32_load(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::F32 => injector.f32_load(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::U64 | DataType::I64 => injector.i64_load(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::F64 => injector.f64_load(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::Null
            | DataType::Str
            | DataType::Tuple { .. }
            | DataType::Map { .. }
            | DataType::AssumeGood
            | DataType::Unknown => unimplemented!(),
        };
    }

    pub fn set_in_mem<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        var_offset: u32,
        mem_id: u32,
        ty: &DataType,
        injector: &mut T,
    ) {
        // perform the correct store based on the type of data at this memory location
        match ty {
            DataType::U8 | DataType::I8 => injector.i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::U16 | DataType::I16 => injector.i32_store16(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::U32 | DataType::I32 | DataType::Boolean => injector.i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::F32 => injector.f32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::U64 | DataType::I64 => injector.i64_store(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::F64 => injector.f64_store(MemArg {
                align: 0,
                max_align: 0,
                offset: var_offset as u64,
                memory: mem_id,
            }),
            DataType::Null
            | DataType::Str
            | DataType::Tuple { .. }
            | DataType::Map { .. }
            | DataType::AssumeGood
            | DataType::Unknown => unimplemented!(),
        };
    }

    pub fn copy_to_mem_global_ptr<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        src_mem_id: u32,
        src_offset: LocalID,
        src_len: LocalID,
        dst_mem_id: u32,
        dst_mem_tracker: GlobalID,
        func: &mut T,
    ) {
        self.copy_to_mem(
            src_mem_id,
            src_offset,
            src_len,
            dst_mem_id,
            |func| func.global_get(dst_mem_tracker),
            func,
        );

        // update the destination memory's tracker
        func.local_get(src_len)
            .global_get(dst_mem_tracker)
            .i32_add()
            .global_set(dst_mem_tracker);
    }

    fn copy_to_mem<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal, F>(
        &self,
        src_mem_id: u32,
        src_offset: LocalID,
        src_len: LocalID,
        dst_mem_id: u32,
        get_ptr: F,
        func: &mut T,
    ) where
        F: Fn(&mut T) -> &mut T,
    {
        let i = func.add_local(WirmType::I32);
        let tmp = func.add_local(WirmType::I32);

        let src_mem = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: src_mem_id,
        };
        let dst_mem = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: dst_mem_id,
        };

        #[rustfmt::skip]
        func.loop_stmt(BlockType::Empty)
            // write new data
            .local_get(src_offset)
            .local_get(i)
            .i32_add()
            .i32_load8_u(src_mem) // load new char
            .local_set(tmp);
        get_ptr(func)
            .local_get(i)
            .i32_add()
            .local_get(tmp)
            .i32_store8(dst_mem) // store new char
            // update i
            .i32_const(1)
            .local_get(i)
            .i32_add()
            .local_set(i)
            // continue loop if we're still less than the length of the string
            .local_get(i)
            .local_get(src_len)
            .i32_lt_signed()
            .br_if(0)
            .end();
    }

    pub fn copy_to_mem_local_ptr<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        src_mem_id: u32,
        src_offset: LocalID,
        src_len: LocalID,
        dst_mem_id: u32,
        dst_mem_ptr: LocalID,
        func: &mut T,
    ) {
        self.copy_to_mem(
            src_mem_id,
            src_offset,
            src_len,
            dst_mem_id,
            |func| func.local_get(dst_mem_ptr),
            func,
        );
    }

    // =====================
    // ==== Allocations ====
    // =====================
    pub fn update_alloc_mem_tracker(&mut self, offset: u32, func: &mut FunctionBuilder) {
        // increment the memory byte offset global
        if let Some(tracker) = self.alloc_var_mem_tracker_global {
            func.global_get(tracker)
                .u32_const(offset)
                .i32_add()
                .global_set(tracker);
        } else {
            panic!("alloc tracker not set")
        }
    }
    fn gen_mem_checker_fn_for(
        &mut self,
        wasm: &mut Module,
        mem_id: u32,
        tracker: GlobalID,
    ) -> FunctionID {
        // specify params
        let bytes_needed = LocalID(0);
        let check_memsize_params = vec![WirmType::I32];

        let mut check_memsize = FunctionBuilder::new(&check_memsize_params, &[]);

        // specify locals
        let bytes_per_page = check_memsize.add_local(WirmType::I32);
        let curr_pages = check_memsize.add_local(WirmType::I32);
        let max_needed_addr = check_memsize.add_local(WirmType::I32);

        check_memsize
            .u32_const(WASM_PAGE_SIZE)
            .local_set(bytes_per_page);
        check_memsize.memory_size(mem_id).local_set(curr_pages);

        check_memsize
            .global_get(tracker)
            .local_get(bytes_needed)
            .i32_add()
            .local_set(max_needed_addr);

        // check if the needed memory range is larger than what is currently available
        check_memsize
            .local_get(bytes_per_page)
            .local_get(curr_pages)
            .i32_mul()
            .local_get(max_needed_addr)
            .i32_lt_unsigned();

        // If it is larger, grow memory by a page
        check_memsize
            .if_stmt(BlockType::Empty)
            .i32_const(1)
            .memory_grow(mem_id)
            .drop()
            .end();

        let check_memsize_fid = check_memsize.finish_module_with_tag(wasm, get_tag_for(&None));
        wasm.set_fn_name(
            check_memsize_fid,
            format!("check_memsize_for_mem{}", mem_id),
        );
        check_memsize_fid
    }
    pub fn gen_mem_checker_fns(&mut self, wasm: &mut Module) {
        if self.base_mem_checker_fid.is_none() {
            self.base_mem_checker_fid =
                Some(*self.gen_mem_checker_fn_for(wasm, self.mem_id, self.mem_tracker_global));
        }
        if self.alloc_mem_checker_fid.is_none() {
            self.alloc_mem_checker_fid = Some(
                *self.gen_mem_checker_fn_for(
                    wasm,
                    self.alloc_var_mem_id
                        .unwrap_or_else(|| panic!("alloc mem id not set")),
                    self.alloc_var_mem_tracker_global
                        .unwrap_or_else(|| panic!("alloc mem tracker id not set")),
                ),
            );
        }
    }
    fn emit_memsize_check_with(
        &self,
        fid: Option<u32>,
        needed_bytes: u32,
        func: &mut FunctionBuilder,
        err: &mut ErrorGen,
    ) {
        let check_memsize_fid = match fid {
            Some(fid) => fid,
            None => {
                err.wei_error(
                    "Unexpected state while generating the memory allocation function. \
                    The memory size checker function has not been generated yet."
                        .to_string(),
                    &None,
                );
                unreachable!()
            }
        };
        func.u32_const(needed_bytes);
        func.call(FunctionID(check_memsize_fid));
    }
    pub fn emit_base_memsize_check(&self, needed_bytes: LocalID, func: &mut FunctionBuilder) {
        let check_memsize_fid = match self.base_mem_checker_fid {
            Some(fid) => fid,
            None => {
                unreachable!(
                    "Unexpected state while generating the memory allocation function. \
                    The memory size checker function has not been generated yet."
                );
            }
        };
        func.local_get(needed_bytes);
        func.call(FunctionID(check_memsize_fid));
    }
    pub fn emit_alloc_memsize_check(
        &self,
        needed_bytes: u32,
        func: &mut FunctionBuilder,
        err: &mut ErrorGen,
    ) {
        self.emit_memsize_check_with(self.alloc_mem_checker_fid, needed_bytes, func, err);
    }
    pub fn emit_store_from_local(
        &mut self,
        curr_offset: u32,
        local: LocalID,
        local_ty: &WirmType,
        mem_id: u32,
        mem_tracker: GlobalID,
        func: &mut FunctionBuilder,
    ) -> u32 {
        // store the local to memory
        func.global_get(mem_tracker);
        func.local_get(local);

        // todo -- store should be conditional on the datatype
        func.i32_store(MemArg {
            align: 0,
            max_align: 0,
            offset: curr_offset as u64,
            memory: mem_id, // instrumentation memory!
        });

        DataType::from_wasm_type(local_ty).num_bytes().unwrap() as u32
    }
    pub fn emit_store8_from_local(
        &mut self,
        curr_offset: u32,
        local: LocalID,
        mem_id: u32,
        mem_tracker: GlobalID,
        func: &mut FunctionBuilder,
    ) -> u32 {
        // store the local to memory
        func.global_get(mem_tracker);
        func.local_get(local);

        // todo -- store should be conditional on the datatype
        func.i32_store8(MemArg {
            align: 0,
            max_align: 0,
            offset: curr_offset as u64,
            memory: mem_id, // instrumentation memory!
        });

        size_of::<u8>() as u32
    }
    pub fn emit_string(&mut self, wasm: &mut Module, val: &mut String) -> bool {
        if self.emitted_strings.contains_key(val) {
            // the string has already been emitted into the module, don't emit again
            return true;
        }
        // assuming that the data ID is the index of the object in the Vec
        let val_bytes = val.as_bytes().to_owned();
        let data_segment = DataSegment {
            data: val_bytes,
            kind: DataSegmentKind::Active {
                memory_index: self.mem_id,
                offset_expr: InitExpr::new(vec![InitInstr::Value(WirmValue::I32(
                    self.curr_mem_offset as i32,
                ))]),
            },
            tag: None,
        };
        wasm.data.push(data_segment);

        // save the memory addresses/lens, so they can be used as appropriate
        self.emitted_strings.insert(
            val.clone(),
            StringAddr {
                mem_offset: self.curr_mem_offset,
                len: val.len(),
            },
        );

        // update curr_mem_offset to account for new data
        self.curr_mem_offset += val.len();
        true
    }
    pub fn lookup_emitted_string(&self, s: &str) -> (u32, u32) {
        if let Some(str_addr) = self.emitted_strings.get(s) {
            (str_addr.mem_offset as u32, str_addr.len as u32)
        } else {
            unreachable!(
                "{} Data segment not available for string: \"{}\"",
                UNEXPECTED_ERR_MSG, s
            )
        }
    }

    pub(crate) fn memory_grow(&mut self, wasm: &mut Module) {
        // If we've allocated any memory, bump the app's memory up to account for that
        if !self.emitted_strings.is_empty() {
            if let Some(mem) = wasm.memories.get_mut(MemoryID(self.mem_id)) {
                let req_pages = (((self.curr_mem_offset as i32 + ENGINE_BUFFER_MAX_SIZE) as u32
                    / WASM_PAGE_SIZE)
                    + 1) as u64;
                if mem.ty.initial < req_pages {
                    mem.ty.initial = req_pages;
                }
            }
        }
    }

    pub(crate) fn update_memory_global_ptrs(&mut self, wasm: &mut Module) {
        // use this function to account for the statically-used memory
        let buffer_size = if let Some(buffer_start) = self.engine_mem_start_global {
            // engine buffer should start after the statically used memory
            wasm.mod_global_init_expr(
                buffer_start,
                InitExpr::new(vec![InitInstr::Value(WirmValue::I32(
                    self.curr_mem_offset as i32,
                ))]),
            );
            ENGINE_BUFFER_MAX_SIZE
        } else {
            // There's no engine buffer!
            0
        };

        // memory tracker should point after the engine buffer
        wasm.mod_global_init_expr(
            self.mem_tracker_global,
            InitExpr::new(vec![InitInstr::Value(WirmValue::I32(
                self.curr_mem_offset as i32 + buffer_size,
            ))]),
        );
    }
}

pub struct StringAddr {
    pub mem_offset: usize,
    pub len: usize,
}

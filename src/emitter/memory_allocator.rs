use crate::common::error::ErrorGen;
use crate::parser::types::DataType;
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID, MemoryID};
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::ir::types::{BlockType, InitExpr, Value as OrcaValue};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::{DataSegment, DataSegmentKind, Instructions, Module, Opcode};
use std::collections::HashMap;
use wasmparser::MemArg;

pub const WASM_PAGE_SIZE: u32 = 65_536;
pub const VAR_BLOCK_BASE_VAR: &str = "var_block_base_offset";

const UNEXPECTED_ERR_MSG: &str =
    "MemoryAllocator: Looks like you've found a bug...please report this behavior! Exiting now...";

pub struct MemoryAllocator {
    pub mem_id: u32,
    pub curr_mem_offset: usize,
    // Constant pool for strings emitted thus far
    pub emitted_strings: HashMap<String, StringAddr>,

    // The Wasm Global ID for the global that tracks the current point
    // in memory that we can allocate to
    pub mem_tracker_global: GlobalID,

    // The Wasm func ID for a function that can be called to check
    // the used memory (mem_tracker_global value) vs. the current memory size.
    // It will grow the memory if necessary.
    pub used_mem_checker_fid: Option<u32>,
}
impl MemoryAllocator {
    // ===================
    // ==== Get / Set ====
    // ===================

    pub(crate) fn emit_addr<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        table: &SymbolTable,
        injector: &mut T,
        err: &mut ErrorGen,
    ) {
        // get the var block base offset variable
        let Some(Record::Var {
            addr: Some(VarAddr::Local {
                addr: var_block_start,
            }),
            ..
        }) = table.lookup_var(VAR_BLOCK_BASE_VAR, &None, err, true)
        else {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
            return;
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
        err: &mut ErrorGen,
    ) {
        self.emit_addr(table, injector, err);

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

    pub fn copy_to_mem_and_save<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        src_mem_id: u32,
        src_offset: LocalID,
        src_len: LocalID,
        dst_mem_id: u32,
        dst_offset: u32,
        func: &mut T,
    ) {
        let i = func.add_local(OrcaType::I32);
        let tmp = func.add_local(OrcaType::I32);

        let app_mem = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: self.mem_id,
        };
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
        let mem_tracker = self.mem_tracker_global;

        #[rustfmt::skip]
        func.loop_stmt(BlockType::Empty)
            // save old data
            .u32_const(dst_offset)
            .local_get(i)
            .i32_add()            // mem pointer
            .i32_load8_u(dst_mem) // load old char
            .local_set(tmp)

            .global_get(mem_tracker)
            .local_get(i)
            .i32_add()
            .local_get(tmp)
            .i32_store8(app_mem) // store old char

            // write new data
            .local_get(src_offset)
            .local_get(i)
            .i32_add()
            .i32_load8_u(src_mem) // load new char
            .local_set(tmp)
            .u32_const(dst_offset)
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

    pub fn copy_back_saved_mem<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        src_len: LocalID,
        dst_mem_id: u32,
        dst_offset: u32,
        func: &mut T,
    ) {
        let i = func.add_local(OrcaType::I32);
        let tmp = func.add_local(OrcaType::I32);

        let app_mem = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: self.mem_id,
        };
        let dst_mem = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: dst_mem_id,
        };
        let mem_tracker = self.mem_tracker_global;

        // write back old data
        func.i32_const(0)
            .local_set(i)
            .loop_stmt(BlockType::Empty)
            // load old data
            .global_get(mem_tracker)
            .local_get(i)
            .i32_add() // mem pointer
            .i32_load8_u(app_mem)
            .local_set(tmp)
            // write back old data
            .u32_const(dst_offset)
            .local_get(i)
            .i32_add()
            .local_get(tmp)
            .i32_store8(dst_mem) // store old char
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

    // =====================
    // ==== Allocations ====
    // =====================
    pub fn update_mem_tracker(&mut self, offset: u32, func: &mut FunctionBuilder) {
        // increment the memory byte offset global
        func.global_get(self.mem_tracker_global)
            .u32_const(offset)
            .i32_add()
            .global_set(self.mem_tracker_global);
    }
    pub fn gen_mem_checker_fn(&mut self, wasm: &mut Module) {
        if self.used_mem_checker_fid.is_none() {
            // specify params
            let bytes_needed = LocalID(0);
            let check_memsize_params = vec![OrcaType::I32];

            let mut check_memsize = FunctionBuilder::new(&check_memsize_params, &[]);

            // specify locals
            let bytes_per_page = check_memsize.add_local(OrcaType::I32);
            let curr_pages = check_memsize.add_local(OrcaType::I32);
            let max_needed_addr = check_memsize.add_local(OrcaType::I32);

            check_memsize
                .u32_const(WASM_PAGE_SIZE)
                .local_set(bytes_per_page);
            check_memsize.memory_size(self.mem_id).local_set(curr_pages);

            check_memsize
                .global_get(self.mem_tracker_global)
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
                .memory_grow(self.mem_id)
                .drop()
                .end();

            let check_memsize_fid = check_memsize.finish_module(wasm);
            wasm.set_fn_name(check_memsize_fid, "check_memsize".to_string());
            self.used_mem_checker_fid = Some(*check_memsize_fid);
        }
    }
    pub fn emit_memsize_check(
        &self,
        needed_bytes: u32,
        func: &mut FunctionBuilder,
        err: &mut ErrorGen,
    ) {
        let check_memsize_fid = match self.used_mem_checker_fid {
            Some(fid) => fid,
            None => {
                err.wizard_error(
                    true,
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
    pub fn emit_store_from_local(
        &mut self,
        curr_offset: u32,
        local: LocalID,
        local_ty: &OrcaType,
        func: &mut FunctionBuilder,
    ) -> u32 {
        // store the local to memory
        func.global_get(self.mem_tracker_global);
        func.local_get(local);

        // todo -- store should be conditional on the datatype
        func.i32_store(MemArg {
            align: 0,
            max_align: 0,
            offset: curr_offset as u64,
            memory: self.mem_id, // instrumentation memory!
        });

        DataType::from_wasm_type(local_ty).num_bytes().unwrap() as u32
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
                offset_expr: InitExpr::new(vec![Instructions::Value(OrcaValue::I32(
                    self.curr_mem_offset as i32,
                ))]),
            },
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
    pub fn lookup_emitted_string(&self, s: &String, err: &mut ErrorGen) -> (u32, u32) {
        if let Some(str_addr) = self.emitted_strings.get(s) {
            (str_addr.mem_offset as u32, str_addr.len as u32)
        } else {
            err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Data segment not available for string: \"{s}\"",
                )),
                None,
            );
            unreachable!()
        }
    }

    pub(crate) fn memory_grow(&mut self, wasm: &mut Module) {
        // If we've allocated any memory, bump the app's memory up to account for that
        if !self.emitted_strings.is_empty() {
            if let Some(mem) = wasm.memories.get_mut(MemoryID(self.mem_id)) {
                let req_pages = ((self.curr_mem_offset as u32 / WASM_PAGE_SIZE) + 1) as u64;
                if mem.ty.initial < req_pages {
                    mem.ty.initial = req_pages;
                }
            }
        }
    }

    pub(crate) fn update_memory_global_ptr(&mut self, wasm: &mut Module) {
        // use this function to account for the statically-used memory
        wasm.mod_global_init_expr(
            self.mem_tracker_global,
            InitExpr::new(vec![Instructions::Value(OrcaValue::I32(
                self.curr_mem_offset as i32,
            ))]),
        )
    }
}

pub struct StringAddr {
    pub mem_offset: usize,
    pub len: usize,
}

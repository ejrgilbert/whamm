use crate::common::error::ErrorGen;
use crate::parser::types::DataType;
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::ir::types::{BlockType, InitExpr, Value as OrcaValue};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::{DataSegment, DataSegmentKind, Instructions, Module, Opcode};
use std::collections::HashMap;
use wasmparser::MemArg;

pub const WASM_PAGE_SIZE: u32 = 65_536;
pub const VAR_BLOCK_BASE_VAR: &str = "var_block_base_offset";

pub struct MemoryAllocator {
    pub mem_id: u32,
    pub curr_mem_offset: usize,
    pub required_initial_mem_size: u64,
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

    fn calc_offset<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        var_offset: u32,
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

        // calculate the true offset
        injector.local_get(LocalID(*var_block_start));
        injector.u32_const(var_offset);
        injector.i32_add();
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
        self.calc_offset(var_offset, table, injector, err);

        // perform the correct load based on the type of data at this memory location
        match ty {
            DataType::U8 => injector.i32_load8_u(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::I8 => injector.i32_load8_s(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::U16 => injector.i32_load16_u(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::I16 => injector.i32_load16_s(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::U32 | DataType::I32 | DataType::Boolean => injector.i32_load(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::F32 => injector.f32_load(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::U64 | DataType::I64 => injector.i64_load(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::F64 => injector.f64_load(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
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
        mem_id: u32,
        ty: &DataType,
        var_offset: u32,
        table: &SymbolTable,
        injector: &mut T,
        err: &mut ErrorGen,
    ) {
        self.calc_offset(var_offset, table, injector, err);

        // perform the correct store based on the type of data at this memory location
        match ty {
            DataType::U8 | DataType::I8 => injector.i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::U16 | DataType::I16 => injector.i32_store16(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::U32 | DataType::I32 | DataType::Boolean => injector.i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::F32 => injector.f32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::U64 | DataType::I64 => injector.i64_store(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            }),
            DataType::F64 => injector.f64_store(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
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
        let data_id = wasm.data.len();
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
                data_id: data_id as u32,
                mem_offset: self.curr_mem_offset,
                len: val.len(),
            },
        );

        // update curr_mem_offset to account for new data
        self.curr_mem_offset += val.len();
        true
    }

    pub(crate) fn memory_grow(&mut self, wasm: &mut Module) {
        // If we've allocated any memory, bump the app's memory up to account for that

        // TODO -- this doesn't actually increase the required_initial_mem_size at any point
        if !self.emitted_strings.is_empty() {
            if let Some(mem) = wasm.memories.get_mut(0) {
                if mem.initial < self.required_initial_mem_size {
                    mem.initial = self.required_initial_mem_size;
                }
            }
        }
    }
}

pub struct StringAddr {
    pub data_id: u32,
    pub mem_offset: usize,
    pub len: usize,
}

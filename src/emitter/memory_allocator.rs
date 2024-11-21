use crate::common::error::ErrorGen;
use crate::emitter::utils::whamm_type_to_wasm_type;
use crate::parser::types::DataType;
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{GlobalID, LocalID};
use orca_wasm::ir::types::Value as OrcaValue;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::{DataSegment, DataSegmentKind, InitExpr, Module, Opcode};
use std::collections::HashMap;
use wasmparser::MemArg;

pub const VAR_BLOCK_BASE_VAR: &str = "var_block_base_offset";

pub struct MemoryAllocator {
    pub mem_id: u32,
    pub curr_mem_offset: usize,
    pub required_initial_mem_size: u64,
    pub emitted_strings: HashMap<String, StringAddr>,

    // The Wasm Global ID for the global that tracks the current point
    // in memory that we can allocate to
    pub mem_tracker_global: GlobalID,
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
            | DataType::AssumeGood => unimplemented!(),
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
            | DataType::AssumeGood => unimplemented!(),
        };
    }

    // =====================
    // ==== Allocations ====
    // =====================
    pub fn alloc_mem_space(
        &mut self,
        next_var_offset: u32,
        ty: &DataType,
        func: &mut FunctionBuilder,
    ) -> (VarAddr, u32) {
        // increment the memory byte offset global
        func.global_get(self.mem_tracker_global);

        let bytes_used = ty.num_bytes().unwrap() as u32;
        func.u32_const(bytes_used);
        func.i32_add();
        func.global_set(self.mem_tracker_global);

        // create the VarAddr
        let orca_ty = whamm_type_to_wasm_type(ty);
        (
            VarAddr::MemLoc {
                mem_id: self.mem_id,
                ty: if orca_ty.len() == 1 {
                    *orca_ty.first().unwrap()
                } else {
                    todo!()
                },
                var_offset: next_var_offset,
            },
            bytes_used,
        )
    }
    pub fn emit_store_from_local(
        &mut self,
        next_var_offset: u32,
        local: LocalID,
        local_ty: &DataType,
        func: &mut FunctionBuilder,
    ) -> (VarAddr, u32) {
        // store the local to memory
        func.global_get(self.mem_tracker_global);
        func.local_get(local);
        func.i32_store(wasmparser::MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: self.mem_id, // instrumentation memory!
        });

        self.alloc_mem_space(next_var_offset, local_ty, func)
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
                offset_expr: InitExpr::Value(OrcaValue::I32(self.curr_mem_offset as i32)),
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

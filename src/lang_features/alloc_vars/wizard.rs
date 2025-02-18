use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::StringAddr;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::generator::wizard::ast::UnsharedVar;
use crate::lang_features::report_vars::Metadata as ReportMetadata;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::{DataType, Definition};
use crate::verifier::types::{Record, VarAddr};
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::LocalID;
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::Opcode;
use std::collections::HashMap;
use wasmparser::MemArg;

#[derive(Default)]
pub struct UnsharedVarHandler {
    // DT -> fid of handler
    // pub alloc_type_handlers: HashMap<DataType, u32>
}
impl UnsharedVarHandler {
    pub fn emit_alloc_func(
        &mut self,
        unshared_to_alloc: &mut [UnsharedVar],
        emitter: &mut ModuleEmitter,
        err: &mut ErrorGen,
    ) -> (Option<u32>, String) {
        // Called once per probe definition with `unshared` OR `report` vars.

        // $alloc description:
        // Will generate a function that allocates the memory required.
        // The order of the data will go in the order of the `unshared_to_alloc` param.
        // The function will return the memory offset to use by the probe logic.
        // Will also need to update some global value that keeps track of the already (use memory allocator?)
        // allocated memory space. Make sure to check that memory size is big enough!

        // TODO: `decl_init` statements should be run ONCE (can be in $alloc func)

        if unshared_to_alloc.is_empty() {
            (None, "".to_string())
        } else {
            // Generate the used_mem_checker function first
            emitter.mem_allocator.gen_mem_checker_fn(emitter.app_wasm);

            // Generate the strings necessary for the report variables
            for var in unshared_to_alloc.iter_mut() {
                if !var.is_report {
                    continue;
                }
                let Some(ReportMetadata::Local { name, probe_id, .. }) = &mut var.report_metadata
                else {
                    err.unexpected_error(
                        true,
                        Some("Report variable metadata should be set, but it's not".to_string()),
                        None,
                    );
                    unreachable!()
                };

                // handle variable name
                // handle probe_id
                emitter.mem_allocator.emit_string(emitter.app_wasm, name);
                emitter
                    .mem_allocator
                    .emit_string(emitter.app_wasm, probe_id);

                if matches!(var.ty, DataType::Str) {
                    // handle variables that are strings
                    todo!()
                }
                // (once they're emitted, the addresses will be available in MemoryAllocator::emitted_strings)
            }

            // specify params
            let fid = Local {
                id: LocalID(0),
                ty: OrcaType::I32,
            };
            let pc = Local {
                id: LocalID(1),
                ty: OrcaType::I32,
            };

            // params: (fid, pc)
            let alloc_params = vec![fid.ty, pc.ty];
            // results: mem_offset
            let alloc_results = vec![OrcaType::I32];

            let mut alloc = FunctionBuilder::new(&alloc_params, &alloc_results);
            // specify locals
            let orig_offset = Local {
                id: alloc.add_local(OrcaType::I32),
                ty: OrcaType::I32,
            };
            let curr_offset = Local {
                id: alloc.add_local(OrcaType::I32),
                ty: OrcaType::I32,
            };

            // remember the original memory offset
            alloc
                .global_get(emitter.mem_allocator.mem_tracker_global)
                .local_tee(orig_offset.id)
                .local_set(curr_offset.id);

            // track what's been allocated for this function thus far
            let mut curr_offset = 0;

            // Make sure memory is large enough for the memory that will be allocated!
            let num_bytes =
                self.calc_bytes_to_alloc(ReportVars::report_var_header_bytes(), unshared_to_alloc);
            emitter
                .mem_allocator
                .emit_memsize_check(num_bytes, &mut alloc, err);

            // alloc each var
            for UnsharedVar {
                ty,
                name,
                is_report,
                report_metadata,
            } in unshared_to_alloc.iter()
            {
                // println!("Allocating var of type: {ty}");
                let prev_offset = curr_offset;

                if *is_report {
                    // Emit the `report` var header (linked list)
                    curr_offset += emitter.report_vars.alloc_report_var_header(
                        ty,
                        emitter.mem_allocator.curr_mem_offset as u32,
                        curr_offset,
                        emitter.mem_allocator.mem_id,
                        emitter.mem_allocator.mem_tracker_global,
                        &mut alloc,
                        emitter.app_wasm,
                    );
                }

                // Store the header for the probe (this could be one per probe...but we're duplicating per variable
                // to make the flushing logic simpler)
                curr_offset += self.store_probe_header(&mut alloc, curr_offset, &fid, &pc, emitter);
                // Store the header for this variable
                curr_offset +=
                    self.store_var_header(&mut alloc, curr_offset, report_metadata, emitter, err);

                // allocate the space for the datatype value
                let var_addr = VarAddr::MemLoc {
                    mem_id: emitter.mem_allocator.mem_id,
                    ty: ty.clone(),
                    var_offset: curr_offset,
                };

                emitter.table.put(
                    name.clone(),
                    Record::Var {
                        ty: ty.clone(),
                        name: name.clone(),
                        value: None,
                        def: Definition::User,
                        is_report_var: *is_report,
                        addr: Some(var_addr),
                        loc: None,
                    },
                );

                curr_offset += ty.num_bytes().unwrap() as u32;

                if *is_report {
                    // now that we know the amount of memory we just used, update the next_addr ptr
                    // in the linked list
                    let var_mem_usage = curr_offset - prev_offset;
                    emitter.report_vars.update_next_addr_ptr(
                        ty,
                        var_mem_usage,
                        curr_offset,
                        emitter.mem_allocator.curr_mem_offset as u32,
                        emitter.mem_allocator.mem_id,
                        emitter.mem_allocator.mem_tracker_global,
                        &mut alloc,
                        emitter.app_wasm,
                    );
                }
            }

            // update the memory allocator global to account for all the added data
            emitter
                .mem_allocator
                .update_mem_tracker(curr_offset, &mut alloc);

            // return the base memory offset where this function's var block starts
            // return the location where the value will be stored in memory!
            alloc.local_get(orig_offset.id);

            let alloc_id = alloc.finish_module(emitter.app_wasm);
            emitter
                .app_wasm
                .exports
                .add_export_func(format!("${}", *alloc_id), *alloc_id);
            (Some(*alloc_id), "fid, pc".to_string())
        }
    }

    fn store_probe_header(
        &self,
        func: &mut FunctionBuilder,
        curr_offset: u32,
        fid: &Local,
        pc: &Local,
        emitter: &mut ModuleEmitter,
    ) -> u32 {
        // | fid | pc  |
        // | i32 | i32 |
        let mut bytes_used = 0;

        // store fid
        bytes_used += emitter.mem_allocator.emit_store_from_local(
            curr_offset + bytes_used,
            fid.id,
            &fid.ty,
            func,
        );

        // store pc
        bytes_used += emitter.mem_allocator.emit_store_from_local(
            curr_offset + bytes_used,
            pc.id,
            &pc.ty,
            func,
        );

        bytes_used
    }

    fn store_var_header(
        &self,
        func: &mut FunctionBuilder,
        curr_offset: u32,
        report_metadata: &Option<ReportMetadata>,
        emitter: &mut ModuleEmitter,
        err: &mut ErrorGen,
    ) -> u32 {
        // | name_ptr | name_len | script_id | probe_id_ptr | probe_id_len |
        // | i32      | u8       | u8        | i32          | u8           |
        let mut bytes_used = 0;
        let mem_tracker_global = emitter.mem_allocator.mem_tracker_global;

        let Some(ReportMetadata::Local {
            name,
            script_id,
            probe_id,
            ..
        }) = &report_metadata
        else {
            err.unexpected_error(
                true,
                Some("Report variable metadata should be set, but it's not".to_string()),
                None,
            );
            unreachable!()
        };

        let err_msg = Some(
            "Report variable metadata string should be emitted, but it's not been.".to_string(),
        );

        // store (name_ptr, name_len)
        let Some(StringAddr {
            mem_offset, len, ..
        }) = emitter.mem_allocator.emitted_strings.get(name)
        else {
            err.unexpected_error(true, err_msg, None);
            unreachable!()
        };
        let name_ptr = *mem_offset as u32;
        func.global_get(mem_tracker_global)
            .u32_const(name_ptr)
            .i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: emitter.mem_allocator.mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&name_ptr) as u32;

        if *len as u32 > u8::MAX as u32 {
            err.wizard_error(false, format!("Unable to encode report variable metadata for '{name}', string is too long, must be less than {} characters", u8::MAX), &None)
        }
        let name_len = *len as u8;
        func.global_get(mem_tracker_global)
            .u32_const(name_len as u32)
            .i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: emitter.mem_allocator.mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&name_len) as u32;

        // store script_id
        func.global_get(mem_tracker_global)
            .u32_const(*script_id as u32)
            .i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: emitter.mem_allocator.mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(script_id) as u32;

        // store (probe_id_ptr, probe_id_len)
        let Some(StringAddr {
            mem_offset, len, ..
        }) = emitter.mem_allocator.emitted_strings.get(probe_id)
        else {
            err.unexpected_error(true, err_msg, None);
            unreachable!()
        };
        let probe_id_ptr = *mem_offset as u32;
        func.global_get(mem_tracker_global)
            .u32_const(probe_id_ptr)
            .i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: emitter.mem_allocator.mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&probe_id_ptr) as u32;

        if *len as u32 > u8::MAX as u32 {
            err.wizard_error(false, format!("Unable to encode report variable metadata for '{name}', string is too long, must be less than {} characters", u8::MAX), &None)
        }
        let probe_id_len = *len as u8;
        func.global_get(mem_tracker_global)
            .u32_const(probe_id_len as u32)
            .i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: emitter.mem_allocator.mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&probe_id_len) as u32;

        bytes_used
    }

    fn calc_bytes_to_alloc(
        &self,
        report_header_bytes: u32,
        unshared_to_alloc: &mut [UnsharedVar],
    ) -> u32 {
        let mut num_bytes: u32 = 0;

        let i32_bytes = 4;
        let u8_bytes = 4;

        // to store the probe header
        // | fid | pc  |
        // | i32 | i32 |
        num_bytes += 2 * i32_bytes;

        unshared_to_alloc.iter().for_each(|var| {
            if var.is_report {
                // will need to account for the pointer to next report var probably...
                num_bytes += report_header_bytes;
            }
            // to store the header (per var)
            // | name_ptr | name_len | script_id | probe_id_ptr | probe_id_len |
            // | i32      | u8       | u8        | i32          | u8           |
            num_bytes += (2 * i32_bytes) + (3 * u8_bytes);

            // to store the value
            num_bytes += var.ty.num_bytes().unwrap() as u32;
        });

        num_bytes
    }
}

pub fn load_unshared_vars(
    _alloc_mem_offset: VarAddr,
    _allocated_vars: Vec<(String, DataType)>,
) -> HashMap<String, (VarAddr, DataType)> {
    // increment by bytes used by each variable's DataType as we load them
    // will be used to load the next variable AND to save in the VarAddr!
    let _used_bytes = 0;

    // alloc_mem_offset: parameter that specifies the result of calling $alloc (should be wasm local var)
    // alloc_vars: Vec<(var_name, var_ty)>, in the order they should appear in memory starting at
    //     the offset `alloc_mem_offset`
    // result: The new local variables: name -> (addr, ty)
    //     as a hashmap to enable caller to place in SymbolTable and handle report variables
    //     the new VarAddr will be a pointer into memory with an offset (real addr should be alloc_mem_offset + len_of_prev_vars)

    // At start of probe logic, pull the current values of the allocated variables from memory.
    //   Add these VarAddrs to the symbol table.
    //   Can now emit the rest of the probe body logic as normal.

    todo!()
}

pub fn save_allocated_vars(
    _alloc_mem_offset: VarAddr,
    _allocated_vars: HashMap<String, (VarAddr, DataType)>,
) {
    // At end of probe logic, save the values of allocated variables back into memory.

    todo!()
}

struct Local {
    id: LocalID,
    ty: OrcaType,
}

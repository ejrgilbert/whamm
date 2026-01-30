#![allow(clippy::too_many_arguments)]

use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::{StringAddr, VAR_BLOCK_BASE_VAR};
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::tag_handler::get_tag_for;
use crate::emitter::utils::{EmitCtx, emit_stmt};
use crate::generator::ast::{UnsharedVar, WhammParams};
use crate::lang_features::report_vars::Metadata as ReportMetadata;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::{DataType, Definition, Statement};
use crate::verifier::types::{Record, VarAddr};
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{GlobalID, LocalID};
use wirm::ir::types::{BlockType, DataType as WirmType, InitExpr, Value as WirmValue};
use wirm::module_builder::AddLocal;
use wirm::opcode::MacroOpcode;
use wirm::wasmparser::MemArg;
use wirm::{InitInstr, Module, Opcode};

pub struct UnsharedVarHandler {
    prev_fid: GlobalID,
    prev_fname_ptr: GlobalID,
}

impl UnsharedVarHandler {
    pub fn new(wasm: &mut Module) -> Self {
        let mut add_global_i32 = || -> GlobalID {
            wasm.add_global_with_tag(
                InitExpr::new(vec![InitInstr::Value(WirmValue::I32(-1))]),
                WirmType::I32,
                true,
                false,
                get_tag_for(&None),
            )
        };
        Self {
            prev_fid: add_global_i32(),
            prev_fname_ptr: add_global_i32(),
        }
    }
    pub fn emit_alloc_func(
        &self,
        unshared_to_alloc: &mut [UnsharedVar],
        init_args: &WhammParams,
        init_logic: &mut [Statement],
        emitter: &mut ModuleEmitter,
        err: &mut ErrorGen,
    ) -> (Option<u32>, String) {
        // | next_ptr | fname_ptr | fname_len | fid | pc  | name_ptr | name_len | script_id | probe_id_ptr | probe_id_len |
        // | i32      | i32       | i8        | i32 | i32 | i32      | u8       | u8        | i32          | u8           |

        // Called once per probe definition with `unshared` OR `report` vars.

        // $alloc description:
        // Will generate a function that allocates the memory required.
        // The order of the data will go in the order of the `unshared_to_alloc` param.
        // The function will return the memory offset to use by the probe logic.
        // Will also need to update some global value that keeps track of the already (use memory allocator?)
        // allocated memory space. Make sure to check that memory size is big enough!

        // TODO: `decl_init` statements should be run ONCE (can be in $alloc func)

        if unshared_to_alloc.is_empty() {
            return (None, "".to_string());
        }
        // Generate the used_mem_checker function first
        emitter.mem_allocator.gen_mem_checker_fns(emitter.app_wasm);

        // Generate the strings necessary for the report variables
        for var in unshared_to_alloc.iter_mut() {
            if !var.is_report {
                continue;
            }
            let Some(ReportMetadata::Local { name, probe_id, .. }) = &mut var.report_metadata
            else {
                unreachable!("Report variable metadata should be set, but it's not");
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
        let fname_ptr = Local {
            id: LocalID(0),
            ty: WirmType::I32,
        };
        let fname_len = Local {
            id: LocalID(1),
            ty: WirmType::I32,
        };
        let fid = Local {
            id: LocalID(2),
            ty: WirmType::I32,
        };
        let addr_fid = VarAddr::Local { addr: *fid.id };
        let pc = Local {
            id: LocalID(3),
            ty: WirmType::I32,
        };
        let addr_pc = VarAddr::Local { addr: *pc.id };

        // BASE params: (fname_ptr, fname_len, fid, pc)
        let mut param_str = "fname, fid, pc".to_string();
        let mut alloc_params = vec![fname_ptr.ty, fname_len.ty, fid.ty, pc.ty];
        // results: mem_offset
        let alloc_results = vec![WirmType::I32];

        // now extend params to account for any initialization logic that requires more data
        ModuleEmitter::handle_params(init_args, &mut alloc_params, &mut param_str, emitter.table);

        let mut alloc = FunctionBuilder::new(&alloc_params, &alloc_results);
        // specify locals
        let orig_offset = Local {
            id: alloc.add_local(WirmType::I32),
            ty: WirmType::I32,
        };
        let addr_offset = VarAddr::Local {
            addr: *orig_offset.id,
        };
        let curr_offset = Local {
            id: alloc.add_local(WirmType::I32),
            ty: WirmType::I32,
        };
        let new_fname_ptr = Local {
            id: alloc.add_local(WirmType::I32),
            ty: WirmType::I32,
        };

        // remember the original memory offset
        let alloc_var_mem_id = emitter
            .mem_allocator
            .alloc_var_mem_id
            .unwrap_or_else(|| panic!("alloc mem id not set"));
        let alloc_var_mem_tracker_global = emitter
            .mem_allocator
            .alloc_var_mem_tracker_global
            .unwrap_or_else(|| panic!("alloc mem tracker id not set"));
        alloc
            .global_get(alloc_var_mem_tracker_global)
            .local_tee(orig_offset.id)
            .local_set(curr_offset.id);

        // track what's been allocated for this function thus far
        let mut curr_offset = 0;

        // Make sure memory is large enough for the memory that will be allocated!
        let num_bytes =
            self.calc_bytes_to_alloc(ReportVars::report_var_header_bytes(), unshared_to_alloc);
        emitter
            .mem_allocator
            .emit_base_memsize_check(fname_len.id, &mut alloc);
        emitter
            .mem_allocator
            .emit_alloc_memsize_check(num_bytes, &mut alloc, err);

        // save fname
        if !unshared_to_alloc.is_empty() {
            self.store_fname(
                &mut alloc,
                &fname_ptr,
                &new_fname_ptr,
                &fname_len,
                &fid,
                emitter,
            );
        }

        // alloc each var
        for UnsharedVar {
            ty,
            name,
            is_report,
            report_metadata,
            ..
        } in unshared_to_alloc.iter()
        {
            let prev_offset = curr_offset;

            if *is_report {
                // Emit the `report` var header (linked list)
                curr_offset += emitter.report_vars.alloc_report_var_header(
                    ty,
                    curr_offset,
                    alloc_var_mem_id,
                    alloc_var_mem_tracker_global,
                    &mut alloc,
                    emitter.app_wasm,
                );

                // Store the header for the probe (this could be one per probe...but we're duplicating per variable
                // to make the flushing logic simpler)
                curr_offset += self.store_probe_header(
                    &mut alloc,
                    curr_offset,
                    &new_fname_ptr,
                    &fname_len,
                    &fid,
                    &pc,
                    emitter,
                );
                // Store the header for this variable
                curr_offset +=
                    self.store_var_header(&mut alloc, curr_offset, report_metadata, emitter, err);
            }

            // allocate the space for the datatype value
            let var_addr = VarAddr::MemLoc {
                mem_id: alloc_var_mem_id,
                ty: ty.clone(),
                var_offset: curr_offset,
            };

            // If the variable is of a type that must be initialized, do it here!
            self.init_var_object(&mut alloc, ty, &var_addr, emitter, err);

            emitter.table.put(
                name.clone(),
                Record::Var {
                    ty: ty.clone(),
                    value: None,
                    def: Definition::User,
                    addr: Some(vec![var_addr]),
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
                    alloc_var_mem_id,
                    alloc_var_mem_tracker_global,
                    &mut alloc,
                    emitter.app_wasm,
                );
            }
        }

        if !init_logic.is_empty() {
            // TODO -- this is making assumptions on the state that will be
            //  needed by the init logic, should pull this in the metadata-collector
            emitter.table.put(
                "fid".to_string(),
                Record::Var {
                    ty: DataType::I32,
                    value: None,
                    def: Definition::CompilerStatic,
                    addr: Some(vec![addr_fid]),
                    loc: None,
                },
            );
            emitter.table.put(
                "pc".to_string(),
                Record::Var {
                    ty: DataType::I32,
                    value: None,
                    def: Definition::CompilerStatic,
                    addr: Some(vec![addr_pc]),
                    loc: None,
                },
            );
            emitter.table.put(
                VAR_BLOCK_BASE_VAR.to_string(),
                Record::Var {
                    ty: DataType::I32,
                    value: None,
                    def: Definition::CompilerStatic,
                    addr: Some(vec![addr_offset]),
                    loc: None,
                },
            );
            for stmt in init_logic.iter_mut() {
                emit_stmt(
                    stmt,
                    emitter.strategy,
                    &mut alloc,
                    &mut EmitCtx::new(
                        emitter.registry,
                        emitter.table,
                        emitter.mem_allocator,
                        &mut emitter.locals_tracker,
                        emitter.map_lib_adapter,
                        "UnsharedVarHandler: Looks like you've found a bug...please report this behavior!",
                        err,
                    ),
                );
            }
        }

        // update the memory allocator global to account for all the added data
        emitter
            .mem_allocator
            .update_alloc_mem_tracker(curr_offset, &mut alloc);

        // return the base memory offset where this function's var block starts
        // return the location where the value will be stored in memory!
        alloc.local_get(orig_offset.id);

        let alloc_id = alloc.finish_module_with_tag(emitter.app_wasm, get_tag_for(&None));
        emitter.app_wasm.exports.add_export_func_with_tag(
            format!("${}", *alloc_id),
            *alloc_id,
            get_tag_for(&None),
        );
        (Some(*alloc_id), param_str)
    }

    fn init_var_object(
        &self,
        func: &mut FunctionBuilder,
        ty: &DataType,
        var_addr: &VarAddr,
        emitter: &mut ModuleEmitter,
        err: &mut ErrorGen,
    ) {
        if let DataType::Map { .. } = ty {
            let VarAddr::MemLoc {
                mem_id, var_offset, ..
            } = var_addr
            else {
                panic!(
                    "var_addr should be of type VarAddr::MemLoc, but was: {:?}",
                    var_addr
                )
            };
            let mem_tracker_global = emitter
                .mem_allocator
                .alloc_var_mem_tracker_global
                .unwrap_or_else(|| panic!("alloc memory tracker not set"));
            func.global_get(mem_tracker_global);
            emitter
                .map_lib_adapter
                .map_create_dynamic(ty.clone(), func, err);

            // the ID is now at ToS
            // save at allocated map ID memory space
            func.i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: *var_offset as u64,
                memory: *mem_id,
            });
        }
    }

    fn store_fname(
        &self,
        func: &mut FunctionBuilder,
        fname_ptr: &Local,
        new_fname_ptr: &Local,
        fname_len: &Local,
        fid: &Local,
        emitter: &mut ModuleEmitter,
    ) {
        let base_mem = emitter.mem_allocator.mem_id;
        let base_mem_tracker = emitter.mem_allocator.mem_tracker_global;
        let engine_mem = emitter
            .mem_allocator
            .engine_mem_id
            .unwrap_or_else(|| panic!("engine memory id not set"));

        // Check if we're still visiting the same function that we were in before.
        // If so, use the already-saved-off fname!
        #[rustfmt::skip]
        func.global_get(self.prev_fid)
            .local_get(fid.id)
            .i32_eq();
        func.if_stmt(BlockType::Empty)
            // if prev_fid == curr_fid:
            //      we're visiting the same function as before, reuse fname pointer from before
            .global_get(self.prev_fname_ptr)
            .local_set(new_fname_ptr.id)
            .else_stmt()
            // else:
            //      we're in a new function, use new fname!
            .global_get(base_mem_tracker)
            .local_set(new_fname_ptr.id);

        // save off the fname to the Strings memory
        emitter.mem_allocator.copy_mem(
            engine_mem,
            fname_ptr.id,
            fname_len.id,
            base_mem,
            base_mem_tracker,
            func,
        );

        // save to the fname tracker globals (for use in next $alloc call)
        func.local_get(fid.id)
            .global_set(self.prev_fid)
            .local_get(new_fname_ptr.id)
            .global_set(self.prev_fname_ptr);

        func.end();
    }

    fn store_probe_header(
        &self,
        func: &mut FunctionBuilder,
        curr_offset: u32,
        fname_ptr: &Local,
        fname_len: &Local,
        fid: &Local,
        pc: &Local,
        emitter: &mut ModuleEmitter,
    ) -> u32 {
        // | fname_ptr | fname_len | fid | pc  |
        // | i32       | i8        | i32 | i32 |
        let mut bytes_used = 0;

        let mem_id = emitter
            .mem_allocator
            .alloc_var_mem_id
            .unwrap_or_else(|| panic!("alloc memory id not set"));
        let mem_tracker_global = emitter
            .mem_allocator
            .alloc_var_mem_tracker_global
            .unwrap_or_else(|| panic!("alloc memory tracker not set"));

        // store fname_ptr
        bytes_used += emitter.mem_allocator.emit_store_from_local(
            curr_offset + bytes_used,
            fname_ptr.id,
            &fname_ptr.ty,
            mem_id,
            mem_tracker_global,
            func,
        );

        // store fname_len
        bytes_used += emitter.mem_allocator.emit_store8_from_local(
            curr_offset + bytes_used,
            fname_len.id,
            mem_id,
            mem_tracker_global,
            func,
        );

        // store fid
        bytes_used += emitter.mem_allocator.emit_store_from_local(
            curr_offset + bytes_used,
            fid.id,
            &fid.ty,
            mem_id,
            mem_tracker_global,
            func,
        );

        // store pc
        bytes_used += emitter.mem_allocator.emit_store_from_local(
            curr_offset + bytes_used,
            pc.id,
            &pc.ty,
            mem_id,
            mem_tracker_global,
            func,
        );

        assert_eq!(13, bytes_used);
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
        let mem_id = emitter
            .mem_allocator
            .alloc_var_mem_id
            .unwrap_or_else(|| panic!("alloc memory id not set"));
        let mem_tracker_global = emitter
            .mem_allocator
            .alloc_var_mem_tracker_global
            .unwrap_or_else(|| panic!("alloc memory tracker not set"));

        let Some(ReportMetadata::Local {
            name,
            script_id,
            probe_id,
            ..
        }) = &report_metadata
        else {
            unreachable!("Report variable metadata should be set, but it's not");
        };

        let err_msg =
            "Report variable metadata string should be emitted, but it's not been.".to_string();

        // store (name_ptr, name_len)
        let Some(StringAddr {
            mem_offset, len, ..
        }) = emitter.mem_allocator.emitted_strings.get(name)
        else {
            unreachable!("{}", err_msg);
        };
        let name_ptr = *mem_offset as u32;
        func.global_get(mem_tracker_global)
            .u32_const(name_ptr)
            .i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&name_ptr) as u32;

        if *len as u32 > u8::MAX as u32 {
            err.wei_error(format!("Unable to encode report variable metadata for '{name}', string is too long, must be less than {} characters", u8::MAX), &None)
        }
        let name_len = *len as u8;
        func.global_get(mem_tracker_global)
            .u32_const(name_len as u32)
            .i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&name_len) as u32;

        // store script_id
        func.global_get(mem_tracker_global)
            .u32_const(*script_id as u32)
            .i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(script_id) as u32;

        // store (probe_id_ptr, probe_id_len)
        let Some(StringAddr {
            mem_offset, len, ..
        }) = emitter.mem_allocator.emitted_strings.get(probe_id)
        else {
            unreachable!("{}", err_msg);
        };
        let probe_id_ptr = *mem_offset as u32;
        func.global_get(mem_tracker_global)
            .u32_const(probe_id_ptr)
            .i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&probe_id_ptr) as u32;

        if *len as u32 > u8::MAX as u32 {
            err.wei_error(format!("Unable to encode report variable metadata for '{name}', string is too long, must be less than {} characters", u8::MAX), &None)
        }
        let probe_id_len = *len as u8;
        func.global_get(mem_tracker_global)
            .u32_const(probe_id_len as u32)
            .i32_store8(MemArg {
                align: 0,
                max_align: 0,
                offset: (curr_offset + bytes_used) as u64,
                memory: mem_id, // instrumentation memory!
            });
        bytes_used += size_of_val(&probe_id_len) as u32;

        assert_eq!(11, bytes_used);
        bytes_used
    }

    fn calc_bytes_to_alloc(
        &self,
        report_header_bytes: u32,
        unshared_to_alloc: &mut [UnsharedVar],
    ) -> u32 {
        let mut num_bytes: u32 = 0;

        let i32_bytes = size_of::<i32>() as u32;
        let u8_bytes = size_of::<u8>() as u32;

        unshared_to_alloc.iter().for_each(|var| {
            if var.is_report {
                // to store the pointer to next report var
                num_bytes += report_header_bytes;

                // to store the header (per var)
                // | fname_ptr  | fname_len | fid | pc  | name_ptr | name_len | script_id | probe_id_ptr | probe_id_len |
                // | i32        | u8        | i32 | i32 | i32      | u8       | u8        | i32          | u8           |
                num_bytes += (5 * i32_bytes) + (4 * u8_bytes);
            }

            // to store the value
            num_bytes += var.ty.num_bytes().unwrap() as u32;
        });

        num_bytes
    }
}

struct Local {
    id: LocalID,
    ty: WirmType,
}

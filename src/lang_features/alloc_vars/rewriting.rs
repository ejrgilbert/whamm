#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;
use orca_wasm::{DataSegment, DataSegmentKind, Instructions, Module};
use orca_wasm::ir::id::MemoryID;
use orca_wasm::ir::types::{InitExpr, Value as OrcaValue};
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::{MemoryAllocator, StringAddr, WASM_PAGE_SIZE};
use crate::emitter::utils::whamm_type_to_wasm_global;
use crate::generator::ast::UnsharedVar;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::report_vars::{Metadata, NULL_PTR_IN_MEM, ReportVars};
use crate::parser::types::{DataType, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};


// TODO -- emit this stuff into a SEPARATE memory
//   this will make the curr_mem_offset work!
pub struct UnsharedVarHandler {
    allocated_vars: Vec<AllocatedVar>,
    report_trackers: HashMap<DataType, ReportAllocTracker>,
    curr_mem_offset: u32,
    mem_id: u32,
}
impl UnsharedVarHandler {
    pub fn new(mem_id: u32) -> Self {
        Self {
            allocated_vars: Vec::default(),
            report_trackers: HashMap::default(),
            curr_mem_offset: 0,
            mem_id
        }
    }

    pub(crate) fn memory_grow(&mut self, wasm: &mut Module) {
        // If we've allocated any memory, bump the app's memory up to account for that
        if let Some(mem) = wasm.memories.get_mut(MemoryID(self.mem_id)) {
            let req_pages = ((self.curr_mem_offset as u32 / WASM_PAGE_SIZE) + 1) as u64;
            if mem.ty.initial < req_pages {
                mem.ty.initial = req_pages;
            }
        }
    }
    pub fn get_curr_offset(&self) -> u32 {
        self.curr_mem_offset
    }
    pub fn get_mem_id(&self) -> u32 {
        self.mem_id
    }
    pub fn setup_module(&self, wasm: &mut Module) -> HashMap<DataType, u32> {
        self.setup_data(wasm);
        self.setup_globals(wasm)
    }

    /// The data goes into a new memory!!
    /// This is to enable a statically known value for the VAR_BLOCK_BASE_VAR at every match location.
    /// If we were to append to instrumentation memory, it'd mix with the emitted strings...which
    /// could mess up the linked list pointer max! This just protects from that scenario in a straightforward
    /// way (no need to do weird calculations or extend the bits of the linked list pointers).
    fn setup_data(&self, wasm: &mut Module) {
        // setup the data segment
        let mut bytes = vec![];

        // generate the data segment bytes
        for var in self.allocated_vars.iter() {
            bytes.extend(var.encode());
        }

        let data = DataSegment {
            data: bytes,
            kind: DataSegmentKind::Active {
                memory_index: self.mem_id,
                offset_expr: InitExpr::new(vec![Instructions::Value(OrcaValue::I32(0))]),
            },
        };
        wasm.add_data(data);
    }
    fn setup_globals(&self, wasm: &mut Module) -> HashMap<DataType, u32> {
        let mut global_trackers = HashMap::default();

        for (ty, ReportAllocTracker {first_var, ..}) in self.report_trackers.iter() {
            if let Some(first_var) = first_var {
                if let Some(AllocatedVar {mem_offset, ..}) = self.allocated_vars.get(*first_var as usize) {
                    let (global_id, _) = whamm_type_to_wasm_global(wasm, &DataType::I32, Some(InitExpr::new(vec![Instructions::Value(OrcaValue::I32(*mem_offset as i32))])));
                    global_trackers.insert(ty.clone(), *global_id);
                } else {
                    panic!("First var not found in allocated_vars list!");
                }
            } else {
                panic!("First var should be set by now!");
            }
        }

        global_trackers
    }
    pub fn allocate_vars(
        &mut self,
        vars: &[&UnsharedVar],
        fid: u32,
        pc: u32,
        // var_name: &str,
        // ty: &DataType,
        // is_report: bool,
        // report_metadata: &mut Option<Metadata>,
        // addr: &mut Option<VarAddr>,
        table: &mut SymbolTable,
        mem_allocator: &mut MemoryAllocator,
        map_lib_adapter: &mut MapLibAdapter,
        report_vars: &mut ReportVars,
        wasm: &mut Module,
        // err_msg: &str,
        err: &mut ErrorGen,
    ) {
        if vars.is_empty() {
            return;
        }

        // This function:
        //     Allocates the memory required for this variable and sets the `addr` to the memory offset where it's stored
        // The order of the data will go in the order of declarations in the probe body (for now).
        // Will also need to update some value that keeps track of the already
        // allocated memory space (use memory allocator?).
        //    -- should i have a separate memory for alloc vars? (what happens if i emit more strings for report vars?)
        //    -- can have my own instance of memory allocator
        //    -- should i track a HUGE data segment that just gradually is appended to?
        // After finished emitting a script: check that memory size is big enough!

        for var in vars.iter() {
            // TODO -- duplicate code in wizard::UnsharedVarHandler::emit_alloc_func
            // 0. Generate the strings necessary for the report variables
            if !var.is_report {
                continue;
            }
            report_vars.all_used_report_dts.insert(var.ty.clone());

            let Some(Metadata::Local { name, probe_id, .. }) = &var.report_metadata
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
            mem_allocator.emit_string(wasm, &mut name.clone());
            mem_allocator
                .emit_string(wasm, &mut probe_id.clone());

            if matches!(var.ty, DataType::Str) {
                // handle variables that are strings
                todo!()
            }

            // (once they're emitted, the addresses will be available in MemoryAllocator::emitted_strings)
        }

        // track what's been allocated for this function thus far
        let mut curr_offset = 0;

        // alloc each var
        for UnsharedVar {
            ty,
            name,
            is_report,
            report_metadata,
        } in vars.iter()
        {
            // if matches!(ty, DataType::Map {..}) {
            //     println!("map");
            // }
            let ty_tracker = self.report_trackers.entry(ty.clone()).or_insert(ReportAllocTracker::default());

            // look up in symbol table
            let Some(Record::Var { addr, .. }) = table.lookup_var_mut(name, true)
            else {
                panic!("unexpected type");
            };

            if *is_report {
                // 2. If is_report, prep the report var header (linked list)
                if let Some(prev_idx) = &mut ty_tracker.last_var {
                    if let Some(prev_var) = self.allocated_vars.get_mut(*prev_idx) {
                        let ptr = (self.curr_mem_offset + curr_offset) - prev_var.mem_offset;
                        prev_var.report_var_header = Some(ReportVarHeader {
                            next: ptr
                        });
                    } else {
                        panic!("Couldn't look up var")
                    }
                    // update to point to where THIS var will be in the list!
                    *prev_idx = self.allocated_vars.len();
                } else {
                    // update to point to where THIS var will be in the list!
                    ty_tracker.last_var = Some(self.allocated_vars.len())
                }

                if ty_tracker.first_var.is_none() {
                    // update to point to where THIS var will be in the list!
                    ty_tracker.first_var = Some(self.allocated_vars.len() as u32);
                }
            }

            // 3. Store the header for the probe (this could be one per probe...but we're duplicating per variable
            //    to make the flushing logic simpler)
            let probe_header = ProbeHeader::new(fid, pc);

            // 4. Store the header for this variable
            let var_header = VarHeader::new(report_metadata, mem_allocator, err);

            // TODO -- see if we need to init maps??

            let value = if matches!(ty, DataType::Map {..}) {
                let map_id = map_lib_adapter.emit_map_init(
                    name.clone(),
                    // addr,
                    ty,
                    *is_report,
                    false,
                    report_vars,
                    wasm,
                    err,
                );
                Some(Value::gen_i32(map_id as i32))
            } else {
                None
            };

            let allocated_var = AllocatedVar {
                mem_offset: self.curr_mem_offset + curr_offset,
                value,
                ty: ty.clone(),
                report_var_header: if *is_report {
                    Some(ReportVarHeader::null_ptr())
                } else {
                    None
                },
                probe_header,
                var_header
            };

            // var_addr points to the memory location of the value, skips the header!
            *addr = Some(VarAddr::MemLoc {
                mem_id: self.mem_id,
                ty: ty.clone(),
                var_offset: curr_offset + allocated_var.num_bytes_header() as u32,
            });

            curr_offset += allocated_var.num_bytes() as u32;

            self.allocated_vars.push(allocated_var);
        }

        self.curr_mem_offset += curr_offset;

        // 5. Allocate the space for the datatype value
        // 6. If the variable is of a type that must be initialized, do it here!
        // 7. now that we know the amount of memory we just used, update the next_addr ptr
        //    in the linked list
        // 8. update the memory allocator global to account for all the added data


        // match addr {
        //     Some(VarAddr::Global { .. }) | None => {
        //         if let Some(id) = self.use_available_gid(ty) {
        //             if is_report {
        //                 report_vars.put_local_metadata(id, var_name.to_string(), ty.clone(), err);
        //             }
        //
        //             *addr = Some(VarAddr::Global { addr: id });
        //             true
        //         } else {
        //             false
        //         }
        //     }
        //     Some(VarAddr::Local { .. })
        //     | Some(VarAddr::MapId { .. })
        //     | Some(VarAddr::MemLoc { .. }) => {
        //         //this shouldn't happen for unshared vars - need to err
        //         err.unexpected_error(
        //             true,
        //             Some(format!("{err_msg} Expected Global VarAddr.")),
        //             None,
        //         );
        //         false
        //     }
        // }
    }
}

struct AllocatedVar {
    mem_offset: u32,
    value: Option<Value>,
    ty: DataType,
    report_var_header: Option<ReportVarHeader>,
    probe_header: ProbeHeader,
    var_header: VarHeader
}
impl AllocatedVar {
    fn encode(&self) -> Vec<u8> {
        let mut res = vec![];

        if let Some(report_header) = &self.report_var_header {
            res.extend(report_header.encode())
        }
        res.extend(self.probe_header.encode());
        res.extend(self.var_header.encode());

        if let Some(value) = &self.value {
            res.extend(value.encode());
        } else {
            // just make space for the allocated variable's value
            if let Some(num_bytes) = self.ty.num_bytes() {
                for _ in 0..num_bytes {
                    res.push(0u8);
                }
            }
        }

        res
    }
    fn num_bytes(&self) -> usize {
        let mut used = 0;

        used += self.num_bytes_header();
        used += self.ty.num_bytes().unwrap();

        used
    }
    fn num_bytes_header(&self) -> usize {
        let mut used = 0;

        if self.report_var_header.is_some() {
            used += ReportVarHeader::num_bytes();
        }
        used += ProbeHeader::num_bytes();
        used += VarHeader::num_bytes();

        used
    }
}
#[derive(Default)]
struct ReportVarHeader {
    next: u32,
}
impl ReportVarHeader {
    fn null_ptr() -> Self {
        Self {
            next: NULL_PTR_IN_MEM as u32
        }
    }

    fn encode(&self) -> Box<[u8]> {
        Box::new(self.next.to_le_bytes())
    }
    fn num_bytes() -> usize {
        size_of::<i32>()
    }
}
#[derive(Default)]
struct ProbeHeader {
    fid: u32,
    pc: u32
}
impl ProbeHeader {
    fn new(fid: u32, pc: u32) -> Self {
        Self {
            fid,
            pc
        }
    }
    fn encode(&self) -> Vec<u8> {
        let mut res = self.fid.to_le_bytes().to_vec();
        res.extend(self.pc.to_le_bytes());

        res
    }
    fn num_bytes() -> usize {
        size_of::<i32>() * 2
    }
}
#[derive(Default)]
struct VarHeader {
    name_ptr: u32,
    name_len: u8,
    script_id: u8,
    probe_id_ptr: u32,
    probe_id_len: u8
}
impl VarHeader {
    fn encode(&self) -> Vec<u8> {
        let mut res = self.name_ptr.to_le_bytes().to_vec();
        res.extend(self.name_len.to_le_bytes());
        res.extend(self.script_id.to_le_bytes());
        res.extend(self.probe_id_ptr.to_le_bytes());
        res.extend(self.probe_id_len.to_le_bytes());

        res
    }
    fn num_bytes() -> usize {
        (size_of::<u32>() * 2) + (size_of::<u8>() * 3)
    }
    fn new(
        report_metadata: &Option<Metadata>,
        mem_allocator: &mut MemoryAllocator,
        err: &mut ErrorGen
    ) -> Self {
        let Some(Metadata::Local {
                    name,
                     script_id,
                     probe_id,
                     ..
                 }) = &report_metadata
        else {
            panic!("Report variable metadata should be set, but it's not");
        };

        let Some(StringAddr {
                     mem_offset: name_ptr, len: name_len, ..
                 }) = mem_allocator.emitted_strings.get(name)
        else {
            panic!("Report variable metadata string should be emitted, but it's not been.");
        };
        if *name_len as u32 > u8::MAX as u32 {
            err.wizard_error(false, format!("Unable to encode report variable metadata for '{name}', string is too long, must be less than {} characters", u8::MAX), &None)
        }
        let Some(StringAddr {
                     mem_offset: probe_id_ptr, len: probe_id_len, ..
                 }) = mem_allocator.emitted_strings.get(probe_id)
        else {
            panic!("Report variable metadata string should be emitted, but it's not been.");
        };
        if *probe_id_len as u32 > u8::MAX as u32 {
            err.wizard_error(false, format!("Unable to encode report variable metadata for '{name}', string is too long, must be less than {} characters", u8::MAX), &None)
        }
        Self {
            name_ptr: *name_ptr as u32,
            name_len: *name_len as u8,
            script_id: *script_id,
            probe_id_ptr: *probe_id_ptr as u32,
            probe_id_len: *probe_id_len as u8,
        }
    }
}

#[derive(Default)]
struct ReportAllocTracker {
    // global that points to the memory location of the first allocated report variable of this type
    first_var: Option<u32>,
    // global that points to the most-recently allocated report variable of this type
    last_var: Option<usize>,
}
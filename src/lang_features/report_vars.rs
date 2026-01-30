#![allow(clippy::too_many_arguments)]
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::tag_handler::get_tag_for;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use itertools::Itertools;
use log::info;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, GlobalID, LocalID};
use wirm::ir::types::{BlockType, DataType as WirmType, InitExpr, Value};
use wirm::module_builder::AddLocal;
use wirm::opcode::MacroOpcode;
use wirm::wasmparser::MemArg;
use wirm::{InitInstr, Module, Opcode};

pub const NULL_PTR_IN_MEM: i32 = -1;
pub const NULL_PTR_IN_GLOBAL: i32 = -1;

pub struct ReportVars {
    pub variable_metadata: HashMap<VarAddr, (WirmType, Metadata)>,
    pub all_metadata: HashSet<Metadata>,
    pub all_used_report_dts: HashSet<DataType>,
    pub curr_location: LocationData,

    // $alloc tracking for in-memory variables
    flush_tracker: FlushTracker,
    alloc_tracker: HashMap<DataType, ReportAllocTracker>,
}
impl Default for ReportVars {
    fn default() -> Self {
        Self::new()
    }
}
// ===========================
// ==== CODE FOR TRACKING ====
// ===========================
impl ReportVars {
    pub fn new() -> Self {
        ReportVars {
            variable_metadata: HashMap::new(),
            all_metadata: HashSet::new(),
            all_used_report_dts: HashSet::new(),
            curr_location: LocationData::Global { script_id: u8::MAX },
            alloc_tracker: HashMap::default(),
            flush_tracker: FlushTracker {
                flush_var_metadata_fid: None,
            },
        }
    }
    pub fn put_global_metadata(&mut self, gid: u32, name: String, whamm_ty: &DataType) -> bool {
        if !matches!(self.curr_location, LocationData::Global { .. }) {
            unreachable!(
                "Expected global location data, but got: {:?}",
                self.curr_location
            );
        }
        self.all_used_report_dts.insert(whamm_ty.clone());
        let metadata = Metadata::new(name.clone(), whamm_ty.clone(), &self.curr_location);
        self.variable_metadata.insert(
            VarAddr::Global { addr: gid },
            (metadata.get_wasm_ty(), metadata.clone()),
        );
        if !self.all_metadata.insert(metadata) {
            unreachable!("Duplicate metadata for map with name: {}", name);
        }
        true
    }
    pub fn put_map_metadata(&mut self, map_id: u32, name: String, ty: DataType) {
        self.all_used_report_dts.insert(ty.clone());
        let metadata = Metadata::new(name.clone(), ty, &self.curr_location);
        self.variable_metadata.insert(
            VarAddr::MapId { addr: map_id },
            (metadata.get_wasm_ty(), metadata.clone()),
        );
        if !self.all_metadata.insert(metadata) {
            unreachable!("Duplicate metadata for map with name: {}", name);
        };
    }
    pub fn print_metadata(&self) {
        if self.all_metadata.is_empty() {
            return;
        }
        let mut info = "Metadata:\n".to_string();

        // Collect and sort variable_metadata by key
        let mut sorted_variable_metadata: Vec<_> = self.variable_metadata.iter().collect();
        sorted_variable_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_variable_metadata {
            match key {
                VarAddr::Local { addr } => info += &format!("LocalID: {} -> {:?}", addr, value),
                VarAddr::Global { addr } => info += &format!("GlobalID: {} -> {:?}", addr, value),
                VarAddr::MapId { addr } => info += &format!("MapID: {} -> {:?}", addr, value),
                VarAddr::MemLoc {
                    mem_id, var_offset, ..
                } => info += &format!("MemAddr: ({}@{}) -> {:?}", mem_id, var_offset, value),
            }
        }

        info!("{info}");
    }
}

// ===========================
// ==== CODE FOR EMITTING ====
// ===========================
impl ReportVars {
    // =========================
    // ==== WEI-SPECIFIC ====
    // =========================

    // ==== Call flush functions at program exit ====
    pub fn setup_flush_data_segments(
        &mut self,
        wasm: &mut Module,
        memory_allocator: &mut MemoryAllocator,
    ) -> HashMap<VarAddr, (DataType, (u32, u32))> {
        // this needs to be a separate function to not have multiple
        // mutable references to the Wasm module at once.

        // TODO -- may be able to remove dupe code here!
        let id_type = "memaddr".to_string();

        memory_allocator.emit_string(wasm, &mut format!(", {id_type}, "));
        memory_allocator.emit_string(wasm, &mut ", ".to_string());
        memory_allocator.emit_string(wasm, &mut "script".to_string());

        for dt in self.all_used_report_dts.iter() {
            memory_allocator.emit_string(wasm, &mut format!("{dt}, "));
            let wasm_tys = dt.to_wasm_type();
            let wasm_ty_str = if wasm_tys.len() > 1 {
                // todo support tuples, strings, etc.
                unimplemented!()
            } else {
                get_wasm_ty_str(wasm_tys.first().unwrap())
            };
            memory_allocator.emit_string(wasm, &mut format!("{wasm_ty_str}, "));
        }

        // global report variable metadata
        self.get_var_metadata(wasm, memory_allocator)
    }
    pub fn get_var_metadata(
        &mut self,
        wasm: &mut Module,
        memory_allocator: &mut MemoryAllocator,
    ) -> HashMap<VarAddr, (DataType, (u32, u32))> {
        self.variable_metadata
            .iter()
            .map(|(key, (_, value))| {
                let mut s = format!("{key}, {}, ", value.to_csv(&key.ty()));
                memory_allocator.emit_string(wasm, &mut s);
                let addr = memory_allocator.emitted_strings.get(&s).unwrap();

                (
                    key.clone(),
                    (
                        value.get_whamm_ty(),
                        (addr.mem_offset as u32, addr.len as u32),
                    ),
                )
            })
            .collect()
    }
    pub fn emit_flush_logic(
        &mut self,
        func: &mut FunctionBuilder,
        var_meta: &HashMap<VarAddr, (DataType, (u32, u32))>,
        mem_allocator: &MemoryAllocator,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        header_info: (u32, u32),
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) {
        // There will be one function per report variable type used in the script.
        // So, if the script only uses i32 report variables, there will be one function
        // called to flush the final state.
        //
        // These functions will contain the logic to handle its respective datatype.
        // For i32, for map, etc.
        //
        // There will be 2 global variables used per data type as well:
        // - one global that points to the memory location of the first allocated report
        //   variable of this type
        // - one global that points to the most-recently allocated report variable of
        //   this type
        // The global variables are necessary since they will be used both during variable
        // allocation and during variable flush.

        // The ReportVars struct needs to have fields to keep track of the emitted globals per DT
        // (to be used in the $alloc methods)!

        // print the in-memory variables
        // TODO -- can I combine this with the other logic instead?
        self.emit_locals_flush(
            func,
            mem_allocator,
            io_adapter,
            map_lib_adapter,
            header_info,
            mem_id,
            wasm,
            err,
        );

        // print the global variables
        self.emit_globals_flush(func, var_meta, io_adapter, map_lib_adapter, err);
    }

    pub fn configure_trackers(&mut self, trackers: HashMap<DataType, u32>) {
        for (ty, id) in trackers.iter() {
            self.alloc_tracker
                .entry(ty.clone())
                .and_modify(|tracker| {
                    tracker.first_var = Some(*id);
                })
                .or_insert(ReportAllocTracker {
                    first_var: Some(*id),
                    last_var: None,
                });
        }
    }

    pub fn emit_locals_flush(
        &mut self,
        func: &mut FunctionBuilder,
        mem_allocator: &MemoryAllocator,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        header_info: (u32, u32),
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) {
        io_adapter.putsln(header_info.0, header_info.1, func, err);
        self.call_dt_flushers(
            func,
            mem_allocator,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        );
    }

    pub fn emit_globals_flush(
        &self,
        func: &mut FunctionBuilder,
        var_meta_str: &HashMap<VarAddr, (DataType, (u32, u32))>,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        // for each of the report globals, emit the printing logic
        let sorted_metadata = var_meta_str.iter().sorted_by_key(|data| data.0);
        for (addr, (whamm_ty, (str_addr, str_len))) in sorted_metadata.into_iter() {
            io_adapter.puts(*str_addr, *str_len, func, err);

            match addr {
                VarAddr::Local { .. } => panic!("Shouldn't be trying to flush a local variable..."),
                VarAddr::MemLoc { .. } => {
                    panic!("Shouldn't be trying to flush a memaddr in this function...")
                }
                VarAddr::Global { addr } => {
                    // get the value of this report global
                    func.global_get(GlobalID(*addr));
                    match whamm_ty {
                        DataType::U8 => io_adapter.call_putu8(func, err),
                        DataType::I8 => io_adapter.call_puti8(func, err),
                        DataType::U16 => io_adapter.call_putu16(func, err),
                        DataType::I16 => io_adapter.call_puti16(func, err),
                        DataType::I32 => io_adapter.call_puti32(func, err),
                        // special case for unsigned integers (so the print is correctly signed)
                        DataType::U32 => io_adapter.call_putu32(func, err),
                        DataType::I64 => io_adapter.call_puti64(func, err),
                        DataType::U64 => io_adapter.call_putu64(func, err),
                        DataType::F32 => io_adapter.call_putf32(func, err),
                        DataType::F64 => io_adapter.call_putf64(func, err),
                        DataType::Boolean => io_adapter.call_putbool(func, err),
                        other => unimplemented!(
                            "printing for this type has not been implemented: {}",
                            other
                        ),
                    }
                }
                VarAddr::MapId { addr } => {
                    // print the value(s) of this map
                    map_lib_adapter.print_map(*addr, func, err);
                }
            }
            io_adapter.putln(func, err);
        }
    }

    pub fn call_dt_flushers(
        &mut self,
        func: &mut FunctionBuilder,
        mem_allocator: &MemoryAllocator,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) {
        // Make sure the metadata flusher func exists
        self.emit_flush_var_metadata_fn(mem_allocator, io_adapter, mem_id, wasm, err);

        // iterate through all the data type flush functions and emit calls
        // sort these by the datatype to make the flush deterministic!
        let sorted_keys = self.alloc_tracker.keys().sorted_by_key(|ty| ty.id());
        for dt in sorted_keys.into_iter() {
            match dt {
                DataType::U8 => {
                    let fid = self.emit_flush_u8_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I8 => {
                    let fid = self.emit_flush_i8_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::U16 => {
                    let fid =
                        self.emit_flush_u16_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I16 => {
                    let fid =
                        self.emit_flush_i16_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::U32 => {
                    let fid =
                        self.emit_flush_u32_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I32 => {
                    let fid =
                        self.emit_flush_i32_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::U64 => {
                    let fid =
                        self.emit_flush_u64_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I64 => {
                    let fid =
                        self.emit_flush_i64_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::F32 => {
                    let fid =
                        self.emit_flush_f32_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::F64 => {
                    let fid =
                        self.emit_flush_f64_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::Boolean => {
                    let fid =
                        self.emit_flush_bool_fn(io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::Map { .. } => {
                    let fid =
                        self.emit_flush_map_fn(dt, io_adapter, map_lib_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                dt => {
                    unimplemented!("Flushing {dt} is not supported yet.")
                }
            }
        }
    }
    fn emit_flush_var_metadata_fn(
        &mut self,
        mem_allocator: &MemoryAllocator,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) {
        // ======================== REPORT CSV FLUSH ============================
        // type, id_type, id, name, script_id, fname, fid, pc, probe_id, value(s)
        let id_type = "memaddr".to_string();

        let i32_bytes = size_of::<i32>() as i32;
        let u8_bytes = size_of::<u8>() as i32;

        // handles all but 'value(s)' since this is common between all variable types
        let dt = LocalID(0); // use to figure out which 'type' to print
        let orig_addr = LocalID(1);
        let mut flush_fn = FunctionBuilder::new(&[WirmType::I64, WirmType::I32], &[WirmType::I32]);

        let fname_ptr = flush_fn.add_local(WirmType::I32); // u32
        let fname_len = flush_fn.add_local(WirmType::I32); // u8

        let fid = flush_fn.add_local(WirmType::I32); // u32
        let pc = flush_fn.add_local(WirmType::I32); // u32

        let name_ptr = flush_fn.add_local(WirmType::I32); // u32
        let name_len = flush_fn.add_local(WirmType::I32); // u8

        let script_id = flush_fn.add_local(WirmType::I32); // u8

        let probe_id_ptr = flush_fn.add_local(WirmType::I32); // u32
        let probe_id_len = flush_fn.add_local(WirmType::I32); // u8

        let mut curr_offset = 0;
        let mut memarg = |num_bytes: i32| -> MemArg {
            let arg = MemArg {
                align: 0,
                max_align: 0,
                offset: curr_offset,
                memory: mem_id,
            };
            curr_offset += num_bytes as u64;
            arg
        };

        // header format:
        // | fname_ptr  | fname_len | fid | pc  | name_ptr | name_len | script_id | probe_id_ptr | probe_id_len |
        // | i32        | u8        | i32 | i32 | i32      | u8       | u8        | i32          | u8           |

        // load fname_addr (i32)
        flush_fn
            .local_get(orig_addr)
            .i32_load(memarg(i32_bytes))
            .local_set(fname_ptr);

        // load fname_len (u8)
        flush_fn
            .local_get(orig_addr)
            .i32_load8_u(memarg(u8_bytes))
            .local_set(fname_len);

        // load fid (i32)
        flush_fn
            .local_get(orig_addr)
            .i32_load(memarg(i32_bytes))
            .local_set(fid);

        // load pc (i32)
        flush_fn
            .local_get(orig_addr)
            .i32_load(memarg(i32_bytes))
            .local_set(pc);

        // load name_ptr (i32)
        flush_fn
            .local_get(orig_addr)
            .i32_load(memarg(i32_bytes))
            .local_set(name_ptr);

        // load name_len (u8)
        flush_fn
            .local_get(orig_addr)
            .i32_load8_u(memarg(u8_bytes))
            .local_set(name_len);

        // load script_id (u8)
        flush_fn
            .local_get(orig_addr)
            .i32_load8_u(memarg(u8_bytes))
            .local_set(script_id);

        // load probe_id_ptr (i32)
        flush_fn
            .local_get(orig_addr)
            .i32_load(memarg(i32_bytes))
            .local_set(probe_id_ptr);

        // load probe_id_len (u8)
        flush_fn
            .local_get(orig_addr)
            .i32_load8_u(memarg(u8_bytes))
            .local_set(probe_id_len);

        // print 'id'
        flush_fn.local_get(orig_addr);
        io_adapter.call_puti32(&mut flush_fn, err);

        // print 'id_type'
        let (addr, len) = mem_allocator.lookup_emitted_string(&format!(", {id_type}, "));
        io_adapter.puts(addr, len, &mut flush_fn, err);

        // print 'name'
        flush_fn.local_get(name_ptr).local_get(name_len);
        io_adapter.call_puts_internal(&mut flush_fn, err);
        let (comma_addr, comma_len) = mem_allocator.lookup_emitted_string(", ");
        io_adapter.puts(comma_addr, comma_len, &mut flush_fn, err);

        // print 'whamm_type' per supported report variable datatype
        assert!(!self.all_used_report_dts.is_empty());
        let mut first = true;
        for ty in self.all_used_report_dts.iter() {
            if !first {
                flush_fn.else_stmt();
            }
            Self::flush_ty_metadata(&mut flush_fn, ty, &dt, mem_allocator, io_adapter, err);
            first = false;
        }
        // All other datatypes should dynamically trap
        flush_fn.else_stmt().unreachable();
        // (need to close all if stmts with 'end')
        for _ in 0..self.all_used_report_dts.len() {
            flush_fn.end();
        }

        // print 'script_id'
        let (addr, len) = mem_allocator.lookup_emitted_string("script");
        io_adapter.puts(addr, len, &mut flush_fn, err);
        flush_fn.local_get(script_id);
        io_adapter.call_puti32(&mut flush_fn, err);
        io_adapter.puts(comma_addr, comma_len, &mut flush_fn, err);

        // print '"fname"'
        flush_fn.local_get(fname_ptr).local_get(fname_len);
        io_adapter.call_puts_internal(&mut flush_fn, err);
        io_adapter.puts(comma_addr, comma_len, &mut flush_fn, err);

        // print 'fid, pc'
        flush_fn.local_get(fid);
        io_adapter.call_puti32(&mut flush_fn, err);
        io_adapter.puts(comma_addr, comma_len, &mut flush_fn, err);
        flush_fn.local_get(pc);
        io_adapter.call_puti32(&mut flush_fn, err);
        io_adapter.puts(comma_addr, comma_len, &mut flush_fn, err);

        // print 'probe_id'
        flush_fn.local_get(probe_id_ptr).local_get(probe_id_len);
        io_adapter.call_puts_internal(&mut flush_fn, err);
        io_adapter.puts(comma_addr, comma_len, &mut flush_fn, err);

        // return the pointer to the next place in memory (should point to value(s))
        flush_fn
            .local_get(orig_addr)
            .i32_const(curr_offset as i32)
            .i32_add();

        let flush_fid = flush_fn.finish_module_with_tag(wasm, get_tag_for(&None));
        wasm.set_fn_name(flush_fid, "flush_var_metadata".to_string());
        self.flush_tracker.flush_var_metadata_fid = Some(*flush_fid);
    }
    fn flush_ty_metadata<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        flush_fn: &mut T,
        dt: &DataType,
        dt_local: &LocalID,
        mem_allocator: &MemoryAllocator,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        let mut s = DefaultHasher::new();
        dt.hash(&mut s);
        let hash = s.finish();

        flush_fn
            .local_get(*dt_local)
            .i64_const(hash as i64)
            .i64_eq()
            .if_stmt(BlockType::Empty);
        let (addr, len) = mem_allocator.lookup_emitted_string(&format!("{dt}, "));
        io_adapter.puts(addr, len, flush_fn, err);
        let wasm_tys = dt.to_wasm_type();
        let wasm_ty = if wasm_tys.len() > 1 {
            // todo support tuples, strings, etc.
            unimplemented!()
        } else {
            get_wasm_ty_str(wasm_tys.first().unwrap())
        };
        let (addr, len) = mem_allocator.lookup_emitted_string(&format!("{wasm_ty}, "));
        io_adapter.puts(addr, len, flush_fn, err);
    }

    // ==== Flush functions per datatype ====
    fn emit_flush_fn(
        &self,
        flush_dt: &dyn Fn(
            &mut FunctionBuilder,
            &MemArg,
            &mut IOAdapter,
            &mut MapLibAdapter,
            &mut ErrorGen,
        ),
        dt: DataType,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        let Some(flush_metadata_fid) = self.flush_tracker.flush_var_metadata_fid else {
            unreachable!(
                "Should have the flush variable metadata function ID, but it's not been generated yet."
            );
        };
        let mem_arg = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: mem_id,
        };

        // ================================= REPORT CSV FLUSH ====================================
        // id, id_type, name, whamm_type, wasm_type, script_id, fname, fid, pc, probe_id, value(s)

        // handles the 'value(s)' output
        let mut flush_fn = FunctionBuilder::new(&[], &[]);

        let curr_addr = flush_fn.add_local(WirmType::I32);
        let next_addr = flush_fn.add_local(WirmType::I32);

        // check that we actually have some of this DT to flush
        // if not: return
        flush_fn
            .block(BlockType::Empty)
            .global_get(GlobalID(
                self.alloc_tracker.get(&dt).unwrap().first_var.unwrap(),
            ))
            .i32_const(NULL_PTR_IN_GLOBAL)
            .i32_eq()
            .br_if(0);

        flush_fn
            .global_get(GlobalID(
                self.alloc_tracker.get(&dt).unwrap().first_var.unwrap(),
            ))
            .local_set(curr_addr);

        flush_fn.loop_stmt(BlockType::Empty);

        #[rustfmt::skip]
        // save the next_addr
        flush_fn
            .local_get(curr_addr)
            .i32_load(mem_arg)
            .local_tee(next_addr)

            // If the next_addr is a NULL_PTR, leave it so...
            // otherwise calculate the actual next address:
            //    next_addr = curr_addr + next_addr_offset
            .i32_const(NULL_PTR_IN_MEM)
            .i32_ne()
            .if_stmt(BlockType::Empty)
                // check if there's a bug that would cause an infinite loop
                .local_get(next_addr)
                .i32_eqz()
                .if_stmt(BlockType::Empty)
                    .unreachable()
                .end()
                .local_get(curr_addr)
                .local_get(next_addr)
                .i32_add()
                .local_set(next_addr)
            .end();

        // update memory pointer
        flush_fn
            .i32_const(4)
            .local_get(curr_addr)
            .i32_add()
            .local_set(curr_addr);

        // use return of flush_metadata_func to know where value(s) starts!
        let mut s = DefaultHasher::new();
        dt.hash(&mut s);
        let dt_hash = s.finish();
        flush_fn
            .i64_const(dt_hash as i64)
            .local_get(curr_addr)
            .call(FunctionID(flush_metadata_fid))
            .local_tee(curr_addr);

        // print the value(s), uses returned curr_addr that is now pointing to
        // the true location of the value in memory
        flush_dt(&mut flush_fn, &mem_arg, io_adapter, map_lib_adapter, err);

        // check if we should loop
        // while next_addr != NULL_PTR: curr_addr = next_addr; continue;
        #[rustfmt::skip]
        flush_fn
            .local_get(next_addr)
            .i32_const(NULL_PTR_IN_MEM)
            .i32_ne()
            .if_stmt(BlockType::Empty)
                .local_get(next_addr)
                .local_set(curr_addr)
                .br(1)
            .end();
        // otherwise, fall through to end.

        flush_fn.end().end();

        let flush_fid = flush_fn.finish_module_with_tag(wasm, get_tag_for(&None));
        wasm.set_fn_name(flush_fid, format!("flush_{}_vars", dt));

        *flush_fid
    }

    fn emit_flush_u8_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_u8,
            DataType::U8,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_u8(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load8_u(*mem_arg);
        io_adapter.call_putu8(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_i8_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i8,
            DataType::I8,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_i8(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load8_s(*mem_arg);
        io_adapter.call_puti8(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_u16_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_u16,
            DataType::U16,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_u16(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load16_u(*mem_arg);
        io_adapter.call_putu16(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_i16_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i16,
            DataType::I16,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_i16(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load16_s(*mem_arg);
        io_adapter.call_puti16(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_u32_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_u32,
            DataType::U32,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_u32(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load(*mem_arg);
        io_adapter.call_putu32(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_i32_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i32,
            DataType::I32,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_i32(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load(*mem_arg);
        io_adapter.call_puti32(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_u64_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_u64,
            DataType::U64,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_u64(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i64_load(*mem_arg);
        io_adapter.call_putu64(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_i64_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i64,
            DataType::I64,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_i64(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i64_load(*mem_arg);
        io_adapter.call_puti64(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_f32_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_f32,
            DataType::F32,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_f32(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.f32_load(*mem_arg);
        io_adapter.call_putf32(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_f64_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_f64,
            DataType::F64,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_f64(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.f64_load(*mem_arg);
        io_adapter.call_putf64(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_bool_fn(
        &self,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_bool,
            DataType::Boolean,
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_bool(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        _map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load(*mem_arg);
        io_adapter.call_putbool(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_map_fn(
        &self,
        dt: &DataType,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_map,
            dt.clone(),
            io_adapter,
            map_lib_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_map(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        map_lib_adapter: &mut MapLibAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load(*mem_arg);
        map_lib_adapter.call_print_map(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    pub fn report_var_header_bytes() -> u32 {
        // | next_addr |
        // | i32       |
        size_of::<i32>() as u32
    }
    pub fn alloc_report_var_header(
        &mut self,
        data_type: &DataType,
        var_offset: u32,
        mem_id: u32,
        mem_tracker_global: GlobalID,
        alloc_func: &mut FunctionBuilder,
        wasm: &mut Module,
    ) -> u32 {
        // We will have a linked list in memory of the report variables. One linked list per type.
        //
        // | next            | var_header    | value                  |
        // | mem_offset: i32 | lots-of-stuff | var_data: datatype_len |
        //
        // Each variable location will have two pieces of information:
        // - mem_offset: i32: The memory offset to the next report variable of this type,
        //   it is the offset from the current memory location.
        // - var_data: datatype_len: The actual value of the variable, the type/len is dependent
        //   on the type of the variable. Since a function is called that iterates over each datatype's linked list of report variables, it will know how to parse the variable contents!
        let mut used_bytes = 0;

        let tracker = self
            .alloc_tracker
            .entry(data_type.clone())
            .or_insert(ReportAllocTracker {
                // data_type: data_type.clone(),
                // flush_func: None,
                first_var: None,
                last_var: None,
            });

        let first_var = if let Some(first_var) = tracker.first_var {
            GlobalID(first_var)
        } else {
            // On the first allocation for a datatype, the global that points to the first memory
            // location is updated to point to the memory address.
            let gid = wasm.add_global_with_tag(
                InitExpr::new(vec![InitInstr::Value(Value::I32(NULL_PTR_IN_GLOBAL))]),
                WirmType::I32,
                true,
                false,
                get_tag_for(&None),
            );
            tracker.first_var = Some(*gid);

            GlobalID(*gid)
        };

        // if the first_var isn't pointing to anything yet, point it here!
        #[rustfmt::skip]
        alloc_func.global_get(first_var)
            .i32_const(NULL_PTR_IN_GLOBAL)
            .i32_eq()
            .if_stmt(BlockType::Empty)
                .global_get(mem_tracker_global)
                .u32_const(var_offset)
                .i32_add()
                .global_set(first_var)
            .end();

        // put header in memory at curr_addr, value is: NULL_PTR
        alloc_func.global_get(mem_tracker_global); // (where to store)
        alloc_func.i32_const(NULL_PTR_IN_GLOBAL); // (what to store)
        alloc_func.i32_store(MemArg {
            align: 0,
            max_align: 0,
            offset: var_offset as u64,
            memory: mem_id,
        });
        used_bytes += size_of_val(&NULL_PTR_IN_GLOBAL);

        assert_eq!(4, used_bytes);
        used_bytes as u32
    }

    pub fn update_next_addr_ptr(
        &mut self,
        data_type: &DataType,
        curr_var_mem_usage: u32,
        total_mem_usage: u32,
        mem_id: u32,
        mem_tracker_global: GlobalID,
        alloc_func: &mut FunctionBuilder,
        wasm: &mut Module,
    ) {
        let tracker = self
            .alloc_tracker
            .entry(data_type.clone())
            .or_insert(ReportAllocTracker {
                first_var: None,
                last_var: None,
            });

        let last_var = if let Some(last_var) = tracker.last_var {
            GlobalID(last_var)
        } else {
            let gid = wasm.add_global_with_tag(
                InitExpr::new(vec![InitInstr::Value(Value::I32(NULL_PTR_IN_GLOBAL))]),
                WirmType::I32,
                true,
                false,
                get_tag_for(&None),
            );
            tracker.last_var = Some(*gid);
            GlobalID(*gid)
        };

        // TODO -- ONLY RUN THIS IF THERE ARE MULTIPLE REPORT VARS IN THE SCRIPT

        // When a new variable is allocated in memory, the global containing the memory address
        // of the most-recently allocated variable of that type is used to update the next pointer
        // to the difference between the previous and the current memory address (to find the offset).
        // Then, that global is updated to the current memory address.

        // check that last_var is NOT a NULLPTR
        #[rustfmt::skip]
        alloc_func.global_get(last_var)
            .i32_const(NULL_PTR_IN_GLOBAL)
            .i32_eq()
            .i32_eqz()
            .if_stmt(BlockType::Empty)
                // put header at last_var_addr, value is: (curr_addr - last_var_addr)
                // (where to store)
                .global_get(last_var)
                // (what to store)
                // (mem_tracker_global - last_var_gid) + (total_mem_used_for_this_probe - usage_for_this_var)
                .global_get(mem_tracker_global)
                .global_get(last_var)
                .i32_sub()
                .u32_const(total_mem_usage - curr_var_mem_usage)
                .i32_add()
                .i32_store(MemArg {
                    align: 0,
                    max_align: 0,
                    // offset: used_bytes as u64,
                    offset: 0,
                    memory: mem_id,
                })
            .end();

        // point `last_var` here!
        alloc_func
            .global_get(mem_tracker_global)
            .u32_const(total_mem_usage - curr_var_mem_usage)
            .i32_add()
            .global_set(last_var);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Metadata {
    Global {
        name: String,
        whamm_ty: DataType,
        wasm_ty: WirmType,
        script_id: u8,
    },
    Local {
        name: String,
        whamm_ty: DataType,
        wasm_ty: WirmType,
        script_id: u8,
        bytecode_loc: BytecodeLoc,
        probe_id: String,
    },
}
impl From<&LocationData> for Metadata {
    fn from(loc: &LocationData) -> Self {
        match loc {
            LocationData::Local {
                script_id,
                bytecode_loc,
                probe_id,
                ..
            } => Self::Local {
                name: "".to_string(),
                whamm_ty: DataType::I32,
                wasm_ty: WirmType::I32,
                script_id: *script_id,
                bytecode_loc: bytecode_loc.clone(),
                probe_id: probe_id.clone(),
            },
            LocationData::Global { script_id } => Self::Global {
                name: "".to_string(),
                whamm_ty: DataType::I32,
                wasm_ty: WirmType::I32,
                script_id: *script_id,
            },
        }
    }
}
impl Metadata {
    pub fn new(name: String, whamm_ty: DataType, loc: &LocationData) -> Self {
        let mut meta = Self::from(loc);
        meta.set_name(name);
        let wasm_ty = whamm_ty.to_wasm_type();
        if wasm_ty.len() > 1 {
            unimplemented!()
        } else {
            meta.set_wasm_ty(*wasm_ty.first().unwrap());
        }
        meta.set_whamm_ty(whamm_ty);
        meta
    }
    pub fn set_name(&mut self, new_name: String) {
        match self {
            Self::Local { name, .. } | Self::Global { name, .. } => *name = new_name,
        }
    }
    pub fn set_whamm_ty(&mut self, new_ty: DataType) {
        match self {
            Self::Local { whamm_ty, .. } | Self::Global { whamm_ty, .. } => *whamm_ty = new_ty,
        }
    }
    pub fn get_whamm_ty(&self) -> DataType {
        match self {
            Self::Local { whamm_ty, .. } | Self::Global { whamm_ty, .. } => whamm_ty.clone(),
        }
    }
    pub fn set_wasm_ty(&mut self, new_ty: WirmType) {
        match self {
            Self::Local { wasm_ty, .. } | Self::Global { wasm_ty, .. } => *wasm_ty = new_ty,
        }
    }
    pub fn get_wasm_ty(&self) -> WirmType {
        match self {
            Self::Local { wasm_ty, .. } | Self::Global { wasm_ty, .. } => *wasm_ty,
        }
    }
    pub fn setup_csv_header(wasm: &mut Module, mem_allocator: &mut MemoryAllocator) -> (u32, u32) {
        let mut header = "\n================================= REPORT CSV FLUSH ====================================\nid, id_type, name, whamm_type, wasm_type, script_id, fname, fid, pc, probe_id, value(s)"
            .to_string();
        mem_allocator.emit_string(wasm, &mut header);
        let addr = mem_allocator.emitted_strings.get(&header).unwrap();

        (addr.mem_offset as u32, addr.len as u32)
    }
    pub fn to_csv(&self, id_ty: &str) -> String {
        let (name, whamm_ty, wasm_ty, script_id, bytecode_loc, probe_id) = match self {
            Metadata::Global {
                name,
                whamm_ty,
                wasm_ty,
                script_id,
            } => (
                name.as_str(),
                whamm_ty.to_string(),
                get_wasm_ty_str(wasm_ty),
                *script_id,
                // skip: fname, pc, fid
                ", , ",
                "",
            ),
            Metadata::Local {
                name,
                whamm_ty,
                wasm_ty,
                script_id,
                bytecode_loc,
                probe_id,
            } => (
                name.as_str(),
                whamm_ty.to_string(),
                get_wasm_ty_str(wasm_ty),
                *script_id,
                &*bytecode_loc.to_string(),
                probe_id.as_str(),
            ),
        };
        format!(
            "{id_ty}, {name}, {whamm_ty}, {wasm_ty}, script{script_id}, {}, {probe_id}",
            bytecode_loc
        )
    }
}
#[derive(Clone, Debug)]
pub enum LocationData {
    Global {
        script_id: u8,
    },
    Local {
        script_id: u8,
        bytecode_loc: BytecodeLoc,
        probe_id: String,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BytecodeLoc {
    pub fid: u32,
    pc: u32,
}
impl Display for BytecodeLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.fid, self.pc)
    }
}
impl BytecodeLoc {
    pub(crate) fn new(fid: u32, pc: u32) -> Self {
        Self { fid, pc }
    }
}

struct ReportAllocTracker {
    // global that points to the memory location of the first allocated report variable of this type
    first_var: Option<u32>,
    // global that points to the most-recently allocated report variable of this type
    last_var: Option<u32>,
}
struct FlushTracker {
    flush_var_metadata_fid: Option<u32>,
}

fn get_wasm_ty_str(wasm_ty: &WirmType) -> String {
    let s = match wasm_ty {
        WirmType::I8 => "i8",
        WirmType::I16 => "i16",
        WirmType::I32 => "i32",
        WirmType::I64 => "i64",
        WirmType::F32 => "f32",
        WirmType::F64 => "f64",
        WirmType::V128 => "v128",
        WirmType::FuncRef => "funcref",
        WirmType::FuncRefNull => "funcref_null",
        WirmType::ExternRef => "externref",
        WirmType::ExternRefNull => "externref_null",
        WirmType::Any => "any",
        WirmType::AnyNull => "any_null",
        WirmType::None => "none",
        WirmType::NoneNull => "none_null",
        WirmType::NoExtern => "noextern",
        WirmType::NoExternNull => "noextern_null",
        WirmType::NoFunc => "nofunc",
        WirmType::NoFuncNull => "nofunc_null",
        WirmType::Eq => "eq",
        WirmType::EqNull => "eq_null",
        WirmType::Struct => "struct",
        WirmType::StructNull => "struct_null",
        WirmType::Array => "array",
        WirmType::ArrayNull => "array_null",
        WirmType::I31 => "i31",
        WirmType::I31Null => "i31_null",
        WirmType::Exn => "exn",
        WirmType::NoExn => "noexn",
        WirmType::Module { .. } => "module",
        WirmType::RecGroup(_) => "recgroup",
        WirmType::CoreTypeId(_) => "core_type_id",
        WirmType::Cont => "cont",
        WirmType::NoCont => "nocont",
    };

    s.to_string()
}

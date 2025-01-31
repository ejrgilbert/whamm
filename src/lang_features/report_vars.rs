#![allow(clippy::too_many_arguments)]
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::parser::types::DataType;
use itertools::Itertools;
use log::info;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::types::{BlockType, DataType as OrcaType, InitExpr, Value};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::{Instructions, Module, Opcode};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use wasmparser::MemArg;

pub const NULL_PTR_IN_MEM: i32 = -1;
pub const NULL_PTR_IN_GLOBAL: i32 = -1;

pub struct ReportVars {
    //MapID -> Metadata
    pub map_metadata: HashMap<u32, Metadata>,
    //GID -> Metadata
    pub variable_metadata: HashMap<u32, (OrcaType, Metadata)>,
    pub all_metadata: HashSet<Metadata>,
    pub curr_location: LocationData,

    // $alloc tracking for Wizard
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
            map_metadata: HashMap::new(),
            variable_metadata: HashMap::new(),
            all_metadata: HashSet::new(),
            curr_location: LocationData::Global { script_id: u8::MAX },
            alloc_tracker: HashMap::default(),
            flush_tracker: FlushTracker {
                flush_var_metadata_fid: None,
            },
        }
    }
    pub fn put_global_metadata(
        &mut self,
        gid: u32,
        name: String,
        whamm_ty: &DataType,
        err: &mut ErrorGen,
    ) -> bool {
        if !matches!(self.curr_location, LocationData::Global { .. }) {
            err.unexpected_error(
                true,
                Some(format!(
                    "Expected global location data, but got: {:?}",
                    self.curr_location
                )),
                None,
            );
            return false;
        }
        let metadata = Metadata::new(name.clone(), whamm_ty.clone(), &self.curr_location);
        self.variable_metadata
            .insert(gid, (metadata.get_wasm_ty(), metadata.clone()));
        if !self.all_metadata.insert(metadata) {
            err.unexpected_error(
                true,
                Some(format!("Duplicate metadata for map with name: {}", name)),
                None,
            );
            return false;
        }
        true
    }
    pub fn put_local_metadata(
        &mut self,
        gid: u32,
        name: String,
        ty: DataType,
        err: &mut ErrorGen,
    ) -> bool {
        if !matches!(self.curr_location, LocationData::Local { .. }) {
            err.unexpected_error(
                true,
                Some(format!(
                    "Expected local location data, but got: {:?}",
                    self.curr_location
                )),
                None,
            );
            return false;
        }

        let metadata = Metadata::new(name.clone(), ty, &self.curr_location);
        self.variable_metadata
            .insert(gid, (metadata.get_wasm_ty(), metadata.clone()));
        if !self.all_metadata.insert(metadata) {
            err.unexpected_error(
                true,
                Some(format!("Duplicate metadata with name: {}", name)),
                None,
            );
            return false;
        }
        true
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
            info += &format!("GID: {} -> {:?}", key, value);
        }

        // Collect and sort map_metadata by key
        let mut sorted_map_metadata: Vec<_> = self.map_metadata.iter().collect();
        sorted_map_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_map_metadata {
            info += &format!("MapID: {} -> {:?}", key, value);
        }

        info!("{info}");
    }
}

// ===========================
// ==== CODE FOR EMITTING ====
// ===========================
impl ReportVars {
    // =========================
    // ==== WIZARD-SPECIFIC ====
    // =========================

    // ==== Call flush functions at program exit ====
    pub fn setup_flush_data_segments(
        &mut self,
        wasm: &mut Module,
        memory_allocator: &mut MemoryAllocator,
    ) {
        // this needs to be a separate function to not have multiple
        // mutable references to the Wasm module at once.
        let id_type = "memaddr".to_string();

        memory_allocator.emit_string(wasm, &mut format!(", {id_type}, "));
        memory_allocator.emit_string(wasm, &mut ", ".to_string());
        memory_allocator.emit_string(wasm, &mut ":".to_string());
        memory_allocator.emit_string(wasm, &mut "script".to_string());

        let supported_dts = vec![
            DataType::U8,
            DataType::I8,
            DataType::U16,
            DataType::I16,
            DataType::U32,
            DataType::I32,
            DataType::U64,
            DataType::I64,
            DataType::F32,
            DataType::F64,
            DataType::Boolean,
        ];
        for dt in supported_dts.iter() {
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
    }
    pub fn emit_flush_logic(
        &mut self,
        func: &mut FunctionBuilder,
        mem_allocator: &MemoryAllocator,
        io_adapter: &mut IOAdapter,
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

        io_adapter.putsln(header_info.0, header_info.1, func, err);
        self.call_dt_flushers(func, mem_allocator, io_adapter, mem_id, wasm, err);
    }

    fn call_dt_flushers(
        &mut self,
        func: &mut FunctionBuilder,
        mem_allocator: &MemoryAllocator,
        io_adapter: &mut IOAdapter,
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
                    let fid = self.emit_flush_u8_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I8 => {
                    let fid = self.emit_flush_i8_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::U16 => {
                    let fid = self.emit_flush_u16_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I16 => {
                    let fid = self.emit_flush_i16_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::U32 => {
                    let fid = self.emit_flush_u32_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I32 => {
                    let fid = self.emit_flush_i32_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::U64 => {
                    let fid = self.emit_flush_u64_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::I64 => {
                    let fid = self.emit_flush_i64_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::F32 => {
                    let fid = self.emit_flush_f32_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::F64 => {
                    let fid = self.emit_flush_f64_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                DataType::Boolean => {
                    let fid = self.emit_flush_bool_fn(io_adapter, mem_id, wasm, err);
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
        // ==================== REPORT CSV FLUSH ========================
        // type, id_type, id, name, script_id, fid:pc, probe_id, value(s)
        let id_type = "memaddr".to_string();
        let mem_arg = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: mem_id,
        };
        let i32_bytes = 4;
        let u8_bytes = 1;

        // handles all but 'value(s)' since this is common between all variable types
        let dt = LocalID(0); // use to figure out which 'type' to print
        let addr = LocalID(1);
        let mut flush_fn = FunctionBuilder::new(&[OrcaType::I64, OrcaType::I32], &[OrcaType::I32]);

        let curr_addr = flush_fn.add_local(OrcaType::I32);

        let fid = flush_fn.add_local(OrcaType::I32); // u32
        let pc = flush_fn.add_local(OrcaType::I32); // u32

        let name_ptr = flush_fn.add_local(OrcaType::I32); // u32
        let name_len = flush_fn.add_local(OrcaType::I32); // u8

        let script_id = flush_fn.add_local(OrcaType::I32); // u8

        let probe_id_ptr = flush_fn.add_local(OrcaType::I32); // u32
        let probe_id_len = flush_fn.add_local(OrcaType::I32); // u8

        // load values from memory into the locals
        flush_fn.local_get(addr).local_set(curr_addr);

        // header format:
        // | fid | pc  | name_ptr | name_len | script_id | probe_id_ptr | probe_id_len |
        // | i32 | i32 | i32      | u8       | u8        | i32          | u8           |

        // load fid (i32)
        flush_fn
            .local_get(curr_addr)
            .i32_load(mem_arg)
            .local_set(fid);

        // TODO -- use offset in load instead of updating the curr_addr every time

        // update memory pointer
        flush_fn
            .i32_const(i32_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_tee(curr_addr);

        // load pc (i32)
        flush_fn.i32_load(mem_arg).local_set(pc);

        // update memory pointer
        flush_fn
            .i32_const(i32_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_tee(curr_addr);

        // load name_ptr (i32)
        flush_fn.i32_load(mem_arg).local_set(name_ptr);

        // update memory pointer
        flush_fn
            .i32_const(i32_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_tee(curr_addr);

        // load name_len (u8)
        flush_fn.i32_load8_u(mem_arg).local_set(name_len);

        // update memory pointer
        flush_fn
            .i32_const(u8_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_tee(curr_addr);

        // load script_id (u8)
        flush_fn.i32_load8_u(mem_arg).local_set(script_id);

        // update memory pointer
        flush_fn
            .i32_const(u8_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_tee(curr_addr);

        // | fid | pc  | name_ptr | name_len | script_id | probe_id_ptr | probe_id_len |
        // | i32 | i32 | i32      | u8       | u8        | i32          | u8           |

        // load probe_id_ptr (i32)
        flush_fn.i32_load(mem_arg).local_set(probe_id_ptr);

        // update memory pointer
        flush_fn
            .i32_const(i32_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_tee(curr_addr);

        // load probe_id_len (u8)
        flush_fn.i32_load8_u(mem_arg).local_set(probe_id_len);

        // update memory pointer
        flush_fn
            .i32_const(u8_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_set(curr_addr);

        // print 'id'
        flush_fn.local_get(addr);
        io_adapter.call_puti32(&mut flush_fn, err);

        // print 'id_type'
        let (addr, len) = mem_allocator.lookup_emitted_string(&format!(", {id_type}, "), err);
        io_adapter.puts(addr, len, &mut flush_fn, err);

        // print 'name'
        flush_fn.local_get(name_ptr).local_get(name_len);
        io_adapter.call_intrusive_puts(&mut flush_fn, err);
        let (addr, len) = mem_allocator.lookup_emitted_string(&", ".to_string(), err);
        io_adapter.puts(addr, len, &mut flush_fn, err);

        // print 'whamm_type' per supported report variable datatype
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::U8,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::I8,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::U16,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::I16,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::U32,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::I32,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::U64,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::I64,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::F32,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::F64,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        flush_fn.else_stmt();
        Self::flush_ty_metadata(
            &mut flush_fn,
            DataType::Boolean,
            &dt,
            mem_allocator,
            io_adapter,
            err,
        );

        // All other datatypes should dynamically trap
        // (need to close all if stmts with 'end')
        #[rustfmt::skip]
        flush_fn.else_stmt()
            .unreachable()
            .end()
            .end()
            .end()
            .end()
            .end()
            .end()
            .end()
            .end()
            .end()
            .end()
            .end();

        // print 'script_id'
        let (addr, len) = mem_allocator.lookup_emitted_string(&"script".to_string(), err);
        io_adapter.puts(addr, len, &mut flush_fn, err);
        flush_fn.local_get(script_id);
        io_adapter.call_puti32(&mut flush_fn, err);
        let (addr, len) = mem_allocator.lookup_emitted_string(&", ".to_string(), err);
        io_adapter.puts(addr, len, &mut flush_fn, err);

        // print 'fid:pc'
        flush_fn.local_get(fid);
        io_adapter.call_puti32(&mut flush_fn, err);
        let (addr, len) = mem_allocator.lookup_emitted_string(&":".to_string(), err);
        io_adapter.puts(addr, len, &mut flush_fn, err);
        flush_fn.local_get(pc);
        io_adapter.call_puti32(&mut flush_fn, err);
        let (addr, len) = mem_allocator.lookup_emitted_string(&", ".to_string(), err);
        io_adapter.puts(addr, len, &mut flush_fn, err);

        // print 'probe_id'
        flush_fn.local_get(probe_id_ptr).local_get(probe_id_len);
        io_adapter.call_safe_puts(&mut flush_fn, err);
        let (addr, len) = mem_allocator.lookup_emitted_string(&", ".to_string(), err);
        io_adapter.puts(addr, len, &mut flush_fn, err);

        // return the pointer to the next place in memory (should point to value(s))
        flush_fn.local_get(curr_addr);

        let flush_fid = flush_fn.finish_module(wasm);
        wasm.set_fn_name(flush_fid, "flush_var_metadata".to_string());
        self.flush_tracker.flush_var_metadata_fid = Some(*flush_fid);
    }
    fn flush_ty_metadata<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        flush_fn: &mut T,
        dt: DataType,
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
        let (addr, len) = mem_allocator.lookup_emitted_string(&format!("{dt}, "), err);
        io_adapter.puts(addr, len, flush_fn, err);
        let wasm_tys = dt.to_wasm_type();
        let wasm_ty = if wasm_tys.len() > 1 {
            // todo support tuples, strings, etc.
            unimplemented!()
        } else {
            get_wasm_ty_str(wasm_tys.first().unwrap())
        };
        let (addr, len) = mem_allocator.lookup_emitted_string(&format!("{wasm_ty}, "), err);
        io_adapter.puts(addr, len, flush_fn, err);
    }

    // ==== Flush functions per datatype ====
    fn emit_flush_fn(
        &self,
        flush_dt: &dyn Fn(&mut FunctionBuilder, &MemArg, &mut IOAdapter, &mut ErrorGen),
        dt: DataType,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        let Some(flush_metadata_fid) = self.flush_tracker.flush_var_metadata_fid else {
            err.unexpected_error(true, Some("Should have the flush variable metadata function ID, but it's not been generated yet.".to_string()), None);
            unreachable!()
        };
        let mem_arg = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: mem_id,
        };
        let val_bytes = if let Some(num) = dt.num_bytes() {
            num as i32
        } else {
            unimplemented!("We can't support flushing this report variable datatype yet.")
        };

        // ============================= REPORT CSV FLUSH ================================
        // id, id_type, name, whamm_type, wasm_type, script_id, fid:pc, probe_id, value(s)

        // handles the 'value(s)' output
        let mut flush_fn = FunctionBuilder::new(&[], &[]);

        let curr_addr = flush_fn.add_local(OrcaType::I32);
        let next_addr = flush_fn.add_local(OrcaType::I32);

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

        // print the value(s)
        flush_dt(&mut flush_fn, &mem_arg, io_adapter, err);

        // update memory pointer
        flush_fn
            .i32_const(val_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_set(curr_addr);

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

        flush_fn.end();

        let flush_fid = flush_fn.finish_module(wasm);
        wasm.set_fn_name(flush_fid, format!("flush_{}_vars", dt));

        *flush_fid
    }

    fn emit_flush_u8_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(&Self::flush_u8, DataType::U8, io_adapter, mem_id, wasm, err)
    }

    fn flush_u8(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load8_u(*mem_arg);
        io_adapter.call_putu8(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_i8_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(&Self::flush_i8, DataType::I8, io_adapter, mem_id, wasm, err)
    }

    fn flush_i8(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load8_s(*mem_arg);
        io_adapter.call_puti8(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_u16_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_u16,
            DataType::U16,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_u16(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load16_u(*mem_arg);
        io_adapter.call_putu16(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_i16_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i16,
            DataType::I16,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_i16(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load16_s(*mem_arg);
        io_adapter.call_puti16(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_u32_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_u32,
            DataType::U32,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_u32(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load(*mem_arg);
        io_adapter.call_putu32(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_bool_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i32,
            DataType::Boolean,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn emit_flush_i32_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i32,
            DataType::I32,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_i32(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i32_load(*mem_arg);
        io_adapter.call_puti32(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_u64_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_u64,
            DataType::U64,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_u64(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i64_load(*mem_arg);
        io_adapter.call_putu64(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_i64_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_i64,
            DataType::I64,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_i64(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.i64_load(*mem_arg);
        io_adapter.call_puti64(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_f32_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_f32,
            DataType::F32,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_f32(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.f32_load(*mem_arg);
        io_adapter.call_putf32(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    fn emit_flush_f64_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        self.emit_flush_fn(
            &Self::flush_f64,
            DataType::F64,
            io_adapter,
            mem_id,
            wasm,
            err,
        )
    }

    fn flush_f64(
        flush_fn: &mut FunctionBuilder,
        mem_arg: &MemArg,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        flush_fn.f64_load(*mem_arg);
        io_adapter.call_putf64(flush_fn, err);
        io_adapter.putln(flush_fn, err);
    }

    pub fn report_var_header_bytes() -> u32 {
        // | next_addr |
        // | i32       |
        // let i32_bytes = 4;
        // i32_bytes
        size_of::<i32>() as u32
    }
    pub fn alloc_report_var_header(
        &mut self,
        data_type: &DataType,
        _curr_addr: u32,
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
            let gid = wasm.add_global(
                InitExpr::new(vec![Instructions::Value(Value::I32(NULL_PTR_IN_GLOBAL))]),
                OrcaType::I32,
                true,
                false,
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

        // TODO -- may be able to just skip this
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

        used_bytes as u32
    }

    pub fn update_next_addr_ptr(
        &mut self,
        data_type: &DataType,
        curr_var_mem_usage: u32,
        total_mem_usage: u32,
        _mem_ptr_addr: u32,
        mem_id: u32,
        mem_tracker_global: GlobalID,
        alloc_func: &mut FunctionBuilder,
        wasm: &mut Module,
    ) {
        // println!("curr_var_mem_usage: {curr_var_mem_usage}");

        let tracker = self
            .alloc_tracker
            .entry(data_type.clone())
            .or_insert(ReportAllocTracker {
                // data_type: data_type.clone(),
                // flush_func: None,
                first_var: None,
                last_var: None,
            });

        if let Some(last_var_gid) = tracker.last_var {
            // ONLY RUN THIS IF THERE ARE MULTIPLE REPORT VARS IN THE SCRIPT

            // When a new variable is allocated in memory, the global containing the memory address
            // of the most-recently allocated variable of that type is used to update the next pointer
            // to the difference between the previous and the current memory address (to find the offset).
            // Then, that global is updated to the current memory address.

            // put header at last_var_addr, value is: (curr_addr - last_var_addr)
            // (where to store)
            alloc_func.global_get(GlobalID(last_var_gid));

            // (what to store)
            // (mem_tracker_global - last_var_gid) + (total_mem_used_for_this_probe - usage_for_this_var)
            alloc_func
                .global_get(mem_tracker_global)
                .global_get(GlobalID(last_var_gid))
                .i32_sub()
                .u32_const(total_mem_usage - curr_var_mem_usage)
                .i32_add();

            alloc_func.i32_store(MemArg {
                align: 0,
                max_align: 0,
                // offset: used_bytes as u64,
                offset: 0,
                memory: mem_id,
            });

            // this memory has already been allocated previously!
            // no need to update `used_bytes`

            // update the last_var global to point to the current location
            alloc_func
                .global_get(mem_tracker_global)
                .u32_const(total_mem_usage - curr_var_mem_usage)
                .i32_add()
                .global_set(GlobalID(last_var_gid));
        } else {
            let gid = wasm.add_global(
                InitExpr::new(vec![Instructions::Value(Value::I32(NULL_PTR_IN_GLOBAL))]),
                OrcaType::I32,
                true,
                false,
            );
            tracker.last_var = Some(*gid);
            let last_var = GlobalID(*gid);

            // if the last_var isn't pointing to anything yet, point it here!
            #[rustfmt::skip]
            alloc_func.global_get(last_var)
                .i32_const(NULL_PTR_IN_GLOBAL)
                .i32_eq()
                .if_stmt(BlockType::Empty)
                    .global_get(mem_tracker_global)
                    .u32_const(total_mem_usage - curr_var_mem_usage)
                    .i32_add()
                    .global_set(last_var)
                .end();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Metadata {
    Global {
        name: String,
        whamm_ty: DataType,
        wasm_ty: OrcaType,
        script_id: u8,
    },
    Local {
        name: String,
        whamm_ty: DataType,
        wasm_ty: OrcaType,
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
                wasm_ty: OrcaType::I32,
                script_id: *script_id,
                bytecode_loc: bytecode_loc.clone(),
                probe_id: probe_id.clone(),
            },
            LocationData::Global { script_id } => Self::Global {
                name: "".to_string(),
                whamm_ty: DataType::I32,
                wasm_ty: OrcaType::I32,
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
    pub fn set_wasm_ty(&mut self, new_ty: OrcaType) {
        match self {
            Self::Local { wasm_ty, .. } | Self::Global { wasm_ty, .. } => *wasm_ty = new_ty,
        }
    }
    pub fn get_wasm_ty(&self) -> OrcaType {
        match self {
            Self::Local { wasm_ty, .. } | Self::Global { wasm_ty, .. } => *wasm_ty,
        }
    }
    pub fn setup_csv_header(wasm: &mut Module, mem_allocator: &mut MemoryAllocator) -> (u32, u32) {
        let mut header = r#"
============================= REPORT CSV FLUSH ================================
id, id_type, name, whamm_type, wasm_type, script_id, fid:pc, probe_id, value(s)"#
            .to_string();
        mem_allocator.emit_string(wasm, &mut header);
        let addr = mem_allocator.emitted_strings.get(&header).unwrap();

        (addr.mem_offset as u32, addr.len as u32)
    }
    pub fn to_csv(&self) -> String {
        let (name, whamm_ty, wasm_ty, script_id, bytecode_loc, probe_id) = match self {
            Metadata::Global {
                name,
                whamm_ty,
                wasm_ty,
                script_id,
            } => (
                name.as_str(),
                whamm_ty.to_string(),
                wasm_ty.to_string(),
                *script_id,
                "",
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
            "global_id, {name}, {whamm_ty}, {wasm_ty}, script{script_id}, {}, {probe_id}",
            bytecode_loc
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocationData {
    Global {
        script_id: u8,
    },
    Local {
        script_id: u8,
        bytecode_loc: BytecodeLoc,
        probe_id: String,
        unshared: HashMap<DataType, i32>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BytecodeLoc {
    fid: u32,
    pc: u32,
}
impl Display for BytecodeLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.fid, self.pc)
    }
}
impl BytecodeLoc {
    pub(crate) fn new(fid: u32, pc: u32) -> Self {
        Self { fid, pc }
    }
}

struct ReportAllocTracker {
    // TODO -- may not need these
    // // The data type that this instance tracks allocation of
    // data_type: DataType,
    // // The ID of the flush function for this DataType
    // flush_func: Option<u32>,

    // global that points to the memory location of the first allocated report variable of this type
    first_var: Option<u32>,
    // global that points to the most-recently allocated report variable of this type
    last_var: Option<u32>,
}
struct FlushTracker {
    flush_var_metadata_fid: Option<u32>,
}

fn get_wasm_ty_str(wasm_ty: &OrcaType) -> String {
    let s = match wasm_ty {
        OrcaType::I8 => "i8",
        OrcaType::I16 => "i16",
        OrcaType::I32 => "i32",
        OrcaType::I64 => "i64",
        OrcaType::F32 => "f32",
        OrcaType::F64 => "f64",
        OrcaType::V128 => "v128",
        OrcaType::FuncRef => "funcref",
        OrcaType::FuncRefNull => "funcref_null",
        OrcaType::ExternRef => "externref",
        OrcaType::ExternRefNull => "externref_null",
        OrcaType::Any => "any",
        OrcaType::AnyNull => "any_null",
        OrcaType::None => "none",
        OrcaType::NoExtern => "noextern",
        OrcaType::NoFunc => "nofunc",
        OrcaType::Eq => "eq",
        OrcaType::EqNull => "eq_null",
        OrcaType::Struct => "struct",
        OrcaType::StructNull => "struct_null",
        OrcaType::Array => "array",
        OrcaType::ArrayNull => "array_null",
        OrcaType::I31 => "i31",
        OrcaType::I31Null => "i31_null",
        OrcaType::Exn => "exn",
        OrcaType::NoExn => "noexn",
        OrcaType::Module { .. } => "module",
        OrcaType::RecGroup(_) => "recgroup",
        OrcaType::CoreTypeId(_) => "core_type_id",
        OrcaType::Cont => "cont",
        OrcaType::NoCont => "nocont",
    };

    s.to_string()
}

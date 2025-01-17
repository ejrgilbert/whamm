use crate::common::error::ErrorGen;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::parser::types::DataType;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::types::{BlockType, DataType as OrcaType, InitExpr, Value};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::{Instructions, Module, Opcode};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use wasmparser::MemArg;

pub const NULL_PTR: u32 = 0;

pub struct ReportVars {
    //MapID -> Metadata
    pub map_metadata: HashMap<u32, Metadata>,
    //GID -> Metadata
    pub variable_metadata: HashMap<u32, (OrcaType, Metadata)>,
    pub all_metadata: HashSet<Metadata>,
    pub curr_location: LocationData,
    pub flush_soon: bool,

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
            flush_soon: false,
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
        println!("Metadata:");

        // Collect and sort variable_metadata by key
        let mut sorted_variable_metadata: Vec<_> = self.variable_metadata.iter().collect();
        sorted_variable_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_variable_metadata {
            println!("GID: {} -> {:?}", key, value);
        }

        // Collect and sort map_metadata by key
        let mut sorted_map_metadata: Vec<_> = self.map_metadata.iter().collect();
        sorted_map_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_map_metadata {
            println!("MapID: {} -> {:?}", key, value);
        }
    }
    pub fn mutating_map(&mut self, map_id: u32) {
        //check if the map you are changing is in map_metadata -> flush soon if it is
        if self.map_metadata.contains_key(&map_id) {
            self.flush_soon = true;
        }
    }
    pub fn mutating_var(&mut self, var_id: u32) {
        //check if the var you are changing is in variable_metadata -> flush soon if it is
        if self.variable_metadata.contains_key(&var_id) {
            self.flush_soon = true;
        }
    }
    pub fn performed_flush(&mut self) {
        self.flush_soon = false;
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
    pub fn emit_flush_logic(
        &mut self,
        func: &mut FunctionBuilder,
        io_adapter: &mut IOAdapter,
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

        Metadata::print_csv_header(func, io_adapter, err);
        self.call_dt_flushers(func, io_adapter, mem_id, wasm, err);
    }

    fn call_dt_flushers(
        &mut self,
        func: &mut FunctionBuilder,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) {
        // Make sure the metadata flusher func exists
        self.emit_flush_var_metadata_fn(io_adapter, mem_id, wasm, err);

        // iterate through all the data type flush functions and emit calls
        for dt in self.alloc_tracker.keys() {
            match dt {
                DataType::I32 => {
                    let fid = self.emit_flush_i32_fn(io_adapter, mem_id, wasm, err);
                    func.call(FunctionID(fid));
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }
    fn emit_flush_var_metadata_fn(
        &mut self,
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
        let mut flush_fn = FunctionBuilder::new(&[OrcaType::I32, OrcaType::I32], &[OrcaType::I32]);

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

        // print 'type'
        // TODO -- replace with br_table
        flush_fn
            .local_get(dt)
            .i32_const(2) // see DataType::hash()
            .i32_eq()
            .if_stmt(BlockType::Empty);
        io_adapter.puts("i32, ".to_string(), &mut flush_fn, err);

        // TODO -- handle 'type' for non-i32 values
        flush_fn.else_stmt().unreachable().end();

        // print 'id_type'
        io_adapter.puts(format!("{id_type}, "), &mut flush_fn, err);

        // print 'id'
        flush_fn.local_get(addr);
        io_adapter.call_puti32(&mut flush_fn, err);

        // print 'name'
        flush_fn.local_get(name_ptr).local_get(name_len);
        io_adapter.call_puts(&mut flush_fn, err);
        io_adapter.puts(", ".to_string(), &mut flush_fn, err);

        // print 'script_id'
        io_adapter.puts("script".to_string(), &mut flush_fn, err);
        flush_fn.local_get(script_id);
        io_adapter.call_puti32(&mut flush_fn, err);
        io_adapter.puts(", ".to_string(), &mut flush_fn, err);

        // print 'fid:pc'
        flush_fn.local_get(fid);
        io_adapter.call_puti32(&mut flush_fn, err);
        io_adapter.puts(":".to_string(), &mut flush_fn, err);
        flush_fn.local_get(pc);
        io_adapter.call_puti32(&mut flush_fn, err);
        io_adapter.puts(", ".to_string(), &mut flush_fn, err);

        // print 'probe_id'
        flush_fn.local_get(probe_id_ptr).local_get(probe_id_len);
        io_adapter.call_puts(&mut flush_fn, err);
        io_adapter.puts(", ".to_string(), &mut flush_fn, err);

        // return the pointer to the next place in memory (should point to value(s))
        flush_fn.local_get(curr_addr);

        let flush_fid = flush_fn.finish_module(wasm);
        wasm.set_fn_name(flush_fid, "flush_var_metadata".to_string());
        self.flush_tracker.flush_var_metadata_fid = Some(*flush_fid);
    }

    // ==== Flush functions per datatype ====
    fn emit_flush_i32_fn(
        &self,
        io_adapter: &mut IOAdapter,
        mem_id: u32,
        wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> u32 {
        let Some(flush_metadata_fid) = self.flush_tracker.flush_var_metadata_fid else {
            err.unexpected_error(true, Some("Should have the flush variable metadata function ID, but it's not been generated yet.".to_string()), None);
            unreachable!()
        };
        let i32_bytes = 4;
        let mem_arg = MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: mem_id,
        };

        // ==================== REPORT CSV FLUSH ========================
        // type, id_type, id, name, script_id, fid:pc, probe_id, value(s)

        // handles the 'value(s)' output
        let dt = DataType::I32;
        let mut flush_fn = FunctionBuilder::new(&[], &[]);

        let curr_addr = flush_fn.add_local(OrcaType::I32);
        let next_addr = flush_fn.add_local(OrcaType::I32);

        flush_fn
            .global_get(GlobalID(
                self.alloc_tracker.get(&dt).unwrap().first_var.unwrap(),
            ))
            .local_set(curr_addr);

        flush_fn.loop_stmt(BlockType::Empty);

        // todo: next_addr is offset!
        // alloc_func.u32_const(curr_addr)
        //     .global_get(GlobalID(last_var_gid))
        //     .i32_sub();

        #[rustfmt::skip]
        // save the next_addr
        flush_fn
            .local_get(curr_addr)
            .i32_load(mem_arg)
            .local_tee(next_addr)

            // If the next_addr is a NULL_PTR, leave it so...
            // otherwise calculate the actual next address:
            //    next_addr = curr_addr + next_addr_offset
            .u32_const(NULL_PTR)
            .i32_eq()
            .if_stmt(BlockType::Empty)
                .local_get(curr_addr)
                .local_get(next_addr)
                .i32_add()
                .local_set(next_addr)
            .end();

        // update memory pointer
        flush_fn
            .i32_const(i32_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_set(curr_addr);

        // use return of flush_metadata_func to know where value(s) starts!
        flush_fn
            .i32_const(dt.id())
            .local_get(curr_addr)
            .call(FunctionID(flush_metadata_fid))
            .local_tee(curr_addr);

        // print the value(s)
        flush_fn.i32_load(mem_arg);
        io_adapter.call_puti32(&mut flush_fn, err);
        io_adapter.putln(&mut flush_fn, err);

        // update memory pointer
        flush_fn
            .i32_const(i32_bytes)
            .local_get(curr_addr)
            .i32_add()
            .local_set(curr_addr);

        // check if we should loop
        // while next_addr != NULL_PTR: curr_addr = next_addr; continue;

        #[rustfmt::skip]
        flush_fn
            .local_get(next_addr)
            .u32_const(NULL_PTR)
            .i32_ne()
            .if_stmt(BlockType::Empty)
                .local_get(next_addr)
                .local_set(curr_addr)
                .br(1)
            .end();
        // otherwise, fall through to end.

        flush_fn.end();

        let flush_fid = flush_fn.finish_module(wasm);
        wasm.set_fn_name(flush_fid, "flush_i32_vars".to_string());
        *flush_fid
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
        curr_addr: u32,
        mem_id: u32,
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

        if tracker.first_var.is_none() {
            // On the first allocation for a datatype, the global that points to the first memory
            // location is updated to point to the memory address.
            let gid = wasm.add_global(
                InitExpr::new(vec![Instructions::Value(Value::I32(curr_addr as i32))]),
                OrcaType::I32,
                false,
                false,
            );
            tracker.first_var = Some(*gid);
        }

        // put header in memory at curr_addr, value is: NULL_PTR
        alloc_func.u32_const(curr_addr); // (where to store)
        alloc_func.u32_const(NULL_PTR); // (what to store)
        alloc_func.i32_store(MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: mem_id,
        });
        used_bytes += size_of_val(&NULL_PTR);

        if let Some(last_var_gid) = tracker.last_var {
            // When a new variable is allocated in memory, the global containing the memory address
            // of the most-recently allocated variable of that type is used to update the next pointer
            // to the difference between the previous and the current memory address (to find the offset).
            // Then, that global is updated to the current memory address.

            // put header at last_var_addr, value is: (curr_addr - last_var_addr)
            // (where to store)
            alloc_func.global_get(GlobalID(last_var_gid));

            // (what to store)
            alloc_func
                .u32_const(curr_addr)
                .global_get(GlobalID(last_var_gid))
                .i32_sub();

            alloc_func.i32_store(MemArg {
                align: 0,
                max_align: 0,
                offset: 0,
                memory: mem_id,
            });
        } else {
            let gid = wasm.add_global(
                InitExpr::new(vec![Instructions::Value(Value::I32(curr_addr as i32))]),
                OrcaType::I32,
                false,
                false,
            );
            tracker.last_var = Some(*gid);
        }

        used_bytes as u32
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
    pub fn print_csv_header<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        func: &mut T,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) {
        let header = r#"
============================= REPORT CSV FLUSH ================================
id, id_type, name, whamm_type, wasm_type, script_id, fid:pc, probe_id, value(s)"#
            .to_string();
        io_adapter.putsln(header, func, err);
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
                wasm_ty.to_string(),
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

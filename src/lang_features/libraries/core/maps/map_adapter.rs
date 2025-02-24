#![allow(clippy::too_many_arguments)]
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::lang_features::libraries::core::LibAdapter;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::types::BlockType as OrcaBlockType;
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::{Instrumenter, MacroOpcode};
use orca_wasm::{Location, Module, Opcode};
use std::collections::HashMap;

const UNEXPECTED_ERR_MSG: &str =
    "MapLibAdapter: Looks like you've found a bug...please report this behavior!";

const PRINT_MAP: &str = "print_map";
pub const MAP_LIB_MEM_OFFSET: u32 = 0;

pub struct MapLibAdapter {
    pub is_used: bool,
    // func_name -> fid
    funcs: HashMap<String, u32>,
    map_count: u32,
    pub init_bool_location: u32,

    pub(crate) app_mem: i32,
    pub(crate) lib_mem: i32,

    pub curr_str_offset: Option<u32>,
    pub curr_str_len: Option<u32>,
}
impl Default for MapLibAdapter {
    fn default() -> Self {
        Self::new()
    }
}
impl LibAdapter for MapLibAdapter {
    fn get_funcs(&self) -> &HashMap<String, u32> {
        &self.funcs
    }
    fn get_funcs_mut(&mut self) -> &mut HashMap<String, u32> {
        &mut self.funcs
    }
    fn define_helper_funcs(
        &mut self,
        app_wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        self.emit_helper_funcs(app_wasm, err)
    }
}
impl MapLibAdapter {
    pub fn new() -> Self {
        let funcs = HashMap::from([
            // create map
            ("create_i32_i32_with_id".to_string(), 0),
            ("create_i32_i32".to_string(), 0),
            ("create_i32_bool_with_id".to_string(), 0),
            ("create_i32_bool".to_string(), 0),
            ("create_i32_string_with_id".to_string(), 0),
            ("create_i32_string".to_string(), 0),
            ("create_i32_tuple_with_id".to_string(), 0),
            ("create_i32_tuple".to_string(), 0),
            ("create_i32_map_with_id".to_string(), 0),
            ("create_i32_map".to_string(), 0),
            ("create_string_i32_with_id".to_string(), 0),
            ("create_string_i32".to_string(), 0),
            ("create_string_bool_with_id".to_string(), 0),
            ("create_string_bool".to_string(), 0),
            ("create_string_string_with_id".to_string(), 0),
            ("create_string_string".to_string(), 0),
            ("create_string_tuple_with_id".to_string(), 0),
            ("create_string_tuple".to_string(), 0),
            ("create_string_map_with_id".to_string(), 0),
            ("create_string_map".to_string(), 0),
            ("create_bool_i32_with_id".to_string(), 0),
            ("create_bool_i32".to_string(), 0),
            ("create_bool_bool_with_id".to_string(), 0),
            ("create_bool_bool".to_string(), 0),
            ("create_bool_string_with_id".to_string(), 0),
            ("create_bool_string".to_string(), 0),
            ("create_bool_tuple_with_id".to_string(), 0),
            ("create_bool_tuple".to_string(), 0),
            ("create_bool_map_with_id".to_string(), 0),
            ("create_bool_map".to_string(), 0),
            ("create_tuple_i32_with_id".to_string(), 0),
            ("create_tuple_i32".to_string(), 0),
            ("create_tuple_bool_with_id".to_string(), 0),
            ("create_tuple_bool".to_string(), 0),
            ("create_tuple_string_with_id".to_string(), 0),
            ("create_tuple_string".to_string(), 0),
            ("create_tuple_tuple_with_id".to_string(), 0),
            ("create_tuple_tuple".to_string(), 0),
            ("create_tuple_map_with_id".to_string(), 0),
            ("create_tuple_map".to_string(), 0),
            // insert map
            ("insert_i32_i32".to_string(), 0),
            ("insert_i32_string".to_string(), 0),
            ("insert_string_i32".to_string(), 0),
            ("insert_i32i32i32tuple_i32".to_string(), 0),
            // get from map
            ("get_i32_i32".to_string(), 0),
            ("get_i32_string".to_string(), 0),
            ("get_string_i32".to_string(), 0),
            ("get_i32i32i32tuple_i32".to_string(), 0),
            // printing maps
            ("print_map".to_string(), 0),
        ]);
        MapLibAdapter {
            is_used: false,
            funcs,
            map_count: 0,
            init_bool_location: 0,
            app_mem: -1,
            lib_mem: -1,
            curr_str_offset: None,
            curr_str_len: None,
        }
    }

    pub fn emit_helper_funcs(
        &mut self,
        _app_wasm: &mut Module,
        _err: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        // (nothing to do)
        vec![]
    }

    pub fn map_get<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        key: DataType,
        val: DataType,
        func: &mut T,
        mem_allocator: &MemoryAllocator,
        err: &mut ErrorGen,
    ) {
        let fname = self.map_get_fname(&key, &val, err);
        let src_len = if matches!(key, DataType::Str) {
            Some(self.handle_string_key_before_call(func, mem_allocator))
        } else {
            None
        };

        self.call(&fname, func, err);

        if matches!(key, DataType::Str) {
            let Some(src_len) = src_len else {
                panic!("Expected src_len of String to be set!")
            };
            self.handle_string_key_after_call(src_len, func, mem_allocator);
        }
    }

    fn handle_string_key_before_call<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        func: &mut T,
        mem_allocator: &MemoryAllocator,
    ) -> LocalID {
        let (Some(curr_str_offset), Some(curr_str_len)) = (self.curr_str_offset, self.curr_str_len)
        else {
            panic!("Expected the offset and len to be set for the key String!");
        };

        let src_offset = func.add_local(OrcaType::I32);
        let src_len = func.add_local(OrcaType::I32);

        func.u32_const(curr_str_offset).local_set(src_offset);
        func.u32_const(curr_str_len).local_set(src_len);

        mem_allocator.copy_to_mem_and_save(
            self.app_mem as u32,
            src_offset,
            src_len,
            self.lib_mem as u32,
            MAP_LIB_MEM_OFFSET,
            func,
        );
        src_len
    }

    fn handle_string_key_after_call<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &self,
        src_len: LocalID,
        func: &mut T,
        mem_allocator: &MemoryAllocator,
    ) {
        mem_allocator.copy_back_saved_mem(src_len, self.lib_mem as u32, MAP_LIB_MEM_OFFSET, func);
    }

    pub fn map_insert<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        key: DataType,
        val: DataType,
        func: &mut T,
        mem_allocator: &MemoryAllocator,
        err: &mut ErrorGen,
    ) {
        let fname = self.map_insert_fname(&key, &val, err);
        let src_len = if matches!(&key, DataType::Str) {
            Some(self.handle_string_key_before_call(func, mem_allocator))
        } else {
            None
        };

        self.call(&fname, func, err);

        if matches!(&key, DataType::Str) {
            let Some(src_len) = src_len else {
                panic!("Expected src_len of String to be set!")
            };
            self.handle_string_key_after_call(src_len, func, mem_allocator);
        }
    }

    pub fn map_create_report<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        name: String,
        ty: DataType,
        func: &mut T,
        report_vars: &mut ReportVars,
        err: &mut ErrorGen,
    ) -> u32 {
        let map_id = self.map_create(ty.clone(), func, err);
        //create the metadata for the map
        report_vars.put_map_metadata(map_id, name.clone(), ty, err);
        map_id
    }

    pub fn map_create<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        ty: DataType,
        func: &mut T,
        err: &mut ErrorGen,
    ) -> u32 {
        let (map_id, func_name) = self.create_map_internal(ty, err);
        func.u32_const(map_id);
        self.call(func_name.as_str(), func, err);
        map_id
    }

    pub fn map_create_dynamic<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        ty: DataType,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        // This variation of map_create doesn't know the ID statically
        let func_name = self.create_map_fname_by_map_type(ty, true, err);
        self.call(func_name.as_str(), func, err);
    }

    pub fn print_map<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        map_id: u32,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        func.u32_const(map_id);
        self.call_print_map(func, err)
    }

    // -------------------
    // ==== Utilities ====
    // -------------------

    fn create_map_internal(&mut self, map: DataType, err: &mut ErrorGen) -> (u32, String) {
        let map_id = self.next_map_id();
        let func_name = self.create_map_fname_by_map_type(map, false, err);
        (map_id, func_name)
    }

    pub(crate) fn call_print_map<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        self.call(PRINT_MAP, func, err)
    }

    fn next_map_id(&mut self) -> u32 {
        let map_id = self.map_count;
        self.map_count += 1;
        map_id
    }

    // -------------------------------------
    // Get "to_call" for map functions
    // -------------------------------------

    fn ty_to_str(is_create: bool, ty: &DataType, err: &mut ErrorGen) -> String {
        let mut inner = "".to_string();
        let str = match ty {
            DataType::I32 => "i32",
            DataType::Boolean => "bool",
            DataType::Str => "string",
            DataType::Tuple {
                ty_info: inner_types,
            } => {
                if is_create {
                    "tuple"
                } else {
                    for inner_ty in inner_types.iter() {
                        inner += Self::ty_to_str(is_create, inner_ty, err).as_str();
                    }
                    inner += "tuple";
                    let str = inner.as_str();
                    str
                }
            }
            DataType::Map { .. } => "map",
            DataType::U32 => "i32", // treated the same
            DataType::F32 => "f32",
            DataType::U64 => "i64", // treated the same
            DataType::I64 => "i64",
            DataType::F64 => "f64",
            ty => {
                err.type_check_error(
                    true,
                    format!("Unsupported value type for map library: {:?}", ty),
                    &None,
                );
                ""
            }
        };

        str.to_string()
    }

    //The stuff that actually calls the emitter stuff
    fn create_map_fname_by_map_type(
        &mut self,
        map: DataType,
        is_dynamic: bool,
        err: &mut ErrorGen,
    ) -> String {
        let DataType::Map {
            key_ty: key,
            val_ty: val,
        } = map
        else {
            err.unexpected_error(true, Some("Non-map at no_meta".to_string()), None);
            return "invalid".to_string();
        };

        self.map_create_fname(*key, *val, is_dynamic, err)
    }
    fn map_create_fname(
        &mut self,
        key: DataType,
        val: DataType,
        is_dynamic: bool,
        err: &mut ErrorGen,
    ) -> String {
        let key_name = Self::ty_to_str(true, &key, err);
        let val_name = Self::ty_to_str(true, &val, err);
        let with_id = if !is_dynamic { "_with_id" } else { "" };

        let fname = format!("create_{key_name}_{val_name}{with_id}");
        if self.funcs.contains_key(&fname) {
            fname
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_create_fname: Unsupported map type: {:?} -> {:?}, need function with name '{fname}'",
                    key, val
                ),
                &None,
            );
            "invalid".to_string()
        }
    }
    fn map_insert_fname(&mut self, key: &DataType, val: &DataType, err: &mut ErrorGen) -> String {
        let key_name = Self::ty_to_str(false, key, err);
        let val_name = Self::ty_to_str(false, val, err);

        let fname = format!("insert_{key_name}_{val_name}");
        if self.funcs.contains_key(&fname) {
            fname
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_insert_fname: Unsupported map type: {:?} -> {:?}, need function with name '{fname}'",
                    key, val
                ),
                &None,
            );
            "invalid".to_string()
        }
    }
    fn map_get_fname(&mut self, key: &DataType, val: &DataType, err: &mut ErrorGen) -> String {
        let key_name = Self::ty_to_str(false, key, err);
        let val_name = Self::ty_to_str(false, val, err);

        let fname = format!("get_{key_name}_{val_name}");
        if self.funcs.contains_key(&fname) {
            fname
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_get_fname: Unsupported map type: {:?} -> {:?}, need function with name '{fname}'",
                    key, val
                ),
                &None,
            );
            "invalid".to_string()
        }
    }

    fn call<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        fname: &str,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        let fid = self.get_fid(fname, err);
        func.call(FunctionID(fid));
    }

    // ========================
    // ==== MAP INIT LOGIC ====
    // ========================

    const MAP_INIT_FNAME: &'static str = "instr_init";

    pub fn get_map_init_fid(&self, app_wasm: &mut Module, err: &mut ErrorGen) -> FunctionID {
        match app_wasm
            .functions
            .get_local_fid_by_name(Self::MAP_INIT_FNAME)
        {
            Some(to_call) => to_call,
            None => {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                        No {} function found in the module!",
                        Self::MAP_INIT_FNAME
                    )),
                    None,
                );
                unreachable!();
            }
        }
    }

    pub fn emit_map_init(
        &mut self,
        name: String,
        addr: &mut Option<VarAddr>,
        ty: &mut DataType,
        is_report: bool,
        report_vars: &mut ReportVars,
        app_wasm: &mut Module,
        err: &mut ErrorGen,
    ) {
        //time to set up the map_init fn
        let init_id = self.get_map_init_fid(app_wasm, err);

        let Some(mut init_fn) = app_wasm.functions.get_fn_modifier(init_id) else {
            err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                                No instr_init found in the module!"
                )),
                None,
            );
            return;
        };
        init_fn.before_at(Location::Module {
            func_idx: init_id, // not used
            instr_idx: 0,
        });
        let map_id = if is_report {
            self.map_create_report(name, ty.clone(), &mut init_fn, report_vars, err)
        } else {
            self.map_create(ty.clone(), &mut init_fn, err)
        };

        *addr = Some(VarAddr::MapId { addr: map_id });
    }

    pub fn inject_map_init_check<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        func: &mut T,
        map_init_fid: FunctionID,
    ) {
        if !self.is_used {
            // no maps to init!
            // only inject this IF NEEDED (not all scripts need global init)
            return;
        }

        // 1 means the maps have not been initialized, 0 means they have
        func.global_get(GlobalID(self.init_bool_location));
        func.if_stmt(OrcaBlockType::Empty);
        func.i32_const(0);
        func.global_set(GlobalID(self.init_bool_location));
        func.call(map_init_fid);
        func.end();
    }
}

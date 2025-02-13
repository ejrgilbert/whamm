#![allow(clippy::too_many_arguments)]
use crate::common::error::ErrorGen;
use crate::lang_features::libraries::core::LibAdapter;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use orca_wasm::ir::id::{FunctionID, GlobalID};
use orca_wasm::ir::types::BlockType as OrcaBlockType;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::{Instrumenter, MacroOpcode};
use orca_wasm::{Location, Module, Opcode};
use std::collections::HashMap;

const UNEXPECTED_ERR_MSG: &str =
    "MapLibAdapter: Looks like you've found a bug...please report this behavior!";

const PRINT_MAP: &str = "print_map";

pub struct MapLibAdapter {
    pub is_used: bool,
    // func_name -> fid
    funcs: HashMap<String, u32>,
    map_count: u32,
    pub init_bool_location: u32,
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
            ("create_i32_i32".to_string(), 0),
            ("create_i32_bool".to_string(), 0),
            ("create_i32_string".to_string(), 0),
            ("create_i32_tuple".to_string(), 0),
            ("create_i32_map".to_string(), 0),
            ("create_string_i32".to_string(), 0),
            ("create_string_bool".to_string(), 0),
            ("create_string_string".to_string(), 0),
            ("create_string_tuple".to_string(), 0),
            ("create_string_map".to_string(), 0),
            ("create_bool_i32".to_string(), 0),
            ("create_bool_bool".to_string(), 0),
            ("create_bool_string".to_string(), 0),
            ("create_bool_tuple".to_string(), 0),
            ("create_bool_map".to_string(), 0),
            ("create_tuple_i32".to_string(), 0),
            ("create_tuple_bool".to_string(), 0),
            ("create_tuple_string".to_string(), 0),
            ("create_tuple_tuple".to_string(), 0),
            ("create_tuple_map".to_string(), 0),
            // insert map
            ("insert_i32_i32".to_string(), 0),
            ("insert_i32_string".to_string(), 0),
            ("insert_i32i32i32tuple_i32".to_string(), 0),
            // get from map
            ("get_i32_i32".to_string(), 0),
            ("get_i32_string".to_string(), 0),
            ("get_i32i32i32tuple_i32".to_string(), 0),
            // printing maps
            ("print_map".to_string(), 0),
        ]);
        MapLibAdapter {
            is_used: false,
            funcs,
            map_count: 0,
            init_bool_location: 0,
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
        err: &mut ErrorGen,
    ) {
        let fname = self.map_get_fname(key, val, err);
        self.call(fname.as_str(), func, err);
    }

    pub fn map_insert<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        key: DataType,
        val: DataType,
        func: &mut T,
        err: &mut ErrorGen,
    ) {
        let fname = self.map_insert_fname(key, val, err);
        self.call(fname.as_str(), func, err);
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
        let func_name = self.create_map_fname_by_map_type(map, err);
        (map_id, func_name)
    }

    fn call_print_map<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
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
    fn create_map_fname_by_map_type(&mut self, map: DataType, err: &mut ErrorGen) -> String {
        let DataType::Map {
            key_ty: key,
            val_ty: val,
        } = map
        else {
            err.unexpected_error(true, Some("Non-map at no_meta".to_string()), None);
            return "invalid".to_string();
        };

        self.map_create_fname(*key, *val, err)
    }
    fn map_create_fname(&mut self, key: DataType, val: DataType, err: &mut ErrorGen) -> String {
        let key_name = Self::ty_to_str(true, &key, err);
        let val_name = Self::ty_to_str(true, &val, err);

        let fname = format!("create_{key_name}_{val_name}");
        if self.funcs.contains_key(&fname) {
            fname
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_create_fname: Unsupported map type: {:?} -> {:?}",
                    key, val
                ),
                &None,
            );
            "invalid".to_string()
        }
    }
    fn map_insert_fname(&mut self, key: DataType, val: DataType, err: &mut ErrorGen) -> String {
        let key_name = Self::ty_to_str(false, &key, err);
        let val_name = Self::ty_to_str(false, &val, err);

        let fname = format!("insert_{key_name}_{val_name}");
        if self.funcs.contains_key(&fname) {
            fname
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_insert_fname: Unsupported map type: {:?} -> {:?}",
                    key, val
                ),
                &None,
            );
            "invalid".to_string()
        }
    }
    fn map_get_fname(&mut self, key: DataType, val: DataType, err: &mut ErrorGen) -> String {
        let key_name = Self::ty_to_str(false, &key, err);
        let val_name = Self::ty_to_str(false, &val, err);

        let fname = format!("get_{key_name}_{val_name}");
        if self.funcs.contains_key(&fname) {
            fname
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_get_fname: Unsupported map type: {:?} -> {:?}",
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

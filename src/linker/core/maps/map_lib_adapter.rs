use crate::common::error::ErrorGen;
use crate::parser::types::DataType;
use std::collections::HashSet;
// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
use crate::emitter::report_var_metadata::{LocationData, Metadata, ReportVarMetadata};
use crate::linker::core::LibAdapter;

pub const RESERVED_VAR_METADATA_MAP_ID: u32 = 0;
pub const RESERVED_MAP_METADATA_MAP_ID: u32 = 1;

pub struct MapLibAdapter {
    func_names: HashSet<String>,
    map_count: u32,
    pub init_bool_location: u32,
}
impl Default for MapLibAdapter {
    fn default() -> Self {
        Self::new()
    }
}
impl LibAdapter for MapLibAdapter {
    fn get_fn_names(&self) -> &HashSet<String> {
        &self.func_names
    }
}
impl MapLibAdapter {
    pub fn new() -> Self {
        let func_names = HashSet::from_iter(vec![
            // printing metadata
            "putc".to_string(),
            "puti".to_string(),
            "putln".to_string(),
            "put_i32".to_string(),
            "put_map".to_string(),
            "put_comma".to_string(),
            "print_map".to_string(),
            // "print_map_meta".to_string(),
            // "print_global_i32_meta_helper".to_string(),
            // "set_metadata_header".to_string(),
            // "print_metadata_header".to_string(),
            // create map
            "create_i32_i32".to_string(),
            "create_i32_bool".to_string(),
            "create_i32_string".to_string(),
            "create_i32_tuple".to_string(),
            "create_i32_map".to_string(),
            "create_string_i32".to_string(),
            "create_string_bool".to_string(),
            "create_string_string".to_string(),
            "create_string_tuple".to_string(),
            "create_string_map".to_string(),
            "create_bool_i32".to_string(),
            "create_bool_bool".to_string(),
            "create_bool_string".to_string(),
            "create_bool_tuple".to_string(),
            "create_bool_map".to_string(),
            "create_tuple_i32".to_string(),
            "create_tuple_bool".to_string(),
            "create_tuple_string".to_string(),
            "create_tuple_tuple".to_string(),
            "create_tuple_map".to_string(),
            // insert map
            "insert_i32_i32".to_string(),
            "insert_i32_string".to_string(),
            "insert_i32i32i32tuple_i32".to_string(),
            // get from map
            "get_i32_i32".to_string(),
            "get_i32_from_i32i32i32tuple".to_string(),
        ]);
        //Reserve map 0 for the var metadata map and map 1 for the map metadata map
        MapLibAdapter {
            func_names,
            map_count: 2,
            init_bool_location: 0,
        }
    }

    // --------------------------
    // ==== Map creation fns ====
    // --------------------------

    fn next_map_id(&mut self) -> u32 {
        let map_id = self.map_count;
        self.map_count += 1;
        map_id
    }

    fn create_map_metadata(
        &mut self,
        map_id: u32,
        name: String,
        report_var_metadata: &mut ReportVarMetadata,
        is_local: bool,
        err: &mut ErrorGen,
    ) {
        if is_local {
            if !matches!(
                report_var_metadata.curr_location,
                LocationData::Local { .. }
            ) {
                err.unexpected_error(
                    true,
                    Some(format!("Can only emit local maps when in a local function scope in the target application...but we're in the global scope! See map: {}", name)),
                    None,
                );
            }
        } else if !matches!(
            report_var_metadata.curr_location,
            LocationData::Global { .. }
        ) {
            err.unexpected_error(
                true,
                Some(format!("Can only emit global maps when in the global scope of the target application...but we're in a local function scope! See map: {}", name)),
                None,
            );
        };

        let metadata = Metadata::new(name.clone(), &report_var_metadata.curr_location);
        report_var_metadata
            .map_metadata
            .insert(map_id, metadata.clone());
        if !report_var_metadata.all_metadata.insert(metadata) {
            err.unexpected_error(
                true,
                Some(format!("Duplicate metadata for map with name: {}", name)),
                None,
            );
        };
    }

    pub fn create_report_map(
        &mut self,
        name: String,
        map: DataType,
        report_var_metadata: &mut ReportVarMetadata,
        is_local: bool,
        err: &mut ErrorGen,
    ) -> (u32, Option<String>) {
        let map_id = self.next_map_id();
        //create the metadata for the map
        self.create_map_metadata(map_id, name.clone(), report_var_metadata, is_local, err);

        //create the map based on the types of the key and value in the map
        //"map" is the type of the declaration statement
        let func_name = self.create_map_fname_by_map_type(map, err);
        (map_id, func_name)
    }

    /// Create a map that is not reported (has no metadata)
    pub fn create_map(&mut self, map: DataType, err: &mut ErrorGen) -> (u32, Option<String>) {
        let map_id = self.next_map_id();
        let func_name = self.create_map_fname_by_map_type(map, err);
        (map_id, func_name)
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
            DataType::U32 => "u32",
            DataType::F32 => "f32",
            DataType::U64 => "u64",
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
        err: &mut ErrorGen,
    ) -> Option<String> {
        let DataType::Map {
            key_ty: key,
            val_ty: val,
        } = map
        else {
            err.unexpected_error(true, Some("Non-map at no_meta".to_string()), None);
            return None;
        };

        self.map_create_fname(*key, *val, err)
    }
    pub fn map_create_fname(
        &mut self,
        key: DataType,
        val: DataType,
        err: &mut ErrorGen,
    ) -> Option<String> {
        let key_name = Self::ty_to_str(true, &key, err);
        let val_name = Self::ty_to_str(true, &val, err);

        let fname = format!("create_{key_name}_{val_name}");
        if self.func_names.contains(&fname) {
            Some(fname)
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_create_fname: Unsupported map type: {:?} -> {:?}",
                    key, val
                ),
                &None,
            );
            None
        }
    }
    pub fn map_insert_fname(
        &mut self,
        key: DataType,
        val: DataType,
        err: &mut ErrorGen,
    ) -> Option<String> {
        let key_name = Self::ty_to_str(false, &key, err);
        let val_name = Self::ty_to_str(false, &val, err);

        let fname = format!("insert_{key_name}_{val_name}");
        if self.func_names.contains(&fname) {
            Some(fname)
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_insert_fname: Unsupported map type: {:?} -> {:?}",
                    key, val
                ),
                &None,
            );
            None
        }
    }
    pub fn map_get_fname(
        &mut self,
        key: DataType,
        val: DataType,
        err: &mut ErrorGen,
    ) -> Option<String> {
        let key_name = Self::ty_to_str(false, &key, err);
        let val_name = Self::ty_to_str(false, &val, err);

        let fname = format!("get_{key_name}_{val_name}");
        if self.func_names.contains(&fname) {
            Some(fname)
        } else {
            err.type_check_error(
                true,
                format!(
                    "MapLibAdapter.map_get_fname: Unsupported map type: {:?} -> {:?}",
                    key, val
                ),
                &None,
            );
            None
        }
    }
}

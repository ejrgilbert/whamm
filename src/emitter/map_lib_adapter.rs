use crate::common::error::ErrorGen;
use crate::parser::types::DataType;
// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
use crate::emitter::report_var_metadata::{LocationData, Metadata, ReportVarMetadata};

pub const RESERVED_VAR_METADATA_MAP_ID: u32 = 0;
pub const RESERVED_MAP_METADATA_MAP_ID: u32 = 1;

pub struct MapLibAdapter {
    map_count: u32,
    pub init_bool_location: u32,
}
impl Default for MapLibAdapter {
    fn default() -> Self {
        Self::new()
    }
}
impl MapLibAdapter {
    pub fn new() -> Self {
        //Reserve map 0 for the var metadata map and map 1 for the map metadata map
        MapLibAdapter {
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

        self.create_map_fname(*key, *val, err)
    }
    pub fn create_map_fname(
        &mut self,
        key: DataType,
        val: DataType,
        err: &mut ErrorGen,
    ) -> Option<String> {
        match key {
            DataType::I32 => match val {
                DataType::I32 => Some("create_i32_i32".to_string()),
                DataType::Boolean => Some("create_i32_bool".to_string()),
                DataType::Str => Some("create_i32_string".to_string()),
                DataType::Tuple { .. } => Some("create_i32_tuple".to_string()),
                DataType::Map { .. } => Some("create_i32_map".to_string()),
                _ => {
                    err.unexpected_error(
                        // todo -- this isn't an unexpected error!
                        true,
                        Some(format!(
                            "Unsupported value type for map with I32 key: {:?}",
                            val
                        )),
                        None,
                    );
                    None
                }
            },
            DataType::Str => match val {
                DataType::I32 => Some("create_string_i32".to_string()),
                DataType::Boolean => Some("create_string_bool".to_string()),
                DataType::Str => Some("create_string_string".to_string()),
                DataType::Tuple { .. } => Some("create_string_tuple".to_string()),
                DataType::Map { .. } => Some("create_string_map".to_string()),
                _ => {
                    err.unexpected_error(
                        // todo -- this isn't an unexpected error!
                        true,
                        Some(format!(
                            "Unsupported value type for map with Str key: {:?}",
                            val
                        )),
                        None,
                    );
                    None
                }
            },
            DataType::Boolean {} => match val {
                DataType::I32 => Some("create_bool_i32".to_string()),
                DataType::Boolean => Some("create_bool_bool".to_string()),
                DataType::Str => Some("create_bool_string".to_string()),
                DataType::Tuple { .. } => Some("create_bool_tuple".to_string()),
                DataType::Map { .. } => Some("create_bool_map".to_string()),
                _ => {
                    err.unexpected_error(
                        // todo -- this isn't an unexpected error!
                        true,
                        Some(format!(
                            "Unsupported value type for map with Boolean key: {:?}",
                            val
                        )),
                        None,
                    );
                    None
                }
            },
            DataType::Tuple { .. } => match val {
                DataType::I32 => Some("create_tuple_i32".to_string()),
                DataType::Boolean => Some("create_tuple_bool".to_string()),
                DataType::Str => Some("create_tuple_string".to_string()),
                DataType::Tuple { .. } => Some("create_tuple_tuple".to_string()),
                DataType::Map { .. } => Some("create_tuple_map".to_string()),
                _ => {
                    err.unexpected_error(
                        // todo -- this isn't an unexpected error!
                        true,
                        Some(format!(
                            "Unsupported value type for map with Tuple key: {:?}",
                            val
                        )),
                        None,
                    );
                    None
                }
            },
            _ => {
                err.unexpected_error(
                    // todo -- this isn't an unexpected error!
                    true,
                    Some(format!("Unsupported key type for map: {:?}", key)),
                    None,
                );
                None
            }
        }
    }
    pub fn insert_map_fname(
        &mut self,
        key: DataType,
        val: DataType,
        err: &mut ErrorGen,
    ) -> Option<String> {
        match &key {
            DataType::I32 => match val {
                DataType::I32 => Some("insert_i32_i32".to_string()),
                DataType::Str => Some("insert_i32_string".to_string()),
                _ => {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "not yet supported type for map: {:?} -> {:?}",
                            key, val
                        )),
                        None,
                    );
                    None
                }
            },
            DataType::Tuple { ty_info } => {
                if *ty_info
                    == vec![
                        Box::new(DataType::I32),
                        Box::new(DataType::I32),
                        Box::new(DataType::I32),
                    ]
                {
                    match val {
                        DataType::I32 => Some("insert_i32i32i32tuple_i32".to_string()),
                        _ => {
                            err.unexpected_error(
                                true,
                                Some(format!(
                                    "not yet supported type for map: {:?} -> {:?}",
                                    key, val
                                )),
                                None,
                            );
                            None
                        }
                    }
                } else {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "not yet supported type for map: {:?} -> {:?}",
                            key, val
                        )),
                        None,
                    );
                    None
                }
            }
            _ => {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "not yet supported type for map: {:?} -> {:?}",
                        key, val
                    )),
                    None,
                );
                None
            }
        }
    }
    pub fn get_map_fname(
        &mut self,
        key: DataType,
        val: DataType,
        err: &mut ErrorGen,
    ) -> Option<String> {
        let unsupported_type = format!("Map type not supported yet: {:?} -> {:?}", key, val);
        match key {
            DataType::I32 => match val {
                DataType::I32 => Some("get_i32_i32".to_string()),
                _ => {
                    err.unexpected_error(true, Some(unsupported_type.clone()), None);
                    None
                }
            },
            DataType::Tuple { ty_info } => {
                if ty_info
                    == vec![
                        Box::new(DataType::I32),
                        Box::new(DataType::I32),
                        Box::new(DataType::I32),
                    ]
                {
                    match val {
                        DataType::I32 => Some("get_i32_from_i32i32i32tuple".to_string()),
                        _ => {
                            err.unexpected_error(true, Some(unsupported_type.clone()), None);
                            None
                        }
                    }
                } else {
                    err.unexpected_error(true, Some(unsupported_type.clone()), None);
                    None
                }
            }
            _ => {
                err.unexpected_error(true, Some(unsupported_type.clone()), None);
                None
            }
        }
    }
}

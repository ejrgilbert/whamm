
use crate::parser::types::{DataType, Expr, Value};
// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
use core::panic;
use crate::emitter::report_var_metadata::{ReportVarMetadata, Metadata};




pub fn get_key_unwrapped(key: Expr) -> Value {
    match key {
        Expr::Primitive { val, .. } => val,
        _ => {
            panic!("Error: Expected a primitive value for the key in map get");
        }
    }
}
pub struct MapLibAdapter {
    map_count: i32,
}
impl Default for MapLibAdapter {
    fn default() -> Self {
        Self::new()
    }
}
impl MapLibAdapter {
    pub fn new() -> Self {
        MapLibAdapter {
            map_count: 0,
        }
    }
    pub fn get_map_count(&self) -> i32 {
        self.map_count
    }
    pub fn set_map_count(&mut self, new_count: i32) {
        self.map_count = new_count;
    }
    pub fn increment_map_count(&mut self) {
        self.map_count += 1;
    }
    pub fn put_map_metadata(&mut self, map_id: i32, map_data: Metadata, report_var_metadata: &mut ReportVarMetadata) {
        report_var_metadata.map_metadata.insert(map_id, map_data.clone());
        if !report_var_metadata.all_metadata.insert(map_data) {
            panic!(
                "Error: Metadata already exists for this object - duplicate metadata not allowed"
            );
        }
    }
    pub fn create_local_map_meta(
        &mut self,
        map_id: i32,
        name: String,
        script_id: i32,
        bytecode_loc: i32,
        probe_id: i32,
        report_var_metadata: &mut ReportVarMetadata,
    ) {
        //call the put code for the metadata
        let metadata = Metadata::Local {
            name,
            script_id,
            bytecode_loc,
            probe_id,
        };
        self.put_map_metadata(map_id, metadata, report_var_metadata);
    }
    pub fn create_global_map_meta(&mut self, map_id: i32, name: String, script_id: i32, report_var_metadata: &mut ReportVarMetadata) {
        let metadata = Metadata::Global { name, script_id };
        self.put_map_metadata(map_id, metadata, report_var_metadata);
    }
    pub fn create_local_map(
        &mut self,
        name: String,
        script_id: i32,
        bytecode_loc: i32,
        probe_id: i32,
        map: DataType,
        report_var_metadata: &mut ReportVarMetadata,
    ) -> (String, i32) {
        //create the metadata for the map
        let map_id = self.get_map_count();
        self.increment_map_count();
        self.create_local_map_meta(map_id, name, script_id, bytecode_loc, probe_id, report_var_metadata);

        //create the map based on the types of the key and value in the map
        //"map" is the type of the declaration statement
        match map {
            DataType::Map { key_ty, val_ty } => (self.create_map_insert(*key_ty, *val_ty), map_id),
            _ => {
                panic!("Error: Expected a map type, got something else");
            }
        }
        //returns the map id for this new map
    }
    pub fn create_global_map(
        &mut self,
        name: String,
        script_id: i32,
        map: DataType,
        report_var_metadata: &mut ReportVarMetadata,
    ) -> (String, i32) {
        let map_id = self.get_map_count();
        self.increment_map_count();
        self.create_global_map_meta(map_id, name, script_id, report_var_metadata);

        match map {
            DataType::Map { key_ty, val_ty } => (self.create_map_insert(*key_ty, *val_ty), map_id),
            _ => {
                panic!("Error: Expected a map type, got something else");
            }
        }
    }
    pub fn create_no_meta_map(&mut self, map: DataType) -> (String, i32) {
        let map_id = self.get_map_count();
        self.increment_map_count();
        match map {
            DataType::Map { key_ty, val_ty } => (self.create_map_insert(*key_ty, *val_ty), map_id),
            _ => {
                panic!("Error: Expected a map type, got something else");
            }
        }
    }

    //The stuff that actually calls the emitter stuff
    pub fn create_map_insert(&mut self, key: DataType, val: DataType) -> String {
        match key {
            DataType::I32 => match val {
                DataType::I32 => "create_i32_i32".to_string(),
                DataType::Boolean => "create_i32_bool".to_string(),
                DataType::Str => "create_i32_string".to_string(),
                DataType::Tuple { .. } => "create_i32_tuple".to_string(),
                DataType::Map { .. } => "create_i32_map".to_string(),
                _ => {
                    panic!("Error: Unsupported value type for map: {:?}", val);
                }
            },
            DataType::Str => match val {
                DataType::I32 => "create_string_i32".to_string(),
                DataType::Boolean => "create_string_bool".to_string(),
                DataType::Str => "create_string_string".to_string(),
                DataType::Tuple { .. } => "create_string_tuple".to_string(),
                DataType::Map { .. } => "create_string_map".to_string(),
                _ => {
                    panic!("Error: Unsupported value type for map: {:?}", val);
                }
            },
            DataType::Boolean {} => match val {
                DataType::I32 => "create_bool_i32".to_string(),
                DataType::Boolean => "create_bool_bool".to_string(),
                DataType::Str => "create_bool_string".to_string(),
                DataType::Tuple { .. } => "create_bool_tuple".to_string(),
                DataType::Map { .. } => "create_bool_map".to_string(),
                _ => {
                    panic!("Error: Unsupported value type for map: {:?}", val);
                }
            },
            DataType::Tuple { .. } => match val {
                DataType::I32 => "create_tuple_i32".to_string(),
                DataType::Boolean => "create_tuple_bool".to_string(),
                DataType::Str => "create_tuple_string".to_string(),
                DataType::Tuple { .. } => "create_tuple_tuple".to_string(),
                DataType::Map { .. } => "create_tuple_map".to_string(),
                _ => {
                    panic!("Error: Unsupported value type for map: {:?}", val);
                }
            },
            _ => {
                panic!("Error: Unsupported value type for map: {:?}", val);
            }
        }
    }
    pub fn set_map_insert(&mut self, key: DataType, val: DataType) -> String {
        match key {
            DataType::I32 => match val {
                DataType::I32 => "insert_i32_i32".to_string(),
                _ => {
                    panic!("Error: Not yet supported value type for map");
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
                        DataType::I32 => "insert_map_i32i32i32tuple_i32".to_string(),
                        _ => {
                            panic!("Error: Not yet supported value type for map");
                        }
                    }
                } else {
                    panic!("Error: Not yet supported key type for map");
                }
            }
            _ => {
                panic!("Error: Not yet supported key type for map");
            }
        }
    }
    pub fn create_map_get(&mut self, key: DataType, val: DataType) -> String {
        match key {
            DataType::I32 => match val {
                DataType::I32 => "get_i32_i32".to_string(),
                _ => {
                    panic!("Error: Not yet supported value type for map");
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
                        DataType::I32 => "get_i32_from_i32i32i32tuple".to_string(),
                        _ => {
                            panic!("Error: Not yet supported value type for map");
                        }
                    }
                } else {
                    panic!("Error: Not yet supported key type for map");
                }
            }
            _ => {
                panic!("Error: Not yet supported key type for map");
            }
        }
    }
}

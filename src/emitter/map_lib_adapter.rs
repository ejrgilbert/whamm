use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::DataType;
// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
use crate::emitter::report_var_metadata::{Metadata, ReportVarMetadata};

pub struct MapLibAdapter {
    map_count: i32,
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
    // -------------------------------------
    // Helpers
    // -------------------------------------
    pub fn get_map_count(&self) -> i32 {
        self.map_count
    }
    pub fn set_map_count(&mut self, new_count: i32) {
        self.map_count = new_count;
    }
    pub fn increment_map_count(&mut self) {
        self.map_count += 1;
    }
    pub fn put_map_metadata(
        &mut self,
        map_id: i32,
        map_data: Metadata,
        report_var_metadata: &mut ReportVarMetadata,
    ) -> bool {
        //FALSE MEANS AN ERROR
        report_var_metadata
            .map_metadata
            .insert(map_id, map_data.clone());
        report_var_metadata.all_metadata.insert(map_data)
    }
    // -------------------------------------
    // Map creation fns
    // -------------------------------------

    pub fn create_local_map_meta(
        &mut self,
        map_id: Option<i32>,
        name: String,
        script_id: String,
        bytecode_loc: (i32, i32),
        probe_id: String,
        report_var_metadata: &mut ReportVarMetadata,
    ) -> (bool, i32) {
        //call the put code for the metadata
        let metadata = Metadata::Local {
            name,
            script_id,
            bytecode_loc,
            probe_id,
        };
        let map_id = match map_id {
            Some(id) => id,
            None => {
                let temp = self.get_map_count();
                self.increment_map_count();
                temp
            }
        };
        (
            self.put_map_metadata(map_id, metadata, report_var_metadata),
            map_id,
        )
    }
    pub fn create_global_map_meta(
        &mut self,
        map_id: Option<i32>,
        name: String,
        script_id: String,
        report_var_metadata: &mut ReportVarMetadata,
    ) -> (bool, i32) {
        let metadata = Metadata::Global { name, script_id };
        let map_id = match map_id {
            Some(id) => id,
            None => {
                let temp = self.get_map_count();
                self.increment_map_count();
                temp
            }
        };
        (
            self.put_map_metadata(map_id, metadata, report_var_metadata),
            map_id,
        )
    }
    pub fn create_local_map(
        &mut self,
        name: String,
        script_id: String,
        bytecode_loc: (i32, i32),
        probe_id: String,
        map: DataType,
        report_var_metadata: &mut ReportVarMetadata,
    ) -> Result<(String, i32), Box<WhammError>> {
        //create the metadata for the map
        let (result, map_id) = self.create_local_map_meta(
            None,
            name.clone(),
            script_id,
            bytecode_loc,
            probe_id,
            report_var_metadata,
        );
        if !result {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("Duplicate metadata for map with name: {}", name)),
                None,
            )));
        }

        //create the map based on the types of the key and value in the map
        //"map" is the type of the declaration statement
        match map {
            DataType::Map { key_ty, val_ty } => match self.create_map_fname(*key_ty, *val_ty) {
                Ok(func_name) => Ok((func_name, map_id)),
                Err(e) => Err(e),
            },
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("Non-map with name: {}", name)),
                None,
            ))),
        }
        //returns the map id for this new map
    }
    pub fn create_global_map(
        &mut self,
        name: String,
        script_id: String,
        map: DataType,
        report_var_metadata: &mut ReportVarMetadata,
    ) -> Result<(String, i32), Box<WhammError>> {
        let (result, map_id) =
            self.create_global_map_meta(None, name.clone(), script_id, report_var_metadata);
        if !result {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("Duplicate metadata for map with name: {}", name)),
                None,
            )));
        }
        match map {
            DataType::Map { key_ty, val_ty } => match self.create_map_fname(*key_ty, *val_ty) {
                Ok(func_name) => Ok((func_name, map_id)),
                Err(e) => Err(e),
            },
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("Non-map with name: {}", name)),
                None,
            ))),
        }
    }
    pub fn create_no_meta_map(&mut self, map: DataType) -> Result<(String, i32), Box<WhammError>> {
        //this one isn't abstracted away because no map_metadata needs to be stored
        let map_id = self.get_map_count();
        self.increment_map_count();
        match map {
            DataType::Map { key_ty, val_ty } => match self.create_map_fname(*key_ty, *val_ty) {
                Ok(func_name) => Ok((func_name, map_id)),
                Err(e) => Err(e),
            },
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some("Non-map at no_meta".to_string()),
                None,
            ))),
        }
    }
    // -------------------------------------
    // Get "to_call" for map functions
    // -------------------------------------

    //The stuff that actually calls the emitter stuff
    pub fn create_map_fname(
        &mut self,
        key: DataType,
        val: DataType,
    ) -> Result<String, Box<WhammError>> {
        match key {
            DataType::I32 => match val {
                DataType::I32 => Ok("create_i32_i32".to_string()),
                DataType::Boolean => Ok("create_i32_bool".to_string()),
                DataType::Str => Ok("create_i32_string".to_string()),
                DataType::Tuple { .. } => Ok("create_i32_tuple".to_string()),
                DataType::Map { .. } => Ok("create_i32_map".to_string()),
                _ => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!("Unsupported value type for map: {:?}", val)),
                    None,
                ))),
            },
            DataType::Str => match val {
                DataType::I32 => Ok("create_string_i32".to_string()),
                DataType::Boolean => Ok("create_string_bool".to_string()),
                DataType::Str => Ok("create_string_string".to_string()),
                DataType::Tuple { .. } => Ok("create_string_tuple".to_string()),
                DataType::Map { .. } => Ok("create_string_map".to_string()),
                _ => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!("Unsupported value type for map: {:?}", val)),
                    None,
                ))),
            },
            DataType::Boolean {} => match val {
                DataType::I32 => Ok("create_bool_i32".to_string()),
                DataType::Boolean => Ok("create_bool_bool".to_string()),
                DataType::Str => Ok("create_bool_string".to_string()),
                DataType::Tuple { .. } => Ok("create_bool_tuple".to_string()),
                DataType::Map { .. } => Ok("create_bool_map".to_string()),
                _ => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!("Unsupported value type for map: {:?}", val)),
                    None,
                ))),
            },
            DataType::Tuple { .. } => match val {
                DataType::I32 => Ok("create_tuple_i32".to_string()),
                DataType::Boolean => Ok("create_tuple_bool".to_string()),
                DataType::Str => Ok("create_tuple_string".to_string()),
                DataType::Tuple { .. } => Ok("create_tuple_tuple".to_string()),
                DataType::Map { .. } => Ok("create_tuple_map".to_string()),
                _ => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!("Unsupported value type for map: {:?}", val)),
                    None,
                ))),
            },
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("Unsupported value type for map: {:?}", val)),
                None,
            ))),
        }
    }
    pub fn insert_map_fname(
        &mut self,
        key: DataType,
        val: DataType,
    ) -> Result<String, Box<WhammError>> {
        match &key {
            DataType::I32 => match val {
                DataType::I32 => Ok("insert_i32_i32".to_string()),
                DataType::Str => Ok("insert_i32_string".to_string()),
                _ => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "not yet supported type for map: {:?} -> {:?}",
                        key, val
                    )),
                    None,
                ))),
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
                        DataType::I32 => Ok("insert_i32i32i32tuple_i32".to_string()),
                        _ => Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!(
                                "not yet supported type for map: {:?} -> {:?}",
                                key, val
                            )),
                            None,
                        ))),
                    }
                } else {
                    Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "not yet supported type for map: {:?} -> {:?}",
                            key, val
                        )),
                        None,
                    )))
                }
            }
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "not yet supported type for map: {:?} -> {:?}",
                    key, val
                )),
                None,
            ))),
        }
    }
    pub fn get_map_fname(
        &mut self,
        key: DataType,
        val: DataType,
    ) -> Result<String, Box<WhammError>> {
        let unsupported_type = format!("Map type not supported yet: {:?} -> {:?}", key, val);
        match key {
            DataType::I32 => match val {
                DataType::I32 => Ok("get_i32_i32".to_string()),
                _ => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(unsupported_type.clone()),
                    None,
                ))),
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
                        DataType::I32 => Ok("get_i32_from_i32i32i32tuple".to_string()),
                        _ => Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(unsupported_type.clone()),
                            None,
                        ))),
                    }
                } else {
                    Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(unsupported_type.clone()),
                        None,
                    )))
                }
            }
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(unsupported_type.clone()),
                None,
            ))),
        }
    }
}

// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually 

#[allow(unused_imports)]
use crate::*;
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

//TODO: remove this instance of datatype and instead use the one in parser
#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    I32,
    U32,
    Boolean,
    Null,
    Str,
    Tuple {
        ty_info: Vec<Box<DataType>>,
    },
    Map {
        key_ty: Box<DataType>,
        val_ty: Box<DataType>,
    },
    AssumeGood,
}



// END TO REMOVE

static MAP_COUNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
static MAP_METADATA: Lazy<Mutex<HashMap<i32, Metadata>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get_map_count() -> i32 {
    let count = MAP_COUNT.lock().unwrap();
    *count
}
pub fn set_map_count(new_count: i32) {
    let mut count = MAP_COUNT.lock().unwrap();
    *count = new_count;
}
pub fn increment_map_count() {
    let mut count = MAP_COUNT.lock().unwrap();
    *count += 1;
}

pub enum Metadata {
    global {
        name: String, 
        script_id: i32,
    },
    local {
        name: String,
        script_id: i32,
        bytecode_loc: i32,
        probe_id: i32,
    },
}

pub fn put_map_metadata(map_id: i32, map_data: Metadata) {
    let mut maps = MAP_METADATA.lock().unwrap();
    maps.insert(map_id, map_data);
}
//TODO: instrument this into the bytecode
pub fn create_local_map_meta(map_id: i32, name: String, script_id: i32, bytecode_loc: i32, probe_id: i32){
    //call the put code for the metadata
    let metadata = Metadata::local {
        name: name,
        script_id: script_id,
        bytecode_loc: bytecode_loc,
        probe_id: probe_id,
    };
    put_map_metadata(map_id, metadata);
}
pub fn create_global_meta(map_id: i32, name: String, script_id: i32) {
    let metadata = Metadata::global {
        name: name,
        script_id: script_id,
    };
    put_map_metadata(map_id, metadata);
}

pub fn create_local_map(name: String, script_id: i32, bytecode_loc: i32, probe_id: i32, map: DataType) -> i32 {
    //create the metadata for the map
    let map_id = get_map_count();
    increment_map_count();

    //TODO: this should be called in WASM, not directly called here
    create_local_map_meta(map_id, name, script_id, bytecode_loc, probe_id);

    //create the map based on the types of the key and value in the map
    //"map" is the type of the declaration statement
    match map{
        DataType::Map{key_ty, val_ty} => {
            create_map_insert(map_id, key_ty, val_ty);
            return map_id;
        }
        _ => {
            panic!("Error: Expected a map type, got something else");
        }
    }
    //returns the map id for this new map
}
pub fn create_global_map(name: String, script_id: i32, map: DataType) -> i32{
    let map_id = get_map_count();
    increment_map_count();

    //TODO: this should be called in WASM, not directly here
    create_global_meta(map_id, name, script_id);

    match map{
        DataType::Map { key_ty, val_ty } => {
            create_map_insert(map_id, key_ty, val_ty);
            return map_id;
        }
        _ => {
            panic!("Error: Expected a map type, got something else");
        }
    }
}
//this map does not have its metadata collected -> not output in the final CSV 
pub fn create_no_meta_map(map: DataType) -> i32{
    let map_id = get_map_count();
    increment_map_count();
    match map{
        DataType::Map { key_ty, val_ty } => {
            create_map_insert(map_id, key_ty, val_ty);
            return map_id;
        }
        _ => {
            panic!("Error: Expected a map type, got something else");
        }
    }
}

pub fn create_map_insert(map_id: i32, key: Box<DataType>, val: Box<DataType>) {
    //TODO: call the correct "create" code based on the key and val types - right now just put those function calls here, not the code to add the call in WASM
    match *key {
        DataType::I32 => {
            match *val {
                DataType::I32 => {
                    create_i32_i32(map_id);
                }
                DataType::Boolean => {
                    create_i32_bool(map_id);
                }
                DataType::Str => {
                    create_i32_string(map_id);
                }
                DataType::Tuple{ .. } => {
                    create_i32_tuple(map_id);
                }
                DataType::Map{ .. } => {
                    create_i32_map(map_id);
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            }
        }
        DataType::Str => {
            match *val {
                DataType::I32 => {
                    create_string_i32(map_id);
                }
                DataType::Boolean => {
                    create_string_bool(map_id);
                }
                DataType::Str => {
                    create_string_string(map_id);
                }
                DataType::Tuple{ .. } => {
                    create_string_tuple(map_id);
                }
                DataType::Map{ .. } => {
                    create_string_map(map_id);
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            }
        }
        DataType::Boolean{} => {
            match *val {
                DataType::I32 => {
                    create_bool_i32(map_id);
                }
                DataType::Boolean => {
                    create_bool_bool(map_id);
                }
                DataType::Str => {
                    create_bool_string(map_id);
                }
                DataType::Tuple{ .. } => {
                    create_bool_tuple(map_id);
                }
                DataType::Map{ .. } => {
                    create_bool_map(map_id);
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            }
        }
        DataType::Tuple{ .. } => {
            match *val {
                DataType::I32 => {
                    create_tuple_i32(map_id);
              }
                DataType::Boolean => {
                    create_tuple_bool(map_id);
                }
                DataType::Str => {
                    create_tuple_string(map_id);
                }
                DataType::Tuple{ .. } => {
                    create_tuple_tuple(map_id);
                }
                DataType::Map{ .. } => {
                    create_tuple_map(map_id);
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            }
        }
        _ => {
            panic!("Error: Unsupported key type for map");
        }
    }
}
//take in a MapGet AST node and return the correct function call 
pub fn map_get(map_id: i32, key: Box<DataType>) {
    match *key {
        DataType::I32 => {
            get_i32(map_id, &0);
        }
        DataType::Str => {
            get_string(map_id, &"hello".to_string());
        }
        DataType::Boolean => {
            get_bool_optional(map_id, &true);
        }
        DataType::Tuple{ .. } => {
            get_tuple_optional(map_id, &0);
        }
        DataType::Map{ .. } => {
            get_map_optional(map_id, &0);
        }
        _ => {
            panic!("Error: Unsupported key type for map");
        }
    }

}
//this is the code that knows which functions to call in lib.rs based on what is in the AST
#![allow(unused)]
use crate::parser::types::{Whamm, WhammVisitor, DataType, Expr, Value};
use crate::common::error::{ErrorGen, WhammError};
use std::any::Any;
// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
use once_cell::sync::Lazy; 
use core::panic;
use std::sync::Mutex;
use std::collections::{HashSet, HashMap};
use walrus::ir::{BinaryOp, ExtendedLoad, Instr, InstrSeqId, LoadKind, MemArg};
use walrus::{
    ActiveData, ActiveDataLocation, DataKind, FunctionBuilder, FunctionId, FunctionKind,
    ImportedFunction, InitExpr, InstrSeqBuilder, LocalFunction, MemoryId, ModuleData, ValType,
};

//convert these from being lazy static to being emitter objects
static MAP_COUNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
//this one is from map_id to Metadata
static MAP_METADATA: Lazy<Mutex<HashMap<i32, Metadata>>> = Lazy::new(|| Mutex::new(HashMap::new()));
//this one is from GID to Metadata
static VARIABLE_METADATA: Lazy<Mutex<HashMap<i32, Metadata>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
//metadata for any variable or map should be unique, so make a hashset and add to it as you go
static ALL_METADATA: Lazy<Mutex<HashSet<Metadata>>> = Lazy::new(|| Mutex::new(HashSet::new()));



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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Metadata {
    Global {
        name: String,
        script_id: i32,
    },
    Local {
        name: String,
        script_id: i32,
        bytecode_loc: i32,
        probe_id: i32,
    },
}

pub fn put_map_metadata(map_id: i32, map_data: Metadata) {
    let mut maps = MAP_METADATA.lock().unwrap();
    maps.insert(map_id, map_data);
    let mut all_metadata = ALL_METADATA.lock().unwrap();
    if !all_metadata.insert(map_data) {
        panic!("Error: Metadata already exists for this object - duplicate metadata not allowed");
    }
}
//TODO: instrument this into the bytecode
pub fn create_local_map_meta(
    map_id: i32,
    name: String,
    script_id: i32,
    bytecode_loc: i32,
    probe_id: i32,
) {
    //call the put code for the metadata
    let metadata = Metadata::Local {
        name: name,
        script_id: script_id,
        bytecode_loc: bytecode_loc,
        probe_id: probe_id,
    };
    put_map_metadata(map_id, metadata);
}
pub fn create_global_meta(map_id: i32, name: String, script_id: i32) {
    let metadata = Metadata::Global {
        name: name,
        script_id: script_id,
    };
    put_map_metadata(map_id, metadata);
}

pub fn create_local_map(
    name: String,
    script_id: i32,
    bytecode_loc: i32,
    probe_id: i32,
    map: DataType,
) -> i32 {
    //create the metadata for the map
    let map_id = get_map_count();
    increment_map_count();

    //TODO: this should be called in WASM, not directly called here
    create_local_map_meta(map_id, name, script_id, bytecode_loc, probe_id);

    //create the map based on the types of the key and value in the map
    //"map" is the type of the declaration statement
    match map {
        DataType::Map { key_ty, val_ty } => {
            create_map_insert(map_id, key_ty, val_ty);
            return map_id;
        }
        _ => {
            panic!("Error: Expected a map type, got something else");
        }
    }
    //returns the map id for this new map
}
pub fn create_global_map(name: String, script_id: i32, map: DataType) -> i32 {
    let map_id = get_map_count();
    increment_map_count();

    //TODO: this should be called in WASM, not directly here
    create_global_meta(map_id, name, script_id);

    match map {
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
pub fn create_no_meta_map(map: DataType) -> i32 {
    let map_id = get_map_count();
    increment_map_count();
    match map {
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
        DataType::I32 => match *val {
            DataType::I32 => {
                create_i32_i32(map_id);
            }
            DataType::Boolean => {
                create_i32_bool(map_id);
            }
            DataType::Str => {
                create_i32_string(map_id);
            }
            DataType::Tuple { .. } => {
                create_i32_tuple(map_id);
            }
            DataType::Map { .. } => {
                create_i32_map(map_id);
            }
            _ => {
                panic!("Error: Unsupported value type for map");
            }
        },
        DataType::Str => match *val {
            DataType::I32 => {
                create_string_i32(map_id);
            }
            DataType::Boolean => {
                create_string_bool(map_id);
            }
            DataType::Str => {
                create_string_string(map_id);
            }
            DataType::Tuple { .. } => {
                create_string_tuple(map_id);
            }
            DataType::Map { .. } => {
                create_string_map(map_id);
            }
            _ => {
                panic!("Error: Unsupported value type for map");
            }
        },
        DataType::Boolean {} => match *val {
            DataType::I32 => {
                create_bool_i32(map_id);
            }
            DataType::Boolean => {
                create_bool_bool(map_id);
            }
            DataType::Str => {
                create_bool_string(map_id);
            }
            DataType::Tuple { .. } => {
                create_bool_tuple(map_id);
            }
            DataType::Map { .. } => {
                create_bool_map(map_id);
            }
            _ => {
                panic!("Error: Unsupported value type for map");
            }
        },
        DataType::Tuple { .. } => match *val {
            DataType::I32 => {
                create_tuple_i32(map_id);
            }
            DataType::Boolean => {
                create_tuple_bool(map_id);
            }
            DataType::Str => {
                create_tuple_string(map_id);
            }
            DataType::Tuple { .. } => {
                create_tuple_tuple(map_id);
            }
            DataType::Map { .. } => {
                create_tuple_map(map_id);
            }
            _ => {
                panic!("Error: Unsupported value type for map");
            }
        },
        _ => {
            panic!("Error: Unsupported key type for map");
        }
    }
}
//take in a MapGet AST node and return the correct function call
// EXPECT THE KEY EXPR TO ALREADY BE FOLDED TO A PRIMATIVE VALUE - also figure out what the lifetime of the return should be
pub fn get_key_unwrapped(key: Expr) -> Box<dyn Any> {
    return match key {
        Expr::Primitive { val, .. } => {
            match val {
                Value::Integer { val , ..} => Box::new(val),
                Value::Boolean { val, .. } => Box::new(val),
                Value::Str { val, .. } => Box::new(val),
                Value::Tuple { ty, vals } => {
                    if ty
                        == (DataType::Tuple {
                            ty_info: vec![
                                Box::new(DataType::I32),
                                Box::new(DataType::I32),
                                Box::new(DataType::I32),
                            ],
                        })
                    {
                        Box::new((vals[0].clone(), vals[1].clone(), vals[2].clone()))
                    } else {
                        //This won't work yet because converting the vector of expr to a tuple type isn't supported yet
                        panic!("Error: Unsupported tuple type for map key");
                    }
                }
            }
        }
        _ => {
            panic!("Error: Expected a primitive value for the key in map get");
        }
    };
}
pub fn map_get(map_id: i32, key: Expr, map_type: DataType) {
    //first, get the key value
    let my_key = get_key_unwrapped(key);
    match map_type {
        DataType::Map { key_ty, val_ty } => {
            match *val_ty {
                //TODO: make these walrus telling it to call this
                DataType::I32 => {
                    let i32i32i32tup = DataType::Tuple {
                        ty_info: vec![
                            Box::new(DataType::I32),
                            Box::new(DataType::I32),
                            Box::new(DataType::I32),
                        ],
                    };
                    if *key_ty == i32i32i32tup {
                        if let Some(my_key) = my_key.downcast_ref::<(i32, i32, i32)>() {
                            get_i32_from_i32i32i32tuple(map_id, my_key.0, my_key.1, my_key.2);
                        }
                    } else {
                        get_i32(map_id, my_key.as_ref());
                    }
                }
                DataType::Boolean => {
                    get_bool(map_id, my_key.as_ref());
                }
                DataType::Str => {
                    get_string(map_id, my_key.as_ref());
                }
                DataType::Tuple { .. } => {
                    get_tuple(map_id, my_key.as_ref());
                }
                DataType::Map { .. } => {
                    get_map(map_id, my_key.as_ref());
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            }
        }
        _ => {
            panic!("Error: Expected Map type, got {:?}", map_type);
        }
    }
}


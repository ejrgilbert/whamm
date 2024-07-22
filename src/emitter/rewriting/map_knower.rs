#![allow(unused)]
use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::{DataType, Expr, Value, Whamm, WhammVisitor};
use std::any::Any;
use std::hash::Hash;
// //this is the code that knows which functions to call in lib.rs based on what is in the AST -> will be in emitter folder eventually
use core::panic;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use walrus::ir::{BinaryOp, ExtendedLoad, Instr, InstrSeqId, LoadKind, MemArg};
use walrus::{
    ActiveData, ActiveDataLocation, DataKind, FunctionBuilder, FunctionId, FunctionKind,
    ImportedFunction, InitExpr, InstrSeqBuilder, LocalFunction, MemoryId, ModuleData, ValType,
};
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
pub fn get_key_unwrapped(key: Expr) -> Value {
    return match key {
        Expr::Primitive { val, .. } => val,
        _ => {
            panic!("Error: Expected a primitive value for the key in map get");
        }
    };
}
pub struct MapKnower {
    map_count: i32,
    map_metadata: HashMap<i32, Metadata>,
    variable_metadata: HashMap<usize, Metadata>,
    all_metadata: HashSet<Metadata>,
}
impl MapKnower {
    pub fn new() -> Self {
        MapKnower {
            map_count: 0,
            map_metadata: HashMap::new(),
            variable_metadata: HashMap::new(),
            all_metadata: HashSet::new(),
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
    pub fn put_map_metadata(&mut self, map_id: i32, map_data: Metadata) {
        self.map_metadata.insert(map_id, map_data.clone());
        if !self.all_metadata.insert(map_data) {
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
    ) {
        //call the put code for the metadata
        let metadata = Metadata::Local {
            name: name,
            script_id: script_id,
            bytecode_loc: bytecode_loc,
            probe_id: probe_id,
        };
        self.put_map_metadata(map_id, metadata);
    }
    pub fn create_global_map_meta(&mut self, map_id: i32, name: String, script_id: i32) {
        let metadata = Metadata::Global {
            name: name,
            script_id: script_id,
        };
        self.put_map_metadata(map_id, metadata);
    }
    pub fn create_local_map(
        &mut self,
        name: String,
        script_id: i32,
        bytecode_loc: i32,
        probe_id: i32,
        map: DataType,
    ) -> (String, i32) {
        //create the metadata for the map
        let map_id = self.get_map_count();
        self.increment_map_count();
        self.create_local_map_meta(map_id, name, script_id, bytecode_loc, probe_id);

        //create the map based on the types of the key and value in the map
        //"map" is the type of the declaration statement
        match map {
            DataType::Map { key_ty, val_ty } => {
                return (self.create_map_insert(*key_ty, *val_ty), map_id);
            }
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
    ) -> (String, i32) {
        let map_id = self.get_map_count();
        self.increment_map_count();
        self.create_global_map_meta(map_id, name, script_id);

        match map {
            DataType::Map { key_ty, val_ty } => {
                return (self.create_map_insert(*key_ty, *val_ty), map_id);
            }
            _ => {
                panic!("Error: Expected a map type, got something else");
            }
        }
    }
    pub fn create_no_meta_map(&mut self, map: DataType) -> (String, i32) {
        let map_id = self.get_map_count();
        self.increment_map_count();
        match map {
            DataType::Map { key_ty, val_ty } => {
                return (self.create_map_insert(*key_ty, *val_ty), map_id);
            }
            _ => {
                panic!("Error: Expected a map type, got something else");
            }
        }
    }

    //The stuff that actually calls the emitter stuff
    pub fn create_map_insert(&mut self, key: DataType, val: DataType) -> String {
        match key {
            DataType::I32 => match val {
                DataType::I32 => {
                    return "create_i32_i32".to_string();
                }
                DataType::Boolean => {
                    return "create_i32_bool".to_string();
                }
                DataType::Str => {
                    return "create_i32_string".to_string();
                }
                DataType::Tuple { .. } => {
                    return "create_i32_tuple".to_string();
                }
                DataType::Map { .. } => {
                    return "create_i32_map".to_string();
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            },
            DataType::Str => match val {
                DataType::I32 => {
                    return "create_string_i32".to_string();
                }
                DataType::Boolean => {
                    return "create_string_bool".to_string();
                }
                DataType::Str => {
                    return "create_string_string".to_string();
                }
                DataType::Tuple { .. } => {
                    return "create_string_tuple".to_string();
                }
                DataType::Map { .. } => {
                    return "create_string_map".to_string();
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            },
            DataType::Boolean {} => match val {
                DataType::I32 => {
                    return "create_bool_i32".to_string();
                }
                DataType::Boolean => {
                    return "create_bool_bool".to_string();
                }
                DataType::Str => {
                    return "create_bool_string".to_string();
                }
                DataType::Tuple { .. } => {
                    return "create_bool_tuple".to_string();
                }
                DataType::Map { .. } => {
                    return "create_bool_map".to_string();
                }
                _ => {
                    panic!("Error: Unsupported value type for map");
                }
            },
            DataType::Tuple { .. } => match val {
                DataType::I32 => {
                    return "create_tuple_i32".to_string();
                }
                DataType::Boolean => {
                    return "create_tuple_bool".to_string();
                }
                DataType::Str => {
                    return "create_tuple_string".to_string();
                }
                DataType::Tuple { .. } => {
                    return "create_tuple_tuple".to_string();
                }
                DataType::Map { .. } => {
                    return "create_tuple_map".to_string();
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
    pub fn set_map_insert(&mut self, key: DataType, val: DataType) -> String {
        match key {
            DataType::I32 => match val {
                DataType::I32 => {
                    return "insert_i32_i32".to_string();
                }
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
                        DataType::I32 => {
                            return "insert_map_i32i32i32tuple_i32".to_string();
                        }
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
                DataType::I32 => {
                    return "get_i32_i32".to_string();
                }
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
                        DataType::I32 => {
                            return "get_i32_from_i32i32i32tuple".to_string();
                        }
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
    //not sure how this one will work as an outside fn that doesn't emit
    // pub fn map_get(map_id: i32, key: Expr, map_type: DataType) {
    //     //first, get the key value
    //     let my_key = get_key_unwrapped(key);
    //     match map_type {
    //         DataType::Map { key_ty, val_ty } => {
    //             match *val_ty {
    //                 //TODO: make these walrus telling it to call this
    //                 DataType::I32 => {
    //                     let i32i32i32tup = DataType::Tuple {
    //                         ty_info: vec![
    //                             Box::new(DataType::I32),
    //                             Box::new(DataType::I32),
    //                             Box::new(DataType::I32),
    //                         ],
    //                     };
    //                     if *key_ty == i32i32i32tup {
    //                         if let Some(my_key) = my_key.downcast_ref::<(i32, i32, i32)>() {
    //                             get_i32_from_i32i32i32tuple(map_id, my_key.0, my_key.1, my_key.2);
    //                         }
    //                     } else {
    //                         get_i32(map_id, my_key.as_ref());
    //                     }
    //                 }
    //                 DataType::Boolean => {
    //                     get_bool(map_id, my_key.as_ref());
    //                 }
    //                 DataType::Str => {
    //                     get_string(map_id, my_key.as_ref());
    //                 }
    //                 DataType::Tuple { .. } => {
    //                     get_tuple(map_id, my_key.as_ref());
    //                 }
    //                 DataType::Map { .. } => {
    //                     get_map(map_id, my_key.as_ref());
    //                 }
    //                 _ => {
    //                     panic!("Error: Unsupported value type for map");
    //                 }
    //             }
    //         }
    //         _ => {
    //             panic!("Error: Expected Map type, got {:?}", map_type);
    //         }
    //     }
    // }
    // pub fn set_wasm_app(&mut self, app: walrus::Module) {
    //     self.wasm_app = app;
    // }
}

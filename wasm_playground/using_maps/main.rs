//TODO: impliment tuples as keys when they don't contain a map 


//library functions for maps in Whamm
#![allow(non_camel_case_types)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::any::Any;

use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

static MY_MAPS: Lazy<Mutex<HashMap<String, AnyMap>>> = Lazy::new(|| Mutex::new(HashMap::new()));


//this should initialize a map of maps -> from string (name) to any type of map

//strings, i32, maps, tuples, bool - all variations
#[derive(Debug, PartialEq, Eq)]
pub enum AnyMap {
    i32_i32_Map(HashMap<i32, i32>),
    i32_string_Map(HashMap<i32, String>),
    i32_map_Map(HashMap<i32, Box<AnyMap>>),
    i32_tuple_Map(HashMap<i32, Box<TupleVariant>>),
    i32_bool_Map(HashMap<i32, bool>),
    string_i32_Map(HashMap<String, i32>),
    string_string_Map(HashMap<String, String>),
    string_map_Map(HashMap<String, Box<AnyMap>>),
    string_tuple_Map(HashMap<String, Box<TupleVariant>>),
    string_bool_Map(HashMap<String, bool>),
    tuple_i32_Map(HashMap<Box<TupleVariant>, i32>),
    tuple_string_Map(HashMap<Box<TupleVariant>, String>),
    tuple_map_Map(HashMap<Box<TupleVariant>, Box<AnyMap>>),
    tuple_tuple_Map(HashMap<Box<TupleVariant>, Box<TupleVariant>>),
    tuple_bool_Map(HashMap<Box<TupleVariant>, bool>),
    bool_i32_Map(HashMap<bool, i32>),
    bool_string_Map(HashMap<bool, String>),
    bool_map_Map(HashMap<bool, Box<AnyMap>>),
    bool_tuple_Map(HashMap<bool, Box<TupleVariant>>),
    bool_bool_Map(HashMap<bool, bool>),
}
trait MapOperations {
    fn insert(&mut self, key: Box<dyn Any>, value: Box<dyn Any>) -> bool;
    fn get_i32(&self, key: &dyn Any) -> Option<&i32>;
    fn get_string(&self, key: &dyn Any) -> Option<&String>;
    fn get_tuple(&self, key: &dyn Any) -> Option<&Box<TupleVariant>>;
    fn get_map(&self, key: &dyn Any) -> Option<&Box<AnyMap>>;
    fn get_bool(&self, key: &dyn Any) -> Option<&bool>;
}

impl MapOperations for AnyMap {
    fn insert(&mut self, key: Box<dyn Any>, value: Box<dyn Any>) -> bool {
        match self {
            AnyMap::i32_i32_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<i32>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::i32_string_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<String>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::i32_map_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<Box<AnyMap>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::i32_tuple_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<Box<TupleVariant>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::i32_bool_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<bool>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::string_i32_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<i32>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::string_string_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<String>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::string_map_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<Box<AnyMap>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::string_tuple_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<Box<TupleVariant>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::string_bool_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<bool>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::tuple_i32_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<Box<TupleVariant>>(), value.downcast::<i32>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::tuple_string_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<Box<TupleVariant>>(), value.downcast::<String>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::tuple_map_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<Box<TupleVariant>>(), value.downcast::<Box<AnyMap>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::tuple_tuple_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<Box<TupleVariant>>(), value.downcast::<Box<TupleVariant>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::tuple_bool_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<Box<TupleVariant>>(), value.downcast::<bool>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::bool_i32_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<i32>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::bool_string_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<String>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::bool_map_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<Box<AnyMap>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::bool_tuple_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<Box<TupleVariant>>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
            AnyMap::bool_bool_Map(ref mut map) => {
                if let(Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<bool>()) {
                    map.insert(*key, *value);
                    return true;
                }
            },
        }
        //only false if it failed to insert something -> means that the key and/or value were not the right type
        false
    }
    fn get_i32(&self, key: &dyn Any) -> Option<&i32> {
        match self {
            AnyMap::i32_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key);
                }
            },
            AnyMap::string_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key);
                }
            },
            AnyMap::tuple_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key);
                }
            },
            AnyMap::bool_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key);
                }
            },
            _ => {}
        }
        None
    }
    fn get_string(&self, key: &dyn Any) -> Option<&String>{
        match self {
            AnyMap::i32_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key);
                }
            },
            AnyMap::string_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key);
                }
            },
            AnyMap::tuple_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key);
                }
            },
            AnyMap::bool_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key);
                }
            },
            _ => {}
        }
        None
    }
    fn get_tuple(&self, key: &dyn Any) -> Option<&Box<TupleVariant>> {
        match self {
            AnyMap::i32_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key);
                }
            },
            AnyMap::string_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key);
                }
            },
            AnyMap::tuple_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key);
                }
            },
            AnyMap::bool_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key);
                }
            },
            _ => {}
        }
        None
    }
    fn get_map(&self, key: &dyn Any) -> Option<&Box<AnyMap>> {
        match self {
            AnyMap::i32_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key);
                }
            },
            AnyMap::string_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key);
                }
            },
            AnyMap::tuple_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key);
                }
            },
            AnyMap::bool_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key);
                }
            },
            _ => {}
        }
        None
    }
    fn get_bool(&self, key: &dyn Any) -> Option<&bool>{
        match self {
            AnyMap::i32_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key);
                }
            },
            AnyMap::string_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key);
                }
            },
            AnyMap::tuple_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key);
                }
            },
            AnyMap::bool_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key);
                }
            },
            _ => {}
        }
        None
    }
}
//have to support strings, i32, maps, tuples, and bool - all variations (yet again)
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TupleVariant {
    i32_i32(i32, i32),
    i32_string(i32, String),
    // i32_map(i32, Box<AnyMap>),
    i32_tuple(i32, Box<TupleVariant>),
    i32_bool(i32, bool),
    string_i32(String, i32),
    string_string(String, String),
    // string_map(String, Box<AnyMap>),
    string_tuple(String, Box<TupleVariant>),
    string_bool(String, bool),
    // map_i32(Box<AnyMap>, i32),
    // map_string(Box<AnyMap>, String),
    // map_map(Box<AnyMap>, Box<AnyMap>),
    // map_tuple(Box<AnyMap>, Box<TupleVariant>),
    // map_bool(Box<AnyMap>, bool),
    tuple_i32(Box<TupleVariant>, i32),
    tuple_string(Box<TupleVariant>, String),
    // tuple_map(Box<TupleVariant>, Box<AnyMap>),
    tuple_tuple(Box<TupleVariant>, Box<TupleVariant>),
    tuple_bool(Box<TupleVariant>, bool),
    bool_i32(bool, i32),
    bool_string(bool, String),
    // bool_map(bool, Box<AnyMap>),
    bool_tuple(bool, Box<TupleVariant>),
    bool_bool(bool, bool),
}
//to make a map to/from a map or string they have to boxed

//functions for creating a map
pub fn create_i32_i32(name: String){
    MY_MAPS.lock().unwrap().insert(name, AnyMap::i32_i32_Map(HashMap::new()));
}

//functions for inserting into a map
pub fn insert_i32_i32(name: String, key: i32, value: i32) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        any_map.insert(Box::new(key), Box::new(value));
        return true;
    }
    false
}

//functions for getting from a map
pub fn get_i32(name: String, key: &dyn Any) -> Option<i32>{
    if let Some(boxed_map) = MY_MAPS.lock().unwrap().get(&name) {
        return match boxed_map.get_i32(key) {
            Some(value) => Some(*value),
            None => None,
        }
    }
    None
}

fn main() {
    create_i32_i32( "test".to_string());
    insert_i32_i32("test".to_string(), 1, 2);
    insert_i32_i32("test".to_string(), 2, 3);
    println!("{:?}", get_i32("test".to_string(), &1));
    let a = get_i32("test".to_string(), &1);
    println!("{:?}", get_i32("test".to_string(), &a.unwrap()));
}
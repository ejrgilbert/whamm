//library functions for maps in Whamm
#![allow(non_camel_case_types)]
#![allow(dead_code)]
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::slice;

use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

static MY_MAPS: Lazy<Mutex<HashMap<i32, AnyMap>>> = Lazy::new(|| Mutex::new(HashMap::new()));
thread_local! {
    static METADATA_HEADER: RefCell<(u32, u32)> = RefCell::new((0,0));
}

//this should initialize a map of maps -> from string (name) to any type of map

//strings, i32, maps, tuples, bool - all variations

#[derive(Debug, PartialEq, Eq, Clone)]

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

pub trait MapOperations {
    fn insert(&mut self, key: Box<dyn Any>, value: Box<dyn Any>) -> bool;
    fn get_i32(&self, key: &dyn Any) -> Option<i32>;
    fn get_string(&self, key: &dyn Any) -> Option<String>;
    fn get_tuple(&self, key: &dyn Any) -> Option<Box<TupleVariant>>;
    fn get_map_mut(&mut self, key: &dyn Any) -> Option<&mut AnyMap>;
    fn get_map(&self, key: &dyn Any) -> Option<Box<AnyMap>>;
    fn get_bool(&self, key: &dyn Any) -> Option<bool>;
    fn dump_map(&self) -> String;
}
impl MapOperations for AnyMap {
    #[no_mangle]
    fn insert(&mut self, key: Box<dyn Any>, value: Box<dyn Any>) -> bool {
        match self {
            AnyMap::i32_i32_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<i32>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::i32_string_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<String>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::i32_map_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<AnyMap>()) {
                    map.insert(*key, value);
                    return true;
                }
            }
            AnyMap::i32_tuple_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) =
                    (key.downcast::<i32>(), value.downcast::<TupleVariant>())
                {
                    map.insert(*key, value);
                    return true;
                }
            }
            AnyMap::i32_bool_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<i32>(), value.downcast::<bool>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::string_i32_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<i32>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::string_string_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<String>())
                {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::string_map_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<AnyMap>())
                {
                    map.insert(*key, value);
                    return true;
                }
            }
            AnyMap::string_tuple_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) =
                    (key.downcast::<String>(), value.downcast::<TupleVariant>())
                {
                    map.insert(*key, value);
                    return true;
                }
            }
            AnyMap::string_bool_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<String>(), value.downcast::<bool>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::tuple_i32_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) =
                    (key.downcast::<TupleVariant>(), value.downcast::<i32>())
                {
                    map.insert(key, *value);
                    return true;
                }
            }
            AnyMap::tuple_string_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) =
                    (key.downcast::<TupleVariant>(), value.downcast::<String>())
                {
                    map.insert(key, *value);
                    return true;
                }
            }
            AnyMap::tuple_map_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) =
                    (key.downcast::<TupleVariant>(), value.downcast::<AnyMap>())
                {
                    map.insert(key, value);
                    return true;
                }
            }
            AnyMap::tuple_tuple_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (
                    key.downcast::<TupleVariant>(),
                    value.downcast::<TupleVariant>(),
                ) {
                    map.insert(key, value);
                    return true;
                }
            }
            AnyMap::tuple_bool_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) =
                    (key.downcast::<TupleVariant>(), value.downcast::<bool>())
                {
                    map.insert(key, *value);
                    return true;
                }
            }
            AnyMap::bool_i32_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<i32>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::bool_string_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<String>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
            AnyMap::bool_map_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<AnyMap>()) {
                    map.insert(*key, value);
                    return true;
                }
            }
            AnyMap::bool_tuple_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) =
                    (key.downcast::<bool>(), value.downcast::<TupleVariant>())
                {
                    map.insert(*key, value);
                    return true;
                }
            }
            AnyMap::bool_bool_Map(ref mut map) => {
                if let (Ok(key), Ok(value)) = (key.downcast::<bool>(), value.downcast::<bool>()) {
                    map.insert(*key, *value);
                    return true;
                }
            }
        }
        //only false if it failed to insert something -> means that the key and/or value were not the right type
        false
    }
    fn get_i32(&self, key: &dyn Any) -> Option<i32> {
        match self {
            AnyMap::i32_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key).copied();
                }
            }
            AnyMap::string_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key).copied();
                }
            }
            AnyMap::tuple_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key).copied();
                }
            }
            AnyMap::bool_i32_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key).copied();
                }
            }
            _ => {}
        }
        None
    }
    fn get_string(&self, key: &dyn Any) -> Option<String> {
        match self {
            AnyMap::i32_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::string_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::tuple_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::bool_string_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key).cloned();
                }
            }
            _ => {}
        }
        None
    }
    fn get_tuple(&self, key: &dyn Any) -> Option<Box<TupleVariant>> {
        match self {
            AnyMap::i32_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::string_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::tuple_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::bool_tuple_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key).cloned();
                }
            }
            _ => {}
        }
        None
    }
    fn get_map_mut(&mut self, key: &dyn Any) -> Option<&mut AnyMap> {
        match self {
            AnyMap::i32_map_Map(ref mut map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get_mut(key).map(|box_any_map| &mut **box_any_map);
                }
            }
            AnyMap::string_map_Map(ref mut map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get_mut(key).map(|box_any_map| &mut **box_any_map);
                }
            }
            AnyMap::tuple_map_Map(ref mut map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get_mut(key).map(|box_any_map| &mut **box_any_map);
                }
            }
            AnyMap::bool_map_Map(ref mut map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get_mut(key).map(|box_any_map| &mut **box_any_map);
                }
            }
            _ => {}
        }
        None
    }
    fn get_map(&self, key: &dyn Any) -> Option<Box<AnyMap>> {
        match self {
            AnyMap::i32_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::string_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::tuple_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::bool_map_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key).cloned();
                }
            }
            _ => {}
        }
        None
    }

    fn get_bool(&self, key: &dyn Any) -> Option<bool> {
        match self {
            AnyMap::i32_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<i32>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::string_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<String>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::tuple_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<Box<TupleVariant>>() {
                    return map.get(key).cloned();
                }
            }
            AnyMap::bool_bool_Map(ref map) => {
                if let Some(key) = key.downcast_ref::<bool>() {
                    return map.get(key).cloned();
                }
            }
            _ => {}
        }
        None
    }
    fn dump_map(&self) -> String {
        match self {
            AnyMap::i32_i32_Map(ref map) => {
                let mut result = String::new();
                for (key, value) in map.iter() {
                    result.push_str(&format!("{}->{};", key, value));
                }
                if result.is_empty() {
                    result = "Empty map".to_string();
                } else {
                    result.pop();
                }
                result
            }
            AnyMap::tuple_i32_Map(ref map) => {
                let mut result = String::new();
                for (key, value) in map.iter() {
                    result.push_str(&format!("{}->{};", key.dump_tuple(), value));
                }
                if result.is_empty() {
                    result = "Empty map".to_string();
                } else {
                    result.pop();
                }
                result
            }
            AnyMap::i32_string_Map(ref map) => {
                println!("DEBUG: dumping i32_string_Map...");
                let mut result = String::new();
                for (key, value) in map.iter() {
                    result.push_str(&format!("{}->{};", key, value));
                }
                if result.is_empty() {
                    result = "Empty map".to_string();
                } else {
                    result.pop();
                }
                result
            }
            _ => "Not implemented".to_string(),
        }
    }
}
//have to support strings, i32, maps, tuples, and bool - all variations (yet again)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    i32_i32_i32(i32, i32, i32),
}

impl TupleVariant {
    pub fn dump_tuple(&self) -> String {
        match self {
            TupleVariant::i32_i32_i32(a, b, c) => {
                format!("({}, {}, {})", a, b, c)
            }
            _ => "Not implemented".to_string(),
        }
    }
}
//to make a map to/from a map or string they have to boxed

//functions for creating a map
#[no_mangle]
pub fn create_i32_i32(name: i32) {
    println!("DEBUG: creating i32_i32 map with name '{name}'");
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::i32_i32_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_i32_string(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::i32_string_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_i32_map(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::i32_map_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_i32_tuple(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::i32_tuple_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_i32_bool(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::i32_bool_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_string_i32(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::string_i32_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_string_string(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::string_string_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_string_map(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::string_map_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_string_tuple(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::string_tuple_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_string_bool(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::string_bool_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_tuple_i32(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::tuple_i32_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_tuple_string(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::tuple_string_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_tuple_map(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::tuple_map_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_tuple_tuple(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::tuple_tuple_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_tuple_bool(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::tuple_bool_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_bool_i32(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::bool_i32_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_bool_string(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::bool_string_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_bool_map(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::bool_map_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_bool_tuple(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::bool_tuple_Map(HashMap::new()));
}
#[no_mangle]
pub fn create_bool_bool(name: i32) {
    MY_MAPS
        .lock()
        .unwrap()
        .insert(name, AnyMap::bool_bool_Map(HashMap::new()));
}

//functions for inserting into a map - use the matching done in the insert function for AnyMap enum
#[no_mangle]
pub fn insert_i32_i32_inner(name: i32, key: i32, value: i32) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_i32_string_inner(name: i32, key: i32, value: String) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_i32_map_inner(name: i32, key: i32, value: AnyMap) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_i32_tuple_inner(name: i32, key: i32, value: TupleVariant) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_i32_bool_inner(name: i32, key: i32, value: bool) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_string_i32_inner(name: i32, key: String, value: i32) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_string_string_inner(name: i32, key: String, value: String) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_string_map_inner(name: i32, key: String, value: AnyMap) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
pub fn insert_string_tuple_inner(name: i32, key: String, value: TupleVariant) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_string_bool_inner(name: i32, key: String, value: bool) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_tuple_i32_inner(name: i32, key: TupleVariant, value: i32) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_tuple_string_inner(name: i32, key: TupleVariant, value: String) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_tuple_map_inner(name: i32, key: TupleVariant, value: AnyMap) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_tuple_tuple_inner(name: i32, key: TupleVariant, value: TupleVariant) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_tuple_bool_inner(name: i32, key: TupleVariant, value: bool) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_bool_i32_inner(name: i32, key: bool, value: i32) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_bool_string_inner(name: i32, key: bool, value: String) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_bool_map_inner(name: i32, key: bool, value: AnyMap) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_bool_tuple_inner(name: i32, key: bool, value: TupleVariant) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}
#[no_mangle]
pub fn insert_bool_bool_inner(name: i32, key: bool, value: bool) -> bool {
    if let Some(any_map) = MY_MAPS.lock().unwrap().get_mut(&name) {
        return any_map.insert(Box::new(key), Box::new(value));
    }
    false
}

//functions for getting from a map
#[no_mangle]
pub fn get_i32_optional(name: i32, key: &dyn Any) -> Option<i32> {
    if let Some(boxed_map) = MY_MAPS.lock().unwrap().get(&name) {
        return boxed_map.get_i32(key);
    }
    None
}
#[no_mangle]
pub fn get_string_optional(name: i32, key: &dyn Any) -> Option<String> {
    if let Some(boxed_map) = MY_MAPS.lock().unwrap().get(&name) {
        return boxed_map.get_string(key).map(|value| value.clone());
    }
    None
}
#[no_mangle]
pub fn get_tuple_optional(name: i32, key: &dyn Any) -> Option<Box<TupleVariant>> {
    if let Some(boxed_map) = MY_MAPS.lock().unwrap().get(&name) {
        return boxed_map.get_tuple(key).map(|value| value.clone());
    }
    None
}
#[no_mangle]
pub fn get_map_mut<'a>(
    my_maps: &'a mut HashMap<i32, AnyMap>,
    name: i32,
    key: &dyn Any,
) -> Option<&'a mut AnyMap> {
    if let Some(boxed_map) = my_maps.get_mut(&name) {
        return match boxed_map.get_map_mut(key) {
            Some(value) => Some(value),
            None => None,
        };
    }
    None
}
#[no_mangle]
pub fn get_map_optional(name: i32, key: &dyn Any) -> Option<AnyMap> {
    if let Some(boxed_map) = MY_MAPS.lock().unwrap().get(&name) {
        return boxed_map.get_map(key).map(|value| *value.clone());
    }
    None
}
#[no_mangle]
pub fn get_bool_optional(name: i32, key: &dyn Any) -> Option<bool> {
    if let Some(boxed_map) = MY_MAPS.lock().unwrap().get(&name) {
        return boxed_map.get_bool(key).map(|value| value);
    }
    None
}
//make public getters that unwrap these optional versions
#[no_mangle]
pub fn get_i32(name: i32, key: &dyn Any) -> i32 {
    match get_i32_optional(name, key) {
        Some(value) => value,
        None => panic!("Key not found in map"),
    }
}
#[no_mangle]
pub fn get_string(name: i32, key: &dyn Any) -> String {
    match get_string_optional(name, key) {
        Some(value) => value,
        None => panic!("Key not found in map"),
    }
}
#[no_mangle]
pub fn get_tuple(name: i32, key: &dyn Any) -> Box<TupleVariant> {
    match get_tuple_optional(name, key) {
        Some(value) => value,
        None => panic!("Key not found in map"),
    }
}
#[no_mangle]
pub fn get_map(name: i32, key: &dyn Any) -> AnyMap {
    match get_map_optional(name, key) {
        Some(value) => value,
        None => panic!("Key not found in map"),
    }
}
#[no_mangle]
pub fn get_bool(name: i32, key: &dyn Any) -> bool {
    match get_bool_optional(name, key) {
        Some(value) => value,
        None => panic!("Key not found in map"),
    }
}
#[no_mangle]
pub fn create_map_i32i32i32tuple_i32(name: i32) {
    create_tuple_i32(name);
}

#[no_mangle]
pub fn insert_i32i32i32tuple_i32(name: i32, key0: i32, key1: i32, key2: i32, value: i32) {
    insert_tuple_i32_inner(name, TupleVariant::i32_i32_i32(key0, key1, key2), value);
}
#[no_mangle]
pub fn insert_i32_i32(name: i32, key: i32, value: i32) {
    println!("DEBUG: inserting ({key}, {value}) into map '{name}'");
    if !insert_i32_i32_inner(name, key, value) {
        panic!("Failed to insert into i32_i32 map");
    }
}

#[no_mangle]
pub fn get_i32i32i32tuple_i32(name: i32, key0: i32, key1: i32, key2: i32) -> i32 {
    get_i32(name, &Box::new(TupleVariant::i32_i32_i32(key0, key1, key2)))
}
#[no_mangle]
pub fn get_i32_i32(name: i32, key: i32) -> i32 {
    println!("getting key '{key}' from map '{name}'");
    get_i32(name, &key)
}
#[no_mangle]
pub fn insert_i32_string(name: i32, key: i32, offset: u32, length: u32) {
    let value = string_from_data(offset, length);
    println!("DEBUG: inserting ({key}, \"{value}\") into map '{name}'");
    if !insert_i32_string_inner(name, key, value) {
        panic!("Failed to insert into i32_string map");
    }
}
#[no_mangle]
pub fn get_string_from_i32string(name: i32, key: i32) -> String {
    get_string(name, &key)
}
#[no_mangle]
pub fn string_from_data(offset: u32, length: u32) -> String {
    let callee_ptr: *const u8 = offset as *const u8;
    let callee_slice: &[u8] =
        unsafe { slice::from_raw_parts(callee_ptr, usize::try_from(length).unwrap()) };
    String::from_utf8(callee_slice.to_vec()).unwrap()
}
#[no_mangle]
pub fn string_to_data(s: String) -> (u32, u32) {
    let (pointer, length, ..) = s.into_raw_parts();

    (pointer as u32, length as u32)
}

#[no_mangle]
pub fn set_metadata_header(offset: u32, len: u32) {
    METADATA_HEADER.with(|header| *header.borrow_mut() = (offset, len));
}

#[no_mangle]
pub fn print_metadata_header() {
    METADATA_HEADER.with(|header| {
        let header = &*header.borrow();
        println!("{}", string_from_data(header.0, header.1));
    });
}

#[no_mangle]
pub fn print_map(map_id: i32) -> String {
    println!("DEBUG: printing map {map_id}");
    let binding = MY_MAPS.lock().unwrap();
    let map = binding.get(&map_id).unwrap();
    format!("{}", map.dump_map())
}
#[no_mangle]
pub fn print_global_i32_meta_helper(global_id: u32, global_meta_offset: u32, global_meta_length: u32, global_val: i32) {
    let global_meta = string_from_data(global_meta_offset, global_meta_length);
    println!("i32,{},{},{}", global_id, global_meta, global_val);
}

#[no_mangle]
pub fn print_map_meta() {
    let mut running_output: String = String::new();
    let binding = MY_MAPS.lock().unwrap();
    let map_meta = binding.get(&1).expect("No metadata for maps found");
    match map_meta {
        AnyMap::i32_string_Map(map) => {
            for (key, value) in map.iter() {
                running_output.push_str(&format!("map,{},{},", key, value));
                let map = binding.get(&key).expect("Metadata but no map found for key: {key}");
                running_output.push_str(&format!("{}\n", map.dump_map()));
            }
        }
        _ => {
            panic!("Invalid metadata for maps");
        }
    }
    println!("{}", running_output);
}

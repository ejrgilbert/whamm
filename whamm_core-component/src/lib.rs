#[allow(warnings)]
mod bindings;

use bindings::Guest;
use whamm_core::io::print::{putc, puts, putu8, puti8, putu16, puti16, putu32, puti32, putu64, puti64, putf32, putf64, putbool};
// use whamm_core::maps::{create_i32_i32_with_id, create_i32_i32, create_i32_bool_with_id, create_i32_bool, create_i32_string_with_id, create_i32_string, create_i32_tuple_with_id, create_i32_tuple, create_i32_map_with_id, create_i32_map, create_string_i32_with_id, create_string_i32, create_string_bool_with_id,
//                        create_string_bool, create_string_string_with_id, create_string_string, create_string_tuple_with_id, create_string_tuple, create_string_map_with_id, create_string_map, create_bool_i32_with_id, create_bool_i32, create_bool_bool_with_id, create_bool_bool, create_bool_string_with_id,
//                        create_bool_string, create_bool_tuple_with_id, create_bool_tuple, create_bool_map_with_id, create_bool_map, create_tuple_i32_with_id, create_tuple_i32, create_tuple_bool_with_id, create_tuple_bool, create_tuple_string_with_id, create_tuple_string, create_tuple_tuple_with_id, create_tuple_tuple, create_tuple_map_with_id, create_tuple_map,
//                        insert_i32_i32, insert_i32_string, insert_string_i32, insert_i32i32tuple_i32, insert_i32i32i32tuple_i32, get_i32_i32, get_i32_string, get_string_i32, get_i32i32tuple_i32, get_i32i32i32tuple_i32, print_map, print_map_as_csv};

struct Component;

impl Guest for Component {
    
    // =============
    // ==== I/O ====
    // =============
    
    fn putc_wrap(c: u8) {
        putc(c);
    }
    fn puts_wrap(a: u32, l: u32) {
        unsafe { puts(a as i32, l as i32) }
    }
    fn putu8_wrap(u: u8) {
        putu8(u)
    }
    fn puti8_wrap(i: i8) {
        puti8(i)
    }
    fn putu16_wrap(u: u16) {
        putu16(u)
    }
    fn puti16_wrap(i: i16) {
        puti16(i)
    }
    fn putu32_wrap(u: u32) {
        putu32(u)
    }
    fn puti32_wrap(i: i32) {
        puti32(i)
    }
    fn putu64_wrap(u: u64) {
        putu64(u)
    }
    fn puti64_wrap(i: i64) {
        puti64(i)
    }
    fn putf32_wrap(f: f32) {
        putf32(f)
    }
    fn putf64_wrap(f: f64) {
        putf64(f)
    }
    fn putbool_wrap(u: u8) {
        putbool(u as i32)
    }
    
    // ==============
    // ==== Maps ====
    // ==============
    
    // INSERT
    // fn create_i32_i32_with_id_wrap(id: i32) {
    //     create_i32_i32_with_id(id)
    // }
    // fn create_i32_i32_wrap() -> i32 {
    //     create_i32_i32()
    // }
    // fn create_i32_bool_with_id_wrap(id: i32) {
    //     create_i32_bool_with_id(id)
    // }
    // fn create_i32_bool_wrap() -> i32 {
    //     create_i32_bool()
    // }
    // fn create_i32_string_with_id_wrap(id: i32) {
    //     create_i32_string_with_id(id)
    // }
    // fn create_i32_string_wrap() -> i32 {
    //     create_i32_string()
    // }
    // fn create_i32_tuple_with_id_wrap(id: i32) {
    //     create_i32_tuple_with_id(id)
    // }
    // fn create_i32_tuple_wrap() -> i32 {
    //     create_i32_tuple()
    // }
    // fn create_i32_map_with_id_wrap(id: i32) {
    //     create_i32_map_with_id(id)
    // }
    // fn create_i32_map_wrap() -> i32 {
    //     create_i32_map()
    // }
    // fn create_string_i32_with_id_wrap(id: i32) {
    //     create_string_i32_with_id(id)
    // }
    // fn create_string_i32_wrap() -> i32 {
    //     create_string_i32()
    // }
    // fn create_string_bool_with_id_wrap(id: i32) {
    //     create_string_bool_with_id(id)
    // }
    // fn create_string_bool_wrap() -> i32 {
    //     create_string_bool()
    // }
    // fn create_string_string_with_id_wrap(id: i32) {
    //     create_string_string_with_id(id)
    // }
    // fn create_string_string_wrap() -> i32 {
    //     create_string_string()
    // }
    // fn create_string_tuple_with_id_wrap(id: i32) {
    //     create_string_tuple_with_id(id)
    // }
    // fn create_string_tuple_wrap() -> i32 {
    //     create_string_tuple()
    // }
    // fn create_string_map_with_id_wrap(id: i32) {
    //     create_string_map_with_id(id)
    // }
    // fn create_string_map_wrap() -> i32 {
    //     create_string_map()
    // }
    // fn create_bool_i32_with_id_wrap(id: i32) {
    //     create_bool_i32_with_id(id)
    // }
    // fn create_bool_i32_wrap() -> i32 {
    //     create_bool_i32()
    // }
    // fn create_bool_bool_with_id_wrap(id: i32) {
    //     create_bool_bool_with_id(id)
    // }
    // fn create_bool_bool_wrap() -> i32 {
    //     create_bool_bool()
    // }
    // fn create_bool_string_with_id_wrap(id: i32) {
    //     create_bool_string_with_id(id)
    // }
    // fn create_bool_string_wrap() -> i32 {
    //     create_bool_string()
    // }
    // fn create_bool_tuple_with_id_wrap(id: i32) {
    //     create_bool_tuple_with_id(id)
    // }
    // fn create_bool_tuple_wrap() -> i32 {
    //     create_bool_tuple()
    // }
    // fn create_bool_map_with_id_wrap(id: i32) {
    //     create_bool_map_with_id(id)
    // }
    // fn create_bool_map_wrap() -> i32 {
    //     create_bool_map()
    // }
    // fn create_tuple_i32_with_id_wrap(id: i32) {
    //     create_tuple_i32_with_id(id)
    // }
    // fn create_tuple_i32_wrap() -> i32 {
    //     create_tuple_i32()
    // }
    // fn create_tuple_bool_with_id_wrap(id: i32) {
    //     create_tuple_bool_with_id(id)
    // }
    // fn create_tuple_bool_wrap() -> i32 {
    //     create_tuple_bool()
    // }
    // fn create_tuple_string_with_id_wrap(id: i32) {
    //     create_tuple_string_with_id(id)
    // }
    // fn create_tuple_string_wrap() -> i32 {
    //     create_tuple_string()
    // }
    // fn create_tuple_tuple_with_id_wrap(id: i32) {
    //     create_tuple_tuple_with_id(id)
    // }
    // fn create_tuple_tuple_wrap() -> i32 {
    //     create_tuple_tuple()
    // }
    // fn create_tuple_map_with_id_wrap(id: i32) {
    //     create_tuple_map_with_id(id)
    // }
    // fn create_tuple_map_wrap() -> i32 {
    //     create_tuple_map()
    // }
    //
    // // INSERT
    // fn insert_i32_i32_wrap(id: i32, key: i32, value: i32) {
    //     insert_i32_i32(id, key, value)
    // }
    // fn insert_i32_string_wrap(id: i32, key: i32, val_offset: i32, val_length: i32) {
    //     let ptr: *const u8 = val_offset as *const u8;
    //     insert_i32_string(id, key, ptr, val_length as usize)
    // }
    // fn insert_string_i32_wrap(id: i32, key_offset: i32, key_length: i32, val: i32) {
    //     let ptr: *const u8 = key_offset as *const u8;
    //     insert_string_i32(id, ptr, key_length as usize, val)
    // }
    // fn insert_i32i32tuple_i32_wrap(id: i32, key0: i32, key1: i32, value: i32) {
    //     insert_i32i32tuple_i32(id, key0, key1, value)
    // }
    // fn insert_i32i32i32tuple_i32_wrap(id: i32, key0: i32, key1: i32, key2: i32, value: i32) {
    //     insert_i32i32i32tuple_i32(id, key0, key1, key2, value)
    // }
    //
    // // GET
    // fn get_i32_i32_wrap(id: i32, key: i32) -> i32 {
    //     get_i32_i32(id, key)
    // }
    // fn get_i32_string_wrap(id: i32, key: i32) -> String {
    //     get_i32_string(id, key)
    // }
    // fn get_string_i32_wrap(id: i32, key_offset: i32, key_length: i32) -> i32 {
    //     let ptr: *const u8 = key_offset as *const u8;
    //     get_string_i32(id, ptr, key_length as usize)
    // }
    // fn get_i32i32tuple_i32_wrap(id: i32, key0: i32, key1: i32) -> i32 {
    //     get_i32i32tuple_i32(id, key0, key1)
    // }
    // fn get_i32i32i32tuple_i32_wrap(id: i32, key0: i32, key1: i32, key2: i32) -> i32 {
    //     get_i32i32i32tuple_i32(id, key0, key1, key2)
    // }
    //
    // // PRINT
    // fn print_map_wrap(id: i32) {
    //     print_map(id)
    // }
    // fn print_map_as_csv_wrap(id: i32) {
    //     print_map_as_csv(id)
    // }
}

bindings::export!(Component with_types_in bindings);

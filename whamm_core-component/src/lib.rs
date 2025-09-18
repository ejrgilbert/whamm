#[allow(warnings)]
mod bindings;

use bindings::Guest;

#[allow(unused_imports)]
use whamm_core_impl::io::print::{putc as impl_putc, puts as impl_puts, putu8 as impl_putu8, puti8 as impl_puti8, putu16 as impl_putu16, puti16 as impl_puti16, putu32 as impl_putu32, puti32 as impl_puti32, putu64 as impl_putu64, puti64 as impl_puti64, putf32 as impl_putf32, putf64 as impl_putf64, putbool as impl_putbool};
use whamm_core_impl::maps::{create_i32_i32_with_id as impl_create_i32_i32_with_id, create_i32_i32 as impl_create_i32_i32, create_i32_bool_with_id as impl_create_i32_bool_with_id, create_i32_bool as impl_create_i32_bool, create_i32_string_with_id as impl_create_i32_string_with_id, create_i32_string as impl_create_i32_string, create_i32_tuple_with_id as impl_create_i32_tuple_with_id, create_i32_tuple as impl_create_i32_tuple, create_i32_map_with_id as impl_create_i32_map_with_id, create_i32_map as impl_create_i32_map, create_string_i32_with_id as impl_create_string_i32_with_id, create_string_i32 as impl_create_string_i32, create_string_bool_with_id as impl_create_string_bool_with_id,
                       create_string_bool as impl_create_string_bool, create_string_string_with_id as impl_create_string_string_with_id, create_string_string as impl_create_string_string, create_string_tuple_with_id as impl_create_string_tuple_with_id, create_string_tuple as impl_create_string_tuple, create_string_map_with_id as impl_create_string_map_with_id, create_string_map as impl_create_string_map, create_bool_i32_with_id as impl_create_bool_i32_with_id, create_bool_i32 as impl_create_bool_i32, create_bool_bool_with_id as impl_create_bool_bool_with_id, create_bool_bool as impl_create_bool_bool, create_bool_string_with_id as impl_create_bool_string_with_id,
                       create_bool_string as impl_create_bool_string, create_bool_tuple_with_id as impl_create_bool_tuple_with_id, create_bool_tuple as impl_create_bool_tuple, create_bool_map_with_id as impl_create_bool_map_with_id, create_bool_map as impl_create_bool_map, create_tuple_i32_with_id as impl_create_tuple_i32_with_id, create_tuple_i32 as impl_create_tuple_i32, create_tuple_bool_with_id as impl_create_tuple_bool_with_id, create_tuple_bool as impl_create_tuple_bool, create_tuple_string_with_id as impl_create_tuple_string_with_id, create_tuple_string as impl_create_tuple_string, create_tuple_tuple_with_id as impl_create_tuple_tuple_with_id, create_tuple_tuple as impl_create_tuple_tuple, create_tuple_map_with_id as impl_create_tuple_map_with_id, create_tuple_map as impl_create_tuple_map,
                       insert_i32_i32 as impl_insert_i32_i32, insert_i32_string as impl_insert_i32_string, insert_string_i32 as impl_insert_string_i32, insert_i32i32tuple_i32 as impl_insert_i32i32tuple_i32, insert_i32i32i32tuple_i32 as impl_insert_i32i32i32tuple_i32, get_i32_i32 as impl_get_i32_i32, get_string_i32 as impl_get_string_i32, get_i32i32tuple_i32 as impl_get_i32i32tuple_i32, get_i32i32i32tuple_i32 as impl_get_i32i32i32tuple_i32, print_map as impl_print_map, print_map_as_csv as impl_print_map_as_csv};

struct Component;

impl Guest for Component {
    
    // =============
    // ==== I/O ====
    // =============
    
    fn putc(c: u8) {
        impl_putc(c);
    }
    fn puts(a: u32, l: u32) {
        unsafe { impl_puts(a as i32, l as i32) }
    }
    fn putu8(u: u8) {
        impl_putu8(u)
    }
    fn puti8(i: i8) {
        impl_puti8(i)
    }
    fn putu16(u: u16) {
        impl_putu16(u)
    }
    fn puti16(i: i16) {
        impl_puti16(i)
    }
    fn putu32(u: u32) {
        impl_putu32(u)
    }
    fn puti32(i: i32) {
        impl_puti32(i)
    }
    fn putu64(u: u64) {
        impl_putu64(u)
    }
    fn puti64(i: i64) {
        impl_puti64(i)
    }
    fn putf32(f: f32) {
        impl_putf32(f)
    }
    fn putf64(f: f64) {
        impl_putf64(f)
    }
    fn putbool(u: u8) {
        impl_putbool(u as i32)
    }
    
    // ==============
    // ==== Maps ====
    // ==============
    
    // INSERT
    fn create_i32_i32_with_id(id: i32) {
        impl_create_i32_i32_with_id(id)
    }
    fn create_i32_i32() -> i32 {
        impl_create_i32_i32()
    }
    fn create_i32_bool_with_id(id: i32) {
        impl_create_i32_bool_with_id(id)
    }
    fn create_i32_bool() -> i32 {
        impl_create_i32_bool()
    }
    fn create_i32_string_with_id(id: i32) {
        impl_create_i32_string_with_id(id)
    }
    fn create_i32_string() -> i32 {
        impl_create_i32_string()
    }
    fn create_i32_tuple_with_id(id: i32) {
        impl_create_i32_tuple_with_id(id)
    }
    fn create_i32_tuple() -> i32 {
        impl_create_i32_tuple()
    }
    fn create_i32_map_with_id(id: i32) {
        impl_create_i32_map_with_id(id)
    }
    fn create_i32_map() -> i32 {
        impl_create_i32_map()
    }
    fn create_string_i32_with_id(id: i32) {
        impl_create_string_i32_with_id(id)
    }
    fn create_string_i32() -> i32 {
        impl_create_string_i32()
    }
    fn create_string_bool_with_id(id: i32) {
        impl_create_string_bool_with_id(id)
    }
    fn create_string_bool() -> i32 {
        impl_create_string_bool()
    }
    fn create_string_string_with_id(id: i32) {
        impl_create_string_string_with_id(id)
    }
    fn create_string_string() -> i32 {
        impl_create_string_string()
    }
    fn create_string_tuple_with_id(id: i32) {
        impl_create_string_tuple_with_id(id)
    }
    fn create_string_tuple() -> i32 {
        impl_create_string_tuple()
    }
    fn create_string_map_with_id(id: i32) {
        impl_create_string_map_with_id(id)
    }
    fn create_string_map() -> i32 {
        impl_create_string_map()
    }
    fn create_bool_i32_with_id(id: i32) {
        impl_create_bool_i32_with_id(id)
    }
    fn create_bool_i32() -> i32 {
        impl_create_bool_i32()
    }
    fn create_bool_bool_with_id(id: i32) {
        impl_create_bool_bool_with_id(id)
    }
    fn create_bool_bool() -> i32 {
        impl_create_bool_bool()
    }
    fn create_bool_string_with_id(id: i32) {
        impl_create_bool_string_with_id(id)
    }
    fn create_bool_string() -> i32 {
        impl_create_bool_string()
    }
    fn create_bool_tuple_with_id(id: i32) {
        impl_create_bool_tuple_with_id(id)
    }
    fn create_bool_tuple() -> i32 {
        impl_create_bool_tuple()
    }
    fn create_bool_map_with_id(id: i32) {
        impl_create_bool_map_with_id(id)
    }
    fn create_bool_map() -> i32 {
        impl_create_bool_map()
    }
    fn create_tuple_i32_with_id(id: i32) {
        impl_create_tuple_i32_with_id(id)
    }
    fn create_tuple_i32() -> i32 {
        impl_create_tuple_i32()
    }
    fn create_tuple_bool_with_id(id: i32) {
        impl_create_tuple_bool_with_id(id)
    }
    fn create_tuple_bool() -> i32 {
        impl_create_tuple_bool()
    }
    fn create_tuple_string_with_id(id: i32) {
        impl_create_tuple_string_with_id(id)
    }
    fn create_tuple_string() -> i32 {
        impl_create_tuple_string()
    }
    fn create_tuple_tuple_with_id(id: i32) {
        impl_create_tuple_tuple_with_id(id)
    }
    fn create_tuple_tuple() -> i32 {
        impl_create_tuple_tuple()
    }
    fn create_tuple_map_with_id(id: i32) {
        impl_create_tuple_map_with_id(id)
    }
    fn create_tuple_map() -> i32 {
        impl_create_tuple_map()
    }
    
    // INSERT
    fn insert_i32_i32(id: i32, key: i32, value: i32) {
        impl_insert_i32_i32(id, key, value)
    }
    fn insert_i32_string(id: i32, key: i32, val_offset: i32, val_length: i32) {
        let ptr: *const u8 = val_offset as *const u8;
        impl_insert_i32_string(id, key, ptr, val_length as usize)
    }
    fn insert_string_i32(id: i32, key_offset: i32, key_length: i32, val: i32) {
        let ptr: *const u8 = key_offset as *const u8;
        impl_insert_string_i32(id, ptr, key_length as usize, val)
    }
    fn insert_i32i32tuple_i32(id: i32, key0: i32, key1: i32, value: i32) {
        impl_insert_i32i32tuple_i32(id, key0, key1, value)
    }
    fn insert_i32i32i32tuple_i32(id: i32, key0: i32, key1: i32, key2: i32, value: i32) {
        impl_insert_i32i32i32tuple_i32(id, key0, key1, key2, value)
    }
    
    // GET
    fn get_i32_i32(id: i32, key: i32) -> i32 {
        impl_get_i32_i32(id, key)
    }
    // fn get_i32_string(id: i32, key: i32) -> String {
    //     impl_get_i32_string(id, key)
    // }
    fn get_string_i32(id: i32, key_offset: i32, key_length: i32) -> i32 {
        let ptr: *const u8 = key_offset as *const u8;
        impl_get_string_i32(id, ptr, key_length as usize)
    }
    fn get_i32i32tuple_i32(id: i32, key0: i32, key1: i32) -> i32 {
        impl_get_i32i32tuple_i32(id, key0, key1)
    }
    fn get_i32i32i32tuple_i32(id: i32, key0: i32, key1: i32, key2: i32) -> i32 {
        impl_get_i32i32i32tuple_i32(id, key0, key1, key2)
    }
    
    // PRINT
    fn print_map(id: i32) {
        impl_print_map(id)
    }
    fn print_map_as_csv(id: i32) {
        impl_print_map_as_csv(id)
    }
}

bindings::export!(Component with_types_in bindings);

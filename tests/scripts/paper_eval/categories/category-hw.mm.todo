// TODO -- to have this work, I'll need to support the following:
// 1. deterministic event match ordering
// 2. WhammParam
// 3. Utility to write to core_lib memory, then write back.
//    - write_to_lib_mem(offset, len): Writes to the library memory (starting at 0) and saves previous data to mem_alloc_global offset
//    - map_insert_string_i32(0, len, value): Inserts value into a map<string, i32>
//    - reset_lib_mem(len): Writes the saved previous data back (starting at lib_mem:0) starting at mem_alloc_global offset until 'len'

report var dyn_categories: map<str, i32>;

wasm:opcode:*:before {
    dyn_categories[category]++;
}
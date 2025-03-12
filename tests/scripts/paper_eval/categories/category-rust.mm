report var dyn_categories: map<u32, i32>;

wasm:opcode:*:before {
    dyn_categories[category_id]++;
}
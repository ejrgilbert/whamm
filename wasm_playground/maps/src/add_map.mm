wasm:opcode:call:after /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "do_something" 
/ {
    map<i32, i32> a;
}
wasm:bytecode:call:after /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_perform"
/ {
    var i: i32;
    i = 0;
    i++;
}
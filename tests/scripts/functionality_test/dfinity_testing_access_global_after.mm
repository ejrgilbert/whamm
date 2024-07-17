bool i;
wasm:bytecode:call:after /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new"
/ {
    i = false;
}
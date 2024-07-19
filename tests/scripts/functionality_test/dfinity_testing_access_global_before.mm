bool i;

wasm::call:before /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new"
/ {
    i = 0;
}
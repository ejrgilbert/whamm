/*
 * This will perform an asynchronous fault in Dfinity (redirect call to fault_injector canister endpoint).
 */
wasm::call(arg0: i32, arg1: i32, arg2: i32, arg3: i32):alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {
    alt_call_by_name("instr_redirect_to_fault_injector");
}

/*
 * This will perform an asynchronous fault in Dfinity (redirect call to fault_injector canister endpoint).
 */
i32 count;
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {
// TODO -- change back to new_target_fn_name body when able to lookup function names with Orca
//     new_target_fn_name = "instr_redirect_to_fault_injector";
    count = 10;
}

/*
 * This will perform an asynchronous fault in Dfinity (redirect call to fault_injector canister endpoint).
 */
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new" &&
    strpaircmp((arg0, arg1), "bookings") &&
    strpaircmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
}
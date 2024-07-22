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
// TODO -- to get this to work with Orca, we must change this to the
//     pattern we've planned...to insert the call at the point in the
//     body rather than defining a global to be emitted later into the
//     end of the probe by offset.
//     This doesn't work in Orca since we don't really emit by offsets.
//     new_target_fn_name = "instr_redirect_to_fault_injector";
//     alt_call_by_name("instr_redirect_to_fault_injector");
    count = 10;
}

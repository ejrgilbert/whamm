/*
 * This will perform an asynchronous fault in Dfinity (redirect call to fault_injector canister endpoint).
 */
wasm:ic0:call_new:alt / canister_name == "bookings" && endpoint == "record" / {
    new_target_fn_name = redirect_call_to_asynchronous_fault_injector_canister;
}
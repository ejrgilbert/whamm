/*
 * This will perform an asynchronous fault in Dfinity (redirect call to fault_injector canister endpoint).
 */
wasm:ic0:call_new:alt {
    new_target_fn_name = redirect_to_fault_injector;
}
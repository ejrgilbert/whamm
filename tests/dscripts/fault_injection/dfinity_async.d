/*
 * This will perform an asynchronous fault in Dfinity (redirect call to fault_injector canister endpoint).
 */
fault_injection:ic0:call_new:alt / canister_name == "bookings" && endpoint == "record" / {
    redirect_call_to_asynchronous_fault_injector_canister
}
/*
 * This will perform an asynchronous fault in Dfinity (redirect call to fault_injector canister endpoint).
 */
dfinity:ic0:call_new:alt {
    redirect_to_fault_injector;
}
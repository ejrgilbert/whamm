/*
 * This will perform a synchronous fault in Dfinity (return non-zero on call_perform).
 * NOTE: This will instrument and fail EVERY call_perform as there is no predicate on
 *       the target canister, to add granularity, another probe on call_new will need
 *       to be added to collect call site/target canister info.
 *       See file: `dfinity_sync-with-pred.d`
 */
wasm:bytecode:call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_perform"
/ {
    new_target_fn_name = "instr_inject_synchronous_fault";
}
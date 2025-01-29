wasm:opcode:call:before / fname == "main" && target_fn_name == "calc" / {
    report var count;
    if (arg0 == 1 || arg1 == 1) {
       count++;
    }
}

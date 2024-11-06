report i32 count;
wasm:opcode:call:before / fname == "main" && target_fn_name == "calc" / {
   if (arg0 == 1 || arg1 == 1) {
       count++;
   }
}


// To get report variables working on wizard target:
// - create a "wasm:end" $end function
// -

wasm:opcode:call:before / fname == "main" && target_fn_name == "calc" / {
// wasm:opcode:call:before / fid == 5 / {
    unshared i32 count;
   if (arg0 == 1 || arg1 == 1) {
       count++;
   }
}

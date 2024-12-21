
// To get report variables working on wizard target:
// - create a "wasm:end" $end function
// -

wasm:opcode:call:before / imm0 == 31 / { // fid of "calc"
    report unshared i32 count;
   if (arg0 == 1 || arg1 == 1) {
       count++;
   }
}

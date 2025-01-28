var count: i32;

wasm:opcode:call:before / (fid == 3 && pc != 2) / {
   if (arg0 == 1) {
       count++;
   }
}

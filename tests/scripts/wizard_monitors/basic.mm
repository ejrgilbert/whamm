var count: i32;

wasm:opcode:call(arg0: i32):before / (fid == 3 && opidx != 2) / {
   if (arg0 == 1) {
       count++;
   }
}

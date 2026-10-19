report var n32: i32;
report var n64: i32;
wasm:opcode:drop(arg0: i32):before { n32 = n32 + 1; }
wasm:opcode:drop(arg0: i64):before { n64 = n64 + 1; }

use alpha;
wasm:opcode:call:before {
  report var p: i32;
  p = alpha.mem_alloc(4);
}

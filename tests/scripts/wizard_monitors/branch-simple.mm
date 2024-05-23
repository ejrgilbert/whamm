// TODO -- the key needs to be changed to "(wasm_bytecode_loc, i32)"
map<(i32, i32), i32> count;
wasm::br|br_if:before {
  i32 index;
  // "tos" is defined as the top-of-stack
  index = tos != 0 ? 1 : 0;
  // count stores an array of counters
  // TODO -- add map op support and uncomment the following
  // count[probe_func, pc, index]++;
}
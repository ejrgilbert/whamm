/* Facts:
 * Bytecode names are probe types
 * Bound variables
 * - probe_func  : (whamm standard) is the Wasm function
 * - tos         : Wasm top-of-stack
 * - pc          : Wasm program counter
 * - local0...N  : Wasm locals
 * -
 * need to access top-of-stack, locals, program counter
 * need a handle to the function ("func")
*/

report var count: map<(u32, u32, i32), i32>;

wasm::br:before {
  // branch always taken for `br`
  // count stores an array of counters
  count[(fid, pc, 1)]++;
}

wasm::br_if:before {
  // which branch was taken?
  var index: i32;
  index = arg0 != 0 ? 1 : 0;

  // count stores an array of counters
  count[(fid, pc, index)]++;
}

wasm::br_table:before {
  // which branch was taken?
  var index: i32;
  index = arg0 < (num_targets - 1) ? targets[arg0 as u32] as i32 : default_target as i32;

  // count stores an array of counters
  count[(fid, pc, index)]++;
}

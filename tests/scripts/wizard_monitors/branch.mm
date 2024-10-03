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

report map<(u32, u32, i32), i32> count;

wasm::br:before {
  // count stores an array of counters
  count[(fid, pc, 1)]++;
}

wasm::br_if:before {
  i32 index;
  // "arg0" is defined as the top-of-stack
  index = arg0 != 0 ? 1 : 0;
  // count stores an array of counters
  count[(fid, pc, index)]++;
}

// wasm::br_table:before {
//   // "num_targets" is the number of targets of a br_table
//   // "arg0" is defined as the top-of-stack
//
//   // TODO -- static maps like this can be emitted as a select!
//   int index = arg0 >= num_targets ? targets[arg0] : arg0;
//   // count stores an array of counters
//   count[(fid, pc, index)]++;
// }

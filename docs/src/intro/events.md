# Instrumentable Events #

Currently available `packages`:
- `wasm:opcode`: to instrument Wasm opcodes
- `wasm:func`: to instrument Wasm functions
  - currently supports `entry` and `exit`
  - future: `unwind`
- `wasm:block`: to instrument Wasm basic blocks

`Packages` to be added:
- `thread` operation events
- `gc` operation events
- `memory` access (read/write) events
- `table` access (read/write) events
- WASI `component` operation events, e.g. `wasi:http:send_req:alt`
- `wasm:begin`/`wasm:end` events
- `traps`
- `exception` throw/rethrow/catch events

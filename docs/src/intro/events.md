# Instrumentable Events #

Currently available `packages`:
- `wasm:bytecode`, e.g. `wasm:bytecode:call:alt`

`Packages` to be added:
- `thread` operation events
- `gc` operation events
- `function` enter/exit/unwind events, e.g. `wasm:fn:enter:before`
- `memory` access (read/write) events
- `table` access (read/write) events
- WASI `component` operation events, e.g. `wasi:http:send_req:alt`
- `BEGIN`/`END` events
- `traps`
- `exception` throw/rethrow/catch events

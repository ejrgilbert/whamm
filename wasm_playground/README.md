# The Wasm Playground #

The `.wat` files here are used to write out the code for dsl-provided functions.
Each `.wat` file provides the dsl-provided function along with a test function. To
run the tests, simply run: `wasmtime <WAT>`. If there is an error from hitting `unreachable`,
the test has failed. No output means that the tests passed. The reason for this structure
is that it keeps us from having to define and link a `logger`, which feels like overkill
for this purpose.

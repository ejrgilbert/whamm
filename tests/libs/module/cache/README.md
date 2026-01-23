# Whamm! User Library -- CacheSimulator #

## Cache Specifications ##

- Size: 1 MB
- Associativity: 4 way set-associative
- Block Size: 128 byte block
- Replacement Policy: LRU

## To Build ##

To build:
- In the base of this project (`whamm/tests/libs/module/cache`), execute: `cargo build --release --target wasm32-wasip1`
- The built `wasm` binary will be located at `whamm/user_libs/cache/target/wasm32-wasip1/release/cache.wasm`
  - If you rebuild this library, you'll need to copy this wasm module to `whamm/tests/libs/module/cache` for it to be picked up by the whamm tests!
# Whamm! User Library -- CacheSimulator #

## Cache Specifications ##

- Size: 1 MB
- Associativity: 4 way set-associative
- Block Size: 128 byte block
- Replacement Policy: LRU

## To Build ##

To build:
- In the base of this project (`whamm/user_libs/cache_sim`), execute: `cargo build --release --target wasm32-wasip1`
- The built `wasm` binary will be located at `whamm/user_libs/cache_sim/target/wasm32-wasip1/release/cache_sim.wasm`

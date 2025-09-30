# Cache Simulator #

Here is an example monitor that can be written in `Whamm`, it calls a provided cache implementation to simulate cache lookups on memory operations.

```
// CacheSim: instruments memory accesses and simulates a cache attached to the memory.

use cache;

wasm:opcode:*load*|*store*:before {
    report var hit: u32;
    report var miss: u32;

    var result: i32 = cache.check_access(effective_addr as i32, data_size as i32);
    var num_hits: i32 = (result & 0xFFFF0000) >> 16;
    var num_misses: i32 = (result & 0x0000FFFF);

    hit = hit + (num_hits as u32);
    miss = miss + (num_misses as u32);
}
```

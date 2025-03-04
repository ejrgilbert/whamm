// ==================
// ---- CacheSim ----
// ==================

// CacheSim: instruments memory accesses and simulates a cache attached to the memory.

use cache;

wasm:opcode:*load*|*store*:before {
    report var hit: u32;
    report var miss: u32;

//     var result: i32 = cache.check_access(effective_addr, data_size as i32);
    var eff_addr: u32 = addr + offset as u32;
    var result: i32 = cache.check_access(eff_addr as i32, data_size as i32);
    var addr0_res: i32 = result & 0xFFFF0000;
    var addr1_res: i16 = (result & 0x0000FFFF) as i16;

    if (addr0_res != 0) {
        hit++;
    } else {
        miss++;
    }

    // Only add the second result if it was actually calculated (not -1)
    if (addr1_res != -1) {
        if (addr1_res != 0) {
            hit++;
        } else {
            miss++;
        }
    }
}
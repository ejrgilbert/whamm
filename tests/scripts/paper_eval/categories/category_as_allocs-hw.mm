// TODO -- to have this work, I'll need to support the following:
// 1. deterministic event match ordering
// 2. WhammParam
// 3. Utility to write to core_lib memory, then write back.
//    - write_to_lib_mem(offset, len): Writes to the library memory (starting at 0) and saves previous data to mem_alloc_global offset
//    - map_insert_string_i32(0, len, value): Inserts value into a map<string, i32>
//    - reset_lib_mem(len): Writes the saved previous data back (starting at lib_mem:0) starting at mem_alloc_global offset until 'len'

report var const: u32;
report var misc: u32;
report var control: u32;
report var local: u32;
report var global: u32;
report var table: u32;
report var load: u32;
report var store: u32;
report var mem: u32;
report var arith: u32;
report var compare: u32;
report var convert: u32;
report var exn: u32;
report var simd: u32;
report var ref: u32;
report var gc: u32;
report var atomic: u32;

wasm:opcode:*:before {
    if (category == "const") {
        const++;
    } else {
        if (category == "misc") {
            misc++;
        } else {
            if (category == "control") {
                control++;
            } else {
                if (category == "local") {
                    local++;
                } else {
                    if (category == "global") {
                        global++;
                    } else {
                        if (category == "table") {
                            table++;
                        } else {
                             if (category == "load") {
                                 load++;
                             } else {
                                   if (category == "store") {
                                       store++;
                                   } else {
                                       if (category == "mem") {
                                           mem++;
                                       } else {
                                           if (category == "arith") {
                                               arith++;
                                           } else {
                                               if (category == "compare") {
                                                   compare++;
                                               } else {
                                                   if (category == "convert") {
                                                       convert++;
                                                   } else {
                                                       if (category == "exn") {
                                                           exn++;
                                                       } else {
                                                           if (category == "simd") {
                                                               simd++;
                                                           } else {
                                                               if (category == "ref") {
                                                                   ref++;
                                                               } else {
                                                                   if (category == "gc") {
                                                                       gc++;
                                                                   } else {
                                                                       if (category == "atomic") {
                                                                           atomic++;
                                                                       }
                                                                  }
                                                              }
                                                          }
                                                      }
                                                  }
                                              }
                                          }
                                      }
                                  }
                              }
                        }
                    }
                }
            }
        }
    }
}
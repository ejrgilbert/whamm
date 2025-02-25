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
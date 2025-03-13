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
    if (category_name == "const") {
        const++;
    } else {
        if (category_name == "misc") {
            misc++;
        } else {
            if (category_name == "control") {
                control++;
            } else {
                if (category_name == "local") {
                    local++;
                } else {
                    if (category_name == "global") {
                        global++;
                    } else {
                        if (category_name == "table") {
                            table++;
                        } else {
                             if (category_name == "load") {
                                 load++;
                             } else {
                                   if (category_name == "store") {
                                       store++;
                                   } else {
                                       if (category_name == "mem") {
                                           mem++;
                                       } else {
                                           if (category_name == "arith") {
                                               arith++;
                                           } else {
                                               if (category_name == "compare") {
                                                   compare++;
                                               } else {
                                                   if (category_name == "convert") {
                                                       convert++;
                                                   } else {
                                                       if (category_name == "exn") {
                                                           exn++;
                                                       } else {
                                                           if (category_name == "simd") {
                                                               simd++;
                                                           } else {
                                                               if (category_name == "ref") {
                                                                   ref++;
                                                               } else {
                                                                   if (category_name == "gc") {
                                                                       gc++;
                                                                   } else {
                                                                       if (category_name == "atomic") {
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
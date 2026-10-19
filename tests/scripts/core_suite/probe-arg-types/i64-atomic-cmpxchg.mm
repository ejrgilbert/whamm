report var repl: i64;
report var expd: i64;
report var maddr: u32;
wasm:opcode:i64.atomic.rmw.cmpxchg:before {
    repl = arg0;
    expd = arg1;
    maddr = arg2;
}

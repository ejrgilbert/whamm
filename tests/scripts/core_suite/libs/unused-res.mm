// This is just to test that unused results are dropped for Statement::Expr

use whamm_core;

whamm_core.mem_alloc(1);
strcmp((0, 1), "record");
read_str(0, 0, 0);

wasm:opcode:call:before {
    report var i: i32 = 1;
    whamm_core.mem_alloc(1);
    strcmp((0, 1), "record");
    read_str(0, 0, 0);
}
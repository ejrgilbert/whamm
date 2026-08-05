use whamm_core;

report var body_ran: i32;

wasm:opcode:call:before {
    body_ran = body_ran + 1;
}

wasm:report {
    body_ran = body_ran + 100;
    whamm_core.puti32(body_ran);
    whamm_core.putc(10); // newline
}

var i: i32 = 0;

wasm:func:entry {
    report var at_entry0: i32;
    at_entry0 = i;
    i++;
}

wasm:func:entry {
    report var at_entry1: i32;
    at_entry1 = i;
    i++;
}

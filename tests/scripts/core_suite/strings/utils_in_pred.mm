wasm:func:entry / fname.starts_with("basic") / {
    report var num: i32 = 1;
}
wasm:func:entry / fname.ends_with("stmt") / {
    report var num: i32 = 2;
}
wasm:func:entry / fname.ends_with("stmt") && !fname.starts_with("if") / {
    report var num: i32 = 3;
}
wasm:func:entry / fname.contains("if") / {
    report var num: i32 = 4;
}

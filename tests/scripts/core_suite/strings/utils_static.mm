wasm:opcode:drop:before {
    report var s: str = "hello world";
    report var l: u32 = s.len();
    report var start_exp_true: bool = s.starts_with("hello");
    report var start_exp_false: bool = s.starts_with("nope");

    report var end_exp_true: bool = s.ends_with("world");
    report var end_exp_false: bool = s.ends_with("nope");

    report var contains_exp_true0: bool = s.contains("hello");
    report var contains_exp_true1: bool = s.contains("world");
    report var contains_exp_true2: bool = s.contains("lo wo");
    report var contains_exp_false: bool = s.contains("asdf ");
}

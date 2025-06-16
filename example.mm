linux:syscall:mkdir:before {
    report var count: u32;
    count = count + arg1 as u32;
}
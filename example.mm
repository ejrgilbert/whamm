linux:syscall:mkdir:before {
    report var count: u32;
    count = count + dirmode as u32;
}
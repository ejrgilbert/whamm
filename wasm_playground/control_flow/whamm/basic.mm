
// To get report variables working on wizard target:
// - create a "wasm:end" $end function
// -

wasm:opcode:call:before { // fid of "calc"
    report unshared var count: i32;
//    if (arg0 == 1 || arg1 == 1) {
//        count++;
//    }
    count++;
}

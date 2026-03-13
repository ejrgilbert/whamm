var a: i32;
var b: i32;

wasm:opcode:call:after    { a = imm0 as i32; }
wasm:opcode:i32.load:after { b = res0 as i32; }

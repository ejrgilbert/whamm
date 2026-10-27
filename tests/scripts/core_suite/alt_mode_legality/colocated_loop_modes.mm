report var en: i32;
report var ex: i32;
report var af: i32;
report var a: i32;

wasm:opcode:loop:entry { en++; }
wasm:opcode:loop:exit { ex++; }
wasm:opcode:loop:after { af++; }
wasm:opcode:loop:alt { a++; }

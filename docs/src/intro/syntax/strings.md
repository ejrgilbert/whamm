# Strings #

`Whamm` has very basic support for strings; they are injected into the instrumented Wasm program
and are represented as the tuple: `(memory_address, length)`.

```
var a: str = "";
var b: str = "The quick brown fox";
// var c: string = null; // INVALID -- strings cannot be set to null
```

## Escapes ##

Strings can use the `\` character to escape some sequences including:
- `\n` newline
- `\t` tab
- `\"` double quote
- `\'` single quote
- `\\` backslash
- `\0` null byte
- `\x(HEX_DIGIT{2})` to insert hex
- `\u(HEX_DIGIT+)` to insert unicode
-


```
// encodes the string literal: "hello drop😀😀A"
report var s: str = "hello drop\u{1F600}😀\x41\n";
```

# Strings and Libraries #

Strings can be passed to / pulled from libraries through memory operations. To have
this work in your own libraries, you will have to expose functions to allocate/free
memory.

Write to a library:
```
use whamm_core;

wasm:opcode:drop:before {
    // initialize the string to pass
    var s: str = "hello world!";
    var l: u32 = s.len();

    // allocate the right number of bytes to store the string
    var ptr: i32 = whamm_core.mem_alloc(l as i32);

    // write the string to the target library's memory
    write_str(memid(whamm_core), ptr, s);

    // call a function in the library that uses the passed string
    // (this just prints the string)
    whamm_core.puts(ptr, l as i32);

    // free the allocated memory
    whamm_core.mem_free(ptr);
}
```

Pull from a library:
```
use alpha;

wasm:opcode:drop:before {
    // allocate some space in memory to store the library's string
    var MAX: i32 = 100;
    var ptr: i32 = alpha.mem_alloc(MAX);

    // write a string to memory, return the length of the string written
    var l: i32 = alpha.write_alphabet(ptr, MAX);

    // read the string from the library's memory
    var s: str  = read_str(memid(alpha), ptr, l as u32);

    // free the memory we've just used
    alpha.mem_free(ptr);
}
```

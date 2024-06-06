# Strings #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

[//]: # (`whamm!` has very basic support for strings; they are injected into the instrumented Wasm program and are represented as the tuple: `&#40;memory_address, length&#41;`.)

[//]: # (Follows are the strings docs from the Virgil repo that I can customize to this context.)
[//]: # (String literals are translated into arrays of bytes and usable as arrays of bytes in your program.
        In fact, the `string` type is just an alias for `Array<byte>`. The two types are completely interchangeable.)

[//]: # ()
[//]: # (```)

[//]: # (var a: string = "";)

[//]: # (var b: string = "The quick brown fox";)

[//]: # (var c: string = null;)

[//]: # (```)

[//]: # ()
[//]: # (## Escapes ##)

[//]: # ()
[//]: # (Strings can use the '\' character to escape some characters, such as carriage return, newline, tab, and quotes within strings.)

[//]: # ()
[//]: # (```)

[//]: # (// newline, tab, carriage-return, backslash, single-quote and double-quote)

[//]: # (var a: string = "\n\t\r\\\'\"";)

[//]: # (```)

[//]: # (## Strings are arrays ##)

[//]: # ()
[//]: # (Remember that strings are simply arrays of bytes. The individual bytes can be accessed just as a normal byte array, as can the length. Out of bounds accesses cause exceptions as well.)

[//]: # ()
[//]: # (```)

[//]: # (var a: string = "abcvar";)

[//]: # (var b: byte = a[0]; // strings are just arrays of bytes)

[//]: # (```)

[//]: # ()
[//]: # (```)

[//]: # (def main&#40;&#41; {)

[//]: # (    var a = "abcvar";)

[//]: # (    var x = a[11]; // produces !BoundsCheckException)

[//]: # (})

[//]: # (```)

# Maps #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

[//]: # (`whamm!` provides maps for storage of key-value pairs.)

[//]: # (This is similar to `java`'s `Map` and `python`'s `dict` types.)

[//]: # (In fact, it is exactly `Rust`'s `HashMap` type...since `whamm!` leverages this `Rust` type under-the-hood!)

[//]: # (Follows are the arrays docs from the Virgil repo that I can customize to this context.)

[//]: # (```)

[//]: # (// create a new array with Array<Type>.new&#40;length&#41;)

[//]: # (var a: Array<int> = Array<int>.new&#40;3&#41;;)

[//]: # (var b: Array<bool> = Array<bool>.new&#40;7&#41;;)

[//]: # (// we can omit the variable type when it's clear from context)

[//]: # (var c = Array<bool>.new&#40;7&#41;;)

[//]: # (```)

[//]: # ()
[//]: # (Unlike most other languages, Virgil has no special syntax for array types. Instead, they are simply written as `Array<T>`. To allocate a new array, we use the `new` keyword as if it were a _member_ of the array type. The elements of the array will be initialized to the default value for the element type.)

[//]: # ()
[//]: # (## Literals ##)

[//]: # ()
[//]: # (We can also use the `[ ... ]` syntax for creating array literals. The expressions enclosed in the brackets are evaluated, a new array of the appropriate length is created, and that array is initialized with the elements.)

[//]: # ()
[//]: # (```)

[//]: # (// [ ... ] creates an array of uniform type)

[//]: # (var a: Array<int> = [0, 1, 2];)

[//]: # (var b: Array<bool> = [true, false, true];)

[//]: # (var c: Array<byte> = [];)

[//]: # (```)

[//]: # ()
[//]: # (Usually, the type of the array can be inferred, either directly from the element expressions themselves, or from the surrounding context.)

[//]: # ()
[//]: # (```)

[//]: # (var d = [9, 4, 5];       // Array<int>)

[//]: # (var d: Array<byte> = []; // new empty byte array)

[//]: # (```)

[//]: # ()
[//]: # (## Multi-dimensional Arrays ##)

[//]: # ()
[//]: # (```)

[//]: # (// multi-dimensional arrays are simply arrays of arrays)

[//]: # (var a: Array<Array<int>> = [];)

[//]: # (var b: Array<Array<int>> = [[0]];)

[//]: # (// and can have different lengths &#40;non-rectangular&#41;)

[//]: # (var c: Array<Array<int>> = [[0, 1], [2, 3, 4]];)

[//]: # (// non-rectangular array with type inference)

[//]: # (var d = [[0, 1], [2, 3, 4]];)

[//]: # (```)

[//]: # ()
[//]: # (## Reading and writing elements ##)

[//]: # ()
[//]: # (Reading and writing elements of arrays uses the `[ ... ]` syntax like arrays in many other languages. The index expression into the array must be of an integer type &#40;i.e. not specifically just `int`, any type `iN` or `uN`&#41;.)

[//]: # ()
[//]: # (```)

[//]: # (var a: Array<bool> = [true, false];)

[//]: # (var x: bool = a[0];    // array element read)

[//]: # (var y: bool = a[0uL];  // array element read of very large index)

[//]: # (var z: int = a.length; // read of array length)

[//]: # (```)

[//]: # ()
[//]: # (```)

[//]: # (def main&#40;&#41; {)

[//]: # (    var x = Array<int>.new&#40;3&#41;;)

[//]: # (    x[0] = 11; // assignment to array element)

[//]: # (    var y = x[0];)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (## Bounds and null checks ##)

[//]: # ()
[//]: # (Accesses of Virgil arrays are dynamically checked against the bounds. An access of a null array results in a `!NullCheckException` and using an index out of the range `[0, array.length&#41;` will result in a `!BoundsCheckException`.)

[//]: # ()
[//]: # (```)

[//]: # (def main&#40;&#41; {)

[//]: # (var a: Array<int>;)

[//]: # (    a[0] = 0; // produces !NullCheckException)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (```)

[//]: # (def main&#40;&#41; {)

[//]: # (    var x = Array<int>.new&#40;3&#41;;)

[//]: # (    x[3] = 11; // produces !BoundsCheckException)

[//]: # (    var y = x[0];)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (## Composability ##)

[//]: # ()
[//]: # (Unlike most other languages, Virgil arrays can be constructed with _any_ element type, even `void`. There are no special cases or exceptions to remember. For any valid type `T`, `Array<T>` is also a valid type. This works with [primitives]&#40;Primitives.md&#41;, `void`, [tuples]&#40;Tuples.md&#41;, arrays, [classes]&#40;Classes.md&#41;, and [functions]&#40;Functions.md&#41;. No exceptions!)

[//]: # ()
[//]: # (```)

[//]: # (// if T is a legal type, then Array<T> is a legal type, even T=void)

[//]: # (var a: Array<void> = [&#40;&#41;];)

[//]: # (var b: Array<void> = [&#40;&#41;, &#40;&#41;, &#40;&#41;];)

[//]: # (var c: Array<void> = [];)

[//]: # (```)

[//]: # ()
[//]: # (Why is this useful? Composability makes the language regular so that you don't have to remember special cases. It means that arrays compose well with [functions]&#40;Functions.md&#41; and [type parameters]&#40;Typeparams.md&#41;, as we will see later.)

[//]: # ()
[//]: # (## Type inference ##)

[//]: # ()
[//]: # (We've seen that we can omit the type declaration for most variable declarations, but we can also often omit the element type in an array creation if the element type can be inferred from the context.)

[//]: # ()
[//]: # (```)

[//]: # (// often the element type of an array creation can be inferred)

[//]: # (var a: Array<int> = Array.new&#40;3&#41;;)

[//]: # (var b: Array<bool> = Array.new&#40;7&#41;;)

[//]: # (```)

[//]: # ()
[//]: # (## Not co-variant ##)

[//]: # ()
[//]: # (Unlike Java, Virgil arrays are _not_ co-variantly typed. See the section on [variance]&#40;Variance.md&#41; for more details.)
# Functions #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

[//]: # (Functions are essential in programming as they enable code modularity, reuse, and organization. By encapsulating specific tasks into discrete units, functions allow developers to write cleaner, more manageable code. They promote code reuse by allowing the same block of code to be executed from multiple places within a program, reducing redundancy and potential for errors, as well as enhancing readability and maintainability. )

[//]: # ()
[//]: # (## Compiler-Defined Functions ##)

[//]: # (Some functions will be automatically defined by the compiler based on the providers you have included in your script. These can be called just like user defined functions)

[//]: # ()
[//]: # (## Function Definitions ## )

[//]: # (Before being able to call a function, you must define it. We allow functions to be declared anywhere in a script that is not nested within another function, if/else block, or probe.)

[//]: # ()
[//]: # (Formal Syntax: `ID ~ "&#40;" ~ &#40;&#40; type ~ ID &#41; ~ &#40;"," ~ type ~ ID &#41;*&#41; ? ~ "&#41;" ~ &#40;"->" ~ type&#41; ? ~ block` )

[//]: # ()
[//]: # (If there is no declared return type, denoted by "->" followed by a `type` before the block, the default return type is `&#40;&#41;` -- this is effectively "void" or "empty tuple")

[//]: # (It is required to have a `return` statement for all possible flows through a function if it has a non-void return type and to return a value whose type must match the return type of the function.)

[//]: # ()
[//]: # (Examples of Function Definitions:)

[//]: # (```)

[//]: # (//This is a function without a return type)

[//]: # (i32 i = 0; )

[//]: # (my_function&#40;i32 param&#41; {)

[//]: # (    i = param;)

[//]: # (    return; //this is not required, but allowed)

[//]: # (    i++; //this code is unreachable )

[//]: # (})

[//]: # (```)

[//]: # (```)

[//]: # (//This is another function without a return type)

[//]: # (i32 count;)

[//]: # (my_function&#40;&#41; {)

[//]: # (    count++; //function does not require a return, as it has no return type)

[//]: # (})

[//]: # (my_function2&#40;&#41; -> &#40;&#41; { // This is functionally equivalent to my_function)

[//]: # (    count++;)

[//]: # (})

[//]: # (```)

[//]: # (```)

[//]: # (//here are functions with a return type)

[//]: # (dummy_fn&#40;&#41; -> i32 {)

[//]: # (    return 5;)

[//]: # (})

[//]: # (add_ints&#40;i32 a, i32 b&#41; -> i32 {)

[//]: # (    return a + b;)

[//]: # (})

[//]: # (larger_than_5&#40;i32 num&#41; -> bool {)

[//]: # (    return num > 5;)

[//]: # (})

[//]: # (```)

[//]: # (```)

[//]: # (//Here is an example of functions using if/else logic and function calls &#40;see below&#41;)

[//]: # (i32 my_var = 5;)

[//]: # (my_function&#40;bool param&#41; -> i32 {)

[//]: # (    if&#40;param&#41;{)

[//]: # (        my_var++;)

[//]: # (        return 0;)

[//]: # (    }else{)

[//]: # (        my_var--;)

[//]: # (        return my_var;)

[//]: # (    })

[//]: # (    //as all possible flows through the function have a return statement, all later code will be unreachable and does not require a return statement.)

[//]: # (})

[//]: # (```)

[//]: # (## Function Calls ##)

[//]: # (After a function is declared, either via the compiler or inside the script, they can be used within other functions and within probes. When called, functions execute the code specified in their definition and return a value with type matching the type of that function)

[//]: # ()
[//]: # (NOTE: You cannot call functions outside of probes or other functions)

[//]: # ()
[//]: # (Formal Syntax: `ID ~ "&#40;" ~ &#40; arg &#41;? ~ &#40; "," ~ arg &#41;* ~ "&#41;"`)

[//]: # ()
[//]: # ()
[//]: # (Examples:)

[//]: # (```)

[//]: # (i32 a = 0;)

[//]: # (inner_fn&#40;&#41; {)

[//]: # (    a++;)

[//]: # (})

[//]: # (outer_fn&#40;&#41; -> i32 {)

[//]: # (    inner_fn&#40;&#41;;)

[//]: # (    return a + 5;)

[//]: # (})

[//]: # (//"wasm:begin" is our probe that executes on wasm startup)

[//]: # (wasm:begin {)

[//]: # (    inner_fn&#40;&#41;; // call without assigning to something when void)

[//]: # (    i32 local1 = outer_fn&#40;&#41;; // call with assigning to a local when non-void)

[//]: # (    outer_fn&#40;&#41;; // you can call without assigning to something when non-void)

[//]: # (})

[//]: # (```)

[//]: # (```)

[//]: # (larger_than_5&#40;i32 num&#41; -> bool {)

[//]: # (    return num > 5;)

[//]: # (})

[//]: # (//"wasm:begin" is our probe that executes on wasm startup)

[//]: # (wasm:begin{)

[//]: # (    bool local1 = larger_than_5&#40;6&#41;;)

[//]: # (})

[//]: # (```)
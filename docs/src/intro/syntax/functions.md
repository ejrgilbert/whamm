# Functions #
Functions are essential in programming as they enable code modularity, reuse, and organization. By encapsulating specific tasks into discrete units, functions allow developers to write cleaner, more manageable code. They promote code reuse by allowing the same block of code to be executed from multiple places within a program, reducing redundancy and potential for errors, as well as enhancing readability and maintability. 

## Compiler-Defined Functions ##
Some functions will be automatically defined by the compiler based on the providers you have included in your script. These can be called just like user defined functions

## Function Definitions ## 
Before being able to call a function, you must define it. We allow functions to be declared anywhere in a script that is not nested within another function, if/else block, or probe.

Formal Syntax: ID ~ "(" ~ (( type ~ ID ) ~ ("," ~ type ~ ID )*) ? ~ ")" ~ ("->" ~ type) ? ~ block 

If there is no declared return type, denoted by "->" followed by a type before the block, the default return type is "()" -- this is effectively "void" or "empty tuple"
You need a return statement for all possible flows through a function if it has a non-void return type

Examples of Function Definitions:
```
//This is a function without a return type
i32 i = 0; 
my_function(i32 param) {
    i = param;
    return; //this is not required, but allowed
    i++; //this code is unreachable 
}
```
```
//This is another function without a return type
i32 count;
my_function() {
    count++; //function does not require a return, as it has no return type
}
my_function2() -> () { // This is functionally equivalent to my_function
    count++;
}
```
```
//here are functions with a return type
dummy_fn() -> i32 {
    return 5;
}
add_ints(i32 a, i32 b) -> i32 {
    return a + b;
}
larger_than_5(i32 num) -> bool {
    return num > 5;
}
```

## Function Calls ##
After a function is declared, either via the compiler or inside the script, they can be used within other functions and within probes. When called, functions execute the code specified in their definition and return a value with type matching the type of that function

NOTE: You cannot call functions outside of probes or other functions

Formal Syntax: ID ~ "(" ~ ( arg )? ~ ( "," ~ arg )* ~ ")"


Examples:
```
i32 a = 0;
inner_fn() {
    a++;
}
outer_fn() -> i32 {
    inner_fn();
    return a + 5;
}
//"BEGIN" is our probe that executes on wasm startup
BEGIN {
    inner_fn(); // call without assigning to something when void
    i32 local1 = outer_fn(); // call with assigning to a local when non-void
    outer_fn(); // you can call without assigning to something when non-void
}
```
```
larger_than_5(i32 num) -> bool {
    return num > 5;
}
//"BEGIN" is our probe that executes on wasm startup
BEGIN{
    bool local1 = larger_than_5(6);
}
```
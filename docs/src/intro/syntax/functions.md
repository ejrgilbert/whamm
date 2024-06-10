# Functions #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

[//]: # (Follows are the strings docs from the Virgil repo that I can customize to this context.)
[//]: # (Methods are the fundamental building block of Virgil III code. Just as with variables, a method is declared within a _scope_, such as a file, a  [class]&#40;Classes.md&#41; or [component]&#40;Components.md&#41;. We've already seen the definition of a _main_ method for a program. Let's look at another method.)

[//]: # ()
[//]: # (```)

[//]: # (// recursive computation of fibonacci sequence)

[//]: # (def fib&#40;i: int&#41; -> int {)

[//]: # (    if &#40;i <= 1&#41; return 1; // base case)

[//]: # (    return fib&#40;i - 1&#41; + fib&#40;i - 2&#41;; // recursive calls)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (This example declares a `fib` method using the `def` keyword. In this case the `fib` method takes a single parameter of type `int` and computes the requested Fibonacci number by adding the result of two recursive calls.)

[//]: # ()
[//]: # (## Parameters ##)

[//]: # ()
[//]: # (A method can have zero or more parameters that are declared in parentheses `&#40; ... &#41;` following the name. The syntax for parameters is like Pascal. We first write the parameter name, then a colon `:`, followed by its type. Similar to declaring variables, the colon serves as a visual cue that a type follows. However, unlike variables, we must always specify the type of a parameter to a method.)

[//]: # ()
[//]: # (```)

[//]: # (def first&#40;&#41; {)

[//]: # (    second&#40;112&#41;;)

[//]: # (})

[//]: # (def second&#40;a: int&#41; {)

[//]: # (    var x: int = a;)

[//]: # (    third&#40;x, false&#41;;)

[//]: # (})

[//]: # (def third&#40;a: int, b: bool&#41; {)

[//]: # (    fourth&#40;a, b&#41;;)

[//]: # (})

[//]: # (def fourth&#40;a: &#40;int, bool&#41;&#41; {)

[//]: # (    var x = a.0;)

[//]: # (    var y = a.1;)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (## Return Type ##)

[//]: # ()
[//]: # (Methods can have an optional return type. To specify a return type, we simply follow the parameters with an arrow `->` and then the return type. If a method has no declared return type, then it implicitly returns `void`.)

[//]: # ()
[//]: # (```)

[//]: # (def first&#40;&#41; {)

[//]: # (    return; // return of void)

[//]: # (})

[//]: # (def second&#40;&#41; -> int {)

[//]: # (    return 13; // return of value)

[//]: # (})

[//]: # (def third&#40;&#41; -> bool {)

[//]: # (    return false;)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (## Calls ##)

[//]: # ()
[//]: # (As we can see from the above examples, calls to methods have the usual syntax. We simply write its name, followed by the arguments in parentheses `&#40; ... &#41;`, separated by commas. The argument expressions are evaluated in left-to-right order and then the method is invoked.)

[//]: # ()
[//]: # (## Void ##)

[//]: # ()
[//]: # (Recall that `void` is just like any other type in Virgil. When it comes to methods, `void` can legally appear as the parameter type or return type of a method, and we can explicitly return the `void` value `&#40;&#41;` from within the body of the return. Thus, the statement `return;` is simply shorthand for `return &#40;&#41;;`.)

[//]: # ()
[//]: # (```)

[//]: # (def first&#40;&#41; { // implicitly returns void)

[//]: # (    second&#40;&#41;;)

[//]: # (})

[//]: # (def second&#40;&#41; -> void { // explicitly returns void)

[//]: # (    return third&#40;&#41;;)

[//]: # (})

[//]: # (def third&#40;v: void&#41; { // explicitly takes void)

[//]: # (    fourth&#40;&#41;;)

[//]: # (    return;)

[//]: # (})

[//]: # (def fourth&#40;v: void&#41; -> void {)

[//]: # (    return &#40;&#41;;)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (Better yet, `void` is so uniformly treated that we can actually _chain_ the invocations of `void` methods together in arbitrary ways.)

[//]: # ()
[//]: # (```)

[//]: # (def first&#40;&#41; {)

[//]: # (    return second&#40;&#41;; // second&#40;&#41; returns void)

[//]: # (})

[//]: # (def second&#40;&#41; {)

[//]: # (    third&#40;fourth&#40;&#41;&#41;; // fourth&#40;&#41; returns void and third&#40;&#41; accepts void)

[//]: # (    return System.puts&#40;"Second"&#41;; // puts&#40;&#41; returns void)

[//]: # (})

[//]: # (def third&#40;&#41; {)

[//]: # (    if &#40;true&#41; return fourth&#40;&#41;;)

[//]: # (    else return fifth&#40;&#41;;)

[//]: # (})

[//]: # (def fourth&#40;&#41; {)

[//]: # (    // ...)

[//]: # (})

[//]: # (def fifth&#40;&#41; {)

[//]: # (    // ...)

[//]: # (})

[//]: # (```)

[//]: # ()
[//]: # (Why is chaining in this way useful? It turns out that it works really well with [type parameters]&#40;Typeparams.md&#41;. Returning the implicit `void` result of another call can often save a line of code, for example, by shortening a branch to a single line, like in the example of the `third` method above.)
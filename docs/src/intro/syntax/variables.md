# Variables #

Variables store data, such as numbers and strings.

```
// Declaring a new variable `<type> <var_name>;`:
i32 i;
```

```
// Assigning a value to a variable `<var_name> = <value>;`:
i = 0;

// Variables can also be set to the result of an expression `<var_name> = <expression>;`:
i = 1 + 2;
i = add(1, 2) + 9; // (assuming that the `add` fn is in scope and returns an `i32`)
```

## Scopes ##

Each variable is associated with some _scope_, which is the range of the program in which it is active and accessible.
We will see how there are scopes tied to [functions](functions.md), [probes](probes.md), and [scripts](scripts.md).
The syntax for declaring and assigning to variables is consistent across these contexts.

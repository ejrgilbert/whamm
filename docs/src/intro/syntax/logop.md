# Logical Operators #

Logical operators allow joining multiple boolean expressions.
Like C/C++ and Java, the `&&` and `||` operators provide for logical-and and logical-or.
Both operators have _short-circuit evaluation_; they only evaluate the right-hand-side expression if the left-hand-side evaluates to `true` or `false`, respectively.

```
bool a;
a = false && false; // == false
a = false && true;  // == false
a = true && false;  // == false
a = true && true;   // == true
```

```
bool a;
a = false || false; // == false
a = false || true;  // == true
a = true || false;  // == true
a = true || true;   // == true
```

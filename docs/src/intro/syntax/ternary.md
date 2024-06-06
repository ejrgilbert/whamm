# Ternary Expressions #

`whamm!` supports a version of the "conditional" expression that chooses one of two values based on a condition.
The syntax follows C, C++, and Java, which uses `? :`.

```
// with declared types
int a;
a = 1 > 0 ? 16 : 27; // == 16
a = 1 < 0 ? 17 : 29; // == 29
```

## Short-circuit evaluation ##

The ternary expression will only evaluate the branch corresponding to the value of the condition.
In other words, it short-circuits.

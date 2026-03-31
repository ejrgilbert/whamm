# Tuples #

Tuples provide a quick and easy way to combine one or more values into a composite value.

```

var a: (i32, i32);
a = (0, 1);

var b: (bool, i32);
b = (true, 78);
```

## Member access ##

Tuple elements can be accessed as if they were fields of the tuple value.
Instead of field names, the elements are named as integer literals, starting with `0`.

```
// tuple members are accessed with '.' and numbered from 0
var a: (bool, i32);
a = (true, 1);

var b: bool;
b = a.0;    // == true

var c: i32;
c = a.1;    // == 1

var d: (i32, (i32, i32));
d = (1, (12, 13));

var e: i32;
e = d.1.0;  // == 12
```

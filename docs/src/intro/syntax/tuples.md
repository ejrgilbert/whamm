# Tuples #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

[//]: # (Tuples provide a quick and easy way to combine one or more values into a composite value.)

[//]: # ()
[//]: # (```)

[//]: # (&#40;i32, i32&#41; a;)

[//]: # (a = &#40;0, 1&#41;;)

[//]: # ()
[//]: # (&#40;bool, i32&#41; b;)

[//]: # (b = &#40;true, 78&#41;;)

[//]: # (```)

[//]: # ()
[//]: # (## Member access ##)

[//]: # ()
[//]: # (Tuple elements can be accessed as if they were fields of the tuple value.)

[//]: # (Instead of field names, the elements are named as integer literals, starting with `0`.)

[//]: # ()
[//]: # (```)

[//]: # (// tuple members are accessed with '.' and numbered from 0)

[//]: # (&#40;bool, i32&#41; a;)

[//]: # (a = &#40;true, 1&#41;;)

[//]: # ()
[//]: # (bool b;)

[//]: # (b = a.0;    // == true)

[//]: # ()
[//]: # (i32 c;)

[//]: # (c = a.1;    // == 1)

[//]: # ()
[//]: # (&#40;i32, &#40;i32, i32&#41;&#41; d;)

[//]: # (d = &#40;1, &#40;12, 13&#41;&#41;;)

[//]: # (i32 e;)

[//]: # (e = d.1.0;  // == 12)

[//]: # (```)

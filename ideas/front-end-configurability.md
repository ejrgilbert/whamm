# Front-End Configurability #

It would be nice to be able to configure the front end of the compiler easily (could support wizeng interface smoothly this way with no other changes).

```bash
whamm --provider provider.txt
```

Where `provider.txt`:
```text
linux:syscall:open(path: u32, mode:u32)
linux:syscall:read(static pid: u32)

...
```

There would need to be a way to define **variables** that are:
- static
- dynamic
- at what point they're supplied

There would need to be a way to define **functions** that are:
- static
- dynamic
- at what point they're supplied
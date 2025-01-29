# How to get state while an application is running? #

Answer: Adding query capabilities to the system API (could be the answer to bytecode rewriting as well).

Example:
```webassembly
(module
    ;; Call this function any time this specific syscall occurs
    ;; (for instrumenting wasm running in the kernel)
    (export "linux:syscall:open" (func 0))
    
    ;; Hook up this call to the VM with some query string
    ;; to be handled by this function (gives monitor status
    ;; before wasm:exit) 
    (export "query:foobar" (func 3))
    
    ;; Hook up this call to the VM to render some visualization
    ;; of monitor state (maybe interesting to do with Wasm threads?)
    ;; may also be better to just have external visualization app
    ;; do this that queries the "query:foobar" above.
    (export "render@fps" (func 4))
    
    ;; normal instrumentation capabilities:
    (export "wasm:opcode:..." (func 1))
    (export "wasm:exit" (func 2))
)
```
use alpha;
use whamm_core;

wasm:opcode:call:before {
    // Reproduces issue #305: passing a non-method call as an argument to a
    // bound user-library function used to panic in the type checker because
    // the inner call inherited the outer obj context (`alpha`) during arg
    // type-checking. Here the inner obj context (`whamm_core`) must win
    // over the outer (`alpha`) while arg type-checking/emission is happening.
    alpha.mem_free(whamm_core.mem_alloc(4));
}

// TODO -- provider definitions for:
//         1. DscriptCore
//         2. wasm (call, alt, etc.)

// provider defs should:
// 2. have functions to emit code for the fields/functions they provide

use walrus::Module;

pub trait Provider {
    fn emit_fn(&self, module: &mut Module, name: &String) -> bool;
}
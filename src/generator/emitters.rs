pub trait Emitter {
    // TODO -- flesh out required functions, follow syntax below
    // fn emit_probe(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WasmEmitter {
}

impl Emitter for WasmEmitter {
    // TODO
}

// TODO -- VirgilEmitter

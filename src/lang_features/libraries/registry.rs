use crate::common::error::ErrorGen;
use std::collections::{HashMap, HashSet};
use wasi_common::WasiCtx;
use wasi_common::sync::{WasiCtxBuilder, add_to_linker};
use wasmtime::*;

pub(crate) struct WasmService {
    store: Store<WasiCtx>,
    instance: Instance,
}

impl WasmService {
    fn new(engine: &Engine, module: &Module) -> Result<Self> {
        // Provide WASI imports/store (if there are any); all instances in the store
        // share this context. `WasiCtxBuilder` provides a number of ways to
        // configure what the target program will have access to.
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .inherit_env()?
            .build();

        let mut store = Store::new(engine, wasi);

        // Set up a linker that knows about WASI
        let mut linker = Linker::new(engine);
        add_to_linker(&mut linker, |ctx: &mut WasiCtx| ctx)?;

        // Instantiate the module with the linker (this links in WASI)
        let instance = linker.instantiate(&mut store, module)?;

        // let instance = Instance::new(&mut store, module, &[])?;
        Ok(Self { store, instance })
    }

    pub fn call(
        &mut self,
        lib_name: &str,
        func_name: &str,
        args: &[Val],
        results: &mut [Val],
        err: &mut ErrorGen,
    ) {
        if let Some(func) = self.instance.get_func(&mut self.store, func_name) {
            if let Err(e) = func.call(&mut self.store, args, results) {
                err.add_internal_error(
                    &format!("Failed to call function {lib_name}.{func_name}: {}", e),
                    &None,
                );
            }
        } else {
            err.add_internal_error(
                &format!("Could not find function for {lib_name}.{func_name}"),
                &None,
            );
        }
    }
}

#[derive(Default)]
pub(crate) struct WasmRegistry {
    services: HashMap<String, Box<WasmService>>,
}

impl WasmRegistry {
    pub(crate) fn new(
        static_libs: &HashSet<String>,
        user_libs: &HashMap<String, String>, // name -> path
        err: &mut ErrorGen,
    ) -> Self {
        let engine = Engine::default();

        let mut services = HashMap::new();
        for static_lib in static_libs.iter() {
            if let Some(path) = user_libs.get(static_lib) {
                let module = Module::from_file(&engine, path).unwrap();
                let service = WasmService::new(&engine, &module).unwrap();
                services.insert(static_lib.clone(), Box::new(service));
            } else {
                err.add_internal_error(
                    &format!("Could not find user library for static lib: {static_lib}"),
                    &None,
                );
            }
        }

        Self { services }
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut WasmService> {
        self.services.get_mut(name).map(|b| b.as_mut())
    }
}

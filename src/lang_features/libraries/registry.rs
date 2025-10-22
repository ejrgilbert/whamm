use wasmtime::*;
use std::collections::{HashMap, HashSet};

pub(crate) struct WasmService {
    store: Store<()>,
    instance: Instance,
}

impl WasmService {
    fn new(engine: &Engine, module: &Module) -> Result<Self> {
        let mut store = Store::new(engine, ());
        let instance = Instance::new(&mut store, module, &[])?;
        Ok(Self { store, instance })
    }

    pub fn call(&mut self, lib_name: &str, func_name: &str, args: &[Val], results: &mut [Val]) {
        if let Some(func) = self.instance.get_func(&mut self.store, func_name) {
            if let Err(e) = func.call(&mut self.store, args, results) {
                // TODO -- make this an internal error
                panic!("[internal error] Failed to call function {lib_name}.{func_name}: {}", e);
            }
        } else {
            panic!("[internal error] Could not find function for {lib_name}.{func_name}")
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
        user_libs: &HashMap<String, String> // name -> path
    ) -> Self {
        let engine = Engine::default();

        let mut services = HashMap::new();
        for static_lib in static_libs.iter() {
            if let Some(path) = user_libs.get(static_lib) {
                let module = Module::from_file(&engine, &path).unwrap();
                let service = WasmService::new(&engine, &module).unwrap();
                services.insert(static_lib.clone(), Box::new(service));
            } else {
                panic!("[internal error] Could not find user library for static lib: {static_lib}");
            }
        }

        Self { services }
    }

    fn insert(&mut self, name: impl Into<String>, service: WasmService) {
        self.services.insert(name.into(), Box::new(service));
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut WasmService> {
        self.services.get_mut(name).map(|b| b.as_mut())
    }
}

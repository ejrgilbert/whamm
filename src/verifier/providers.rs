use crate::verifier::types;

use types::SymbolTable;

use log::error;
use std::process::exit;

fn split_name(name: &String) -> (String, String, String, String) {
    // Get the parts of the name
    let parts: Vec<&str> = name.split(":").collect();
    let provider = parts.get(0).unwrap().to_uppercase();
    let module = parts.get(1).unwrap().to_uppercase();
    let function = parts.get(2).unwrap().to_uppercase();
    let ty = parts.get(3).unwrap().to_uppercase();

    (provider, module, function, ty)
}

pub trait Provider {
    fn add_symbols(&self, table: &mut SymbolTable) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DtraceCoreProvider {
    methods: Vec<String>
}

impl DtraceCoreProvider {
    pub(crate) fn new() -> Self {
        DtraceCoreProvider {
            methods: vec![
                "strcmp".to_string()
            ]
        }
    }
}

impl Provider for DtraceCoreProvider {
    fn add_symbols(&self, table: &mut SymbolTable) -> bool {
        // TODO -- might need to add VarRecords for the params?
        for method in self.methods.iter() {
            table.add_core_method(method.clone());
            table.exit_scope();
        }
        true
    }
}

// ===================
// = PROBE PROVIDERS =
// ===================

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProbeProvider {}

impl ProbeProvider {
    pub(crate) fn new() -> Self {
        ProbeProvider {}
    }
}

impl Provider for ProbeProvider {
    fn add_symbols(&self, table: &mut SymbolTable) -> bool {
        let (provider, _module, _function, _ty) = split_name(table.get_curr_scope_name());

        match provider.as_str() {
            "WASM" => {
                // add symbols from the Wasm provider
                let wasm_provider = WasmProvider::new();
                return wasm_provider.add_symbols(table);
            },
            "BEGIN" | "END" => {
                // ignore, nothing to do for these at the moment
            },
            _ => {
                error!("Unsupported probe provider: {}", table.get_curr_scope_name());
                exit(1);
            }
        }
        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct WasmProvider {}

impl WasmProvider {
    pub(crate) fn new() -> Self {
        WasmProvider {}
    }
}

impl Provider for WasmProvider {
    fn add_symbols(&self, table: &mut SymbolTable) -> bool {
        let (_provider, _module, function, _ty) = split_name(table.get_curr_scope_name());

        // Currently there are no WasmProvider-specific symbols

        match function.as_str() {
            "CALL" => {
                // add symbols from the Wasm provider
                let call_provider = CallProvider::new();
                return call_provider.add_symbols(table);
            },
            _ => {
                error!("Unsupported probe function: {}", table.get_curr_scope_name());
                exit(1);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CallProvider {
    locals: Vec<String>
}

impl CallProvider {
    pub fn new() -> Self {
        CallProvider {
            locals: vec![
                "target_fn_type".to_string(),
                "target_fn_module".to_string(),
                "target_fn_name".to_string(),
                "new_target_fn_name".to_string() // TODO -- should this be in an Alt provider?
            ]
        }
    }
}

impl Provider for CallProvider {
    fn add_symbols(&self, table: &mut SymbolTable) -> bool {
        for local in self.locals.iter() {
            table.add_probe_local(local.clone());
        }
        true
    }
}
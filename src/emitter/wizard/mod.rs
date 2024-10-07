use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::Module;
use crate::emitter::report_var_metadata::ReportVarMetadata;
use crate::emitter::rewriting::module_emitter::MemoryTracker;
use crate::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::verifier::types::SymbolTable;

// TODO -- see: https://github.com/ejrgilbert/whamm/blob/0e8336956eb7d6a0ab741147576ba0f5dcdac1ca/src/emitter/wizard/module_emitter.rs

pub struct WizardEmitter<'a, 'b, 'c, 'd, 'e, 'f> {
    pub wasm: &'a mut Module<'b>,
    pub emitting_func: Option<FunctionBuilder<'b>>,
    pub table: &'c mut SymbolTable,
    mem_tracker: &'d mut MemoryTracker,
    pub map_lib_adapter: &'e mut MapLibAdapter,
    pub report_var_metadata: &'f mut ReportVarMetadata,
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f> WizardEmitter<'a, 'b, 'c, 'd, 'e, 'f> {
    pub fn new(
        wasm: &'a mut Module<'b>,
        table: &'c mut SymbolTable,
        mem_tracker: &'d mut MemoryTracker,
        map_lib_adapter: &'e mut MapLibAdapter,
        report_var_metadata: &'f mut ReportVarMetadata,
    ) -> Self {
        Self {
            wasm,
            emitting_func: None,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            table,
            fn_providing_contexts: vec!["whamm".to_string()],
        }
    }
}
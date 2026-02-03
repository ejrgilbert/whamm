use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::generator::ast::{AstVisitor, Metadata, Probe, Script, WhammParam};
use crate::lang_features::libraries::core::utils::utils_adapter::UtilsAdapter;
use crate::lang_features::libraries::core::{LibAdapter, LibPackage};
use crate::parser::types::{Block, DataType, Expr, Statement};
use wirm::ir::id::FunctionID;
use wirm::Module;

pub mod utils_adapter;

pub struct UtilsPackage {
    is_used: bool,
    pub adapter: UtilsAdapter,
}
impl UtilsPackage {
    pub fn new(_: u32) -> Self {
        Self {
            is_used: false,
            adapter: UtilsAdapter::new(),
        }
    }
}

impl LibPackage for UtilsPackage {
    fn is_used(&self) -> bool {
        self.is_used
    }
    fn is_used_in_global_scope(&self) -> bool {
        false // doesn't matter
    }
    fn import_memory(&self) -> bool {
        false
    }
    fn set_lib_mem_id(&mut self, _: i32) {}
    fn set_instr_mem_id(&mut self, _: i32) {}
    fn get_adapter(&self) -> &dyn LibAdapter {
        &self.adapter
    }
    fn get_adapter_mut(&mut self) -> &mut dyn LibAdapter {
        &mut self.adapter
    }
    fn set_adapter_usage(&mut self, is_used: bool) {
        self.adapter.is_used = is_used;
    }
    fn set_global_adapter_usage(&mut self, _is_used: bool) {
        // nothing to do here
    }
    fn define_helper_funcs(
        &mut self,
        utils: &UtilsAdapter,
        mem_allocator: &mut MemoryAllocator,
        app_wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        self.adapter
            .define_helper_funcs(utils, mem_allocator, app_wasm, err)
    }
}
impl AstVisitor<bool> for UtilsPackage {
    fn visit_ast(&mut self, _: &[Script]) -> bool {
        unreachable!()
    }

    fn visit_script(&mut self, _: &Script) -> bool {
        unreachable!()
    }

    fn visit_probe(&mut self, _: &Probe) -> bool {
        unreachable!()
    }

    fn visit_metadata(&mut self, _: &Metadata) -> bool {
        unreachable!()
    }

    fn visit_whamm_param(&mut self, _: &WhammParam) -> bool {
        unreachable!()
    }

    fn visit_fn(&mut self, _: &crate::parser::types::Fn) -> bool {
        unreachable!()
    }

    fn visit_formal_param(&mut self, _: &(Expr, DataType)) -> bool {
        unreachable!()
    }

    fn visit_block(&mut self, _: &Block) -> bool {
        unreachable!()
    }

    fn visit_stmt(&mut self, _: &Statement) -> bool {
        unreachable!()
    }

    fn visit_datatype(&mut self, _: &DataType) -> bool {
        unreachable!()
    }
}

pub mod map_adapter;

use crate::common::error::ErrorGen;
use crate::emitter::InjectStrategy;
use crate::generator::ast::{AstVisitor, Metadata, Probe, Script, WhammParam};
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::core::{LibAdapter, LibPackage};
use crate::parser::types::{Block, DataType, Expr, Statement};
use log::debug;
use wirm::Module;
use wirm::ir::id::FunctionID;

pub struct MapLibPackage {
    strategy: InjectStrategy,
    is_used: bool,
    pub used_in_global_scope: bool,
    pub adapter: MapLibAdapter,
}
impl MapLibPackage {
    pub fn new(strategy: InjectStrategy) -> Self {
        Self {
            strategy,
            is_used: false,
            used_in_global_scope: false,
            adapter: MapLibAdapter::default(),
        }
    }
}
impl LibPackage for MapLibPackage {
    fn is_used(&self) -> bool {
        self.is_used
    }

    fn is_used_in_global_scope(&self) -> bool {
        self.used_in_global_scope
    }

    fn import_memory(&self) -> bool {
        true
    }
    fn set_lib_mem_id(&mut self, mem_id: i32) {
        self.adapter.lib_mem = mem_id;
    }
    fn set_instr_mem_id(&mut self, mem_id: i32) {
        self.adapter.instr_mem = mem_id;
    }

    fn get_fn_names(&self) -> Vec<String> {
        self.adapter.get_fn_names()
    }

    fn add_fid_to_adapter(&mut self, fname: &str, fid: u32) {
        self.adapter.add_fid(fname, fid);
    }
    fn set_adapter_usage(&mut self, is_used: bool) {
        self.adapter.is_used = is_used;
    }
    fn set_global_adapter_usage(&mut self, is_used: bool) {
        self.adapter.used_in_global_scope = is_used;
    }

    fn define_helper_funcs(
        &mut self,
        app_wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        self.adapter.define_helper_funcs(app_wasm, err)
    }
}
impl AstVisitor<bool> for MapLibPackage {
    fn visit_ast(&mut self, ast: &[Script]) -> bool {
        // visit scripts
        for script in ast.iter() {
            self.is_used |= self.visit_script(script);
            if self.is_used {
                return true;
            }
        }
        self.is_used
    }

    fn visit_script(&mut self, script: &Script) -> bool {
        // visit user-defined functions
        for f in script.fns.iter() {
            if self.visit_fn(f) {
                return true;
            }
        }

        // visit globals
        for (name, global) in script.globals.iter() {
            if let DataType::Map { .. } = global.ty {
                debug!("{name} is a map!");
                self.used_in_global_scope = true;
                return true;
            }
        }

        // visit global statements
        for stmt in script.global_stmts.iter() {
            if self.visit_stmt(stmt) {
                self.used_in_global_scope = true;
                return true;
            }
        }

        // visit probes
        // visit ALL!! so we can see if there's global scope maps
        let mut has_maps = false;
        for probe in script.probes.iter() {
            has_maps |= self.visit_probe(probe);
        }
        has_maps
    }

    fn visit_probe(&mut self, probe: &Probe) -> bool {
        // visit ALL!! so we can see if there's global scope maps
        let mut has_maps = self.visit_metadata(&probe.metadata);
        if let Some(body) = &probe.body {
            for stmt in body.stmts.iter() {
                has_maps |= self.visit_stmt(stmt);
            }
        }
        has_maps
    }

    fn visit_metadata(&mut self, metadata: &Metadata) -> bool {
        for param in metadata.pred_args.params.iter() {
            if self.visit_whamm_param(param) {
                return true;
            }
        }
        for param in metadata.body_args.params.iter() {
            if self.visit_whamm_param(param) {
                return true;
            }
        }
        false
    }

    fn visit_whamm_param(&mut self, param: &WhammParam) -> bool {
        self.visit_datatype(&param.ty)
    }

    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> bool {
        for param in f.params.iter() {
            if self.visit_formal_param(param) {
                return true;
            }
        }
        if self.visit_datatype(&f.results) {
            return true;
        }
        if self.visit_block(&f.body) {
            return true;
        }
        false
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> bool {
        if self.visit_datatype(&param.1) {
            return true;
        }
        false
    }

    fn visit_block(&mut self, block: &Block) -> bool {
        // visit ALL!! so we can see if there's global scope maps
        let mut has_maps = false;
        for stmt in &block.stmts {
            has_maps |= self.visit_stmt(stmt);
        }
        has_maps
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> bool {
        match stmt {
            Statement::Decl {
                ty: DataType::Map { .. },
                var_id,
                ..
            } => {
                if let Expr::VarId { name, .. } = var_id {
                    debug!("{name} is a map!");
                }
                true
            }
            Statement::UnsharedDecl { decl, .. } => {
                if let Statement::Decl {
                    ty: DataType::Map { .. },
                    var_id,
                    ..
                } = &**decl
                {
                    if let Expr::VarId { name, .. } = var_id {
                        debug!("{name} is a map!");
                    }
                    if matches!(self.strategy, InjectStrategy::Rewriting) {
                        // TODO -- this needs to change when I refactor to use
                        //    allocated memory for probes! (that's the reason it's true here)
                        self.used_in_global_scope = true;
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> bool {
        if let DataType::Map { .. } = datatype {
            return true;
        }
        false
    }
}

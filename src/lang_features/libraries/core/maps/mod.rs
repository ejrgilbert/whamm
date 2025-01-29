pub mod map_adapter;

use crate::common::error::ErrorGen;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::core::{LibAdapter, LibPackage};
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Expr, Script, Statement, UnOp, Value, Whamm, WhammVisitor,
};
use log::debug;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::Module;

#[derive(Default)]
pub struct MapLibPackage {
    is_used: bool,
    pub adapter: MapLibAdapter,
}
impl LibPackage for MapLibPackage {
    fn is_used(&self) -> bool {
        self.is_used
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

    fn define_helper_funcs(
        &mut self,
        app_wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        self.adapter.define_helper_funcs(app_wasm, err)
    }
}
impl WhammVisitor<bool> for MapLibPackage {
    fn visit_whamm(&mut self, whamm: &Whamm) -> bool {
        // visit scripts
        for script in whamm.scripts.iter() {
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
                return true;
            }
        }

        // visit global statements
        for stmt in script.global_stmts.iter() {
            if self.visit_stmt(stmt) {
                return true;
            }
        }

        // visit providers
        for (_name, provider) in script.providers.iter() {
            if self.visit_provider(provider) {
                return true;
            }
        }
        false
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> bool {
        if provider.requires_map_lib() {
            return true;
        }
        for package in provider.packages() {
            if self.visit_package(package) {
                return true;
            }
        }
        false
    }

    fn visit_package(&mut self, package: &dyn Package) -> bool {
        if package.requires_map_lib() {
            return true;
        }
        for event in package.events() {
            if self.visit_event(event) {
                return true;
            }
        }
        false
    }

    fn visit_event(&mut self, event: &dyn Event) -> bool {
        if event.requires_map_lib() {
            return true;
        }
        for (_mode, probe_list) in event.probes().iter() {
            for probe in probe_list.iter() {
                if self.visit_probe(probe) {
                    return true;
                }
            }
        }
        false
    }

    fn visit_probe(&mut self, probe: &Box<dyn Probe>) -> bool {
        if let Some(body) = &probe.body() {
            for stmt in body.stmts.iter() {
                if self.visit_stmt(stmt) {
                    return true;
                }
            }
        }
        false
    }

    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> bool {
        for param in f.params.iter() {
            if self.visit_formal_param(param) {
                return true;
            }
        }
        if self.visit_datatype(&f.return_ty) {
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
        for stmt in &block.stmts {
            if self.visit_stmt(stmt) {
                return true;
            }
        }
        false
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> bool {
        if let Statement::Decl {
            ty: DataType::Map { .. },
            var_id,
            ..
        } = stmt
        {
            if let Expr::VarId { name, .. } = var_id {
                debug!("{name} is a map!");
            }
            return true;
        }
        false
    }

    fn visit_expr(&mut self, _expr: &Expr) -> bool {
        // can just check at variable declaration level.
        unreachable!()
    }

    fn visit_unop(&mut self, _unop: &UnOp) -> bool {
        // can just check at variable declaration level.
        unreachable!()
    }

    fn visit_binop(&mut self, _binop: &BinOp) -> bool {
        // can just check at variable declaration level.
        unreachable!()
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> bool {
        if let DataType::Map { .. } = datatype {
            return true;
        }
        false
    }

    fn visit_value(&mut self, _val: &Value) -> bool {
        // can just check at variable declaration level.
        unreachable!()
    }
}

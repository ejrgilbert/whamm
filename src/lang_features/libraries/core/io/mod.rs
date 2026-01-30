pub mod io_adapter;

use crate::common::error::ErrorGen;
use crate::generator::ast::{AstVisitor, Metadata, Probe, Script, WhammParam};
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::{LibAdapter, LibPackage};
use crate::parser::types::{Block, DataType, Expr, Statement};
use log::debug;
use wirm::Module;
use wirm::ir::id::FunctionID;

pub struct IOPackage {
    is_used: bool,
    pub adapter: IOAdapter,
}
impl IOPackage {
    pub fn new(mem_tracker_global: u32) -> Self {
        Self {
            is_used: false,
            adapter: IOAdapter::new(mem_tracker_global),
        }
    }
}

impl LibPackage for IOPackage {
    fn is_used(&self) -> bool {
        self.is_used
    }
    fn is_used_in_global_scope(&self) -> bool {
        false // doesn't matter
    }
    fn import_memory(&self) -> bool {
        false
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
    fn set_global_adapter_usage(&mut self, _is_used: bool) {
        // nothing to do here
    }
    fn define_helper_funcs(
        &mut self,
        app_wasm: &mut Module,
        err: &mut ErrorGen,
    ) -> Vec<FunctionID> {
        self.adapter.define_helper_funcs(app_wasm, err)
    }
}
impl AstVisitor<bool> for IOPackage {
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
            if global.report {
                debug!("{name} is a report variable!");
                return true;
            }
        }

        // visit global statements
        for stmt in script.global_stmts.iter() {
            if self.visit_stmt(stmt) {
                return true;
            }
        }

        // visit probes
        for probe in script.probes.iter() {
            if self.visit_probe(probe) {
                return true;
            }
        }
        false
    }

    fn visit_probe(&mut self, probe: &Probe) -> bool {
        if let Some(body) = &probe.body {
            for stmt in body.stmts.iter() {
                if self.visit_stmt(stmt) {
                    return true;
                }
            }
        }
        false
    }

    fn visit_metadata(&mut self, _: &Metadata) -> bool {
        unreachable!()
    }

    fn visit_whamm_param(&mut self, _: &WhammParam) -> bool {
        unreachable!()
    }

    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> bool {
        self.visit_block(&f.body)
    }

    fn visit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        unreachable!()
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
        match stmt {
            Statement::UnsharedDeclInit { decl, .. } => self.visit_stmt(decl),
            Statement::UnsharedDecl { is_report, .. } => *is_report,
            _ => false,
        }
    }

    fn visit_datatype(&mut self, _datatype: &DataType) -> bool {
        // can just check at variable declaration level.
        unreachable!()
    }
}

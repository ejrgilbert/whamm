pub mod io_adapter;

use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::{LibAdapter, LibPackage};
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Expr, Script, Statement, UnOp, Value, Whamm, WhammVisitor,
};
use log::debug;

#[derive(Default)]
pub struct IOPackage {
    is_used: bool,
    pub adapter: IOAdapter,
}
impl LibPackage for IOPackage {
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
}
impl WhammVisitor<bool> for IOPackage {
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

        // visit providers
        for (_name, provider) in script.providers.iter() {
            if self.visit_provider(provider) {
                return true;
            }
        }
        false
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> bool {
        for package in provider.packages() {
            if self.visit_package(package) {
                return true;
            }
        }
        false
    }

    fn visit_package(&mut self, package: &dyn Package) -> bool {
        for event in package.events() {
            if self.visit_event(event) {
                return true;
            }
        }
        false
    }

    fn visit_event(&mut self, event: &dyn Event) -> bool {
        for (_mode, probe_list) in event.probes().iter() {
            for probe in probe_list {
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
        if let Statement::AllocDecl { is_report, .. } = stmt {
            *is_report
        } else {
            false
        }
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

    fn visit_datatype(&mut self, _datatype: &DataType) -> bool {
        // can just check at variable declaration level.
        unreachable!()
    }

    fn visit_value(&mut self, _val: &Value) -> bool {
        // can just check at variable declaration level.
        unreachable!()
    }
}
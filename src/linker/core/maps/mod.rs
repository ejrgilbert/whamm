pub mod map_lib_adapter;

use std::collections::HashSet;
use log::{debug, trace};
use crate::linker::core::{LibAdapter, LibPackage};
use crate::linker::core::maps::map_lib_adapter::MapLibAdapter;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{BinOp, Block, DataType, Expr, Script, Statement, UnOp, Value, Whamm, WhammVisitor};

#[derive(Default)]
pub struct MapLibPackage {
    is_used: bool,
    adapter: MapLibAdapter
}
impl WhammVisitor<()> for MapLibPackage {
    fn visit_whamm(&mut self, whamm: &Whamm) -> () {
        trace!("Entering: MapLibPackage::visit_whamm");
        // visit scripts
        whamm
            .scripts
            .iter()
            .for_each(|script| self.visit_script(script));
        trace!("Exiting: MapLibPackage::visit_whamm");
    }

    fn visit_script(&mut self, script: &Script) -> () {
        trace!("Entering: MapLibPackage::visit_script");
        // visit user-defined functions
        for f in script.fns.iter() {
            self.visit_fn(f);
        }

        // visit globals
        for (name, global) in script.globals.iter() {
            match global.ty {
                DataType::Map {..} => {
                    debug!("{name} is a map!");
                    self.is_used = true
                },
                _ => {}
            }
        }

        // visit global statements
        for stmt in script.global_stmts.iter() {
            self.visit_stmt(stmt);
        }

        // visit providers
        script
            .providers
            .iter()
            .for_each(|(_name, provider)| self.visit_provider(provider));
        trace!("Exiting: MapLibPackage::visit_script");
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> () {
        trace!("Entering: MapLibPackage::visit_provider");
        provider
            .packages()
            .for_each(|package| self.visit_package(package));
        trace!("Exiting: MapLibPackage::visit_provider");
    }

    fn visit_package(&mut self, package: &dyn Package) -> () {
        trace!("Entering: MapLibPackage::visit_package");
        package.events().for_each(|event| {
            self.visit_event(event);
        });
        trace!("Exiting: MapLibPackage::visit_package");
    }

    fn visit_event(&mut self, event: &dyn Event) -> () {
        trace!("Entering: MapLibPackage::visit_event");
        event.probes().iter().for_each(|(_mode, probe_list)| {
            probe_list.iter().for_each(|probe| {
                self.visit_probe(probe);
            });
        });
        trace!("Entering: MapLibPackage::visit_event");
    }

    fn visit_probe(&mut self, probe: &Box<dyn Probe>) -> () {
        trace!("Entering: MapLibPackage::visit_probe");
        for stmt in &probe.body().as_ref().unwrap().stmts {
            self.visit_stmt(stmt);
        }
        trace!("Entering: MapLibPackage::visit_probe");
    }

    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> () {
        trace!("Entering: MapLibPackage::visit_fn");
        for param in f.params.iter() {
            self.visit_formal_param(param);
        }
        self.visit_datatype(&f.return_ty);
        trace!("Entering: MapLibPackage::visit_fn");
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> () {
        trace!("Entering: MapLibPackage::visit_formal_param");
        self.visit_datatype(&param.1);
        trace!("Entering: MapLibPackage::visit_formal_param");
    }

    fn visit_block(&mut self, block: &Block) -> () {
        trace!("Entering: MapLibPackage::visit_block");
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
        trace!("Exiting: MapLibPackage::visit_block");
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> () {
        trace!("Entering: MapLibPackage::visit_stmt");
        if let Statement::Decl {ty: DataType::Map {..}, var_id, ..} = stmt {
            if let Expr::VarId {name, ..} = var_id {
                debug!("{name} is a map!");
            }
            self.is_used = true;
        }
        trace!("Exiting: MapLibPackage::visit_stmt");
    }

    fn visit_expr(&mut self, _expr: &Expr) -> () {
        // can just check at variable declaration level.
        unreachable!()
    }

    fn visit_unop(&mut self, _unop: &UnOp) -> () {
        // can just check at variable declaration level.
        unreachable!()
    }

    fn visit_binop(&mut self, _binop: &BinOp) -> () {
        // can just check at variable declaration level.
        unreachable!()
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> () {
        trace!("Entering: MapLibPackage::visit_datatype");
        if let DataType::Map {..} = datatype {
            self.is_used = true;
        }
        trace!("Exiting: MapLibPackage::visit_datatype");
    }

    fn visit_value(&mut self, _val: &Value) -> () {
        // can just check at variable declaration level.
        unreachable!()
    }
}
impl LibPackage for MapLibPackage {
    fn is_used(&self) -> bool {
        self.is_used
    }
    // fn get_lib_adapter(&self) -> Box<&dyn LibAdapter> {
    //     Box::new(&self.adapter)
    // }
    fn get_fn_names(&self) -> &HashSet<String> {
        self.adapter.get_fn_names()
    }
}
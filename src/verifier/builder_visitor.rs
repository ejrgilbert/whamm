use std::collections::HashMap;
use crate::parser::types as parser_types;
use parser_types::{DataType, Dscript, Dtrace, DtraceVisitor, Expr, Fn, Function, Module, Op, Probe, Provider, Statement, Value};
use crate::verifier::types::{Record, ScopeType, SymbolTable};

use log::{error, trace};

pub struct SymbolTableBuilder {
    pub table: SymbolTable,

    // TODO -- these should be updated as they are entered/exited
    curr_dtrace: Option<usize>,   // indexes into this::table::records
    curr_dscript: Option<usize>,  // indexes into this::table::records
    curr_provider: Option<usize>, // indexes into this::table::records
    curr_module: Option<usize>,   // indexes into this::table::records
    curr_function: Option<usize>, // indexes into this::table::records
    curr_probe: Option<usize>,    // indexes into this::table::records

    curr_fn: Option<usize>,       // indexes into this::table::records
}
impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            table: SymbolTable::new(),
            curr_dtrace: None,
            curr_dscript: None,
            curr_provider: None,
            curr_module: None,
            curr_function: None,
            curr_probe: None,
            curr_fn: None,
        }
    }

    fn get_var_name(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::VarId {name} => Some(name.clone()),
            _ => None
        }
    }

    fn add_dscript(&mut self, dscript: &Dscript) {
        if self.table.lookup(&dscript.name).is_some() {
            error!("duplicated dscript [ {} ]", &dscript.name);
        }

        // create record
        let dscript_rec = Record::Dscript {
            name: dscript.name.clone(),
            fns: vec![],
            globals: vec![],
            providers: vec![],
        };

        // Add dscript to scope
        let id = self.table.put(dscript.name.clone(), dscript_rec);

        // Add dscript to current dtrace record
        match self.table.get_record_mut(self.curr_dtrace.unwrap()).unwrap() {
            Record::Dtrace { dscripts, .. } => {
                dscripts.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter dscript scope
        self.table.enter_scope();

        // set scope name and type
        self.table.set_curr_scope_info(dscript.name.clone(), ScopeType::Dscript);
        self.table.set_curr_dscript(id.clone());
    }

    fn add_provider(&mut self, provider: &Provider) {
        if self.table.lookup(&provider.name).is_some() {
            error!("duplicated provider [ {} ]", &provider.name);
        }

        // create record
        let provider_rec = Record::Provider {
            name: provider.name.clone(),
            fns: vec![],
            globals: vec![],
            modules: vec![],
        };

        // Add provider to scope
        let id = self.table.put(provider.name.clone(), provider_rec);

        // Add provider to current dscript record
        match self.table.get_record_mut(self.curr_dscript.unwrap()).unwrap() {
            Record::Dscript { providers, .. } => {
                providers.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter provider scope
        self.table.enter_scope();

        // set scope name and type
        self.table.set_curr_scope_info(provider.name.clone(), ScopeType::Provider);
    }

    fn add_module(&mut self, module: &Module) {
        if self.table.lookup(&module.name).is_some() {
            error!("duplicated module [ {} ]", &module.name);
        }

        // create record
        let module_rec = Record::Module {
            name: module.name.clone(),
            fns: vec![],
            globals: vec![],
            functions: vec![],
        };

        // Add module to scope
        let id = self.table.put(module.name.clone(), module_rec);

        // Add module to current provider record
        match self.table.get_record_mut(self.curr_provider.unwrap()).unwrap() {
            Record::Provider { modules, .. } => {
                modules.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter module scope
        self.table.enter_scope();

        // set scope name and type
        self.table.set_curr_scope_info(module.name.clone(), ScopeType::Module);
    }

    fn add_function(&mut self, function: &Function) {
        if self.table.lookup(&function.name).is_some() {
            error!("duplicated function [ {} ]", &function.name);
        }

        // create record
        let function_rec = Record::Function {
            name: function.name.clone(),
            fns: vec![],
            globals: vec![],
            probes: vec![],
        };

        // Add function to scope
        let id = self.table.put(function.name.clone(), function_rec);

        // Add function to current module record
        match self.table.get_record_mut(self.curr_module.unwrap()).unwrap() {
            Record::Module { functions, .. } => {
                functions.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter provider scope
        self.table.enter_scope();

        // set scope name and type
        self.table.set_curr_scope_info(function.name.clone(), ScopeType::Function);
    }

    fn add_probe(&mut self, probe: &Probe) {
        if self.table.lookup(&probe.name).is_some() {
            error!("duplicated probe [ {} ]", &probe.name);
        }

        // create record
        let probe_rec = Record::Probe {
            name: probe.name.clone(),
            fns: vec![],
            globals: vec![],
        };

        // Add probe to scope
        let id = self.table.put(probe.name.clone(), probe_rec);

        // Add probe to current function record
        match self.table.get_record_mut(self.curr_function.unwrap()).unwrap() {
            Record::Function { probes, .. } => {
                probes.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter provider scope
        self.table.enter_scope();

        // set scope name and type
        self.table.set_curr_scope_info(probe.name.clone(), ScopeType::Probe);
    }

    fn add_fn(&mut self, f: &Fn) {
        if self.table.lookup(&f.name).is_some() {
            error!("duplicated fn [ {} ]", &f.name);
        }

        // create record
        let fn_rec = Record::Fn {
            name: f.name.clone(),
            params: vec![],
        };

        // Add fn to scope
        self.table.put(f.name.clone(), fn_rec);

        // TODO add fn record to the right context

        // enter fn scope
        self.table.enter_scope();

        // set scope name and type
        self.table.set_curr_scope_info(f.name.clone(), ScopeType::Fn);
    }

    /// Insert `global` record into scope
    fn add_global(&mut self, ty: DataType, name: String) {
        if self.table.lookup(&name).is_some() {
            error!("duplicated identifier [ {name} ]");
        }

        // TODO add global record to the right context

        // add global
        self.table.put(name.clone(), Record::Var {
            ty,
            name,
        });
    }

    fn visit_globals(&mut self, globals: &HashMap<(DataType, Expr), Option<Value>>) {
        for ((ty, expr), _val) in globals.iter() {
            let name = self.get_var_name(expr).unwrap();
            self.add_global(ty.clone(), name);
        }
    }
}

impl DtraceVisitor<()> for SymbolTableBuilder {
    fn visit_dtrace(&mut self, dtrace: &Dtrace) -> () {
        trace!("Entering: visit_dtrace");
        self.table.set_curr_scope_info("dtrace".to_string(), ScopeType::Dtrace);

        // visit fns
        dtrace.fns.iter().for_each(| f | self.visit_fn(f) );

        // visit globals
        self.visit_globals(&dtrace.globals);

        // visit dscripts
        dtrace.dscripts.iter().for_each(| dscript | self.visit_dscript(dscript));

        trace!("Exiting: visit_dtrace");
    }

    fn visit_dscript(&mut self, dscript: &Dscript) -> () {
        trace!("Entering: visit_dscript");

        self.add_dscript(dscript);
        dscript.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&dscript.globals);
        dscript.providers.iter().for_each(| (_name, provider) | {
            self.visit_provider(provider)
        });
        // TODO visit probes (need to add these, can always be referenced via Scope::containing_dscript)

        trace!("Exiting: visit_dscript");
        self.table.exit_scope();
    }

    fn visit_provider(&mut self, provider: &Provider) -> () {
        trace!("Entering: visit_provider");

        self.add_provider(provider);
        provider.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&provider.globals);
        provider.modules.iter().for_each(| (_name, module) | {
            self.visit_module(module)
        });

        trace!("Exiting: visit_provider");
    }

    fn visit_module(&mut self, module: &Module) -> () {
        trace!("Entering: visit_module");

        self.add_module(module);
        module.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&module.globals);
        module.functions.iter().for_each(| (_name, function) | {
            self.visit_function(function)
        });

        trace!("Exiting: visit_module");
    }

    fn visit_function(&mut self, function: &Function) -> () {
        trace!("Entering: visit_function");

        self.add_function(function);
        function.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&function.globals);
        // TODO visit probe_map -- unsure how this is going to look?

        trace!("Exiting: visit_function");
    }

    fn visit_probe(&mut self, probe: &Probe) -> () {
        trace!("Entering: visit_probe");

        self.add_probe(probe);
        probe.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&probe.globals);

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_probe");
    }

    fn visit_fn(&mut self, f: &parser_types::Fn) -> () {
        trace!("Entering: visit_fn");

        // TODO add fn

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_fn");
    }

    fn visit_stmt(&mut self, _assign: &Statement) -> () {
        // Not visiting function/probe bodies
        unreachable!()
    }

    fn visit_expr(&mut self, _call: &Expr) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_op(&mut self, _op: &Op) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_datatype(&mut self, _datatype: &DataType) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_value(&mut self, _val: &Value) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }
}
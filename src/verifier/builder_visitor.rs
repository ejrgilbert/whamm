use std::collections::HashMap;
use crate::parser::types as parser_types;
use parser_types::{DataType, Whammy, Whamm, WhammVisitor, Expr, Fn, Event, Package, Op, Probe, Provider, Statement, Value};
use crate::verifier::types::{Record, ScopeType, SymbolTable};

use log::{error, trace};
use crate::parser::types::Global;

pub struct SymbolTableBuilder {
    pub table: SymbolTable,

    // TODO -- these should be updated as they are entered/exited
    curr_whamm: Option<usize>,   // indexes into this::table::records
    curr_whammy: Option<usize>,  // indexes into this::table::records
    curr_provider: Option<usize>, // indexes into this::table::records
    curr_package: Option<usize>,   // indexes into this::table::records
    curr_event: Option<usize>, // indexes into this::table::records
    curr_probe: Option<usize>,    // indexes into this::table::records

    curr_fn: Option<usize>,       // indexes into this::table::records
}
impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            table: SymbolTable::new(),
            curr_whamm: None,
            curr_whammy: None,
            curr_provider: None,
            curr_package: None,
            curr_event: None,
            curr_probe: None,
            curr_fn: None,
        }
    }

    fn add_whammy(&mut self, whammy: &Whammy) {
        if self.table.lookup(&whammy.name).is_some() {
            error!("duplicated whammy [ {} ]", &whammy.name);
        }

        // create record
        let whammy_rec = Record::Whammy {
            name: whammy.name.clone(),
            fns: vec![],
            globals: vec![],
            providers: vec![],
        };

        // Add whammy to scope
        let id = self.table.put(whammy.name.clone(), whammy_rec);

        // Add whammy to current whamm record
        match self.table.get_record_mut(&self.curr_whamm.unwrap()).unwrap() {
            Record::Whamm { whammys, .. } => {
                whammys.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter whammy scope
        self.table.enter_scope();
        self.curr_whammy = Some(id.clone());

        // set scope name and type
        self.table.set_curr_scope_info(whammy.name.clone(), ScopeType::Whammy);
        self.table.set_curr_whammy(id.clone());
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
            packages: vec![],
        };

        // Add provider to scope
        let id = self.table.put(provider.name.clone(), provider_rec);

        // Add provider to current whammy record
        match self.table.get_record_mut(&self.curr_whammy.unwrap()).unwrap() {
            Record::Whammy { providers, .. } => {
                providers.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter provider scope
        self.table.enter_scope();
        self.curr_provider = Some(id.clone());

        // set scope name and type
        self.table.set_curr_scope_info(provider.name.clone(), ScopeType::Provider);
    }

    fn add_package(&mut self, package: &Package) {
        if self.table.lookup(&package.name).is_some() {
            error!("duplicated package [ {} ]", &package.name);
        }

        // create record
        let package_rec = Record::Package {
            name: package.name.clone(),
            fns: vec![],
            globals: vec![],
            events: vec![],
        };

        // Add package to scope
        let id = self.table.put(package.name.clone(), package_rec);

        // Add package to current provider record
        match self.table.get_record_mut(&self.curr_provider.unwrap()).unwrap() {
            Record::Provider { packages, .. } => {
                packages.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter package scope
        self.table.enter_scope();
        self.curr_package = Some(id.clone());

        // set scope name and type
        self.table.set_curr_scope_info(package.name.clone(), ScopeType::Package);
    }

    fn add_event(&mut self, event: &Event) {
        if self.table.lookup(&event.name).is_some() {
            error!("duplicated event [ {} ]", &event.name);
        }

        // create record
        let event_rec = Record::Event {
            name: event.name.clone(),
            fns: vec![],
            globals: vec![],
            probes: vec![],
        };

        // Add event to scope
        let id = self.table.put(event.name.clone(), event_rec);

        // Add event to current package record
        match self.table.get_record_mut(&self.curr_package.unwrap()).unwrap() {
            Record::Package { events, .. } => {
                events.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter event scope
        self.table.enter_scope();
        self.curr_event = Some(id.clone());

        // set scope name and type
        self.table.set_curr_scope_info(event.name.clone(), ScopeType::Event);
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

        // Add probe to current event record
        match self.table.get_record_mut(&self.curr_event.unwrap()) {
            Some(Record::Event { probes, .. }) => {
                probes.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }

        // enter probe scope
        self.table.enter_scope();
        self.curr_probe = Some(id.clone());

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
            addr: None
        };

        // Add fn to scope
        let id = self.table.put(f.name.clone(), fn_rec);

        // add fn record to the current record
        self.add_fn_id_to_curr_rec(id);

        // enter fn scope
        self.table.enter_scope();
        self.curr_fn = Some(id.clone());

        // set scope name and type
        self.table.set_curr_scope_info(f.name.clone(), ScopeType::Fn);

        // visit parameters
        f.params.iter().for_each(| param | self.visit_formal_param(param));
    }

    fn add_fn_id_to_curr_rec(&mut self, id: usize) {
        match self.table.get_curr_rec_mut() {
            Some(Record::Whamm { fns, .. }) |
            Some(Record::Whammy { fns, .. }) |
            Some(Record::Provider { fns, .. }) |
            Some(Record::Package { fns, .. }) |
            Some(Record::Event { fns, .. }) |
            Some(Record::Probe { fns, .. }) => {
                fns.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn add_param(&mut self, var_id: &Expr, ty: &DataType) {
        let name = match var_id {
            Expr::VarId {name} => name,
            _ => {
                unreachable!();
            }
        };

        // create record
        let param_rec = Record::Var {
            name: name.clone(),
            ty: ty.clone(),
            value: None,
            addr: None
        };

        // add var to scope
        let id = self.table.put(name.clone(), param_rec);

        // add param to fn record
        match self.table.get_record_mut(&self.curr_fn.unwrap()) {
            Some(Record::Fn { params, .. }) => {
                params.push(id.clone());
            }
            _ => {
                unreachable!()
            }
        }
    }

    /// Insert `global` record into scope
    fn add_global(&mut self, ty: DataType, name: String) {
        if self.table.lookup(&name).is_some() {
            error!("duplicated identifier [ {} ]", name);
        }

        // Add global to scope
        let id = self.table.put(name.clone(), Record::Var {
            ty,
            name,
            value: None,
            addr: None
        });

        // add global record to the current record
        self.add_fn_id_to_curr_rec(id);
    }

    fn visit_globals(&mut self, globals: &HashMap<String, Global>) {
        for (name, global) in globals.iter() {
            self.add_global(global.ty.clone(), name.clone());
        }
    }
}

impl WhammVisitor<()> for SymbolTableBuilder {
    fn visit_whamm(&mut self, whamm: &Whamm) -> () {
        trace!("Entering: visit_whamm");
        let name: String = "whamm".to_string();
        self.table.set_curr_scope_info(name.clone(), ScopeType::Whamm);

        // add whamm record
        let whamm_rec = Record::Whamm {
            name: name.clone(),
            fns: vec![],
            globals: vec![],
            whammys: vec![],
        };

        // Add whamm to scope
        let id = self.table.put(name.clone(), whamm_rec);

        self.curr_whamm = Some(id);

        // visit fns
        whamm.fns.iter().for_each(| f | self.visit_fn(f) );

        // visit globals
        self.visit_globals(&whamm.globals);

        // visit whammys
        whamm.whammys.iter().for_each(| whammy | self.visit_whammy(whammy));

        trace!("Exiting: visit_whamm");
        self.curr_whamm = None;
    }

    fn visit_whammy(&mut self, whammy: &Whammy) -> () {
        trace!("Entering: visit_whammy");

        self.add_whammy(whammy);
        whammy.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&whammy.globals);
        whammy.providers.iter().for_each(| (_name, provider) | {
            self.visit_provider(provider)
        });

        trace!("Exiting: visit_whammy");
        self.table.exit_scope();
        self.curr_whammy = None;
    }

    fn visit_provider(&mut self, provider: &Provider) -> () {
        trace!("Entering: visit_provider");

        self.add_provider(provider);
        provider.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&provider.globals);
        provider.packages.iter().for_each(| (_name, package) | {
            self.visit_package(package)
        });

        trace!("Exiting: visit_provider");
        self.table.exit_scope();
        self.curr_provider = None;
    }

    fn visit_package(&mut self, package: &Package) -> () {
        trace!("Entering: visit_package");

        self.add_package(package);
        package.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&package.globals);
        package.events.iter().for_each(| (_name, event) | {
            self.visit_event(event)
        });

        trace!("Exiting: visit_package");
        self.table.exit_scope();
        self.curr_package = None;
    }

    fn visit_event(&mut self, event: &Event) -> () {
        trace!("Entering: visit_event");

        self.add_event(event);
        event.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&event.globals);

        // visit probe_map
        event.probe_map.iter().for_each(| probes | {
            probes.1.iter().for_each(| probe | {
                self.visit_probe(probe);
            });
        });

        trace!("Exiting: visit_event");
        self.table.exit_scope();
        self.curr_event = None;
    }

    fn visit_probe(&mut self, probe: &Probe) -> () {
        trace!("Entering: visit_probe");

        self.add_probe(probe);
        probe.fns.iter().for_each(| f | self.visit_fn(f) );
        self.visit_globals(&probe.globals);

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_probe");
        self.table.exit_scope();
        self.curr_probe = None;
    }

    // fn visit_predicate(&mut self, _predicate: &Expr) -> () {
    //     unimplemented!()
    // }

    fn visit_fn(&mut self, f: &Fn) -> () {
        trace!("Entering: visit_fn");

        // add fn
        self.add_fn(f);

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_fn");
        self.table.exit_scope();
        self.curr_fn = None;
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> () {
        trace!("Entering: visit_formal_param");

        // add param
        self.add_param(&param.0, &param.1);

        trace!("Exiting: visit_formal_param");
    }

    fn visit_stmt(&mut self, _assign: &Statement) -> () {
        // Not visiting event/probe bodies
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
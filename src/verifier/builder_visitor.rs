use crate::parser::types as parser_types;
use crate::verifier::builder_visitor::parser_types::Location;
use crate::verifier::types::{Record, ScopeType, SymbolTable};
use crate::verifier::verifier::check_duplicate_id;
use parser_types::{BinOp, Block, DataType, Expr, Fn, Script, Statement, UnOp, Value, Whamm};
use std::collections::HashMap;

use crate::common::error::ErrorGen;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{Definition, Global, ProvidedFunction, ProvidedGlobal, WhammVisitorMut};
use log::trace;

const UNEXPECTED_ERR_MSG: &str = "SymbolTableBuilder: Looks like you've found a bug...please report this behavior! Exiting now...";

pub struct SymbolTableBuilder<'a> {
    pub table: SymbolTable,
    pub err: &'a mut ErrorGen,
    pub curr_whamm: Option<usize>,  // indexes into this::table::records
    pub curr_script: Option<usize>, // indexes into this::table::records
    pub curr_provider: Option<usize>, // indexes into this::table::records
    pub curr_package: Option<usize>, // indexes into this::table::records
    pub curr_event: Option<usize>,  // indexes into this::table::records
    pub curr_probe: Option<usize>,  // indexes into this::table::records
    pub curr_fn: Option<usize>,     // indexes into this::table::records
}
impl SymbolTableBuilder<'_> {
    fn add_script(&mut self, script: &Script) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&script.name, &None, true, &self.table, self.err) {
            return;
        }

        // create record
        let script_rec = Record::Script {
            name: script.name.clone(),
            fns: vec![],
            globals: vec![],
            providers: vec![],
        };

        // Add script to scope
        let id = self.table.put(script.name.clone(), script_rec);

        // Add script to current whamm record
        match self
            .table
            .get_record_mut(&self.curr_whamm.unwrap())
            .unwrap()
        {
            Record::Whamm { scripts, .. } => {
                scripts.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter script scope
        if let Err(e) = self.table.enter_scope() {
            self.err.add_error(*e)
        }
        self.curr_script = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(script.name.clone(), ScopeType::Script);
        self.table.set_curr_script(id);
    }
    fn add_provider(&mut self, provider: &dyn Provider) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&provider.name(), &None, true, &self.table, self.err) {
            return;
        }

        // create record
        let provider_rec = Record::Provider {
            name: provider.name().clone(),
            fns: vec![],
            globals: vec![],
            packages: vec![],
        };

        // Add provider to scope
        let id = self.table.put(provider.name().clone(), provider_rec);

        // Add provider to current script record
        match self
            .table
            .get_record_mut(&self.curr_script.unwrap())
            .unwrap()
        {
            Record::Script { providers, .. } => {
                providers.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter provider scope
        if let Err(e) = self.table.enter_scope() {
            self.err.add_error(*e)
        }
        self.curr_provider = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(provider.name().clone(), ScopeType::Provider);
    }

    fn add_package(&mut self, package: &dyn Package) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&package.name(), &None, true, &self.table, self.err) {
            return;
        }

        // create record
        let package_rec = Record::Package {
            name: package.name().clone(),
            fns: vec![],
            globals: vec![],
            events: vec![],
        };

        // Add package to scope
        let id = self.table.put(package.name().clone(), package_rec);

        // Add package to current provider record
        match self.table.get_record_mut(&self.curr_provider.unwrap()) {
            Some(Record::Provider { packages, .. }) => {
                packages.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter package scope
        if let Err(e) = self.table.enter_scope() {
            self.err.add_error(*e)
        }
        self.curr_package = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(package.name().clone(), ScopeType::Package);
    }

    fn add_event(&mut self, event: &dyn Event) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&event.name(), &None, true, &self.table, self.err) {
            return;
        }

        // create record
        let event_rec = Record::Event {
            name: event.name().clone(),
            fns: vec![],
            globals: vec![],
            probes: vec![],
        };

        // Add event to scope
        let id = self.table.put(event.name().clone(), event_rec);

        // Add event to current package record
        match self
            .table
            .get_record_mut(&self.curr_package.unwrap())
            .unwrap()
        {
            Record::Package { events, .. } => {
                events.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter event scope
        if let Err(e) = self.table.enter_scope() {
            self.err.add_error(*e)
        }
        self.curr_event = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(event.name().clone(), ScopeType::Event);
    }

    fn add_probe(&mut self, probe: &dyn Probe) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&probe.mode_name(), &None, true, &self.table, self.err) {
            return;
        }

        // create record
        let probe_rec = Record::Probe {
            mode: probe.mode_name().clone(),
            fns: vec![],
            globals: vec![],
        };

        // Add probe to scope
        let id = self.table.put(probe.mode_name().clone(), probe_rec);

        // Add probe to current event record
        match self.table.get_record_mut(&self.curr_event.unwrap()) {
            Some(Record::Event { probes, .. }) => {
                probes.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter probe scope
        if let Err(e) = self.table.enter_scope() {
            self.err.add_error(*e)
        }
        self.curr_probe = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(probe.mode_name().clone(), ScopeType::Probe);
    }

    fn add_fn(&mut self, f: &mut Fn) {
        let f_id: &parser_types::FnId = &f.name;
        //if there is another id with the same name in the table -> should cause an error because 2 functions with the same name
        if let Some(other_fn_id) = self.table.lookup(&f_id.name) {
            //check if the other id has a record
            if let Some(other_rec) = self.table.get_record(other_fn_id) {
                let curr_loc = &f_id.loc;
                let other_loc = other_rec.loc();
                match (curr_loc, other_loc) {
                    //case for both having loc -> both user def
                    (Some(curr_loc), Some(other_loc)) => {
                        self.err.duplicate_identifier_error(
                            false,
                            f_id.name.clone(),
                            Some(curr_loc.line_col.clone()),
                            Some(other_loc.line_col.clone()),
                        );
                    }
                    //case for curr having a location and other doesn't -> either other is comp_def or there is compiler error
                    (Some(curr_loc), None) => {
                        //make sure it is actually comp def
                        if other_rec.is_comp_provided() {
                            self.err.compiler_fn_overload_error(
                                false,
                                f_id.name.clone(),
                                Some(curr_loc.line_col.clone()),
                            );
                        } else {
                            //case for no location but not comp def
                            self.err.unexpected_error(
                                true,
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                None,
                            );
                        }
                    }
                    //case for curr not having a loc -> shouldn't happen: either user def without a loc or 2 comp def with same name
                    (None, _) => {
                        self.err.unexpected_error(
                            true,
                            Some("No location found for function conflicting with compiler def function. User-def fn has no location, or 2 compiler-def functions with same ID".to_string()),
                            None,
                        );
                    }
                }
            } else {
                // This should never be the case -> ID is in the table but doesn't have a record associated with it
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
                unreachable!()
            };
        }
        //This MUST run if the above wasn't a fatal error -> otherwise there are scoping errors
        // create record
        let fn_rec = Record::Fn {
            name: f.name.clone(),
            def: f.def.clone(),
            params: vec![],
            ret_ty: f.return_ty.clone(),
            addr: None,
            loc: f.name.loc.clone(),
        };

        // Add fn to scope
        let id = self.table.put(f.name.name.clone(), fn_rec);

        // add fn record to the current record
        self.add_fn_id_to_curr_rec(id);

        // enter fn scope
        if let Err(e) = self.table.enter_scope() {
            self.err.add_error(*e)
        }
        self.curr_fn = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(f.name.name.clone(), ScopeType::Fn);

        // visit parameters
        f.params
            .iter_mut()
            .for_each(|param| self.visit_formal_param(param));
    }

    fn add_global_id_to_curr_rec(&mut self, id: usize) {
        match self.table.get_curr_rec_mut() {
            Some(Record::Whamm { globals, .. })
            | Some(Record::Script { globals, .. })
            | Some(Record::Provider { globals, .. })
            | Some(Record::Package { globals, .. })
            | Some(Record::Event { globals, .. })
            | Some(Record::Probe { globals, .. }) => {
                globals.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }
    }

    fn add_fn_id_to_curr_rec(&mut self, id: usize) {
        match self.table.get_curr_rec_mut() {
            Some(Record::Whamm { fns, .. })
            | Some(Record::Script { fns, .. })
            | Some(Record::Provider { fns, .. })
            | Some(Record::Package { fns, .. })
            | Some(Record::Event { fns, .. })
            | Some(Record::Probe { fns, .. }) => {
                fns.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }
    }

    fn add_param(&mut self, var_id: &Expr, ty: &DataType) {
        let name = match var_id {
            Expr::VarId { name, .. } => name,
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
                // should have exited above (since it's a fatal error)
                unreachable!()
            }
        };

        // create record
        let param_rec = Record::Var {
            name: name.clone(),
            ty: ty.clone(),
            value: None,
            is_comp_provided: false,
            addr: None,
            loc: var_id.loc().clone(),
        };

        // add var to scope
        let id = self.table.put(name.clone(), param_rec);

        // add param to fn record
        match self.table.get_record_mut(&self.curr_fn.unwrap()) {
            Some(Record::Fn { params, .. }) => {
                params.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }
    }

    /// Insert `global` record into scope
    fn add_global(
        &mut self,
        ty: DataType,
        name: String,
        is_comp_provided: bool,
        loc: Option<Location>,
    ) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&name, &loc, is_comp_provided, &self.table, self.err) {
            return;
        }
        // Add global to scope
        let id = self.table.put(
            name.clone(),
            Record::Var {
                ty,
                name,
                value: None,
                is_comp_provided,
                addr: None,
                loc,
            },
        );

        // add global record to the current record
        self.add_global_id_to_curr_rec(id);
    }

    fn visit_provided_globals(&mut self, globals: &HashMap<String, ProvidedGlobal>) {
        for (name, ProvidedGlobal { global, .. }) in globals.iter() {
            self.add_global(global.ty.clone(), name.clone(), true, None);
        }
    }
}

impl WhammVisitorMut<()> for SymbolTableBuilder<'_> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) {
        trace!("Entering: visit_whamm");
        let name: String = "whamm".to_string();
        self.table
            .set_curr_scope_info(name.clone(), ScopeType::Whamm);

        // add whamm record
        let whamm_rec = Record::Whamm {
            name: name.clone(),
            fns: vec![],
            globals: vec![],
            scripts: vec![],
        };

        // Add whamm to scope
        let id = self.table.put(name.clone(), whamm_rec);

        self.curr_whamm = Some(id);

        // visit fns
        whamm
            .fns
            .iter_mut()
            .for_each(|ProvidedFunction { function, .. }| self.visit_fn(function));

        // visit globals
        self.visit_provided_globals(&whamm.globals);

        // visit scripts
        whamm
            .scripts
            .iter_mut()
            .for_each(|script| self.visit_script(script));

        trace!("Exiting: visit_whamm");
        self.curr_whamm = None;
    }

    fn visit_script(&mut self, script: &mut Script) {
        trace!("Entering: visit_script");
        self.add_script(script);

        script.fns.iter_mut().for_each(|f| self.visit_fn(f));
        script.global_stmts.iter_mut().for_each(|stmt| {
            if let Statement::Decl { ty, var_id, .. } = stmt {
                if let Expr::VarId { name, .. } = &var_id {
                    // Add global variable to script globals (triggers the init_generator to emit them!)
                    script.globals.insert(
                        name.clone(),
                        Global {
                            def: Definition::User,
                            ty: ty.clone(),
                            var_name: var_id.clone(),
                            value: None,
                        },
                    );
                } else {
                    self.err.unexpected_error(
                        true,
                        Some(format!(
                            "{} \
                Variable declaration var_id is not the correct Expr variant!!",
                            UNEXPECTED_ERR_MSG
                        )),
                        None,
                    );
                }
            }

            self.visit_stmt(stmt)
        });
        script
            .providers
            .iter_mut()
            .for_each(|(_name, provider)| self.visit_provider(provider));

        trace!("Exiting: visit_script");
        if let Err(e) = self.table.exit_scope() {
            self.err.add_error(*e)
        }
        self.curr_script = None;
    }

    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) {
        trace!("Entering: visit_provider");

        self.add_provider(provider.as_ref());
        provider
            .get_provided_fns_mut()
            .iter_mut()
            .for_each(|f| self.visit_fn(&mut f.function));
        self.visit_provided_globals(provider.get_provided_globals());
        provider
            .packages_mut()
            .for_each(|package| self.visit_package(package));

        trace!("Exiting: visit_provider");
        if let Err(e) = self.table.exit_scope() {
            self.err.add_error(*e)
        }
        self.curr_provider = None;
    }

    fn visit_package(&mut self, package: &mut dyn Package) {
        trace!("Entering: visit_package");

        self.add_package(package);
        package
            .get_provided_fns_mut()
            .iter_mut()
            .for_each(|f| self.visit_fn(&mut f.function));
        self.visit_provided_globals(package.get_provided_globals());
        package
            .events_mut()
            .for_each(|event| self.visit_event(event));

        trace!("Exiting: visit_package");
        if let Err(e) = self.table.exit_scope() {
            self.err.add_error(*e)
        }
        self.curr_package = None;
    }

    fn visit_event(&mut self, event: &mut dyn Event) {
        trace!("Entering: visit_event");

        self.add_event(event);
        event
            .get_provided_fns_mut()
            .iter_mut()
            .for_each(|f| self.visit_fn(&mut f.function));
        self.visit_provided_globals(event.get_provided_globals());

        // visit probe_map
        event.probes_mut().iter_mut().for_each(|probes| {
            probes.1.iter_mut().for_each(|probe| {
                self.visit_probe(probe);
            });
        });

        trace!("Exiting: visit_event");
        if let Err(e) = self.table.exit_scope() {
            self.err.add_error(*e)
        }
        self.curr_event = None;
    }

    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) {
        trace!("Entering: visit_probe");

        self.add_probe(probe.as_ref());
        probe
            .get_mode_provided_fns_mut()
            .iter_mut()
            .for_each(|f| self.visit_fn(&mut f.function));
        self.visit_provided_globals(probe.get_mode_provided_globals());

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_probe");
        if let Err(e) = self.table.exit_scope() {
            self.err.add_error(*e)
        }
        self.curr_probe = None;
    }

    fn visit_fn(&mut self, f: &mut Fn) {
        trace!("Entering: visit_fn");

        // add fn
        self.add_fn(f);

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_fn");
        if let Err(e) = self.table.exit_scope() {
            self.err.add_error(*e)
        }
        self.curr_fn = None;
    }

    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) {
        trace!("Entering: visit_formal_param");

        // add param
        self.add_param(&param.0, &param.1);

        trace!("Exiting: visit_formal_param");
    }

    fn visit_block(&mut self, _block: &mut Block) {
        // Not visiting Blocks
        self.err
            .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) {
        if self.curr_provider.is_some()
            || self.curr_package.is_some()
            || self.curr_event.is_some()
            || self.curr_probe.is_some()
        {
            self.err.unexpected_error(
                true,
                Some(format!(
                    "{} \
            Only global script statements should be visited!",
                    UNEXPECTED_ERR_MSG
                )),
                None,
            );
        }

        if let Statement::Decl {
            ty, var_id, loc, ..
        } = stmt
        {
            if let Expr::VarId {
                name,
                is_comp_provided,
                ..
            } = &var_id
            {
                // Add symbol to table
                self.add_global(ty.clone(), name.clone(), *is_comp_provided, loc.clone());
            } else {
                self.err.unexpected_error(
                    true,
                    Some(format!(
                        "{} \
                Variable declaration var_id is not the correct Expr variant!!",
                        UNEXPECTED_ERR_MSG
                    )),
                    None,
                );
            }
        }
    }

    fn visit_expr(&mut self, _call: &mut Expr) {
        // Not visiting predicates/statements
        self.err
            .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
    }

    fn visit_unop(&mut self, _unop: &mut UnOp) {
        // Not visiting predicates/statements
        self.err
            .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
    }

    fn visit_binop(&mut self, _binop: &mut BinOp) {
        // Not visiting predicates/statements
        self.err
            .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
    }

    fn visit_datatype(&mut self, _datatype: &mut DataType) {
        // Not visiting predicates/statements
        self.err
            .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
    }

    fn visit_value(&mut self, _val: &mut Value) {
        // Not visiting predicates/statements
        self.err
            .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
    }
}

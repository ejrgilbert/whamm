use crate::common::error::ErrorGen;
use crate::generator::ast::ReqArgs;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types as parser_types;
use crate::parser::types::{
    Definition, FnId, Global, ProvidedFunction, ProvidedGlobal, WhammVisitorMut,
};
use crate::verifier::builder_visitor::parser_types::Location;
use crate::verifier::types::{Record, ScopeType, SymbolTable};
use crate::verifier::verifier::check_duplicate_id;
use itertools::Itertools;
use log::trace;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::Module;
use parser_types::{BinOp, Block, DataType, Expr, Fn, Script, Statement, UnOp, Value, Whamm};
use std::collections::{HashMap, HashSet};
use wasmparser::ExternalKind;

const UNEXPECTED_ERR_MSG: &str = "SymbolTableBuilder: Looks like you've found a bug...please report this behavior! Exiting now...";

pub struct SymbolTableBuilder<'a, 'b, 'c> {
    pub table: SymbolTable,
    pub user_libs: &'b HashMap<String, Module<'c>>,
    pub err: &'a mut ErrorGen,
    pub curr_whamm: Option<usize>,  // indexes into this::table::records
    pub curr_script: Option<usize>, // indexes into this::table::records
    pub curr_provider: Option<usize>, // indexes into this::table::records
    pub curr_package: Option<usize>, // indexes into this::table::records
    pub curr_event: Option<usize>,  // indexes into this::table::records
    pub curr_probe: Option<usize>,  // indexes into this::table::records
    pub curr_fn: Option<usize>,     // indexes into this::table::records

    // track the derived variables that need to be defined
    pub aliases: HashMap<String, String>,
    pub used_derived_vars: HashSet<String>,
    pub derived_vars: HashMap<String, (DataType, Expr)>,

    // bookkeeping for providedfunctions
    pub req_args: ReqArgs,
}
impl SymbolTableBuilder<'_, '_, '_> {
    fn add_script(&mut self, script: &Script) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &script.id.to_string(),
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
            return;
        }

        // create record
        let script_rec = Record::Script {
            id: script.id,
            user_libs: vec![],
            fns: vec![],
            globals: vec![],
            providers: vec![],
        };

        // Add script to scope
        let id = self.table.put(script.id.to_string(), script_rec);

        // Add script to current whamm record
        match self.table.get_record_mut(self.curr_whamm.unwrap()).unwrap() {
            Record::Whamm { scripts, .. } => {
                scripts.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter script scope
        self.table.enter_scope(self.err);
        self.curr_script = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(script.id.to_string(), ScopeType::Script);
        self.table.set_curr_script(id);
    }
    fn add_provider(&mut self, provider: &dyn Provider) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &provider.name(),
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
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
            .get_record_mut(self.curr_script.unwrap())
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
        self.table.enter_scope(self.err);
        self.curr_provider = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(provider.name().clone(), ScopeType::Provider);
    }

    fn add_package(&mut self, package: &dyn Package) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &package.name(),
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
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
        match self.table.get_record_mut(self.curr_provider.unwrap()) {
            Some(Record::Provider { packages, .. }) => {
                packages.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter package scope
        self.table.enter_scope(self.err);
        self.curr_package = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(package.name().clone(), ScopeType::Package);
    }

    fn add_event(&mut self, event: &dyn Event) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &event.name(),
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
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
            .get_record_mut(self.curr_package.unwrap())
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
        self.table.enter_scope(self.err);
        self.curr_event = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(event.name().clone(), ScopeType::Event);
    }

    fn add_probe(&mut self, probe: &dyn Probe) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &probe.mode().name(),
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
            return;
        }

        // create record
        let probe_rec = Record::Probe {
            mode: probe.mode().name(),
            fns: vec![],
            globals: vec![],
        };

        // Add probe to scope
        let id = self.table.put(probe.mode().name(), probe_rec);

        // Add probe to current event record
        match self.table.get_record_mut(self.curr_event.unwrap()) {
            Some(Record::Event { probes, .. }) => {
                probes.push(id);
            }
            _ => {
                self.err
                    .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        }

        // enter probe scope
        self.table.enter_scope(self.err);
        self.curr_probe = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(probe.mode().name(), ScopeType::Probe);
    }

    fn add_user_lib(&mut self, lib_name: &String, loc: &Option<Location>) {
        // NOTE -- we don't have a Library scope! This is because the library
        // functions should be globally accessible within the scope of the
        // script. Not having a scope for the Library supports this!

        if let Some(lib_module) = self.user_libs.get(lib_name) {
            // add user library to the current scope (should be Script)
            // enters a new scope (named 'lib_name')
            // for each exported function in 'lib_module':
            //   -- add new function to the lib scope
            //   -- with the right type information
            // THEN:
            // -- should be able to do a normal function call AND type check
            // -- (after looking up the library scope in the table)
            if check_duplicate_id(lib_name, &None, &Definition::User, &self.table, self.err) {
                return;
            }

            let lib_id = self.table.put(
                lib_name.clone(),
                Record::Library {
                    name: lib_name.clone(),
                    fns: Default::default(),
                },
            );
            match self.table.get_record_mut(self.curr_script.unwrap()) {
                Some(Record::Script { user_libs, .. }) => {
                    user_libs.push(lib_id);
                }
                _ => {
                    self.err
                        .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
                }
            }

            for export in lib_module.exports.iter() {
                // we don't care about non-function exports
                if let ExternalKind::Func = export.kind {
                    let func = lib_module.functions.get(FunctionID(export.index));
                    if let Some(ty) = lib_module.types.get(func.get_type_id()) {
                        let mut params = vec![];
                        for p in ty.params().iter() {
                            params.push(DataType::from_wasm_type(p));
                        }
                        let mut results = vec![];
                        for p in ty.results().iter() {
                            results.push(DataType::from_wasm_type(p));
                        }
                        let fn_name = export.name.clone();
                        let fn_rec = Record::LibFn {
                            lib_name: lib_name.clone(),
                            name: fn_name.clone(),
                            def: Definition::User,
                            params,
                            results,
                            addr: None,
                            loc: None,
                        };

                        // Add fn to library
                        let id = self.table.put(fn_name.clone(), fn_rec);
                        match self.table.get_record_mut(lib_id) {
                            Some(Record::Library { fns, .. }) => {
                                fns.insert(fn_name.clone(), id);
                            }
                            _ => {
                                panic!("{}", UNEXPECTED_ERR_MSG);
                            }
                        }
                    } else {
                        panic!(
                            "UserLib: Could not find type ID for function {}",
                            export.name
                        );
                    }
                }
            }
        } else {
            self.err.parse_error_at_loc(
                true,
                Some("The script uses a library, but it wasn't configured in the CLI".to_string()),
                loc.clone(),
            );
        }
    }

    fn add_fn(&mut self, f: &mut Fn) {
        let f_id: &FnId = &f.name;
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
                            panic!("{}", UNEXPECTED_ERR_MSG);
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
            req_args: self.req_args.clone(),
        };

        // Add fn to scope
        let id = self.table.put(f.name.name.clone(), fn_rec);

        // add fn record to the current record
        self.add_fn_id_to_curr_rec(id);

        // enter fn scope
        self.table.enter_scope(self.err);
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
            def: Definition::User,
            is_report_var: false,
            addr: None,
            loc: var_id.loc().clone(),
        };

        // add var to scope
        let id = self.table.put(name.clone(), param_rec);

        // add param to fn record
        match self.table.get_record_mut(self.curr_fn.unwrap()) {
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
        value: Option<Value>,
        definition: Definition,
        is_report_var: bool,
        loc: Option<Location>,
    ) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&name, &loc, &definition, &self.table, self.err) {
            return;
        }
        // Add global to scope
        let id = self.table.put(
            name.clone(),
            Record::Var {
                ty,
                name,
                value,
                def: definition,
                is_report_var,
                addr: None,
                loc,
            },
        );

        // add global record to the current record
        self.add_global_id_to_curr_rec(id);
    }

    fn visit_provided_globals(
        &mut self,
        globals: &HashMap<String, ProvidedGlobal>,
    ) -> (HashMap<String, String>, HashMap<String, (DataType, Expr)>) {
        let mut aliases = HashMap::new();
        let mut derived = HashMap::new();
        for (
            name,
            ProvidedGlobal {
                global,
                value,
                derived_from,
                ..
            },
        ) in globals.iter()
        {
            if let Some(derived_from) = derived_from {
                if let Expr::VarId { name: alias, .. } = derived_from {
                    // this is a simple alias!
                    aliases.insert(name.clone(), alias.clone());
                } else {
                    // Add derived globals to the probe body itself (to calculate the value)
                    derived.insert(name.clone(), (global.ty.clone(), derived_from.clone()));
                }
            } else {
                // Add other globals to the scope itself
                self.add_global(
                    global.ty.clone(),
                    name.clone(),
                    value.clone(),
                    global.def.clone(),
                    false,
                    None,
                );
            }
        }
        (aliases, derived)
    }
}

impl WhammVisitorMut<()> for SymbolTableBuilder<'_, '_, '_> {
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
        whamm.fns.iter_mut().for_each(
            |ProvidedFunction {
                 function, req_args, ..
             }| {
                self.req_args = req_args.clone();
                self.visit_fn(function)
            },
        );

        // visit globals
        (self.aliases, self.derived_vars) = self.visit_provided_globals(&whamm.globals);

        // visit scripts
        whamm
            .scripts
            .iter_mut()
            .for_each(|script| self.visit_script(script));

        self.aliases.clear();
        self.derived_vars.clear();

        trace!("Exiting: visit_whamm");
        self.curr_whamm = None;
    }

    fn visit_script(&mut self, script: &mut Script) {
        trace!("Entering: visit_script");
        self.add_script(script);

        script.fns.iter_mut().for_each(|f| self.visit_fn(f));
        script.global_stmts.iter_mut().for_each(|stmt| {
            let mut is_report_var = false;
            let stmt = match stmt {
                Statement::UnsharedDecl {
                    decl, is_report, ..
                } => {
                    is_report_var = *is_report;
                    &mut **decl
                }
                _ => stmt,
            };
            if let Statement::Decl { ty, var_id, .. } = stmt {
                if let Expr::VarId { name, .. } = &var_id {
                    // Add global variable to script globals (triggers the init_generator to emit them!)
                    script.globals.insert(
                        name.clone(),
                        Global {
                            def: Definition::User,
                            report: is_report_var,
                            ty: ty.clone(),
                            var_name: var_id.clone(),
                            value: None,
                        },
                    );
                } else {
                    panic!(
                        "{} \
                Variable declaration var_id is not the correct Expr variant!!",
                        UNEXPECTED_ERR_MSG
                    );
                }
            };
            self.visit_stmt(stmt)
        });
        script
            .providers
            .iter_mut()
            .for_each(|(_name, provider)| self.visit_provider(provider));

        trace!("Exiting: visit_script");
        self.table.exit_scope(self.err);
        self.curr_script = None;
    }

    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) {
        trace!("Entering: visit_provider");

        self.add_provider(provider.as_ref());
        provider.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction {
                 function, req_args, ..
             }| {
                self.req_args = req_args.clone();
                self.visit_fn(function);
            },
        );
        let (prov_aliases, prov_derived_vars) =
            self.visit_provided_globals(provider.get_provided_globals());
        let to_remove_alias = prov_aliases.keys().cloned().collect_vec();
        let to_remove_vars = prov_derived_vars.keys().cloned().collect_vec();
        self.aliases.extend(prov_aliases.clone());
        self.derived_vars.extend(prov_derived_vars.clone());
        provider
            .packages_mut()
            .for_each(|package| self.visit_package(package));
        for var in to_remove_alias.iter() {
            self.aliases.remove(var);
        }
        for var in to_remove_vars.iter() {
            self.derived_vars.remove(var);
        }

        trace!("Exiting: visit_provider");
        self.table.exit_scope(self.err);
        self.curr_provider = None;
    }

    fn visit_package(&mut self, package: &mut dyn Package) {
        trace!("Entering: visit_package");

        self.add_package(package);
        package.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction {
                 function, req_args, ..
             }| {
                self.req_args = req_args.clone();
                self.visit_fn(function);
            },
        );
        let (pack_aliases, pack_derived_vars) =
            self.visit_provided_globals(package.get_provided_globals());
        let to_remove_alias = pack_aliases.keys().cloned().collect_vec();
        let to_remove_vars = pack_derived_vars.keys().cloned().collect_vec();
        self.aliases.extend(pack_aliases.clone());
        self.derived_vars.extend(pack_derived_vars.clone());
        package
            .events_mut()
            .for_each(|event| self.visit_event(event));
        for var in to_remove_alias.iter() {
            self.aliases.remove(var);
        }
        for var in to_remove_vars.iter() {
            self.derived_vars.remove(var);
        }

        trace!("Exiting: visit_package");
        self.table.exit_scope(self.err);
        self.curr_package = None;
    }

    fn visit_event(&mut self, event: &mut dyn Event) {
        trace!("Entering: visit_event");

        self.add_event(event);
        event.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction {
                 function, req_args, ..
             }| {
                self.req_args = req_args.clone();
                self.visit_fn(function);
            },
        );
        let (ev_aliases, ev_derived_vars) =
            self.visit_provided_globals(event.get_provided_globals());
        let to_remove_alias = ev_aliases.keys().cloned().collect_vec();
        let to_remove_vars = ev_derived_vars.keys().cloned().collect_vec();
        self.aliases.extend(ev_aliases.clone());
        self.derived_vars.extend(ev_derived_vars.clone());
        // visit probe_map
        event.probes_mut().iter_mut().for_each(|probes| {
            probes.1.iter_mut().for_each(|probe| {
                self.visit_probe(probe);
            });
        });
        for var in to_remove_alias.iter() {
            self.aliases.remove(var);
        }
        for var in to_remove_vars.iter() {
            self.derived_vars.remove(var);
        }

        trace!("Exiting: visit_event");
        self.table.exit_scope(self.err);
        self.curr_event = None;
    }

    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) {
        trace!("Entering: visit_probe");

        self.add_probe(probe.as_ref());
        probe.get_mode_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction {
                 function, req_args, ..
             }| {
                self.req_args = req_args.clone();
                self.visit_fn(function);
            },
        );
        let (probe_aliases, probe_derived_vars) =
            self.visit_provided_globals(probe.get_mode_provided_globals());
        let to_remove_alias = probe_aliases.keys().cloned().collect_vec();
        let to_remove_vars = probe_derived_vars.keys().cloned().collect_vec();
        self.aliases.extend(probe_aliases.clone());
        self.derived_vars.extend(probe_derived_vars.clone());

        // Will not visit predicate/body at this stage
        // visit the predicate/body to handle aliases and derived variables!
        if let Some(predicate) = &mut probe.predicate_mut() {
            self.visit_expr(predicate);
        }

        // Add the derived variables as new variables at the top of the probe body!
        if let Some(body) = probe.body_mut() {
            self.visit_block(body);

            for (var, (ty, expr)) in self.derived_vars.iter() {
                if self.used_derived_vars.contains(var) {
                    // Only define a derived variable if it's used!
                    body.stmts.insert(
                        0,
                        Statement::Decl {
                            ty: ty.clone(),
                            var_id: Expr::VarId {
                                definition: Definition::CompilerDerived,
                                name: var.clone(),
                                loc: None,
                            },
                            loc: None,
                        },
                    );
                    body.stmts.insert(
                        1,
                        Statement::Assign {
                            var_id: Expr::VarId {
                                definition: Definition::CompilerDerived,
                                name: var.clone(),
                                loc: None,
                            },
                            expr: expr.clone(),
                            loc: None,
                        },
                    );
                }
            }
        }

        for var in to_remove_alias.iter() {
            self.aliases.remove(var);
        }
        for var in to_remove_vars.iter() {
            self.derived_vars.remove(var);
        }
        self.used_derived_vars.clear();

        trace!("Exiting: visit_probe");
        self.table.exit_scope(self.err);
        self.curr_probe = None;
    }

    fn visit_fn(&mut self, f: &mut Fn) {
        trace!("Entering: visit_fn");

        // add fn
        self.add_fn(f);

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_fn");
        self.table.exit_scope(self.err);
        self.curr_fn = None;
    }

    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) {
        trace!("Entering: visit_formal_param");

        // add param
        self.add_param(&param.0, &param.1);

        trace!("Exiting: visit_formal_param");
    }

    fn visit_block(&mut self, block: &mut Block) {
        for stmt in block.stmts.iter_mut() {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) {
        if self.curr_provider.is_none()
            && self.curr_package.is_none()
            && self.curr_event.is_none()
            && self.curr_probe.is_none()
        {
            // in the global scope!

            let mut is_report_var = false;
            let stmt = match &stmt {
                Statement::UnsharedDecl {
                    decl, is_report, ..
                } => {
                    is_report_var = *is_report;
                    &**decl
                }
                _ => stmt,
            };
            match stmt {
                Statement::LibImport { lib_name, loc, .. } => {
                    self.add_user_lib(lib_name, loc);
                }
                Statement::Decl {
                    ty, var_id, loc, ..
                } => {
                    if let Expr::VarId {
                        name, definition, ..
                    } = &var_id
                    {
                        // Add symbol to table
                        self.add_global(
                            ty.clone(),
                            name.clone(),
                            None,
                            definition.clone(),
                            is_report_var,
                            loc.clone(),
                        );
                    } else {
                        panic!(
                            "{} \
                Variable declaration var_id is not the correct Expr variant!!",
                            UNEXPECTED_ERR_MSG
                        );
                    }
                }
                _ => {}
            }
        } else {
            // in a probe, we just want to handle aliases and derived variables in this case
            // so we only will visit statements that may contain an expression to handle
            match stmt {
                Statement::Assign { expr, .. } => {
                    self.visit_expr(expr);
                }
                Statement::SetMap { map, key, val, .. } => {
                    self.visit_expr(map);
                    self.visit_expr(key);
                    self.visit_expr(val);
                }
                Statement::Expr { expr, .. } => {
                    self.visit_expr(expr);
                }
                Statement::Return { expr, .. } => {
                    self.visit_expr(expr);
                }
                Statement::If {
                    cond, conseq, alt, ..
                } => {
                    self.visit_expr(cond);
                    self.visit_block(conseq);
                    self.visit_block(alt);
                }
                Statement::UnsharedDecl { .. }
                | Statement::LibImport { .. }
                | Statement::Decl { .. } => {}
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::VarId { name, .. } => {
                // see if this is an alias or derived var!
                if let Some(alias) = self.aliases.get(name) {
                    // this is an alias!
                    *name = alias.clone();
                } else if self.derived_vars.contains_key(name) {
                    self.used_derived_vars.insert(name.clone());
                }
            }
            Expr::UnOp { expr, .. } => {
                self.visit_expr(expr);
            }
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                self.visit_expr(cond);
                self.visit_expr(conseq);
                self.visit_expr(alt);
            }
            Expr::BinOp { lhs, rhs, .. } => {
                self.visit_expr(lhs);
                self.visit_expr(rhs);
            }
            Expr::Call {
                fn_target, args, ..
            } => {
                self.visit_expr(fn_target);
                for arg in args.iter_mut() {
                    self.visit_expr(arg);
                }
            }
            Expr::LibCall { call, .. } => {
                self.visit_expr(call);
            }
            Expr::Primitive { .. } => {}
            Expr::MapGet { map, key, .. } => {
                self.visit_expr(map);
                self.visit_expr(key);
            }
        }
    }

    fn visit_unop(&mut self, _unop: &mut UnOp) {
        // Not visiting predicates/statements
        panic!("{UNEXPECTED_ERR_MSG}");
    }

    fn visit_binop(&mut self, _binop: &mut BinOp) {
        // Not visiting predicates/statements
        panic!("{UNEXPECTED_ERR_MSG}");
    }

    fn visit_datatype(&mut self, _datatype: &mut DataType) {
        // Not visiting predicates/statements
        panic!("{UNEXPECTED_ERR_MSG}");
    }

    fn visit_value(&mut self, _val: &mut Value) {
        // Not visiting predicates/statements
        panic!("{UNEXPECTED_ERR_MSG}");
    }
}

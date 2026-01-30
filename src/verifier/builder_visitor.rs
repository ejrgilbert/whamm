use crate::common::error::ErrorGen;
use crate::generator::ast::StackReq;
use crate::parser::provider_handler::{BoundFunc, BoundVar, Event, Package, Probe, Provider};
use crate::parser::types as parser_types;
use crate::parser::types::{BoundFunction, Definition, FnId, Global, WhammVisitorMut};
use crate::verifier::builder_visitor::parser_types::Location;
use crate::verifier::types::{Record, ScopeType, SymbolTable};
use crate::verifier::verifier::check_duplicate_id;
use itertools::Itertools;
use log::trace;
use parser_types::{Block, DataType, Expr, Fn, Script, Statement, Value, Whamm};
use std::collections::{HashMap, HashSet};
use wirm::Module;
use wirm::ir::id::FunctionID;
use wirm::wasmparser::ExternalKind;

const UNEXPECTED_ERR_MSG: &str = "SymbolTableBuilder: Looks like you've found a bug...please report this behavior! Exiting now...";

pub struct SymbolTableBuilder<'a, 'b, 'c> {
    pub table: SymbolTable,
    pub user_libs: &'b HashMap<String, (Option<String>, Module<'c>)>,
    pub err: &'a mut ErrorGen,
    pub curr_whamm: Option<usize>,  // indexes into this::table::records
    pub curr_script: Option<usize>, // indexes into this::table::records
    pub curr_provider: Option<usize>, // indexes into this::table::records
    pub curr_package: Option<usize>, // indexes into this::table::records
    pub curr_event: Option<usize>,  // indexes into this::table::records
    pub curr_mode: Option<usize>,   // indexes into this::table::records
    pub curr_probe: Option<usize>,  // indexes into this::table::records
    pub curr_fn: Option<usize>,     // indexes into this::table::records

    // track the derived variables that need to be defined
    pub aliases: HashMap<String, String>,
    pub used_derived_vars: HashSet<String>,
    pub derived_vars: HashMap<String, (DataType, Expr)>,

    // bookkeeping for boundfunctions
    pub req_args: StackReq,
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
                unreachable!("{}", UNEXPECTED_ERR_MSG);
            }
        }

        // enter script scope
        self.table.enter_scope();
        self.curr_script = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(script.id.to_string(), ScopeType::Script);
        self.table.set_curr_script(id);
    }
    fn add_provider(&mut self, provider: &Provider) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &provider.def.name,
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
            return;
        }

        // create record
        let provider_rec = Record::Provider {
            fns: vec![],
            vars: vec![],
            packages: vec![],
        };

        // Add provider to scope
        let id = self.table.put(provider.def.name.clone(), provider_rec);

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
                unreachable!("{}", UNEXPECTED_ERR_MSG);
            }
        }

        // enter provider scope
        self.table.enter_scope();
        self.curr_provider = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(provider.def.name.clone(), ScopeType::Provider);
    }

    fn add_package(&mut self, package: &Package) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &package.def.name,
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
            return;
        }

        // create record
        let package_rec = Record::Package {
            fns: vec![],
            vars: vec![],
            events: vec![],
        };

        // Add package to scope
        let id = self.table.put(package.def.name.clone(), package_rec);

        // Add package to current provider record
        match self.table.get_record_mut(self.curr_provider.unwrap()) {
            Some(Record::Provider { packages, .. }) => {
                packages.push(id);
            }
            _ => {
                unreachable!("{}", UNEXPECTED_ERR_MSG);
            }
        }

        // enter package scope
        self.table.enter_scope();
        self.curr_package = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(package.def.name.clone(), ScopeType::Package);
    }

    fn add_event(&mut self, event: &Event) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(
            &event.def.name,
            &None,
            &Definition::CompilerStatic,
            &self.table,
            self.err,
        ) {
            return;
        }

        // create record
        let event_rec = Record::Event {
            fns: vec![],
            vars: vec![],
            modes: vec![],
        };

        // Add event to scope
        let id = self.table.put(event.def.name.clone(), event_rec);

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
                unreachable!("{}", UNEXPECTED_ERR_MSG);
            }
        }

        // enter event scope
        self.table.enter_scope();
        self.curr_event = Some(id);

        // set scope name and type
        self.table
            .set_curr_scope_info(event.def.name.clone(), ScopeType::Event);
    }

    fn add_probe(&mut self, probe: &mut Probe) {
        // TODO -- factor this to reduce duplicate code!
        if self.table.lookup(&probe.kind.name()).is_none() {
            // Add mode to scope
            let mode_rec = Record::Mode { probes: vec![] };
            let id = self.table.put(probe.kind.name(), mode_rec);
            self.curr_mode = Some(id);

            // Add probe to current event record
            match self.table.get_record_mut(self.curr_event.unwrap()) {
                Some(Record::Event { modes, .. }) => {
                    modes.push(id);
                }
                _ => {
                    unreachable!(
                        "{UNEXPECTED_ERR_MSG} Should be able to find the current event in the symbol table."
                    );
                }
            }

            // enter the NEW mode scope
            self.table.enter_scope();
            self.table
                .set_curr_scope_info(probe.kind.name(), ScopeType::Mode);
        } else {
            // enter the already-existing mode scope
            self.table.enter_named_scope(&probe.kind.name());
        }

        // NOTE: Had to duplicate this with a slight difference due to Rust mutable reference rules...
        let probe_scope_id = if let Some(rec_id) = self.table.lookup(&probe.kind.name()) {
            self.curr_mode = Some(rec_id);
            // This probe mode already exists for the event! Directly edit this one
            let Some(Record::Mode { probes }) = self.table.get_record_mut(rec_id) else {
                unreachable!("{UNEXPECTED_ERR_MSG} Could not find record with id: {rec_id}");
            };

            // need to use this as the probe's scope ID
            probes.len()
        } else {
            unreachable!(
                "{UNEXPECTED_ERR_MSG} Should be able to find the probe kind in the symbol table (already visited that scope)!"
            );
        };
        probe.scope_id = probe_scope_id;
        let probe_name = probe_scope_id.to_string();

        // Add probe record
        let probe_rec = Record::Probe {
            fns: vec![],
            vars: vec![],
        };
        let id = self.table.put(probe_name.clone(), probe_rec);

        if let Some(rec_id) = self.table.lookup(&probe.kind.name()) {
            self.curr_mode = Some(rec_id);
            // This probe mode already exists for the event! Directly edit this one
            let Some(Record::Mode { probes }) = self.table.get_record_mut(rec_id) else {
                unreachable!("Could not find record with id: {rec_id}");
            };

            // add probe to the current mode
            probes.push(id);
        } else {
            unreachable!(
                "{UNEXPECTED_ERR_MSG} Should be able to find the probe kind in the symbol table (already visited that scope)!"
            );
        };

        self.curr_probe = Some(id);
        // enter probe scope
        self.table.add_and_enter_new_scope();

        // set scope name and type
        self.table.set_curr_scope_info(probe_name, ScopeType::Probe);
    }

    fn add_user_lib(&mut self, lib_name: &String, loc: &Option<Location>) {
        // NOTE -- we don't have a Library scope! This is because the library
        // functions should be globally accessible within the scope of the
        // script. Not having a scope for the Library supports this!

        if let Some((_, lib_module)) = self.user_libs.get(lib_name) {
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
                    fns: Default::default(),
                },
            );
            match self.table.get_record_mut(self.curr_script.unwrap()) {
                Some(Record::Script { user_libs, .. }) => {
                    user_libs.push(lib_id);
                }
                _ => {
                    unreachable!("{}", UNEXPECTED_ERR_MSG);
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
                            f_id.name.clone(),
                            Some(curr_loc.line_col.clone()),
                            Some(other_loc.line_col.clone()),
                        );
                    }
                    //case for curr having a location and other doesn't -> either other is comp_def or there is compiler error
                    (Some(curr_loc), None) => {
                        //make sure it is actually comp def
                        if other_rec.is_comp_defined() {
                            self.err.compiler_fn_overload_error(
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
                        unreachable!(
                            "No location found for function conflicting with compiler def function. User-def fn has no location, or 2 compiler-def functions with same ID"
                        );
                    }
                }
            } else {
                // This should never be the case -> ID is in the table but doesn't have a record associated with it
                unreachable!("{}", UNEXPECTED_ERR_MSG);
            };
        }
        //This MUST run if the above wasn't a fatal error -> otherwise there are scoping errors
        // create record
        let fn_rec = Record::Fn {
            name: f.name.clone(),
            def: f.def.clone(),
            params: vec![],
            ret_ty: f.results.clone(),
            addr: None,
            loc: f.name.loc.clone(),
            req_args: self.req_args.clone(),
        };

        // Add fn to scope
        let id = self.table.put(f.name.name.clone(), fn_rec);

        // add fn record to the current record
        self.add_fn_id_to_curr_rec(id);

        // enter fn scope
        self.table.enter_scope();
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
            | Some(Record::Provider { vars: globals, .. })
            | Some(Record::Package { vars: globals, .. })
            | Some(Record::Event { vars: globals, .. })
            | Some(Record::Probe { vars: globals, .. }) => {
                globals.push(id);
            }
            _ => {
                unreachable!("{}", UNEXPECTED_ERR_MSG);
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
                unreachable!("{}", UNEXPECTED_ERR_MSG);
            }
        }
    }

    fn add_param(&mut self, var_id: &Expr, ty: &DataType) {
        let name = match var_id {
            Expr::VarId { name, .. } => name,
            _ => {
                unreachable!("{}", UNEXPECTED_ERR_MSG)
            }
        };

        // create record
        let param_rec = Record::Var {
            ty: ty.clone(),
            value: None,
            def: Definition::User,
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
                unreachable!("{}", UNEXPECTED_ERR_MSG);
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
                value,
                def: definition,
                addr: None,
                loc,
            },
        );

        // add global record to the current record
        self.add_global_id_to_curr_rec(id);
    }

    fn visit_bound_vars(&mut self, vars: &[BoundVar]) -> (Vec<String>, Vec<String>) {
        let mut aliases = HashMap::new();
        let mut derived = HashMap::new();
        for BoundVar {
            name,
            ty,
            derived_from,
            lifetime,
            ..
        } in vars.iter()
        {
            if let Some(derived_from) = derived_from {
                if let Expr::VarId { name: alias, .. } = derived_from {
                    // this is a simple alias!
                    aliases.insert(name.clone(), alias.clone());
                } else if let Expr::Primitive { val, .. } = derived_from {
                    // This is a simple value that can be folded away
                    self.add_global(
                        ty.clone(),
                        name.clone(),
                        Some(val.clone()),
                        lifetime.clone(),
                        None,
                    );
                } else {
                    // Add derived globals to the probe body itself (to calculate the value)
                    derived.insert(name.clone(), (ty.clone(), derived_from.clone()));
                }
            } else {
                // Add other globals to the scope itself
                self.add_global(
                    ty.clone(),
                    name.clone(),
                    None, // todo this is just made up
                    lifetime.clone(),
                    None,
                );
            }
        }

        let to_remove_alias = aliases.keys().cloned().collect_vec();
        let to_remove_vars = derived.keys().cloned().collect_vec();
        self.aliases.extend(aliases.clone());
        for (_, expr) in derived.values_mut() {
            self.visit_expr(expr);
        }
        self.derived_vars.extend(derived.clone());

        (to_remove_alias, to_remove_vars)
    }
    fn remove_bound_data(&mut self, to_remove_alias: Vec<String>, to_remove_vars: Vec<String>) {
        for var in to_remove_alias.iter() {
            self.aliases.remove(var);
        }
        for var in to_remove_vars.iter() {
            self.derived_vars.remove(var);
        }
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
            fns: vec![],
            globals: vec![],
            scripts: vec![],
        };

        // Add whamm to scope
        let id = self.table.put(name.clone(), whamm_rec);

        self.curr_whamm = Some(id);

        // visit fns
        whamm.fns.iter_mut().for_each(
            |BoundFunction {
                 function, req_args, ..
             }| {
                self.req_args = req_args.clone();
                self.visit_fn(function)
            },
        );

        // visit globals
        _ = self.visit_bound_vars(&whamm.bound_vars);

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
                Statement::UnsharedDeclInit { decl, .. } => {
                    if let Statement::UnsharedDecl {
                        decl: d, is_report, ..
                    } = decl.as_mut()
                    {
                        is_report_var = *is_report;
                        &mut **d
                    } else {
                        self.err.add_internal_error(&format!("An unshared decl initialization statement should always contain an unshared declaration, but this was: {decl:?}"), decl.loc());
                        return;
                    }
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
                            value: None,
                        },
                    );
                } else {
                    self.err.add_internal_error(&format!(
                        "{UNEXPECTED_ERR_MSG} \
            Variable declaration var_id is not the correct Expr variant!!",
                    ), var_id.loc());
                }
            }
            self.visit_stmt(stmt)
        });

        script
            .providers
            .iter_mut()
            .for_each(|(_name, provider)| self.visit_provider(provider));

        trace!("Exiting: visit_script");
        self.table.exit_scope();
        self.curr_script = None;
    }

    fn visit_provider(&mut self, provider: &mut Provider) {
        trace!("Entering: visit_provider");

        self.add_provider(provider);
        provider
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, req_args, .. }| {
                self.req_args = req_args.clone();
                self.visit_fn(func);
            });
        let (to_remove_alias, to_remove_vars) = self.visit_bound_vars(&provider.def.bound_vars);

        provider
            .packages
            .iter_mut()
            .for_each(|(_, package)| self.visit_package(package));

        self.remove_bound_data(to_remove_alias, to_remove_vars);

        trace!("Exiting: visit_provider");
        self.table.exit_scope();
        self.curr_provider = None;
    }

    fn visit_package(&mut self, package: &mut Package) {
        trace!("Entering: visit_package");

        self.add_package(package);
        package
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, req_args, .. }| {
                self.req_args = req_args.clone();
                self.visit_fn(func);
            });
        let (to_remove_alias, to_remove_vars) = self.visit_bound_vars(&package.def.bound_vars);

        package
            .events
            .iter_mut()
            .for_each(|(_, event)| self.visit_event(event));

        self.remove_bound_data(to_remove_alias, to_remove_vars);

        trace!("Exiting: visit_package");
        self.table.exit_scope();
        self.curr_package = None;
    }

    fn visit_event(&mut self, event: &mut Event) {
        trace!("Entering: visit_event");

        self.add_event(event);
        event
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, req_args, .. }| {
                self.req_args = req_args.clone();
                self.visit_fn(func);
            });
        let (to_remove_alias, to_remove_vars) = self.visit_bound_vars(&event.def.bound_vars);

        // visit probe_map
        event.probes.values_mut().for_each(|probes| {
            probes.iter_mut().for_each(|probe| {
                self.visit_probe(probe);
            });
        });

        self.remove_bound_data(to_remove_alias, to_remove_vars);

        trace!("Exiting: visit_event");
        self.table.exit_scope();
        self.curr_event = None;
    }

    fn visit_probe(&mut self, probe: &mut Probe) {
        trace!("Entering: visit_probe");

        self.add_probe(probe);
        probe
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, req_args, .. }| {
                self.req_args = req_args.clone();
                self.visit_fn(func);
            });
        let (to_remove_alias, to_remove_vars) = self.visit_bound_vars(&probe.def.bound_vars);

        // Will not visit predicate/body at this stage
        // visit the predicate/body to handle aliases and derived variables!
        if let Some(predicate) = &mut probe.predicate {
            self.visit_expr(predicate);
        }

        // Add the derived variables as new variables at the top of the probe body!
        if let Some(body) = &mut probe.body {
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

        self.remove_bound_data(to_remove_alias, to_remove_vars);
        self.used_derived_vars.clear();

        trace!("Exiting: visit_probe");
        self.table.exit_scope(); // exit the probe scope
        self.table.exit_scope(); // exit the mode scope
        self.curr_probe = None;
    }

    fn visit_fn(&mut self, f: &mut Fn) {
        trace!("Entering: visit_fn");

        // add fn
        self.add_fn(f);

        // Will not visit predicate/body at this stage

        trace!("Exiting: visit_fn");
        self.table.exit_scope();
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

            let stmt = match &stmt {
                Statement::UnsharedDecl { decl, .. } => &**decl,
                Statement::UnsharedDeclInit { decl, .. } => &**decl,
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
                Statement::UnsharedDeclInit { .. }
                | Statement::UnsharedDecl { .. }
                | Statement::Decl { .. } => {}
                _ => self.err.add_internal_error(
                    &format!("Should already be handled: {stmt:?}"),
                    stmt.loc(),
                ),
            }
        }
    }

    fn visit_stmt_global(&mut self, stmt: &mut Statement) {
        match stmt {
            Statement::LibImport { .. } => {}
            _ => self.visit_stmt(stmt),
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
            Expr::Primitive {
                val: Value::Tuple { vals, .. },
                ..
            } => {
                for val in vals.iter_mut() {
                    self.visit_expr(val);
                }
            }
            Expr::MapGet { map, key, .. } => {
                self.visit_expr(map);
                self.visit_expr(key);
            }
            _ => {}
        }
    }
    fn visit_value(&mut self, _val: &mut Value) {
        // Not visiting predicates/statements
        panic!("{UNEXPECTED_ERR_MSG}");
    }
}

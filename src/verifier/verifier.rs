use crate::common::error::ErrorGen;
use crate::generator::ast::StackReq;
use crate::parser::provider_handler::{Event, Package, Probe, Provider};
use crate::parser::types::Definition::{CompilerDynamic, CompilerStatic};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Fn, Location, Script, Statement, UnOp, Value, Whamm,
    WhammVisitorMut,
};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::{Record, SymbolTable};
use pest::error::LineColLocation;
use std::collections::{HashMap, HashSet};
use std::vec;
use wirm::Module;

const UNEXPECTED_ERR_MSG: &str =
    "TypeChecker: Looks like you've found a bug...please report this behavior! Exiting now...";

pub fn type_check(ast: &mut Whamm, st: &mut SymbolTable, err: &mut ErrorGen) -> (bool, bool) {
    let mut type_checker = TypeChecker::new(st, err);
    type_checker.visit_whamm(ast);
    let has_reports = type_checker.has_reports;

    // note that parser errors might propagate here
    (!err.has_errors, has_reports)
}

pub fn build_symbol_table(
    ast: &mut Whamm,
    user_libs: &HashMap<String, (Option<String>, Module)>,
    err: &mut ErrorGen,
) -> SymbolTable {
    let mut visitor = SymbolTableBuilder {
        table: SymbolTable::new(),
        user_libs,
        err,
        curr_whamm: None,
        curr_script: None,
        curr_provider: None,
        curr_package: None,
        curr_event: None,
        curr_mode: None,
        curr_probe: None,
        curr_fn: None,
        aliases: HashMap::default(),
        used_derived_vars: HashSet::default(),
        derived_vars: HashMap::default(),
        req_args: StackReq::None,
    };
    visitor.visit_whamm(ast);
    visitor.table
}
pub fn check_duplicate_id(
    name: &str,
    loc: &Option<Location>,
    definition: &Definition,
    table: &SymbolTable,
    err: &mut ErrorGen,
) -> bool {
    if let Some(rec_id) = table.lookup(name) {
        let Some(old_rec) = table.get_record(rec_id) else {
            unreachable!("Could not find record with id: {rec_id}");
        };
        let old_loc = old_rec.loc();
        if old_loc.is_none() {
            //make sure old_rec is comp defined
            if old_rec.is_comp_defined() {
                let new_loc = loc.as_ref().map(|l| l.line_col.clone());
                if loc.is_none() {
                    // happens if new_loc is compiler-defined or is a user-def func without location -- both should throw unexpected error
                    println!("{:#?}", old_rec);
                    println!("{:#?}", table.get_curr_scope());
                    panic!("{UNEXPECTED_ERR_MSG} No location found for record: {name}");
                } else {
                    err.compiler_fn_overload_error(name.to_string(), new_loc);
                }
            } else {
                panic!("{UNEXPECTED_ERR_MSG} Expected other record to be defined by compiler.");
            }
        } else if loc.is_none() {
            // happens if new ID is compiler-defined or is a user-def func without location
            //if new ID is compiler-defined, throw compiler overload error for the old record
            if definition.is_comp_defined() {
                err.compiler_fn_overload_error(
                    name.to_string(),
                    old_loc.clone().map(|l| l.line_col),
                );
            } else {
                //otherwise throw unexpected error as user-def fn has no loc
                unreachable!("{UNEXPECTED_ERR_MSG} Expected record to be compiler defined.");
            }
        } else {
            err.duplicate_identifier_error(
                name.to_string(),
                loc.clone().map(|l| l.line_col),
                old_loc.clone().map(|l| l.line_col),
            );
        }
        return true;
    }
    false
}
struct TypeChecker<'a> {
    table: &'a mut SymbolTable,
    err: &'a mut ErrorGen,
    in_script_global: bool,
    in_function: bool,
    has_reports: bool,
    curr_match_rule: Option<String>,

    // If a lib function call, use this for lookup
    curr_lib: Vec<(String, bool)>,

    // bookkeeping for casting
    curr_loc: Option<Location>,
    outer_cast_fixes_assign: bool,
    assign_ty: Option<DataType>,
    tuple_index: usize,
}

impl<'a> TypeChecker<'a> {
    pub fn new(table: &'a mut SymbolTable, err: &'a mut ErrorGen) -> Self {
        Self {
            table,
            err,
            in_script_global: false,
            in_function: false,
            has_reports: false,
            curr_match_rule: None,
            curr_lib: vec![],
            curr_loc: None,
            outer_cast_fixes_assign: false,
            assign_ty: None,
            tuple_index: 0,
        }
    }
    fn add_local(
        &mut self,
        ty: DataType,
        name: String,
        definition: Definition,
        loc: &Option<Location>,
    ) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&name, loc, &definition, self.table, self.err) {
            return;
        }

        // Add local to scope
        let _ = self.table.put(
            name.clone(),
            Record::Var {
                ty,
                value: None,
                def: definition,
                addr: None,
                loc: loc.clone(),
            },
        );
    }

    fn set_curr_rule(&mut self, val: Option<String>) {
        self.curr_match_rule = val;
    }

    fn get_curr_rule(&self) -> &String {
        let Some(curr_rule) = &self.curr_match_rule else {
            panic!("should have a value associated with the curr_match_rule")
        };
        curr_rule
    }

    fn append_curr_rule(&mut self, val: String) {
        let Some(curr_rule) = &mut self.curr_match_rule else {
            panic!("should have a value associated with the curr_match_rule")
        };
        *curr_rule += &val;
        self.err.update_match_rule(self.curr_match_rule.clone());
    }

    fn handle_type_bounds(&mut self, type_bounds: &[(Expr, DataType)]) {
        // TODO -- fix type bounds bug: put them local to the probe, not global to the event
        //         ALSO need to handle type bounds on provider/package/mode as well!
        for (var, ty_bound) in type_bounds.iter() {
            if let Expr::VarId { name, loc, .. } = var {
                if let Some(id) = self.table.lookup(name) {
                    if let Some(rec) = self.table.get_record_mut(id) {
                        if let Record::Var { ty, def, loc, .. } = rec {
                            if !matches!(def, CompilerDynamic) {
                                self.err.type_check_error(
                                    "Type bounds should only be done for dynamically defined compiler variables (e.g. argN, localN)".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                            }
                            *ty = ty_bound.clone();
                        } else {
                            // unexpected record type
                            unreachable!("{UNEXPECTED_ERR_MSG} Expected Var type")
                        }
                    }
                } else {
                    let _ = self.table.put(
                        name.clone(),
                        Record::Var {
                            ty: ty_bound.clone(),
                            value: None,
                            def: CompilerDynamic,
                            addr: None,
                            loc: loc.clone(),
                        },
                    );
                }
            } else {
                self.err
                    .type_check_error(format!("{UNEXPECTED_ERR_MSG} Expected VarId type"), &None);
            }
        }
    }
}

impl WhammVisitorMut<Option<DataType>> for TypeChecker<'_> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> Option<DataType> {
        // not printing events and globals now
        self.table.reset();

        // since the fn child comes first, we enter the named scope after
        // getting into user defined function and scripts
        // not entering scopes here

        // skip the compiler defined functions
        // we only need to type check user defined functions

        whamm.scripts.iter_mut().for_each(|script| {
            self.visit_script(script);
        });

        None
    }

    fn visit_script(&mut self, script: &mut Script) -> Option<DataType> {
        self.table.enter_named_scope(&script.id.to_string());
        self.in_script_global = true;
        script.global_stmts.iter_mut().for_each(|stmt| {
            self.visit_stmt_global(stmt);
        });
        self.in_script_global = false;
        self.in_function = true;
        script.fns.iter_mut().for_each(|function| {
            self.visit_fn(function);
        });
        self.in_function = false;

        script.providers.iter_mut().for_each(|(_, provider)| {
            self.visit_provider(provider);
        });

        self.table.exit_scope();
        None
    }

    fn visit_provider(&mut self, provider: &mut Provider) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&provider.def.name);
        self.set_curr_rule(Some(provider.def.name.clone()));

        self.handle_type_bounds(&provider.type_bounds);

        provider.packages.values_mut().for_each(|package| {
            self.visit_package(package);
        });

        self.table.exit_scope();
        self.set_curr_rule(None);
        None
    }

    fn visit_package(&mut self, package: &mut Package) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&package.def.name);
        self.append_curr_rule(format!(":{}", package.def.name));

        self.handle_type_bounds(&package.type_bounds);

        package.events.values_mut().for_each(|event| {
            self.visit_event(event);
        });

        self.table.exit_scope();
        // Remove this package from `curr_rule`
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(Some(curr_rule[..curr_rule.rfind(':').unwrap()].to_string()));
        None
    }

    fn visit_event(&mut self, event: &mut Event) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&event.def.name);
        self.append_curr_rule(format!(":{}", event.def.name));

        self.handle_type_bounds(&event.type_bounds);

        // Iterate over the probes in order
        event.probes.values_mut().for_each(|probe| {
            probe.iter_mut().for_each(|probe| {
                self.visit_probe(probe);
            });
        });

        self.table.exit_scope();
        let curr_rule = self.get_curr_rule();
        let new_rule = curr_rule[..curr_rule.rfind(':').unwrap()].to_string();
        self.set_curr_rule(Some(new_rule));
        None
    }

    fn visit_probe(&mut self, probe: &mut Probe) -> Option<DataType> {
        assert!(self.table.enter_named_scope(&probe.kind.name())); // enter mode scope
        assert!(self.table.enter_named_scope(&probe.scope_id.to_string())); // enter probe scope
        self.append_curr_rule(format!(":{}", probe.kind.name()));

        // type check predicate
        if let Some(predicate) = &mut probe.predicate {
            let predicate_loc = predicate.loc().clone().unwrap();
            if let Some(ty) = self.visit_expr(predicate) {
                if ty != DataType::Boolean {
                    self.err.type_check_error(
                        "Predicate must be of type boolean".to_owned(),
                        &Some(predicate_loc.line_col),
                    );
                }
            }
        }

        // type check action
        if let Some(body) = &mut probe.body {
            self.visit_block(body);
        }

        self.table.exit_scope(); // exit the mode scope
        self.table.exit_scope(); // exit the probe scope
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(Some(curr_rule[..curr_rule.rfind(':').unwrap()].to_string()));
        None
    }

    fn visit_fn(&mut self, function: &mut Fn) -> Option<DataType> {
        // TODO: not typechecking user defined functions yet
        // type check body

        self.table.enter_named_scope(&function.name.name);
        if let Some(check_ret_type) = self.visit_block(&mut function.body) {
            //figure out how to deal with void functions (return type is ())
            if check_ret_type != function.results {
                self.err.type_check_error(
                    format!(
                        "The function signature for '{}' returns '{:?}', but the body returns '{:?}'",
                        function.name.name, function.results, check_ret_type
                    ),
                    &function.name.loc.clone().map(|l| l.line_col),
                );
            }
        }

        //return the type of the fn
        self.table.exit_scope();
        Some(function.results.clone())
    }

    fn visit_formal_param(&mut self, _param: &mut (Expr, DataType)) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_block(&mut self, block: &mut Block) -> Option<DataType> {
        let mut ret_type = None;
        let num_statements = block.stmts.len();
        let start_of_range: usize;
        for i in 0..num_statements {
            let temp = self.visit_stmt(&mut block.stmts[i]);
            if temp.is_some() && ret_type.is_none() {
                ret_type = temp;
            } else if ret_type.is_some() {
                start_of_range = i;
                //get the span for the first statement to the last one
                let loc = Location::from(
                    &block.stmts[start_of_range].loc().clone().unwrap().line_col,
                    &block.stmts[num_statements - 1]
                        .loc()
                        .clone()
                        .unwrap()
                        .line_col,
                    None,
                );
                self.err.add_typecheck_warn(
                    "Unreachable code detected, these statement(s) will not be executed"
                        .to_string(),
                    Some(loc.line_col),
                );
                block.results = ret_type.clone();
                return ret_type;
            }
        }
        block.results = ret_type.clone();
        //add a check for return statement type matching the function return type if provided
        ret_type
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) -> Option<DataType> {
        if self.in_function {
            if let Statement::UnsharedDecl { .. } = stmt {
                self.err.type_check_error(
                    "Special declarations are not allowed in the functions".to_owned(),
                    &stmt.loc().clone().map(|l| l.line_col),
                );
                return None;
            }
        }

        if self.in_script_global {
            match stmt {
                //allow declarations and assignment
                Statement::Decl { .. } | Statement::Assign { .. } | Statement::LibImport { .. } => {
                }
                Statement::UnsharedDecl { is_report, .. } => {
                    if *is_report {
                        self.has_reports = true;
                    }
                }
                Statement::UnsharedDeclInit { decl, .. } => {
                    if let Statement::UnsharedDecl { is_report, .. } = **decl {
                        if is_report {
                            self.has_reports = true;
                        }
                    }
                }
                _ => {
                    self.err.type_check_error(
                        format!("Only variable declarations, user lib imports, and assignment are allowed in the global scope, found: {:?}", stmt),
                        &stmt.loc().clone().map(|l| l.line_col),
                    );
                    return None;
                }
            }
        }
        match stmt {
            Statement::Assign { var_id, expr, .. } => {
                // change type in symbol table?
                let (full_loc, rhs_loc) = match (var_id.loc(), expr.loc()) {
                    (Some(var_id_loc), Some(expr_loc)) => (
                        Some(
                            Location::from(&var_id_loc.line_col, &expr_loc.line_col, None).line_col,
                        ),
                        Some(expr_loc.line_col.clone()),
                    ),
                    (None, Some(expr_loc)) => (None, Some(expr_loc.line_col.clone())),
                    _ => (None, None),
                };
                let lhs_ty_op = self.visit_expr(var_id);

                if let Some(lhs_ty) = &lhs_ty_op {
                    if let Expr::UnOp {
                        op: UnOp::Cast { target },
                        ..
                    } = expr
                    {
                        self.outer_cast_fixes_assign = lhs_ty == target;
                    }
                }
                self.assign_ty = lhs_ty_op.clone();
                let rhs_ty_op = self.visit_expr(expr);

                let res = if let (Some(lhs_ty), Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    if lhs_ty == rhs_ty {
                        None
                    } else if rhs_ty.can_implicitly_cast() && lhs_ty.can_implicitly_cast() {
                        match expr.implicit_cast(&lhs_ty) {
                            Ok(_) => None,
                            Err(msg) => {
                                self.err.type_check_error(msg, &rhs_loc);
                                None
                            }
                        }
                    } else {
                        // using a struct in parser to merge two locations
                        self.err.type_check_error(
                            format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                            &full_loc,
                        );

                        None
                    }
                } else {
                    self.err.type_check_error(
                        "Can't get type of lhs or rhs of this assignment".to_string(),
                        &full_loc,
                    );
                    None
                };
                self.outer_cast_fixes_assign = false;
                self.assign_ty = None;
                res
            }
            Statement::UnsharedDeclInit { decl, init, .. } => {
                self.visit_stmt(decl);
                self.visit_stmt(init);
                None
            }
            Statement::UnsharedDecl {
                decl, is_report, ..
            } => {
                if *is_report {
                    self.has_reports = true;
                }
                self.visit_stmt(decl)
            }
            Statement::Expr { expr, .. } => {
                self.visit_expr(expr);
                None
            }
            Statement::Decl {
                ty, var_id, loc, ..
            } => {
                if let Expr::VarId { name, .. } = var_id {
                    //check that if type is map, key_ty is not a map
                    if let DataType::Map { key_ty, .. } = ty {
                        if let DataType::Map { .. } = key_ty.as_ref() {
                            self.err.type_check_error(
                                "Map keys cannot be maps".to_owned(),
                                &loc.clone().map(|l| l.line_col),
                            );
                            return None;
                        }
                    }
                    //check to make sure that that if tuple, doesn't contain a map
                    if let DataType::Tuple { ty_info } = ty {
                        for ty in ty_info {
                            if let DataType::Map { .. } = ty {
                                self.err.type_check_error(
                                    "Tuples cannot contain maps".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return None;
                            }
                        }
                    }
                    if !self.in_script_global {
                        self.add_local(ty.to_owned(), name.to_owned(), Definition::User, loc);
                    }
                } else {
                    unreachable!(
                        "{} \
                    Variable declaration var_id is not the correct Expr variant!!",
                        UNEXPECTED_ERR_MSG
                    );
                }
                None
            }
            Statement::Return { expr, loc: _loc } => self.visit_expr(expr),
            Statement::If {
                cond, conseq, alt, ..
            } => {
                let cond_ty = self.visit_expr(cond);
                if cond_ty != Some(DataType::Boolean) {
                    self.err.type_check_error(
                        format!(
                            "Condition must be of type boolean, found {:?}",
                            cond_ty.unwrap()
                        )
                        .to_owned(),
                        &Some(cond.loc().clone().unwrap().line_col),
                    );
                }
                let ret_ty_conseq = self.visit_block(conseq);
                let ret_ty_alt = self.visit_block(alt);
                if ret_ty_conseq == ret_ty_alt {
                    ret_ty_conseq
                } else {
                    //check if it is assume good
                    let empty_tuple = Some(DataType::Tuple { ty_info: vec![] });
                    match (ret_ty_conseq, ret_ty_alt) {
                        (None, _) | (_, None) => return None,
                        (Some(DataType::AssumeGood), _) | (_, Some(DataType::AssumeGood)) => {
                            return Some(DataType::AssumeGood);
                        }
                        (conseq, _) if conseq == empty_tuple.clone() => return empty_tuple.clone(),
                        (_, alt) if alt == empty_tuple.clone() => return empty_tuple.clone(),
                        (_, _) => {}
                    }
                    //check that they are not returning different types if neither is () or None
                    //error here
                    self.err.type_check_error(
                        "Return type of if and else blocks do not match".to_owned(),
                        &Some(
                            Location::from(
                                &conseq.loc().clone().unwrap().line_col,
                                &alt.loc().clone().unwrap().line_col,
                                None,
                            )
                            .line_col,
                        ),
                    );
                    Some(DataType::AssumeGood)
                }
            }
            Statement::SetMap { map, key, val, loc } => {
                //ensure that map is a map, then get the other stuff from the map info
                let map_ty = self.visit_expr(map);
                let key_ty = self.visit_expr(key);
                let val_ty = self.visit_expr(val);
                match (map_ty.clone(), key_ty.clone(), val_ty.clone()) {
                    (None, _, _) | (_, None, _) | (_, _, None) => {
                        self.err.type_check_error(
                            "Can't get type of map, key or value".to_owned(),
                            &loc.clone().map(|l| l.line_col),
                        );
                        return None;
                    }
                    _ => {
                        //we know that the types are all "Some"
                        let key_ty = key_ty.unwrap();
                        let val_ty = val_ty.unwrap();
                        let map_ty = map_ty.unwrap();
                        if let DataType::Map {
                            key_ty: map_key_ty,
                            val_ty: map_val_ty,
                        } = map_ty
                        {
                            //ensure that the key_ty matches and the val_ty matches
                            if key_ty != *map_key_ty {
                                let key_loc = Some(key.loc().clone().unwrap().line_col);
                                attempt_implicit_cast(
                                    key,
                                    &map_key_ty,
                                    &key_ty,
                                    &key_loc,
                                    "key",
                                    self.err,
                                );
                                return None;
                            }
                            if val_ty != *map_val_ty {
                                let val_loc = Some(val.loc().clone().unwrap().line_col);
                                attempt_implicit_cast(
                                    val,
                                    &map_val_ty,
                                    &val_ty,
                                    &val_loc,
                                    "val",
                                    self.err,
                                );
                                return None;
                            }
                        } else if matches!(map_ty, DataType::AssumeGood) {
                            return Some(DataType::AssumeGood);
                        } else {
                            self.err.type_check_error(
                                "Expected Map type".to_string(),
                                &loc.clone().map(|l| l.line_col),
                            );
                            return None;
                        }
                    }
                }
                None
            }
            _ => {
                self.err.add_internal_error(
                    &format!("Should already be handled: {stmt:?}"),
                    stmt.loc(),
                );
                None
            }
        }
    }

    fn visit_stmt_global(&mut self, stmt: &mut Statement) -> Option<DataType> {
        match stmt {
            Statement::LibImport { .. } => None,
            _ => self.visit_stmt(stmt),
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Option<DataType> {
        match expr {
            Expr::Primitive { val, loc } => {
                self.curr_loc = loc.clone();
                self.visit_value(val)
            }
            Expr::BinOp {
                lhs,
                rhs,
                op,
                done_on,
                ..
            } => {
                let (full_line_col, rhs_line_col) = match (lhs.loc(), rhs.loc()) {
                    (Some(lhs_loc), Some(rhs_loc)) => {
                        let full_line_col =
                            Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None)
                                .line_col
                                .clone();
                        (Some(full_line_col), Some(rhs_loc.line_col.clone()))
                    }
                    (Some(lhs_loc), None) => {
                        let full_line_col =
                            Location::from(&lhs_loc.line_col, &lhs_loc.line_col, None)
                                .line_col
                                .clone();
                        (Some(full_line_col), Some(lhs_loc.line_col.clone()))
                    }
                    (None, Some(rhs_loc)) => (None, Some(rhs_loc.line_col.clone())),
                    _ => (None, None),
                };
                let lhs_ty_op = self.visit_expr(lhs);
                let rhs_ty_op = self.visit_expr(rhs);
                if let (Some(lhs_ty), Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    *done_on = lhs_ty.clone();
                    match op {
                        BinOp::Add
                        | BinOp::Subtract
                        | BinOp::Multiply
                        | BinOp::Divide
                        | BinOp::Modulo => {
                            if matches!(lhs_ty, DataType::AssumeGood) {
                                return Some(rhs_ty);
                            } else if matches!(rhs_ty, DataType::AssumeGood) {
                                return Some(lhs_ty);
                            }

                            if !self.outer_cast_fixes_assign {
                                if let Some(exp_ty) = &self.assign_ty {
                                    if *exp_ty == lhs_ty {
                                        if lhs_ty == rhs_ty {
                                            return Some(lhs_ty);
                                        } else if attempt_implicit_cast(
                                            rhs,
                                            exp_ty,
                                            &rhs_ty,
                                            &full_line_col,
                                            "value",
                                            self.err,
                                        ) {
                                            return Some(exp_ty.clone());
                                        }
                                    } else if attempt_implicit_cast(
                                        lhs,
                                        exp_ty,
                                        &lhs_ty,
                                        &full_line_col,
                                        "value",
                                        self.err,
                                    ) {
                                        return Some(exp_ty.clone());
                                    }
                                }
                            } else if lhs_ty == rhs_ty
                                || attempt_implicit_cast(
                                    rhs,
                                    &lhs_ty,
                                    &rhs_ty,
                                    &rhs_line_col,
                                    "value",
                                    self.err,
                                )
                            {
                                return Some(lhs_ty);
                            }
                            Some(DataType::AssumeGood)
                        }
                        BinOp::And | BinOp::Or => {
                            if matches!(lhs_ty, DataType::AssumeGood)
                                || matches!(rhs_ty, DataType::AssumeGood)
                            {
                                return Some(DataType::Boolean);
                            }
                            if lhs_ty == DataType::Boolean && rhs_ty == DataType::Boolean {
                                Some(DataType::Boolean)
                            } else {
                                self.err.type_check_error(
                                    "Different types for lhs and rhs".to_owned(),
                                    &full_line_col,
                                );
                                Some(DataType::AssumeGood)
                            }
                        }
                        BinOp::EQ | BinOp::NE => {
                            if matches!(lhs_ty, DataType::AssumeGood)
                                || matches!(rhs_ty, DataType::AssumeGood)
                            {
                                return Some(DataType::Boolean);
                            }
                            if lhs_ty == rhs_ty
                                || attempt_implicit_cast(
                                    rhs,
                                    &lhs_ty,
                                    &rhs_ty,
                                    &rhs_line_col,
                                    "value",
                                    self.err,
                                )
                            {
                                Some(DataType::Boolean)
                            } else {
                                Some(DataType::AssumeGood)
                            }
                        }
                        BinOp::GT | BinOp::LT | BinOp::GE | BinOp::LE => {
                            if matches!(lhs_ty, DataType::AssumeGood)
                                || matches!(rhs_ty, DataType::AssumeGood)
                            {
                                return Some(DataType::Boolean);
                            }
                            if lhs_ty == rhs_ty && lhs_ty.can_implicitly_cast()
                                || attempt_implicit_cast(
                                    rhs,
                                    &lhs_ty,
                                    &rhs_ty,
                                    &rhs_line_col,
                                    "value",
                                    self.err,
                                )
                            {
                                Some(DataType::Boolean)
                            } else {
                                Some(DataType::AssumeGood)
                            }
                        }
                        BinOp::LShift => {
                            if matches!(lhs_ty, DataType::F32 | DataType::F64) {
                                self.err.type_check_error(
                                    format!("Left shift operation not allowed on type: {}", lhs_ty),
                                    &full_line_col,
                                );
                                return Some(DataType::AssumeGood);
                            } else if matches!(lhs_ty, DataType::AssumeGood) {
                                return Some(DataType::AssumeGood);
                            } else if matches!(rhs_ty, DataType::AssumeGood) {
                                return Some(lhs_ty);
                            }

                            if lhs_ty.is_numeric()
                                && (lhs_ty == rhs_ty
                                    || attempt_implicit_cast(
                                        rhs,
                                        &lhs_ty,
                                        &rhs_ty,
                                        &rhs_line_col,
                                        "value",
                                        self.err,
                                    ))
                            {
                                return Some(lhs_ty);
                            }
                            Some(DataType::AssumeGood)
                        }
                        BinOp::RShift => {
                            if matches!(lhs_ty, DataType::F32 | DataType::F64) {
                                self.err.type_check_error(
                                    format!(
                                        "Right shift operation not allowed on type: {}",
                                        lhs_ty
                                    ),
                                    &full_line_col,
                                );
                                return Some(DataType::AssumeGood);
                            } else if matches!(lhs_ty, DataType::AssumeGood) {
                                return Some(DataType::AssumeGood);
                            } else if matches!(rhs_ty, DataType::AssumeGood) {
                                return Some(lhs_ty);
                            }

                            if lhs_ty.is_numeric()
                                && (lhs_ty == rhs_ty
                                    || attempt_implicit_cast(
                                        rhs,
                                        &lhs_ty,
                                        &rhs_ty,
                                        &rhs_line_col,
                                        "value",
                                        self.err,
                                    ))
                            {
                                return Some(lhs_ty);
                            }
                            Some(DataType::AssumeGood)
                        }
                        BinOp::BitAnd => {
                            if matches!(lhs_ty, DataType::F32 | DataType::F64) {
                                self.err.type_check_error(
                                    format!(
                                        "The bitwise AND operation not allowed on type: {}",
                                        lhs_ty
                                    ),
                                    &full_line_col,
                                );
                                return Some(DataType::AssumeGood);
                            } else if matches!(lhs_ty, DataType::AssumeGood) {
                                return Some(DataType::AssumeGood);
                            } else if matches!(rhs_ty, DataType::AssumeGood) {
                                return Some(lhs_ty);
                            }

                            if lhs_ty.is_numeric()
                                && (lhs_ty == rhs_ty
                                    || attempt_implicit_cast(
                                        rhs,
                                        &lhs_ty,
                                        &rhs_ty,
                                        &rhs_line_col,
                                        "value",
                                        self.err,
                                    ))
                            {
                                return Some(lhs_ty);
                            }
                            Some(DataType::AssumeGood)
                        }
                        BinOp::BitOr => {
                            if matches!(lhs_ty, DataType::F32 | DataType::F64) {
                                self.err.type_check_error(
                                    format!(
                                        "The bitwise OR operation not allowed on type: {}",
                                        lhs_ty
                                    ),
                                    &full_line_col,
                                );
                                return Some(DataType::AssumeGood);
                            } else if matches!(lhs_ty, DataType::AssumeGood) {
                                return Some(DataType::AssumeGood);
                            } else if matches!(rhs_ty, DataType::AssumeGood) {
                                return Some(lhs_ty);
                            }

                            if lhs_ty.is_numeric()
                                && (lhs_ty == rhs_ty
                                    || attempt_implicit_cast(
                                        rhs,
                                        &lhs_ty,
                                        &rhs_ty,
                                        &rhs_line_col,
                                        "value",
                                        self.err,
                                    ))
                            {
                                return Some(lhs_ty);
                            }
                            Some(DataType::AssumeGood)
                        }
                        BinOp::BitXor => {
                            if matches!(lhs_ty, DataType::F32 | DataType::F64) {
                                self.err.type_check_error(
                                    format!(
                                        "The bitwise XOR operation not allowed on type: {}",
                                        lhs_ty
                                    ),
                                    &full_line_col,
                                );
                                return Some(DataType::AssumeGood);
                            } else if matches!(lhs_ty, DataType::AssumeGood) {
                                return Some(DataType::AssumeGood);
                            } else if matches!(rhs_ty, DataType::AssumeGood) {
                                return Some(lhs_ty);
                            }

                            if lhs_ty.is_numeric()
                                && (lhs_ty == rhs_ty
                                    || attempt_implicit_cast(
                                        rhs,
                                        &lhs_ty,
                                        &rhs_ty,
                                        &rhs_line_col,
                                        "value",
                                        self.err,
                                    ))
                            {
                                return Some(lhs_ty);
                            }
                            Some(DataType::AssumeGood)
                        }
                    }
                } else {
                    self.err.type_check_error(
                        "Can't get type of lhs or rhs of this binary operation".to_string(),
                        &full_line_col,
                    );

                    Some(DataType::AssumeGood)
                }
            }
            Expr::VarId {
                name,
                loc,
                definition,
            } => {
                // get type from symbol table
                if let Some(id) = self.table.lookup(name) {
                    if let Some(rec) = self.table.get_record(id) {
                        if let Record::Var { ty, def, value, .. } = rec {
                            *definition = def.clone();
                            // println!("{name}: {ty}");
                            if let Some(val) = value {
                                // overwrite with a primitive value expression!
                                *expr = Expr::Primitive {
                                    val: val.clone(),
                                    loc: expr.loc().clone(),
                                };
                            }
                            return Some(ty.clone());
                        } else {
                            // unexpected record type
                            unreachable!("{} Expected Var type", UNEXPECTED_ERR_MSG)
                        }
                    }
                }
                self.err.type_check_error(
                    format! {"`{}` not found in symbol table", name},
                    &loc.clone().map(|l| l.line_col),
                );

                Some(DataType::AssumeGood)
            }
            Expr::UnOp {
                op,
                expr: inner_expr,
                done_on,
                loc,
            } => {
                let expr_ty_op = self.visit_expr(inner_expr);
                if let Some(expr_ty) = expr_ty_op {
                    *done_on = expr_ty.clone();
                    match op {
                        UnOp::Cast { target } => {
                            // If the inner expression's type is the same as the cast,
                            // we can remove the cast from the AST!
                            let t = target.clone();
                            Some(t)
                        }
                        UnOp::Not => {
                            if expr_ty == DataType::Boolean {
                                Some(DataType::Boolean)
                            } else {
                                self.err.type_check_error(
                                    "Not operator can only be applied to boolean".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                Some(DataType::AssumeGood)
                            }
                        }
                        UnOp::BitwiseNot => {
                            if matches!(expr_ty, DataType::F32 | DataType::F64) {
                                self.err.type_check_error(
                                    format!(
                                        "The bitwise NOT operation not allowed on type: {}",
                                        expr_ty
                                    ),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return Some(DataType::AssumeGood);
                            } else if matches!(expr_ty, DataType::AssumeGood) {
                                return Some(DataType::AssumeGood);
                            }

                            if expr_ty.is_numeric() {
                                return Some(expr_ty);
                            }
                            Some(DataType::AssumeGood)
                        }
                    }
                } else {
                    self.err.type_check_error(
                        "Can't get type of expr of this unary operation".to_owned(),
                        &loc.clone().map(|l| l.line_col),
                    );
                    Some(DataType::AssumeGood)
                }
            }
            Expr::LibCall {
                lib_name,
                call,
                results,
                annotation,
                ..
            } => {
                self.curr_lib.push((
                    lib_name.clone(),
                    annotation.as_ref().map_or_else(|| false, |a| a.is_static()),
                ));
                let res = self.visit_expr(call);
                *results = res.clone();
                self.curr_lib.pop();

                res
            }
            Expr::Call {
                fn_target,
                args,
                loc,
            } => {
                // lookup type of function
                let mut actual_param_tys = vec![];

                for arg in args.iter_mut() {
                    match self.visit_expr(arg) {
                        Some(ty) => actual_param_tys.push(Some(ty)),
                        _ => {
                            self.err.type_check_error(
                                "Can't get type of argument".to_owned(),
                                &loc.clone().map(|l| l.line_col),
                            );
                            return Some(DataType::AssumeGood);
                        }
                    }
                }

                let fn_name = match fn_target.as_ref() {
                    Expr::VarId { name, .. } => name,
                    _ => {
                        self.err.type_check_error(
                            "Function target must be a valid identifier.".to_owned(),
                            &loc.clone().map(|l| l.line_col),
                        );
                        return Some(DataType::AssumeGood);
                    }
                };

                let curr_lib = self.curr_lib.first();
                let rec = if let Some((lib_name, _)) = &curr_lib {
                    if let Some(id) = self.table.lookup_lib_fn(lib_name, fn_name) {
                        id
                    } else {
                        self.err.type_check_error(
                            "Could not find library function".to_string(),
                            &loc.clone().map(|l| l.line_col),
                        );
                        return Some(DataType::AssumeGood);
                    }
                } else if let Some(id) = self.table.lookup_fn(fn_name, true) {
                    id
                } else {
                    return Some(DataType::AssumeGood);
                };

                let (params, ret_ty, def, loc) = match rec {
                    Record::Fn {
                        params,
                        ret_ty,
                        def,
                        loc,
                        ..
                    } => {
                        // look up param
                        let mut expected_param_tys = vec![];
                        for param in params {
                            if let Some(Record::Var { ty, .. }) = self.table.get_record(*param) {
                                // check if it matches actual param
                                expected_param_tys.push(Some(ty.clone()));
                            }
                        }
                        (expected_param_tys, ret_ty.clone(), def, loc)
                    }
                    Record::LibFn {
                        name,
                        params,
                        results,
                        def,
                        loc,
                        ..
                    } => {
                        let ret_ty = if results.len() > 1 {
                            panic!(
                                "We don't support functions with multiple return types: {}.{}",
                                curr_lib.unwrap().0,
                                name
                            );
                        } else if results.is_empty() {
                            DataType::Tuple { ty_info: vec![] }
                        } else {
                            results.first().unwrap().clone()
                        };
                        let mut expected_param_tys = vec![];
                        for param in params.iter() {
                            expected_param_tys.push(Some(param.clone()));
                        }
                        (expected_param_tys, ret_ty, def, loc)
                    }
                    other => {
                        panic!("Got unexpected record type: {:?}", other)
                    }
                };

                if let Some((_, is_static)) = curr_lib {
                    //disallow (non-static) user-function calls when the in the global state of the script
                    if self.in_script_global && !is_static {
                        self.err.type_check_error(
                            "Non-static calls to libraries are not allowed in the global state of the script"
                                .to_owned(),
                            &loc.clone().map(|l| l.line_col),
                        );
                    }
                } else if self.in_script_global
                    && !(*def == CompilerDynamic || *def == CompilerStatic)
                {
                    //check if in global state and if is_comp_defined is false --> not allowed if both are the case
                    self.err.type_check_error(
                        "Function calls to user def functions are not allowed in the global state of the script"
                            .to_owned(),
                        &loc.clone().map(|l| l.line_col),
                    );
                    //continue to check for other errors even after emitting this one
                }

                for (i, (expected, actual)) in
                    params.iter().zip(actual_param_tys.iter()).enumerate()
                {
                    match (expected, actual) {
                        (Some(expected), Some(actual)) => {
                            // if actual is a tuple, it's not structurally equal
                            if expected != actual {
                                let arg = args.get_mut(i).unwrap();
                                let arg_loc = arg.loc().clone().unwrap();
                                if expected.can_implicitly_cast() && actual.can_implicitly_cast() {
                                    // try to implicitly do a cast here
                                    if let Err(msg) = arg.implicit_cast(expected) {
                                        self.err.type_check_error(msg, &Some(arg_loc.line_col))
                                    }
                                } else {
                                    self.err.type_check_error(
                                        format! {"Expected type {:?} param {}, got {:?}", expected, i, actual},
                                        &Some(arg_loc.line_col)
                                    );
                                }
                            }
                        }
                        _ => {
                            self.err.type_check_error(
                                "Can't get type of argument".to_owned(),
                                &loc.clone().map(|l| l.line_col),
                            );
                        }
                    }
                }

                Some(ret_ty.clone())
            }
            Expr::MapGet { map, key, loc } => {
                //ensure that map is a map, then get the other stuff from the map info
                let map_ty = self.visit_expr(map);
                let key_ty = self.visit_expr(key);
                match (map_ty.clone(), key_ty.clone()) {
                    (None, _) | (_, None) => {
                        self.err.type_check_error(
                            "Can't get type of map or key".to_owned(),
                            &loc.clone().map(|l| l.line_col),
                        );
                        Some(DataType::AssumeGood)
                    }
                    _ => {
                        //we know that the types are all "Some"
                        let key_ty = key_ty.unwrap();
                        let map_ty = map_ty.unwrap();
                        if let DataType::Map {
                            key_ty: map_key_ty,
                            val_ty,
                        } = map_ty
                        {
                            //ensure that the key_ty matches and the val_ty matches
                            if key_ty != *map_key_ty {
                                let key_loc = key.loc().clone().unwrap();
                                if key_ty.can_implicitly_cast() && map_key_ty.can_implicitly_cast()
                                {
                                    // try to implicitly do a cast here
                                    if let Err(msg) = key.implicit_cast(map_key_ty.as_ref()) {
                                        self.err.type_check_error(msg, &Some(key_loc.line_col))
                                    } else {
                                        return Some(*val_ty);
                                    }
                                } else {
                                    self.err.type_check_error(
                                        format! {"Type Mismatch, expected key type: {:?}, actual key type:{:?}", map_key_ty, key_ty},
                                        &Some(key_loc.line_col),
                                    );
                                }
                                return Some(DataType::AssumeGood);
                            }
                            Some(*val_ty)
                        } else if matches!(map_ty, DataType::AssumeGood) {
                            Some(DataType::AssumeGood)
                        } else {
                            self.err.type_check_error(
                                "Expected Map type".to_string(),
                                &loc.clone().map(|l| l.line_col),
                            );
                            Some(DataType::AssumeGood)
                        }
                    }
                }
            }
            Expr::Ternary {
                cond,
                conseq,
                alt,
                ty,
                ..
            } => {
                let saved_exp_ty = self.assign_ty.to_owned();
                self.assign_ty = None;
                let cond_ty = self.visit_expr(cond);
                //have to clone before the "if let" block
                let cond_ty_clone = cond_ty.clone();
                if let Some(ty) = cond_ty {
                    if ty != DataType::Boolean {
                        self.err.type_check_error(
                            format!(
                                "Condition must be of type boolean, found {:?}",
                                cond_ty_clone.unwrap()
                            )
                            .to_owned(),
                            &Some(cond.loc().clone().unwrap().line_col),
                        );
                    }
                }

                self.assign_ty = saved_exp_ty;
                let conseq_ty = self.visit_expr(conseq);
                let alt_ty = self.visit_expr(alt);

                match (alt_ty, conseq_ty.clone()) {
                    (Some(alt_t), Some(conseq_t)) => {
                        if alt_t == conseq_t {
                            *ty = alt_t.clone();
                            conseq_ty
                        } else {
                            self.err.type_check_error(
                                "Consequent and alternative must have the same type".to_owned(),
                                &Some(
                                    Location::from(
                                        &conseq.loc().clone().unwrap().line_col,
                                        &alt.loc().clone().unwrap().line_col,
                                        None,
                                    )
                                    .line_col,
                                ),
                            );
                            Some(DataType::AssumeGood)
                        }
                    }
                    _ => {
                        self.err.type_check_error(
                            "Can't get type of consequent or alternative".to_owned(),
                            &Some(
                                Location::from(
                                    &conseq.loc().clone().unwrap().line_col,
                                    &alt.loc().clone().unwrap().line_col,
                                    None,
                                )
                                .line_col,
                            ),
                        );
                        Some(DataType::AssumeGood)
                    }
                }
            }
        }
    }

    fn visit_value(&mut self, val: &mut Value) -> Option<DataType> {
        match val {
            Value::Number { .. } | Value::Boolean { .. } => {
                if !self.outer_cast_fixes_assign {
                    if let Some(exp_ty) = &self.assign_ty {
                        let exp_ty = if let DataType::Tuple { ty_info } = exp_ty {
                            if let Some(ty) = ty_info.get(self.tuple_index) {
                                ty
                            } else {
                                let loc = self.curr_loc.as_ref().map(|loc| loc.line_col.clone());
                                self.err.type_check_error(
                                    format!(
                                        "TypeError: Tuple value at this location exceeded the expected tuple length of: {}.",
                                        ty_info.len()
                                    ),
                                    &loc,
                                );
                                return Some(DataType::AssumeGood);
                            }
                        } else {
                            exp_ty
                        };

                        let val_ty = val.ty();
                        return if *exp_ty == val_ty {
                            Some(val_ty)
                        } else if exp_ty.can_implicitly_cast() && val_ty.can_implicitly_cast() {
                            match val.implicit_cast(exp_ty) {
                                Ok(_) => Some(val.ty()),
                                Err(msg) => {
                                    let loc =
                                        self.curr_loc.as_ref().map(|loc| loc.line_col.clone());
                                    self.err.type_check_error(format!("CastError: Cannot implicitly cast {msg}. Please add an explicit cast."),
                                                              &loc);
                                    Some(DataType::AssumeGood)
                                }
                            }
                        } else {
                            Some(DataType::Unknown)
                        };
                    }
                }
                Some(val.ty())
            }
            Value::Str { .. } => Some(DataType::Str),
            Value::U32U32Map { .. } => Some(DataType::Map {
                key_ty: Box::new(DataType::U32),
                val_ty: Box::new(DataType::U32),
            }),
            Value::Tuple { ty, vals } => {
                // this ty does not contain the DataType in ty_info
                // Whamm parser doesn't give the ty_info for Tuples
                let tys = vals
                    .iter_mut()
                    .enumerate()
                    .map(|(index, val)| {
                        self.tuple_index = index;
                        self.visit_expr(val)
                    })
                    .collect::<Vec<_>>();

                // assume these expressions (actually just values) all parse
                // and have Some type
                let mut all_tys: Vec<DataType> = Vec::new();
                for ty in tys.iter() {
                    match ty {
                        Some(ty) => all_tys.push(ty.to_owned()),
                        _ => {
                            unreachable!(
                                "{} ALL types should be set for a tuple value.",
                                UNEXPECTED_ERR_MSG
                            )
                        }
                    }
                }
                let tuple_ty = DataType::Tuple { ty_info: all_tys };
                *ty = tuple_ty.clone();
                Some(tuple_ty)
            }
        }
    }
}

fn attempt_implicit_cast(
    to_cast: &mut Expr,
    exp_ty: &DataType,
    actual_ty: &DataType,
    loc: &Option<LineColLocation>,
    name_in_err: &str,
    err: &mut ErrorGen,
) -> bool {
    if exp_ty.can_implicitly_cast() && actual_ty.can_implicitly_cast() {
        // try to implicitly do a cast here
        return if let Err(msg) = to_cast.implicit_cast(exp_ty) {
            err.type_check_error(msg, loc);
            false
        } else {
            true
        };
    } else {
        err.type_check_error(
           format! {"Type Mismatch, expected {name_in_err} type: {:?}, actual {name_in_err} type: {:?}", exp_ty, actual_ty},
            loc
        );
    }
    false
}

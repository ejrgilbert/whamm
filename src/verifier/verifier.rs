use crate::common::error::ErrorGen;
use crate::parser::rules::{Event, Package, Probe, Provider, UNKNOWN_IMMS};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Fn, Location, Script, Statement, UnOp, Value, Whamm,
    WhammVisitorMut,
};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::{Record, SymbolTable};
use std::vec;

const UNEXPECTED_ERR_MSG: &str =
    "TypeChecker: Looks like you've found a bug...please report this behavior! Exiting now...";

pub fn build_symbol_table(ast: &mut Whamm, err: &mut ErrorGen) -> SymbolTable {
    let mut visitor = SymbolTableBuilder {
        table: SymbolTable::new(),
        err,
        curr_whamm: None,
        curr_script: None,
        curr_provider: None,
        curr_package: None,
        curr_event: None,
        curr_probe: None,
        curr_fn: None,
    };
    visitor.visit_whamm(ast);
    visitor.table
}
pub fn check_duplicate_id(
    name: &str,
    loc: &Option<Location>,
    is_comp_provided_new: bool,
    table: &SymbolTable,
    err: &mut ErrorGen,
) -> bool {
    if table.lookup(name).is_some() {
        let old_rec = table.get_record(table.lookup(name).unwrap()).unwrap();
        let old_loc = old_rec.loc();
        if old_loc.is_none() {
            //make sure old_rec is comp provided
            if old_rec.is_comp_provided() {
                let new_loc = loc.as_ref().map(|l| l.line_col.clone());
                if loc.is_none() {
                    // happens if new_loc is compiler-provided or is a user-def func without location -- both should throw unexpected error
                    err.unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
                } else {
                    err.compiler_fn_overload_error(false, name.to_string(), new_loc);
                }
            } else {
                err.unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        } else if loc.is_none() {
            // happens if new ID is compiler-provided or is a user-def func without location
            //if new ID is compiler-provided, throw compiler overload error for the old record
            if is_comp_provided_new {
                err.compiler_fn_overload_error(
                    false,
                    name.to_string(),
                    old_loc.clone().map(|l| l.line_col),
                );
            } else {
                //otherwise throw unexpected error as user-def fn has no loc
                err.unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            }
        } else {
            err.duplicate_identifier_error(
                false,
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
}

impl TypeChecker<'_> {
    fn add_local(
        &mut self,
        ty: DataType,
        name: String,
        is_comp_provided: bool,
        loc: &Option<Location>,
    ) {
        /*check_duplicate_id is necessary to make sure we don't try to have 2 records with the same string pointing to them in the hashmap.
        In some cases, it gives a non-fatal error, but in others, it is fatal. Thats why if it finds any error, we return here ->
        just in case it is non-fatal to avoid having 2 strings w/same name in record */
        if check_duplicate_id(&name, loc, is_comp_provided, self.table, self.err) {
            return;
        }

        // Add local to scope
        let _ = self.table.put(
            name.clone(),
            Record::Var {
                ty,
                name,
                value: None,
                is_comp_provided,
                is_report_var: false,
                addr: None,
                loc: loc.clone(),
            },
        );
    }
}

impl WhammVisitorMut<Option<DataType>> for TypeChecker<'_> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> Option<DataType> {
        // not printing events and globals now
        self.table.reset();

        // since the fn child comes first, we enter the named scope after
        // getting into user defined function and scripts
        // not entering scopes here

        // skip the compiler provided functions
        // we only need to type check user provided functions

        whamm.scripts.iter_mut().for_each(|script| {
            self.visit_script(script);
        });

        None
    }

    fn visit_script(&mut self, script: &mut Script) -> Option<DataType> {
        self.table.enter_named_scope(&script.name);
        self.in_script_global = true;
        script.global_stmts.iter_mut().for_each(|stmt| {
            self.visit_stmt(stmt);
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

        let _ = self.table.exit_scope();
        None
    }

    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&provider.name());

        provider.packages_mut().for_each(|package| {
            self.visit_package(package);
        });

        let _ = self.table.exit_scope();
        None
    }

    fn visit_package(&mut self, package: &mut dyn Package) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&package.name());

        package.events_mut().for_each(|event| {
            self.visit_event(event);
        });

        let _ = self.table.exit_scope();

        None
    }

    fn visit_event(&mut self, event: &mut dyn Event) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&event.name());

        event.probes_mut().iter_mut().for_each(|(_, probe)| {
            probe.iter_mut().for_each(|probe| {
                self.visit_probe(probe);
            });
        });

        let _ = self.table.exit_scope();

        None
    }

    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&(*probe).mode().name());

        // type check predicate
        if let Some(predicate) = &mut probe.predicate_mut() {
            let predicate_loc = predicate.loc().clone().unwrap();
            if let Some(ty) = self.visit_expr(predicate) {
                if ty != DataType::Boolean {
                    self.err.type_check_error(
                        false,
                        "Predicate must be of type boolean".to_owned(),
                        &Some(predicate_loc.line_col),
                    );
                }
            }
        }

        // type check action
        if let Some(body) = &mut probe.body_mut() {
            self.visit_block(body);
        }

        let _ = self.table.exit_scope();

        None
    }

    fn visit_fn(&mut self, function: &mut Fn) -> Option<DataType> {
        // TODO: not typechecking user provided functions yet
        // type check body

        self.table.enter_named_scope(&function.name.name);
        if let Some(check_ret_type) = self.visit_block(&mut function.body) {
            //figure out how to deal with void functions (return type is ())
            if check_ret_type != function.return_ty {
                self.err.type_check_error(
                    false,
                    format!(
                        "The function signature for '{}' returns '{:?}', but the body returns '{:?}'",
                        function.name.name, function.return_ty, check_ret_type
                    ),
                    &function.name.loc.clone().map(|l| l.line_col),
                );
            }
        }

        //return the type of the fn
        let _ = self.table.exit_scope();
        Some(function.return_ty.clone())
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
                block.return_ty = ret_type.clone();
                return ret_type;
            }
        }
        block.return_ty = ret_type.clone();
        //add a check for return statement type matching the function return type if provided
        ret_type
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) -> Option<DataType> {
        if self.in_function {
            if let Statement::ReportDecl { .. } = stmt {
                self.err.type_check_error(
                    false,
                    "Report declarations are not allowed in the functions".to_owned(),
                    &stmt.loc().clone().map(|l| l.line_col),
                );
                return None;
            }
        }

        if self.in_script_global {
            match stmt {
                //allow declarations and assignment
                Statement::Decl { .. }
                | Statement::Assign { .. }
                | Statement::ReportDecl { .. } => {}
                _ => {
                    self.err.type_check_error(
                        false,
                        "Only variable declarations and assignment are allowed in the global scope"
                            .to_owned(),
                        &stmt.loc().clone().map(|l| l.line_col),
                    );
                    return None;
                }
            }
        }
        match stmt {
            Statement::Assign { var_id, expr, .. } => {
                // change type in symbol table?
                let lhs_loc = var_id.loc().clone().unwrap();
                let rhs_loc = expr.loc().clone().unwrap();
                let lhs_ty_op = self.visit_expr(var_id);
                let rhs_ty_op = self.visit_expr(expr);

                // TODO -- conversion between numbers here?
                if let (Some(lhs_ty), Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    if lhs_ty == rhs_ty {
                        None
                    } else if (lhs_ty == DataType::U32 || lhs_ty == DataType::I32)
                        && (rhs_ty == DataType::I32 || rhs_ty == DataType::U32)
                    {
                        // TODO -- make this typechecking actually verify that the values won't overflow!
                        let loc = Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                        self.err.add_typecheck_warn(format!("Comparisons between U32/I32 values, possible overflow issue! \
                                Future versions of whamm will verify this compatibility.\n lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty), Some(loc.line_col));
                        None
                    } else {
                        // using a struct in parser to merge two locations
                        let loc = Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                        self.err.type_check_error(
                            false,
                            format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                            &Some(loc.line_col),
                        );

                        None
                    }
                } else {
                    let loc = Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                    self.err.type_check_error(
                        false,
                        "Can't get type of lhs or rhs of this assignment".to_string(),
                        &Some(loc.line_col),
                    );
                    None
                }
            }
            Statement::ReportDecl { decl, .. } => self.visit_stmt(decl),
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
                                false,
                                "Map keys cannot be maps".to_owned(),
                                &loc.clone().map(|l| l.line_col),
                            );
                            return None;
                        }
                    }
                    //check to make sure that that if tuple, doesn't contain a map
                    if let DataType::Tuple { ty_info } = ty {
                        for ty in ty_info {
                            if let DataType::Map { .. } = ty.as_ref() {
                                self.err.type_check_error(
                                    false,
                                    "Tuples cannot contain maps".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return None;
                            }
                        }
                    }
                    if !self.in_script_global {
                        self.add_local(ty.to_owned(), name.to_owned(), false, loc);
                    }
                } else {
                    self.err.unexpected_error(
                        true,
                        Some(format!(
                            "{} \
                    Variable declaration var_id is not the correct Expr variant!!",
                            UNEXPECTED_ERR_MSG
                        )),
                        var_id.loc().clone().map(|l| l.line_col),
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
                        false,
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
                            return Some(DataType::AssumeGood)
                        }
                        (conseq, _) if conseq == empty_tuple.clone() => return empty_tuple.clone(),
                        (_, alt) if alt == empty_tuple.clone() => return empty_tuple.clone(),
                        (_, _) => {}
                    }
                    //check that they are not returning different types if neither is () or None
                    //error here
                    self.err.type_check_error(
                        false,
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
                            false,
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
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, key:{:?}, map_key:{:?}", key_ty, map_key_ty},
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return None;
                            }
                            if val_ty != *map_val_ty {
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, val:{:?}, map_val:{:?}", val_ty, map_val_ty},
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return None;
                            }
                        } else {
                            self.err.unexpected_error(
                                true,
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                loc.clone().map(|l| l.line_col),
                            );
                            return None;
                        }
                    }
                }
                None
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Option<DataType> {
        match expr {
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::BinOp { lhs, rhs, op, .. } => {
                let lhs_loc = lhs.loc().clone().unwrap();
                let rhs_loc = rhs.loc().clone().unwrap();
                let lhs_ty_op = self.visit_expr(lhs);
                let rhs_ty_op = self.visit_expr(rhs);
                if let (Some(lhs_ty), Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    match op {
                        BinOp::Add
                        | BinOp::Subtract
                        | BinOp::Multiply
                        | BinOp::Divide
                        | BinOp::Modulo => {
                            if lhs_ty == DataType::I32 && rhs_ty == DataType::I32 {
                                Some(DataType::I32)
                            } else if (lhs_ty == DataType::U32 || lhs_ty == DataType::I32)
                                && (rhs_ty == DataType::I32 || rhs_ty == DataType::U32)
                            {
                                // TODO -- make this typechecking actually verify that the values won't overflow!
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.add_typecheck_warn(format!("Comparisons between U32/I32 values, possible overflow issue! \
                                Future versions of whamm will verify this compatibility.\n lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty), Some(loc.line_col));
                                Some(DataType::I32)
                            } else {
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );
                                Some(DataType::AssumeGood)
                            }
                        }
                        BinOp::And | BinOp::Or => {
                            if lhs_ty == DataType::Boolean && rhs_ty == DataType::Boolean {
                                Some(DataType::Boolean)
                            } else {
                                self.err.type_check_error(
                                    false,
                                    "Different types for lhs and rhs".to_owned(),
                                    &None,
                                );
                                Some(DataType::AssumeGood)
                            }
                        }

                        BinOp::EQ | BinOp::NE => {
                            if lhs_ty == rhs_ty {
                                Some(DataType::Boolean)
                            } else if (lhs_ty == DataType::U32 || lhs_ty == DataType::I32)
                                && (rhs_ty == DataType::I32 || rhs_ty == DataType::U32)
                            {
                                // TODO -- make this typechecking actually verify that the values won't overflow!
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.add_typecheck_warn(format!("Comparisons between U32/I32 values, possible overflow issue! \
                                Future versions of whamm will verify this compatibility.\n lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty), Some(loc.line_col));
                                Some(DataType::Boolean)
                            } else {
                                // using a struct in parser to merge two locations
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );

                                Some(DataType::AssumeGood)
                            }
                        }
                        BinOp::GT | BinOp::LT | BinOp::GE | BinOp::LE => {
                            if lhs_ty == DataType::I32 && rhs_ty == DataType::I32 {
                                Some(DataType::Boolean)
                            } else if (lhs_ty == DataType::U32 || lhs_ty == DataType::I32)
                                && (rhs_ty == DataType::I32 || rhs_ty == DataType::U32)
                            {
                                // TODO -- make this typechecking actually verify that the values won't overflow!
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.add_typecheck_warn(format!("Comparisons between U32/I32 values, possible overflow issue! \
                                Future versions of whamm will verify this compatibility.\n lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty), Some(loc.line_col));
                                Some(DataType::Boolean)
                            } else {
                                // using a struct in parser to merge two locations
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );

                                Some(DataType::AssumeGood)
                            }
                        }
                    }
                } else {
                    let loc = Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                    self.err.type_check_error(
                        false,
                        "Can't get type of lhs or rhs of this binary operation".to_string(),
                        &Some(loc.line_col),
                    );

                    Some(DataType::AssumeGood)
                }
            }
            Expr::VarId { name, loc, .. } => {
                // TODO: may have a more principled way to handle this (with SymbolTable)
                // if name is prefixed with arg, report error
                if name.starts_with("arg") && name[3..].parse::<u32>().is_ok() {
                    return Some(DataType::AssumeGood);
                }

                // get type from symbol table
                if let Some(id) = self.table.lookup(name) {
                    if let Some(rec) = self.table.get_record(id) {
                        if let Record::Var { ty, .. } = rec {
                            return Some(ty.clone());
                        } else {
                            // unexpected record type
                            self.err.unexpected_error(
                                true,
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                loc.clone().map(|l| l.line_col),
                            )
                        }
                    }
                } else {
                    // check if this is an unknown immN!
                    if name.starts_with("imm") {
                        if let Some(id) = self.table.lookup(UNKNOWN_IMMS) {
                            if let Some(rec) = self.table.get_record(id) {
                                if let Record::Var { ty, .. } = rec {
                                    return Some(ty.clone());
                                } else {
                                    // unexpected record type
                                    self.err.unexpected_error(
                                        true,
                                        Some(UNEXPECTED_ERR_MSG.to_string()),
                                        loc.clone().map(|l| l.line_col),
                                    )
                                }
                            }
                        }
                    }
                }
                self.err.type_check_error(
                    false,
                    format! {"Can't look up `{}` in symbol table", name},
                    &loc.clone().map(|l| l.line_col),
                );

                Some(DataType::AssumeGood)
            }
            Expr::UnOp { op, expr, loc } => {
                let expr_ty_op = self.visit_expr(expr);
                if let Some(expr_ty) = expr_ty_op {
                    match op {
                        UnOp::Not => {
                            if expr_ty == DataType::Boolean {
                                Some(DataType::Boolean)
                            } else {
                                self.err.type_check_error(
                                    false,
                                    "Not operator can only be applied to boolean".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                Some(DataType::AssumeGood)
                            }
                        }
                    }
                } else {
                    self.err.type_check_error(
                        false,
                        "Can't get type of expr of this unary operation".to_owned(),
                        &loc.clone().map(|l| l.line_col),
                    );
                    Some(DataType::AssumeGood)
                }
            }
            //disallow calls when the in the global state of the script
            Expr::Call {
                fn_target,
                args,
                loc,
            } => {
                // lookup type of function
                let mut actual_param_tys = vec![];

                if let Some(args) = args {
                    for arg in args {
                        match self.visit_expr(arg) {
                            Some(ty) => actual_param_tys.push(Some(ty)),
                            _ => {
                                self.err.type_check_error(
                                    false,
                                    "Can't get type of argument".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return Some(DataType::AssumeGood);
                            }
                        }
                    }
                } // else function has no arguments

                let fn_name = match fn_target.as_ref() {
                    Expr::VarId { name, .. } => name,
                    _ => {
                        self.err.type_check_error(
                            false,
                            "Function target must be a valid identifier.".to_owned(),
                            &loc.clone().map(|l| l.line_col),
                        );
                        return Some(DataType::AssumeGood);
                    }
                };

                if let Some(id) = self.table.lookup(fn_name) {
                    if let Some(Record::Fn {
                        name: _,
                        params,
                        ret_ty,
                        addr: _,
                        def,
                        loc,
                    }) = self.table.get_record(id)
                    {
                        //check if in global state and if is_comp_provided is false --> not allowed if both are the case
                        if self.in_script_global
                            && !(*def == Definition::CompilerDynamic
                                || *def == Definition::CompilerStatic)
                        {
                            self.err.type_check_error(
                                false,
                                "Function calls to user def functions are not allowed in the global state of the script"
                                    .to_owned(),
                                &loc.clone().map(|l| l.line_col),
                            );
                            //continue to check for other errors even after emmitting this one
                        }
                        //check if the
                        // look up param
                        let mut expected_param_tys = vec![];
                        for param in params {
                            if let Some(Record::Var { ty, .. }) = self.table.get_record(param) {
                                // check if it matches actual param
                                expected_param_tys.push(Some(ty.clone()));
                            }
                        }
                        for (i, (expected, actual)) in expected_param_tys
                            .iter()
                            .zip(actual_param_tys.iter())
                            .enumerate()
                        {
                            match (expected, actual) {
                                (Some(expected), Some(actual)) => {
                                    // if actual is a tuple, it's not structural equality
                                    if expected != actual {
                                        self.err.type_check_error(
                                            false,
                                            format! {"Expected type {:?} for the {} param, got {:?}", expected, i+1, actual},
                                            &args.clone().map(|a| a[i].loc().clone().unwrap().line_col),
                                        );
                                    }
                                }
                                _ => {
                                    self.err.type_check_error(
                                        false,
                                        "Can't get type of argument".to_owned(),
                                        &loc.clone().map(|l| l.line_col),
                                    );
                                }
                            }
                        }

                        return Some(ret_ty.clone());
                    } else {
                        self.err.type_check_error(
                            false,
                            format! {"Can't look up `{}` in symbol table", fn_name},
                            &loc.clone().map(|l| l.line_col),
                        );
                    }
                } else {
                    self.err.type_check_error(
                        false,
                        format! {"Function {} not found in symbol table", fn_name},
                        &loc.clone().map(|l| l.line_col),
                    );
                }

                Some(DataType::AssumeGood)
            }
            Expr::MapGet { map, key, loc } => {
                //ensure that map is a map, then get the other stuff from the map info
                let map_ty = self.visit_expr(map);
                let key_ty = self.visit_expr(key);
                match (map_ty.clone(), key_ty.clone()) {
                    (None, _) | (_, None) => {
                        self.err.type_check_error(
                            false,
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
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, key:{:?}, map_key:{:?}", key_ty, map_key_ty},
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return Some(DataType::AssumeGood);
                            }
                            Some(*val_ty)
                        } else {
                            self.err.unexpected_error(
                                true,
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                loc.clone().map(|l| l.line_col),
                            );
                            Some(DataType::AssumeGood)
                        }
                    }
                }
            }
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                let cond_ty = self.visit_expr(cond);
                //have to clone before the "if let" block
                let cond_ty_clone = cond_ty.clone();
                if let Some(ty) = cond_ty {
                    if ty != DataType::Boolean {
                        self.err.type_check_error(
                            false,
                            format!(
                                "Condition must be of type boolean, found {:?}",
                                cond_ty_clone.unwrap()
                            )
                            .to_owned(),
                            &Some(cond.loc().clone().unwrap().line_col),
                        );
                    }
                }

                let conseq_ty = self.visit_expr(conseq);
                let alt_ty = self.visit_expr(alt);

                match (alt_ty, conseq_ty.clone()) {
                    (Some(alt_t), Some(conseq_t)) => {
                        if alt_t == conseq_t {
                            conseq_ty
                        } else {
                            self.err.type_check_error(
                                false,
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
                            false,
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

    fn visit_unop(&mut self, _unop: &mut UnOp) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_binop(&mut self, _binop: &mut BinOp) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_datatype(&mut self, _datatype: &mut DataType) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_value(&mut self, val: &mut Value) -> Option<DataType> {
        match val {
            Value::U32 { .. } => Some(DataType::U32),
            Value::I32 { .. } => Some(DataType::I32),
            Value::F32 { .. } => Some(DataType::F32),
            Value::U64 { .. } => Some(DataType::U64),
            Value::I64 { .. } => Some(DataType::I64),
            Value::F64 { .. } => Some(DataType::F64),
            Value::Str { .. } => Some(DataType::Str),
            Value::Boolean { .. } => Some(DataType::Boolean),
            Value::Tuple { ty: _, vals } => {
                // this ty does not contain the DataType in ty_info
                // Whamm parser doesn't give the ty_info for Tuples
                let tys = vals
                    .iter_mut()
                    .map(|val| self.visit_expr(val))
                    .collect::<Vec<_>>();

                // assume these expressions (actually just values) all parse
                // and have Some type
                let mut all_tys: Vec<Box<DataType>> = Vec::new();
                for ty in tys {
                    match ty {
                        Some(ty) => all_tys.push(Box::new(ty)),
                        _ => self.err.unexpected_error(
                            true,
                            Some(UNEXPECTED_ERR_MSG.to_string()),
                            // This provides some imprecise info about the location of the error
                            Some(vals.iter().next().unwrap().loc().clone().unwrap().line_col),
                        ),
                    }
                }
                Some(DataType::Tuple { ty_info: all_tys })
            }
        }
    }
}

pub fn type_check(ast: &mut Whamm, st: &mut SymbolTable, err: &mut ErrorGen) -> bool {
    let mut type_checker = TypeChecker {
        table: st,
        err,
        in_script_global: false,
        in_function: false,
    };
    type_checker.visit_whamm(ast);
    // note that parser errors might propagate here
    !err.has_errors
}

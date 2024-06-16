use std::vec;

use crate::common::error::ErrorGen;
use crate::parser::types::{
    BinOp, DataType, Event, Expr, Fn, Location, Package, Probe, Provider, Script, Statement, UnOp,
    Value, Whamm, WhammVisitor, WhammVisitorMut,
};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::{Record, SymbolTable};

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

struct TypeChecker {
    table: SymbolTable,
    err: ErrorGen,
}

impl TypeChecker {
    fn add_local(&mut self, ty: DataType, name: String, is_comp_provided: bool) {
        if self.table.lookup(&name).is_some() {
            // This should never be the case since it's controlled by the compiler!
            self.err
                .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            unreachable!()
        }

        // Add local to scope
        let _ = self.table.put(
            name.clone(),
            Record::Var {
                ty,
                name,
                value: None,
                is_comp_provided,
                addr: None,
                loc: None,
            },
        );
    }
}

#[derive(Clone, Copy, Debug)]
enum Opt<T> {
    Some(T),
    AssumeGood,
    None,
}

impl WhammVisitor<Opt<DataType>> for TypeChecker {
    fn visit_whamm(&mut self, whamm: &Whamm) -> Opt<DataType> {
        // not printing events and globals now
        self.table.reset();

        // since the fn child comes first, we enter the named scope after
        // getting into user defined function and scripts
        // not entering scopes here

        // skip the compiler provided functions
        // we only need to type check user provided functions

        whamm.scripts.iter().for_each(|script| {
            self.visit_script(script);
        });

        Opt::None
    }

    fn visit_script(&mut self, script: &Script) -> Opt<DataType> {
        self.table.enter_named_scope(&script.name);

        // TODO: type check user provided functions
        // whamm.fns.iter().for_each(|function| {
        //     self.visit_fn(&mut function.1);
        // });

        script.providers.iter().for_each(|(_, provider)| {
            self.visit_provider(provider);
        });

        let _ = self.table.exit_scope();
        Opt::None
    }

    fn visit_provider(&mut self, provider: &Provider) -> Opt<DataType> {
        let _ = self.table.enter_scope();

        provider.packages.iter().for_each(|(_, package)| {
            self.visit_package(package);
        });

        let _ = self.table.exit_scope();
        Opt::None
    }

    fn visit_package(&mut self, package: &Package) -> Opt<DataType> {
        let _ = self.table.enter_scope();

        package.events.iter().for_each(|(_, event)| {
            self.visit_event(event);
        });

        let _ = self.table.exit_scope();

        Opt::None
    }

    fn visit_event(&mut self, event: &Event) -> Opt<DataType> {
        let _ = self.table.enter_scope();

        event.probe_map.iter().for_each(|(_, probe)| {
            probe.iter().for_each(|probe| {
                self.visit_probe(probe);
            });
        });

        let _ = self.table.exit_scope();

        Opt::None
    }

    fn visit_probe(&mut self, probe: &Probe) -> Opt<DataType> {
        let _ = self.table.enter_scope();

        // type check predicate
        if let Some(predicate) = &probe.predicate {
            let predicate_loc = predicate.loc().clone().unwrap();
            if let Opt::Some(ty) = self.visit_expr(predicate) {
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
        if let Some(body) = &probe.body {
            for stmt in body {
                self.visit_stmt(stmt);
            }
        }

        let _ = self.table.exit_scope();

        Opt::None
    }

    fn visit_fn(&mut self, function: &Fn) -> Opt<DataType> {
        // TODO: not typechecking user provided functions yet
        // type check body
        self.table.enter_named_scope(&function.name.name);
        if let Some(body) = &function.body {
            for stmt in body {
                self.visit_stmt(stmt);
            }
        }
        let _ = self.table.exit_scope();

        // return type
        todo!();
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> Opt<DataType> {
        match stmt {
            Statement::Assign { var_id, expr, .. } => {
                // change type in symbol table?
                let lhs_loc = var_id.loc().clone().unwrap();
                let rhs_loc = expr.loc().clone().unwrap();
                let lhs_ty_op = self.visit_expr(var_id);
                let rhs_ty_op = self.visit_expr(expr);

                if let (Opt::Some(lhs_ty), Opt::Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    if lhs_ty == rhs_ty {
                        Opt::None
                    } else {
                        // using a struct in parser to merge two locations
                        let loc = Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                        self.err.type_check_error(
                            false,
                            format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                            &Some(loc.line_col),
                        );

                        Opt::None
                    }
                } else {
                    let loc = Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                    self.err.type_check_error(
                        false,
                        "Can't get type of lhs or rhs of this assignment".to_string(),
                        &Some(loc.line_col),
                    );
                    Opt::None
                }
            }
            Statement::Expr { expr, .. } => {
                self.visit_expr(expr);
                Opt::None
            }
            Statement::Decl { ty, var_id, .. } => {
                if let Expr::VarId { name, .. } = var_id {
                    self.add_local(ty.to_owned(), name.to_owned(), false);
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
                Opt::None
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> Opt<DataType> {
        match expr {
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::BinOp { lhs, rhs, op, .. } => {
                let lhs_loc = lhs.loc().clone().unwrap();
                let rhs_loc = rhs.loc().clone().unwrap();
                let lhs_ty_op = self.visit_expr(lhs);
                let rhs_ty_op = self.visit_expr(rhs);
                if let (Opt::Some(lhs_ty), Opt::Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    match op {
                        BinOp::Add
                        | BinOp::Subtract
                        | BinOp::Multiply
                        | BinOp::Divide
                        | BinOp::Modulo => {
                            if lhs_ty == DataType::I32 && rhs_ty == DataType::I32 {
                                Opt::Some(DataType::I32)
                            } else {
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );
                                Opt::None
                            }
                        }
                        BinOp::And | BinOp::Or => {
                            if lhs_ty == DataType::Boolean && rhs_ty == DataType::Boolean {
                                Opt::Some(DataType::Boolean)
                            } else {
                                self.err.type_check_error(
                                    false,
                                    "Different types for lhs and rhs".to_owned(),
                                    &None,
                                );
                                Opt::None
                            }
                        }

                        BinOp::EQ | BinOp::NE => {
                            if lhs_ty == rhs_ty {
                                Opt::Some(DataType::Boolean)
                            } else {
                                // using a struct in parser to merge two locations
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );

                                Opt::None
                            }
                        }
                        BinOp::GT | BinOp::LT | BinOp::GE | BinOp::LE => {
                            if lhs_ty == DataType::I32 && rhs_ty == DataType::I32 {
                                Opt::Some(DataType::Boolean)
                            } else {
                                // using a struct in parser to merge two locations
                                let loc =
                                    Location::from(&lhs_loc.line_col, &rhs_loc.line_col, None);
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );

                                Opt::None
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
                    Opt::None
                }
            }
            Expr::VarId { name, loc, .. } => {
                // if name is prefixed with arg, report error
                if name.starts_with("arg") {
                    return Opt::AssumeGood;
                }

                // get type from symbol table
                if let Some(id) = self.table.lookup(name) {
                    if let Some(rec) = self.table.get_record(id) {
                        if let Record::Var { ty, .. } = rec {
                            return Opt::Some(ty.clone());
                        } else {
                            // unexpected record type
                            self.err.unexpected_error(
                                true,
                                Some(UNEXPECTED_ERR_MSG.to_string()),
                                loc.clone().map(|l| l.line_col),
                            )
                        }
                    } else {
                        self.err.type_check_error(
                            false,
                            format! {"Can't look up {} in symbol table", name},
                            &loc.clone().map(|l| l.line_col),
                        );
                    }
                }

                Opt::None
            }
            Expr::UnOp { op, expr, loc } => {
                let expr_ty_op = self.visit_expr(expr);
                if let Opt::Some(expr_ty) = expr_ty_op {
                    match op {
                        UnOp::Not => {
                            if expr_ty == DataType::Boolean {
                                Opt::Some(DataType::Boolean)
                            } else {
                                self.err.type_check_error(
                                    false,
                                    "Not operator can only be applied to boolean".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                Opt::None
                            }
                        }
                    }
                } else {
                    self.err.type_check_error(
                        false,
                        "Can't get type of expr of this unary operation".to_owned(),
                        &loc.clone().map(|l| l.line_col),
                    );
                    Opt::None
                }
            }
            Expr::Call {
                fn_target,
                args,
                loc,
            } => {
                // TODO: finish type checking for function calls
                // lookup type of function

                let mut param_tys = vec![];

                if let Some(args) = args {
                    for arg in args {
                        match self.visit_expr(arg) {
                            Opt::Some(ty) => param_tys.push(Opt::Some(ty)),
                            Opt::AssumeGood => param_tys.push(Opt::AssumeGood),
                            _ => {
                                self.err.type_check_error(
                                    false,
                                    "Can't get type of argument".to_owned(),
                                    &loc.clone().map(|l| l.line_col),
                                );
                                return Opt::None;
                            }
                        }
                    }
                } // else function has no arguments

                let fn_name = match *fn_target.clone() {
                    Expr::VarId { name, .. } => name,
                    _ => {
                        self.err.type_check_error(
                            false,
                            "Function target must be a valid identifier.".to_owned(),
                            &loc.clone().map(|l| l.line_col),
                        );
                        return Opt::None;
                    }
                };

                if let Some(id) = self.table.lookup(&fn_name) {
                    if let Some(Record::Fn {
                        name: _,
                        params,
                        addr: _,
                    }) = self.table.get_record(id)
                    {
                        // TODO how to get the real Datatype of the Param
                        // look up param
                        let mut expected_param_tys = vec![];
                        for param in params {
                            if let Some(Record::Var { ty, .. }) = self.table.get_record(param) {
                                // check if it matches actual param
                                expected_param_tys.push(Opt::Some(ty.clone()));
                            }
                        }
                        for (i, (expected, actual)) in
                            expected_param_tys.iter().zip(param_tys.iter()).enumerate()
                        {
                            match (expected, actual) {
                                (Opt::Some(expected), Opt::Some(actual)) => {
                                    if expected != actual {
                                        self.err.type_check_error(
                                            false,
                                            format! {"Expected type {:?} for the {} param, got {:?}", expected, i+1, actual},
                                            &args.clone().map(|a| a[i].loc().clone().unwrap().line_col),
                                        );
                                    }
                                }
                                // only actual param can be assumed good
                                // also omit the case that there will be Opt::None
                                // in expected
                                (_, Opt::AssumeGood) => {}

                                _ => {
                                    self.err.type_check_error(
                                        false,
                                        "Can't get type of argument".to_owned(),
                                        &loc.clone().map(|l| l.line_col),
                                    );
                                }
                            }
                        }

                        // TODO: where is the return type of the function in the symbol table?
                        // now the only comp provided function is strcmp so bool is fine now
                        return Opt::Some(DataType::Boolean);
                    } else {
                        self.err.type_check_error(
                            false,
                            format! {"Can't look up {} in symbol table", fn_name},
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

                Opt::None
            }
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                let cond_ty = self.visit_expr(cond);
                if let Opt::Some(ty) = cond_ty {
                    if ty != DataType::Boolean {
                        self.err.type_check_error(
                            false,
                            "Condition must be of type boolean".to_owned(),
                            &Some(cond.loc().clone().unwrap().line_col),
                        );
                    }
                }

                let conseq_ty = self.visit_expr(conseq);
                let alt_ty = self.visit_expr(alt);

                match (alt_ty, conseq_ty.clone()) {
                    (Opt::Some(alt_t), Opt::Some(conseq_t)) => {
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
                            Opt::None
                        }
                    }
                    _ => Opt::None,
                }
            }
        }
    }

    fn visit_datatype(&mut self, _datatype: &DataType) -> Opt<DataType> {
        unimplemented!()
    }

    fn visit_value(&mut self, val: &Value) -> Opt<DataType> {
        match val {
            Value::Integer { .. } => Opt::Some(DataType::I32),
            Value::Str { .. } => Opt::Some(DataType::Str),
            Value::Boolean { .. } => Opt::Some(DataType::Boolean),
            Value::Tuple { ty: _, vals } => {
                // Alex TODO: this ty does not contain the DataType
                // and I need to recurse to get the type, why?
                let tys = vals
                    .iter()
                    .map(|val| self.visit_expr(val))
                    .collect::<Vec<_>>();

                // assume these expressions (actually just values) all parse
                // and have Some type
                let mut all_tys: Vec<Box<DataType>> = Vec::new();
                for ty in tys {
                    match ty {
                        Opt::Some(ty) => all_tys.push(Box::new(ty)),
                        Opt::AssumeGood => {
                            // if there's one `arg*` in the tuple,
                            // we assume the type to be good
                            return Opt::AssumeGood;
                        }
                        _ => self.err.unexpected_error(
                            true,
                            Some(UNEXPECTED_ERR_MSG.to_string()),
                            Some(vals.iter().next().unwrap().loc().clone().unwrap().line_col),
                        ),
                    }
                }
                Opt::Some(DataType::Tuple {
                    ty_info: Some(all_tys),
                })
            }
        }
    }

    fn visit_formal_param(&mut self, _param: &(Expr, DataType)) -> Opt<DataType> {
        unimplemented!()
    }

    fn visit_binop(&mut self, _binop: &BinOp) -> Opt<DataType> {
        unimplemented!()
    }

    fn visit_unop(&mut self, _unop: &UnOp) -> Opt<DataType> {
        unimplemented!()
    }
}

pub fn type_check(ast: &Whamm, st: &SymbolTable, err: &mut ErrorGen) -> bool {
    let mut type_checker = TypeChecker {
        table: st.clone(),
        err: err.clone(),
    };
    type_checker.visit_whamm(ast);
    // propagate error
    *err = type_checker.err;
    // check if there are any errors
    // TODO: note that parser errors might propagate here
    !err.has_errors
}

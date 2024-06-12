use crate::common::error::ErrorGen;
use crate::parser::types::{
    BinOp, DataType, Event, Expr, Fn, Package, Probe, Provider, Script, Statement, UnOp, Value,
    Whamm, WhammVisitorMut,
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

#[allow(dead_code)]
struct TypeChecker {
    table: SymbolTable,
    err: ErrorGen,
}

impl TypeChecker {
    /// Insert `global` record into scope
    fn add_global(&mut self, ty: DataType, name: String, is_comp_provided: bool) {
        if self.table.lookup(&name).is_some() {
            // This should never be the case since it's controlled by the compiler!
            self.err
                .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
            unreachable!()
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
                loc: None,
            },
        );

        // add global record to the current record
        self.add_global_id_to_curr_rec(id);
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
}

impl WhammVisitorMut<Option<DataType>> for TypeChecker {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> Option<DataType> {
        // not printing events and globals now
        self.table.reset();

        // since the fn child comes first, we need to enter and exit the fn scope
        // before we get to the script scope
        println!("table: {:?}", self.table);

        // skip the compiler provided functions
        // we only need to type check user provided functions
        
        // TODO type check user provided functions
        // whamm.fns.iter_mut().for_each(|function| {
        //     self.visit_fn(&mut function.1);
        // });

        whamm.scripts.iter_mut().for_each(|script| {
            self.visit_script(script);
        });

        None
    }

    fn visit_script(&mut self, script: &mut Script) -> Option<DataType> {
        let _ = self.table.enter_named_scope(&script.name);

        script.providers.iter_mut().for_each(|(_, provider)| {
            self.visit_provider(provider);
        });

        let _ = self.table.exit_scope();
        None
    }

    fn visit_provider(&mut self, provider: &mut Provider) -> Option<DataType> {
        let _ = self.table.enter_scope();

        provider.packages.iter_mut().for_each(|(_, package)| {
            self.visit_package(package);
        });

        let _ = self.table.exit_scope();
        None
    }

    fn visit_package(&mut self, package: &mut Package) -> Option<DataType> {
        let _ = self.table.enter_scope();

        package.events.iter_mut().for_each(|(_, event)| {
            self.visit_event(event);
        });

        let _ = self.table.exit_scope();

        None
    }

    fn visit_event(&mut self, event: &mut Event) -> Option<DataType> {
        let _ = self.table.enter_scope();

        event.probe_map.iter_mut().for_each(|(_, probe)| {
            probe.iter_mut().for_each(|probe| {
                self.visit_probe(probe);
            });
        });

        let _ = self.table.exit_scope();

        None
    }

    fn visit_probe(&mut self, probe: &mut Probe) -> Option<DataType> {
        let _ = self.table.enter_scope();

        // type check predicate
        if let Some(predicate) = &mut probe.predicate {
            let predicate_loc =
                <Option<crate::parser::types::Location> as Clone>::clone(predicate.loc()).unwrap();
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
        if let Some(body) = &mut probe.body {
            for stmt in body {
                self.visit_stmt(stmt);
            }
        }

        let _ = self.table.exit_scope();

        None
    }

    fn visit_fn(&mut self, function: &mut Fn) -> Option<DataType> {
        
        // type check body
        if let Some(body) = &mut function.body {
            for stmt in body {
                self.visit_stmt(stmt);
            }
        }

        // return type
        todo!();
        
        None
    }

    #[allow(unused_variables)]
    fn visit_stmt(&mut self, stmt: &mut Statement) -> Option<DataType> {
        match stmt {
            crate::parser::types::Statement::Assign { var_id, expr, .. } => {
                // change type in symbol table?
                let lhs_loc =
                    <Option<crate::parser::types::Location> as Clone>::clone(var_id.loc()).unwrap();
                let rhs_loc =
                    <Option<crate::parser::types::Location> as Clone>::clone(expr.loc()).unwrap();
                let lhs_ty_op = self.visit_expr(var_id);
                let rhs_ty_op = self.visit_expr(expr);

                if let (Some(lhs_ty), Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    if lhs_ty == rhs_ty {
                        None
                    } else {
                        // using a struct in parser to merge two locations
                        let loc = crate::parser::types::Location::from(
                            &lhs_loc.line_col,
                            &rhs_loc.line_col,
                            None,
                        );
                        self.err.type_check_error(
                            false,
                            format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                            &Some(loc.line_col),
                        );

                        None
                    }
                } else {
                    eprintln!("Error: Cant get type of lhs or rhs");
                    None
                }
            }
            crate::parser::types::Statement::Expr { expr, .. } => {
                self.visit_expr(expr);
                None
            }
            Statement::Decl { ty, var_id, loc } => {
                if let Expr::VarId { name, .. } = var_id {
                    self.add_global(ty.to_owned(), name.to_owned(), false);
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
                None
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Option<DataType> {
        match expr {
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::BinOp { lhs, rhs, op, .. } => {
                let lhs_loc =
                    <Option<crate::parser::types::Location> as Clone>::clone(lhs.loc()).unwrap();
                let rhs_loc =
                    <Option<crate::parser::types::Location> as Clone>::clone(rhs.loc()).unwrap();
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
                            } else {
                                eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
                                None
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
                                // eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
                                None
                            }
                        }

                        BinOp::EQ | BinOp::NE => {
                            if lhs_ty == rhs_ty {
                                Some(DataType::Boolean)
                            } else {
                                // using a struct in parser to merge two locations
                                let loc = crate::parser::types::Location::from(
                                    &lhs_loc.line_col,
                                    &rhs_loc.line_col,
                                    None,
                                );
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );

                                None
                            }
                        }
                        BinOp::GT | BinOp::LT | BinOp::GE | BinOp::LE => {
                            if lhs_ty == DataType::I32 && rhs_ty == DataType::I32 {
                                Some(DataType::Boolean)
                            } else {
                                // using a struct in parser to merge two locations
                                let loc = crate::parser::types::Location::from(
                                    &lhs_loc.line_col,
                                    &rhs_loc.line_col,
                                    None,
                                );
                                self.err.type_check_error(
                                    false,
                                    format! {"Type Mismatch, lhs:{:?}, rhs:{:?}", lhs_ty, rhs_ty},
                                    &Some(loc.line_col),
                                );

                                None
                            }
                        }
                    }
                } else {
                    eprintln!("Error: Cant get type of lhs or rhs");
                    None
                }
            }
            #[allow(unused_variables)]
            Expr::VarId {
                name,
                loc,
                is_comp_provided,
            } => {
                // get type from symbol table
                // println!("curr scope: {:?}", self.table.get_curr_scope());
                // let _ = self.table.enter_scope(); // Alex: adding interscope here doens't seems to have an effect here (which is reasonable)
                // println!("curr scope: {:?}", self.table.get_curr_scope());

                if let Some(id) = self.table.lookup(name) {
                    println!("LOOK AT ME Var id: {:?}", self.table.get_record(id));
                    if let Some(rec) = self.table.get_record(id) {
                        if let crate::verifier::types::Record::Var { ty, .. } = rec {
                            return Some(ty.to_owned());
                        }
                    } else {
                        eprintln!("Error: Cant get record from symbol table");
                    }
                }

                None
            }
            _ => todo!(),
        }
    }

    #[allow(unused_variables)]
    fn visit_datatype(&mut self, datatype: &mut DataType) -> Option<DataType> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn visit_value(&mut self, val: &mut Value) -> Option<DataType> {
        match val {
            Value::Integer { .. } => Some(DataType::I32),
            Value::Str { .. } => Some(DataType::Str),
            Value::Boolean { .. } => Some(DataType::Boolean),
            Value::Tuple { ty, vals } => {
                // Alex TODO: in the example tuple progarm, the type of this tuple is None, but why??
                let tys = vals
                    .iter_mut()
                    .map(|val| self.visit_expr(val))
                    .collect::<Vec<_>>();
                // assume these expressions (actually just values) all parse
                // and have Some type
                let mut all_tys: Vec<Box<DataType>> = Vec::new();
                for ty in tys {
                    if let Some(ty) = ty {
                        all_tys.push(Box::new(ty));
                    } else {
                        self.err
                            .unexpected_error(true, Some(UNEXPECTED_ERR_MSG.to_string()), None);
                    }
                }
                Some(DataType::Tuple {
                    ty_info: Some(all_tys),
                })
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> Option<DataType> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn visit_binop(&mut self, binop: &mut BinOp) -> Option<DataType> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn visit_unop(&mut self, unop: &mut UnOp) -> Option<DataType> {
        unimplemented!()
    }
}

pub fn type_check(ast: &mut Whamm, st: &SymbolTable, err: &mut ErrorGen) -> bool {
    // Alex TODO typechecking!
    // Alex TODO, is cloning the way to go ??
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

use crate::common::error::ErrorGen;
use crate::parser::types::{
    BinOp, DataType, Event, Expr, Fn, Package, Probe, Provider, Script, Statement, UnOp, Value,
    Whamm, WhammVisitorMut,
};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::SymbolTable;

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

impl WhammVisitorMut<Option<DataType>> for TypeChecker {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> Option<DataType> {
        // not printing events and globals now
        self.table.reset();

        // since the fn child comes first, we need to enter and exit the fn scope
        // before we get to the script scope
        println!("table: {:?}", self.table);

        // skip the compiler provided functions
        // we only need to type check user provided functions
        let _ = self.table.enter_scope();
        let _ = self.table.exit_scope();

        whamm.scripts.iter_mut().for_each(|script| {
            self.visit_script(script);
        });

        None
    }

    fn visit_script(&mut self, script: &mut Script) -> Option<DataType> {
        let _ = self.table.enter_scope();

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
            self.visit_expr(predicate);
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

    fn visit_fn(&mut self, _function: &mut Fn) -> Option<DataType> {
        unimplemented!()
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
            // symbol table should handle declaration
            Statement::Decl { ty, var_id, loc } => None,
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
            Expr::VarId { name, loc } => {
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
            _ => unimplemented!(),
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
            // recurse on expressions?
            // Alex TODO: parsing
            Value::Tuple { ty, .. } => Some(ty.to_owned()),
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

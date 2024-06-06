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
}

impl WhammVisitorMut<Option<DataType>> for TypeChecker {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> Option<DataType> {
        let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
        self.table = build_symbol_table(whamm, &mut err);

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
                self.visit_expr(expr);
                None
            }
            crate::parser::types::Statement::Expr { expr, .. } => {
                self.visit_expr(expr);
                None
            }
            Statement::Decl { ty, var_id, loc } => None,
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Option<DataType> {
        match expr {
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::BinOp { lhs, rhs, op, .. } => {
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
                                eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
                                None
                            }
                        }

                        BinOp::EQ | BinOp::NE => {
                            if lhs_ty == rhs_ty {
                                Some(DataType::Boolean)
                            } else {
                                eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
                                None
                            }
                        }
                        BinOp::GT | BinOp::LT | BinOp::GE | BinOp::LE => {
                            if lhs_ty == DataType::I32 && rhs_ty == DataType::I32 {
                                Some(DataType::Boolean)
                            } else {
                                eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
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
                println!("curr scope: {:?}", self.table.get_curr_scope());
                let _ = self.table.enter_scope(); // Alex: adding interscope here doens't seems to have an effect here (which is reasonable)
                println!("curr scope: {:?}", self.table.get_curr_scope());

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
            // Alex TODO: Not sure how to recurse
            Value::Tuple { ty, .. } => {
                match ty {
                    DataType::Tuple { ty_info } => match ty_info {
                        Some(ve) => {}
                        _ => (),
                    },
                    _ => (),
                }
                None
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

pub fn verify(_ast: &mut Whamm) -> bool {
    // Alex TODO typechecking!
    let mut type_checker = TypeChecker {
        table: SymbolTable::new(),
    };
    type_checker.visit_whamm(_ast);
    true
}

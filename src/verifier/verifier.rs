use crate::common::error::ErrorGen;
use crate::parser::types::{DataType, Event, Expr, Fn, UnOp, BinOp, Package, Probe, Provider, Script, Statement, Value, Whamm, WhammVisitorMut};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::{SymbolTable, ScopeType};

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
    fn visit_whamm(&mut self, whamm: &mut Whamm, ) -> Option<DataType> {

        let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
        self.table = build_symbol_table(whamm, &mut err);

        // not printing events and globals now
        self.table.reset();

        // since the fn child comes first, we need to enter and exit the fn scope
        // before we get to the script scope
        let _ = self.table.enter_scope();
        let _ = self.table.exit_scope();


        for script in &whamm.scripts {
            self.visit_script(&mut script.clone());
        }

        None
    }

    fn visit_script(&mut self, script: &mut Script) -> Option<DataType> {
        
        let _ = self.table.enter_scope();

        for (_, provider) in &script.providers {
            self.visit_provider(&mut provider.clone());
        }

        // let _ = self.table.exit_scope();
        None
    }

    fn visit_provider(&mut self, provider: &mut Provider) -> Option<DataType> {
        let _ = self.table.enter_scope();

        for (_, package) in &provider.packages {
            self.visit_package(&mut package.clone());
        }
        
        let _ = self.table.exit_scope();
        None
    }

    fn visit_package(&mut self, package: &mut Package) -> Option<DataType> {

        let _ = self.table.enter_scope();

        for (_, event) in &package.events {
            self.visit_event(&mut event.clone());
        }

        let _ = self.table.exit_scope();

        None
    }

    fn visit_event(&mut self, event: &mut Event) -> Option<DataType> {

        let _ = self.table.enter_scope();

        for (_, probe) in &event.probe_map {
            for probe in probe {
                self.visit_probe(&mut probe.clone());
            }
        }

        let _ = self.table.exit_scope();

        None
    }

    fn visit_probe(&mut self, probe: &mut Probe) -> Option<DataType> {
        
        // let _ = self.table.enter_scope();
        // type check predicate
        if let Some(predicate) = &probe.predicate {
            self.visit_expr(&mut predicate.clone());
        }

        // type check body
        if let Some(body) = &probe.body {
            for stmt in body {
                self.visit_stmt(&mut stmt.clone());
            }
        }

        // let _ = self.table.exit_scope();

        None
    }

    fn visit_fn(&mut self, function: &mut Fn) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) -> Option<DataType> {
        match stmt {
            crate::parser::types::Statement::Assign {var_id, expr, ..} => {
                // change type in symbol table?
                self.visit_expr(expr);
                None
                
            },
            crate::parser::types::Statement::Expr {expr, ..} => {
                self.visit_expr(expr);
                None
            }
            Statement::Decl { ty, var_id, loc } => {
                None
            }
        }

    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Option<DataType> {
        match expr {
            Expr::Primitive {val, ..} => {
                self.visit_value(val)
            },
            Expr::BinOp {lhs, rhs, op, ..} => {
                let lhs_ty_op = self.visit_expr(lhs);
                let rhs_ty_op = self.visit_expr(rhs);
                if let (Some(lhs_ty), Some(rhs_ty)) = (lhs_ty_op, rhs_ty_op) {
                    match op {
                        BinOp::Add | BinOp::Subtract | BinOp::Multiply | BinOp::Divide | BinOp::Modulo => {
                            if lhs_ty == DataType::I32 && rhs_ty == DataType::I32 {
                                Some(DataType::I32)
                            } else {
                                eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
                                None
                            }
                        },
                        BinOp::And | BinOp::Or => {
                            if lhs_ty == DataType::Boolean && rhs_ty == DataType::Boolean {
                                Some(DataType::Boolean)
                            } else {
                                eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
                                None
                            }
                        },
    
                        (BinOp::EQ | BinOp::NE) => {
                            if lhs_ty == rhs_ty {
                                Some(DataType::Boolean)
                            } else {
                                eprintln!("Error: lhs_ty: {:?}, rhs_ty: {:?}", lhs_ty, rhs_ty);
                                None
                            }
                        },
                        (BinOp::GT | BinOp::LT | BinOp::GE | BinOp::LE) => {
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
            },
            Expr::VarId { name, loc } => {
                // get type from symbol table
                let _ = self.table.enter_scope(); // Alex: adding interscope here doens't seems to have an effect here (which is reasonable)

                if let Some(id) = self.table.lookup(name) {
                    println!("LOOK AT ME Var id: {:?}", self.table.get_record(id));
                    if let Some( rec ) = self.table.get_record(id) {
                        if let crate::verifier::types::Record::Var{ ty, .. } = rec {
                            return Some(ty.to_owned());
                        }
                    } else {
                        eprintln!("Error: Cant get record from symbol table");
                    }
                }
                
                
                None
            },
            _ => unimplemented!()
        }
    }

    fn visit_datatype(&mut self, datatype: &mut DataType) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_value(&mut self, val: &mut Value) -> Option<DataType> {
        match val {
            Value::Integer {..} => Some(DataType::I32),
            Value::Str {..} => Some(DataType::Str),
            Value::Boolean { .. } => Some(DataType::Boolean),
            // recurse on expressions?
            // Alex TODO: Not sure how to recurse
            Value::Tuple { ty, .. } => {
                match ty {
                    DataType::Tuple { ty_info } => {
                        match ty_info {
                            Some(ve) => {
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
                None
            }

            
        }
    }

    fn visit_formal_param(&mut self, param: &mut(Expr, DataType)) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_binop(&mut self, op: &mut BinOp) -> Option<DataType> {
        unimplemented!()
    }

    fn visit_unop(&mut self, op: &mut UnOp) -> Option<DataType> {
        unimplemented!()
    }

}

pub fn verify(_ast: &mut Whamm) -> bool {
    // TODO do typechecking!
    let mut type_checker = TypeChecker {
        table: SymbolTable::new(),
    };
    type_checker.visit_whamm(_ast);
    true
}

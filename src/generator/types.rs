use crate::parser::types::{DataType, Expr, Op, Value};
use crate::verifier::types::Record::Var;
use crate::verifier::types::SymbolTable;

pub struct ExprFolder;
impl ExprFolder {
    pub fn fold_expr(expr: &Expr, table: &SymbolTable) -> Expr {
        match *expr {
            Expr::BinOp { .. } => {
                ExprFolder::fold_binop(expr, table)
            }
            Expr::Call { .. } => {
                ExprFolder::fold_call(expr, table)
            }
            Expr::VarId { .. } => {
                ExprFolder::fold_var_id(expr, table)
            }
            Expr::Primitive { .. } => {
                ExprFolder::fold_primitive(expr, table)
            }
        }
    }
    fn fold_binop(binop: &Expr, table: &SymbolTable) -> Expr {
        match &binop {
            Expr::BinOp {lhs, op, rhs} => {
                let lhs = ExprFolder::fold_expr(&lhs, table);
                let rhs = ExprFolder::fold_expr(&rhs, table);
                match op {
                    Op::And => {
                        let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                        if let Some(lhs_bool) = lhs_val {
                            if let Some(rhs_bool) = rhs_val {
                                // both are boolean primitives
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_bool && rhs_bool,
                                    }
                                };
                            }
                            // only lhs is boolean primitive
                            // - if it's a true,  can drop it
                            // - if it's a false, this expression is false
                            return if lhs_bool {
                                rhs
                            } else {
                                Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: false,
                                    }
                                }
                            }
                        } else {
                            // lhs is not a primitive
                            return if let Some(rhs_bool) = rhs_val {
                                // only rhs is boolean primitive
                                // - if it's a true,  can drop it
                                // - if it's a false, this expression is false
                                if rhs_bool {
                                    lhs
                                } else {
                                    Expr::Primitive {
                                        val: Value::Boolean {
                                            ty: DataType::Boolean,
                                            val: false,
                                        }
                                    }
                                }
                            } else {
                                // rhs is not a primitive
                                // return folded lhs/rhs
                                Expr::BinOp {
                                    lhs: Box::new(lhs),
                                    op: Op::And,
                                    rhs: Box::new(rhs),
                                }
                            }
                        }
                    }
                    Op::Or => {
                        let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                        return if let Some(lhs_bool) = lhs_val {
                            if let Some(rhs_bool) = rhs_val {
                                // both are boolean primitives
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_bool || rhs_bool,
                                    }
                                };
                            }
                            // only lhs is boolean primitive
                            // - if it's a false, can drop it
                            // - if it's a true,  this expression is true
                            if lhs_bool {
                                Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: true,
                                    }
                                }
                            } else {
                                rhs
                            }
                        } else {
                            // lhs is not a primitive
                            if let Some(rhs_bool) = rhs_val {
                                // only rhs is boolean primitive
                                // - if it's a true,  this expression is true
                                // - if it's a false, can drop it
                                if rhs_bool {
                                    Expr::Primitive {
                                        val: Value::Boolean {
                                            ty: DataType::Boolean,
                                            val: true,
                                        }
                                    }
                                } else {
                                    lhs
                                }
                            } else {
                                // rhs is not a primitive
                                // return folded lhs/rhs
                                Expr::BinOp {
                                    lhs: Box::new(lhs),
                                    op: Op::Or,
                                    rhs: Box::new(rhs),
                                }
                            }
                        }
                    }
                    Op::EQ => {
                        let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                        if let Some(lhs_bool) = lhs_val {
                            if let Some(rhs_bool) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_bool == rhs_bool,
                                    }
                                };
                            }
                        }

                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_int == rhs_int,
                                    }
                                };
                            }
                        }
                        let (lhs_val, rhs_val) = ExprFolder::get_str(&lhs, &rhs);
                        if let Some(lhs_str) = lhs_val {
                            if let Some(rhs_str) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_str == rhs_str,
                                    }
                                };
                            }
                        }
                    }
                    Op::NE => {
                        let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                        if let Some(lhs_bool) = lhs_val {
                            if let Some(rhs_bool) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_bool != rhs_bool,
                                    }
                                };
                            }
                        }

                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_int != rhs_int,
                                    }
                                };
                            }
                        }
                        let (lhs_val, rhs_val) = ExprFolder::get_str(&lhs, &rhs);
                        if let Some(lhs_str) = lhs_val {
                            if let Some(rhs_str) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_str != rhs_str,
                                    }
                                };
                            }
                        }
                    }
                    Op::GE => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_int >= rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::GT => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_int > rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::LE => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_int <= rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::LT => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Boolean {
                                        ty: DataType::Boolean,
                                        val: lhs_int < rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::Add => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Integer {
                                        ty: DataType::Integer,
                                        val: lhs_int + rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::Subtract => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Integer {
                                        ty: DataType::Integer,
                                        val: lhs_int - rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::Multiply => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Integer {
                                        ty: DataType::Integer,
                                        val: lhs_int * rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::Divide => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Integer {
                                        ty: DataType::Integer,
                                        val: lhs_int / rhs_int,
                                    }
                                };
                            }
                        }
                    }
                    Op::Modulo => {
                        let (lhs_val, rhs_val) = ExprFolder::get_int(&lhs, &rhs);
                        if let Some(lhs_int) = lhs_val {
                            if let Some(rhs_int) = rhs_val {
                                return Expr::Primitive {
                                    val: Value::Integer {
                                        ty: DataType::Integer,
                                        val: lhs_int % rhs_int,
                                    }
                                };
                            }
                        }
                    }
                }
            },
            _ => {}
        }

        // Cannot fold any more
        binop.clone()
    }
    fn fold_call(call: &Expr, _table: &SymbolTable) -> Expr {
        call.clone()
    }
    fn fold_var_id(var_id: &Expr, table: &SymbolTable) -> Expr {
        match &var_id {
            Expr::VarId {name} => {
                let rec_id = match table.lookup(&name) {
                    Some(rec_id) => rec_id.clone(),
                    _ => {
                        return var_id.clone();
                    }
                };
                let rec = table.get_record(&rec_id);
                match &rec {
                    Some(Var{value, .. }) => {
                        if value.is_some() {
                            return Expr::Primitive {
                                val: value.as_ref().unwrap().clone(),
                            };
                        }
                    }
                    _ => {
                        // ignore
                    }
                }
            },
            _ => {
                // ignore
            }
        }
        var_id.clone()
    }
    fn fold_primitive(primitive: &Expr, _table: &SymbolTable) -> Expr {
        primitive.clone()
    }
    pub fn get_single_bool(expr: &Expr) -> Option<bool> {
        return match expr {
            Expr::Primitive { val: Value::Boolean {val, .. }, ..} => Some(val.clone()),
            _ => None
        };
    }
    pub fn get_bool(lhs: &Expr, rhs: &Expr) -> (Option<bool>, Option<bool>) {
        let lhs_val = ExprFolder::get_single_bool(lhs);
        let rhs_val = ExprFolder::get_single_bool(rhs);
        (lhs_val, rhs_val)
    }
    pub fn get_int(lhs: &Expr, rhs: &Expr) -> (Option<i32>, Option<i32>) {
        let lhs_val = match lhs {
            Expr::Primitive { val: Value::Integer {val: lhs_val, .. }, ..} => Some(lhs_val.clone()),
            _ => None
        };
        let rhs_val = match rhs {
            Expr::Primitive { val: Value::Integer {val: rhs_val, .. }, ..} => Some(rhs_val.clone()),
            _ => None
        };
        (lhs_val, rhs_val)
    }
    pub fn get_str(lhs: &Expr, rhs: &Expr) -> (Option<String>, Option<String>) {
        let lhs_val = match &lhs {
            Expr::Primitive { val: Value::Str {val: lhs_val, .. }, ..} => Some(lhs_val.clone()),
            _ => None
        };
        let rhs_val = match &rhs {
            Expr::Primitive { val: Value::Str {val: rhs_val, .. }, ..} => Some(rhs_val.clone()),
            _ => None
        };
        (lhs_val, rhs_val)
    }
}

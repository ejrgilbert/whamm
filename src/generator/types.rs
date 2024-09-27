use crate::common::error::ErrorGen;
use crate::parser::types::{BinOp, DataType, Expr, UnOp, Value};
use crate::verifier::types::Record::Var;
use crate::verifier::types::SymbolTable;

// =======================================
// = Constant Propagation via ExprFolder =
// =======================================

pub struct ExprFolder;
impl ExprFolder {
    pub fn fold_expr(expr: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        match *expr {
            Expr::UnOp { .. } => ExprFolder::fold_unop(expr, table, err),
            Expr::BinOp { .. } => ExprFolder::fold_binop(expr, table, err),
            Expr::Ternary { .. } => ExprFolder::fold_ternary(expr, table, err),
            Expr::Call { .. } => ExprFolder::fold_call(expr, table),
            Expr::VarId { .. } => ExprFolder::fold_var_id(expr, table, err),
            Expr::Primitive { .. } => ExprFolder::fold_primitive(expr, table),
            Expr::MapGet { .. } => ExprFolder::fold_map_get(expr, table, err),
        }
    }

    fn fold_binop(binop: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        if let Expr::BinOp { lhs, op, rhs, .. } = &binop {
            let lhs = ExprFolder::fold_expr(lhs, table, err);
            let rhs = ExprFolder::fold_expr(rhs, table, err);
            match op {
                BinOp::And => {
                    let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                    return if let Some(lhs_bool) = lhs_val {
                        if let Some(rhs_bool) = rhs_val {
                            // both are boolean primitives
                            return Expr::Primitive {
                                val: Value::Boolean {
                                    ty: DataType::Boolean,
                                    val: lhs_bool && rhs_bool,
                                },
                                loc: None,
                            };
                        }
                        // only lhs is boolean primitive
                        // - if it's a true,  can drop it
                        // - if it's a false, this expression is false
                        if lhs_bool {
                            rhs
                        } else {
                            Expr::Primitive {
                                val: Value::Boolean {
                                    ty: DataType::Boolean,
                                    val: false,
                                },
                                loc: None,
                            }
                        }
                    } else {
                        // lhs is not a primitive
                        if let Some(rhs_bool) = rhs_val {
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
                                    },
                                    loc: None,
                                }
                            }
                        } else {
                            // rhs is not a primitive
                            // return folded lhs/rhs
                            Expr::BinOp {
                                lhs: Box::new(lhs),
                                op: BinOp::And,
                                rhs: Box::new(rhs),
                                loc: None,
                            }
                        }
                    };
                }
                BinOp::Or => {
                    let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                    return if let Some(lhs_bool) = lhs_val {
                        if let Some(rhs_bool) = rhs_val {
                            // both are boolean primitives
                            return Expr::Primitive {
                                val: Value::Boolean {
                                    ty: DataType::Boolean,
                                    val: lhs_bool || rhs_bool,
                                },
                                loc: None,
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
                                },
                                loc: None,
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
                                    },
                                    loc: None,
                                }
                            } else {
                                lhs
                            }
                        } else {
                            // rhs is not a primitive
                            // return folded lhs/rhs
                            Expr::BinOp {
                                lhs: Box::new(lhs),
                                op: BinOp::Or,
                                rhs: Box::new(rhs),
                                loc: None,
                            }
                        }
                    };
                }
                BinOp::EQ => {
                    let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                    if let Some(res) = ExprFolder::fold_bools(&lhs_val, &rhs_val, op) {
                        return res;
                    }

                    let (lhs_val, rhs_val) = ExprFolder::get_i32(&lhs, &rhs);
                    if let Some(res) = ExprFolder::fold_ints(&lhs_val, &rhs_val, op) {
                        return res;
                    }
                    let (lhs_val, rhs_val) = ExprFolder::get_str(&lhs, &rhs);
                    if let Some(res) = ExprFolder::fold_strings(&lhs_val, &rhs_val, op) {
                        return res;
                    }
                }
                BinOp::NE => {
                    let (lhs_val, rhs_val) = ExprFolder::get_bool(&lhs, &rhs);
                    if let Some(res) = ExprFolder::fold_bools(&lhs_val, &rhs_val, op) {
                        return res;
                    }

                    let (lhs_val, rhs_val) = ExprFolder::get_i32(&lhs, &rhs);
                    if let Some(res) = ExprFolder::fold_ints(&lhs_val, &rhs_val, op) {
                        return res;
                    }

                    let (lhs_val, rhs_val) = ExprFolder::get_str(&lhs, &rhs);
                    if let Some(res) = ExprFolder::fold_strings(&lhs_val, &rhs_val, op) {
                        return res;
                    }
                }
                BinOp::GE
                | BinOp::GT
                | BinOp::LE
                | BinOp::LT
                | BinOp::Add
                | BinOp::Subtract
                | BinOp::Multiply
                | BinOp::Divide
                | BinOp::Modulo => {
                    let (lhs_val, rhs_val) = ExprFolder::get_i32(&lhs, &rhs);
                    if let Some(res) = ExprFolder::fold_ints(&lhs_val, &rhs_val, op) {
                        return res;
                    }
                }
            }
        }

        // Cannot fold anymore
        binop.clone()
    }

    fn fold_map_get(expr: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        if let Expr::MapGet { map, key, .. } = &expr {
            let map = ExprFolder::fold_expr(map, table, err);
            let key = ExprFolder::fold_expr(key, table, err);
            return Expr::MapGet {
                map: Box::new(map),
                key: Box::new(key),
                loc: None,
            };
        }
        expr.clone()
    }

    // similar to the logic of fold_binop
    fn fold_unop(unop: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        if let Expr::UnOp { op, expr, .. } = &unop {
            let expr = ExprFolder::fold_expr(expr, table, err);
            return match op {
                UnOp::Not => {
                    let expr_val = ExprFolder::get_single_bool(&expr);
                    if let Some(expr_bool) = expr_val {
                        Expr::Primitive {
                            val: Value::Boolean {
                                ty: DataType::Boolean,
                                val: !expr_bool,
                            },
                            loc: None,
                        }
                    } else {
                        Expr::UnOp {
                            op: UnOp::Not,
                            expr: Box::new(expr),
                            loc: None,
                        }
                    }
                }
            };
        }

        unop.to_owned()
    }

    fn fold_bools(lhs_val: &Option<bool>, rhs_val: &Option<bool>, op: &BinOp) -> Option<Expr> {
        if let Some(lhs_bool) = lhs_val {
            if let Some(rhs_bool) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_bool == rhs_bool,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_bool != rhs_bool,
                        },
                        loc: None,
                    }),
                    _ => None,
                };
            }
        }
        None
    }

    fn fold_ints(lhs_val: &Option<i32>, rhs_val: &Option<i32>, op: &BinOp) -> Option<Expr> {
        if let Some(lhs_int) = lhs_val {
            if let Some(rhs_int) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_int == rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_int != rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_int >= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_int > rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_int <= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_int < rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Add => Some(Expr::Primitive {
                        val: Value::I32 {
                            ty: DataType::I32,
                            val: lhs_int + rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Subtract => Some(Expr::Primitive {
                        val: Value::I32 {
                            ty: DataType::I32,
                            val: lhs_int - rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Multiply => Some(Expr::Primitive {
                        val: Value::I32 {
                            ty: DataType::I32,
                            val: lhs_int * rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Divide => Some(Expr::Primitive {
                        val: Value::I32 {
                            ty: DataType::I32,
                            val: lhs_int / rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Modulo => Some(Expr::Primitive {
                        val: Value::I32 {
                            ty: DataType::I32,
                            val: lhs_int % rhs_int,
                        },
                        loc: None,
                    }),
                    _ => None,
                };
            }
        }
        None
    }

    fn fold_strings(
        lhs_val: &Option<String>,
        rhs_val: &Option<String>,
        op: &BinOp,
    ) -> Option<Expr> {
        if let Some(lhs_str) = lhs_val {
            if let Some(rhs_str) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_str == rhs_str,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            ty: DataType::Boolean,
                            val: lhs_str != rhs_str,
                        },
                        loc: None,
                    }),
                    _ => None,
                };
            }
        }
        None
    }

    fn fold_ternary(ternary: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        match ternary {
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                let cond = ExprFolder::fold_expr(cond, table, err);
                let conseq = ExprFolder::fold_expr(conseq, table, err);
                let alt = ExprFolder::fold_expr(alt, table, err);

                // check if the condition folds to true/false!
                let cond_val = ExprFolder::get_single_bool(&cond);
                return if let Some(cond_bool) = cond_val {
                    // the condition folds to a primitive bool!
                    if cond_bool {
                        // it's a true, evaluates to the conseq
                        conseq
                    } else {
                        // it's a false, evaluates to the alt
                        alt
                    }
                } else {
                    // condition doesn't fold to a primitive, return folded variation.
                    Expr::Ternary {
                        cond: Box::new(cond),
                        conseq: Box::new(conseq),
                        alt: Box::new(alt),
                        loc: None,
                    }
                };
            }
            _ => {
                // ignore
            }
        }
        ternary.clone()
    }

    fn fold_call(call: &Expr, _table: &SymbolTable) -> Expr {
        call.clone()
    }
    fn fold_var_id(var_id: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        if let Expr::VarId { name, .. } = &var_id {
            let Some(Var { value, .. }) = table.lookup_var(name, &None, err, false) else {
                return var_id.clone(); // ignore
            };
            if value.is_some() {
                return Expr::Primitive {
                    val: value.as_ref().unwrap().clone(),
                    loc: None,
                };
            }
        }
        var_id.clone()
    }
    fn fold_primitive(primitive: &Expr, _table: &SymbolTable) -> Expr {
        primitive.clone()
    }
    pub fn get_single_bool(expr: &Expr) -> Option<bool> {
        match expr {
            Expr::Primitive {
                val: Value::Boolean { val, .. },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    pub fn get_bool(lhs: &Expr, rhs: &Expr) -> (Option<bool>, Option<bool>) {
        let lhs_val = ExprFolder::get_single_bool(lhs);
        let rhs_val = ExprFolder::get_single_bool(rhs);
        (lhs_val, rhs_val)
    }
    pub fn get_i32(lhs: &Expr, rhs: &Expr) -> (Option<i32>, Option<i32>) {
        let lhs_val = match lhs {
            Expr::Primitive {
                val: Value::I32 { val: lhs_val, .. },
                ..
            } => Some(*lhs_val),
            Expr::Primitive {
                val: Value::U32 { val: lhs_val, .. },
                ..
            } => {
                // TODO -- check overflow here!
                Some(*lhs_val as i32)
            }
            _ => None,
        };
        let rhs_val = match rhs {
            Expr::Primitive {
                val: Value::I32 { val: rhs_val, .. },
                ..
            } => Some(*rhs_val),
            Expr::Primitive {
                val: Value::U32 { val: rhs_val, .. },
                ..
            } => {
                // TODO -- check overflow here!
                Some(*rhs_val as i32)
            }
            _ => None,
        };
        (lhs_val, rhs_val)
    }
    pub fn get_str(lhs: &Expr, rhs: &Expr) -> (Option<String>, Option<String>) {
        let lhs_val = match &lhs {
            Expr::Primitive {
                val: Value::Str { val: lhs_val, .. },
                ..
            } => Some(lhs_val.clone()),
            _ => None,
        };
        let rhs_val = match &rhs {
            Expr::Primitive {
                val: Value::Str { val: rhs_val, .. },
                ..
            } => Some(rhs_val.clone()),
            _ => None,
        };
        (lhs_val, rhs_val)
    }
}

use crate::common::error::ErrorGen;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::parser::types::{
    BinOp, DataType, Definition, Expr, Location, NumLit, UnOp, Value, expr_to_val,
};
use crate::verifier::types::Record::Var;
use crate::verifier::types::{Record, SymbolTable};
use std::ops::{Add, Div, Mul, Rem, Sub};

// =======================================
// = Constant Propagation via ExprFolder =
// =======================================

pub struct ExprFolder<'a> {
    registry: &'a mut WasmRegistry,
    as_monitor_module: bool,
    curr_loc: Option<Location>,
}
impl<'a> ExprFolder<'a> {
    pub fn fold_expr(
        expr: &Expr,
        registry: &'a mut WasmRegistry,
        as_monitor_module: bool,
        table: &SymbolTable,
        err: &mut ErrorGen,
    ) -> Expr {
        let mut instance = Self {
            registry,
            as_monitor_module,
            curr_loc: None,
        };
        instance.fold_expr_inner(expr, table, err)
    }
    pub fn get_single_bool(
        expr: &Expr,
        registry: &'a mut WasmRegistry,
        as_monitor_module: bool,
    ) -> Option<bool> {
        let mut instance = Self {
            registry,
            as_monitor_module,
            curr_loc: None,
        };
        instance.get_single_bool_inner(expr)
    }
    fn fold_expr_inner(&mut self, expr: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        self.curr_loc = expr.loc().clone();

        match *expr {
            Expr::UnOp { .. } => self.fold_unop(expr, table, err),
            Expr::BinOp { .. } => self.fold_binop(expr, table, err),
            Expr::Ternary { .. } => self.fold_ternary(expr, table, err),
            Expr::Call { .. } => self.fold_call(expr, table),
            Expr::VarId { .. } => self.fold_var_id(expr, table),
            Expr::Primitive { .. } => self.fold_primitive(expr, table, err),
            Expr::MapGet { .. } => self.fold_map_get(expr, table, err),
            Expr::LibCall { .. } => self.fold_lib_call(expr, table, err),
        }
    }

    fn fold_lib_call(&mut self, lib_call: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        self.curr_loc = lib_call.loc().clone();

        if let Expr::LibCall {
            annotation,
            call,
            lib_name,
            results,
            loc,
        } = lib_call
        {
            if let Some(ann) = annotation {
                if ann.is_static() && !self.as_monitor_module {
                    // we're doing bytecode rewriting, so we should statically evaluate this lib call!
                    return self.fold_static_lib_call(lib_call, table, err);
                }
            }
            return Expr::LibCall {
                annotation: annotation.clone(),
                lib_name: lib_name.clone(),
                results: results.clone(),
                loc: loc.clone(),
                call: Box::new(self.fold_expr_inner(call, table, err)),
            };
        }
        lib_call.clone()
    }

    fn fold_static_lib_call(
        &mut self,
        lib_call: &Expr,
        table: &SymbolTable,
        err: &mut ErrorGen,
    ) -> Expr {
        if let Expr::LibCall {
            lib_name,
            call,
            results,
            loc: lib_call_loc,
            ..
        } = lib_call
        {
            if let Expr::Call {
                fn_target, args, ..
            } = call.as_ref()
            {
                if let Expr::VarId {
                    name: func_name, ..
                } = fn_target.as_ref()
                {
                    let mut arg_vals = vec![];
                    for arg in args.iter() {
                        // fold each of these expressions and add to the arg_vals vector
                        let new_arg = self.fold_expr_inner(arg, table, err);
                        if let Some(new_arg) = expr_to_val(&new_arg) {
                            arg_vals.push(new_arg);
                        } else {
                            err.add_internal_error(
                                &format!("couldn't convert to a Wasm value: {:?}", new_arg),
                                lib_call_loc,
                            );
                        }
                    }
                    // todo -- assumes results is set
                    let mut results = if let Some(res) = results.as_ref() {
                        res.to_default_values()
                    } else {
                        err.add_unimplemented_error(
                            "Results should be set at this point!",
                            lib_call_loc,
                        );
                        return lib_call.clone();
                    };
                    if let Some(svc) = self.registry.get_mut(lib_name) {
                        svc.call(lib_name, func_name, &arg_vals, &mut results, err);
                    } else {
                        err.add_internal_error(
                            &format!("could not find the wasm service for lib: {lib_name}"),
                            lib_call_loc,
                        );
                    }

                    if results.len() > 1 {
                        todo!("we don't support multiple return values yet!")
                    }

                    if let Some(res) = results.first() {
                        Expr::Primitive {
                            val: Value::from(res),
                            loc: lib_call_loc.clone(),
                        }
                    } else {
                        Expr::empty_tuple(lib_call_loc)
                    }
                } else {
                    err.add_internal_error(
                        &format!("Expected a name expression, got: {:?}", fn_target),
                        lib_call_loc,
                    );
                    lib_call.clone()
                }
            } else {
                err.add_internal_error(
                    &format!("Expected call expression, got: {:?}", call),
                    lib_call_loc,
                );
                lib_call.clone()
            }
        } else {
            err.add_internal_error(
                &format!("Expected library call expression, got: {:?}", lib_call),
                lib_call.loc(),
            );
            lib_call.clone()
        }
    }

    fn fold_binop(&mut self, binop: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        self.curr_loc = binop.loc().clone();
        if let Expr::BinOp {
            lhs,
            op,
            rhs,
            done_on,
            loc,
        } = &binop
        {
            let lhs = self.fold_expr_inner(lhs, table, err);
            let rhs = self.fold_expr_inner(rhs, table, err);
            match op {
                BinOp::And => {
                    let (lhs_val, rhs_val) = self.get_bool(&lhs, &rhs);
                    return if let Some(lhs_bool) = lhs_val {
                        if let Some(rhs_bool) = rhs_val {
                            // both are boolean primitives
                            return Expr::Primitive {
                                val: Value::Boolean {
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
                                val: Value::Boolean { val: false },
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
                                    val: Value::Boolean { val: false },
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
                                done_on: done_on.clone(),
                                loc: None,
                            }
                        }
                    };
                }
                BinOp::Or => {
                    let (lhs_val, rhs_val) = self.get_bool(&lhs, &rhs);
                    return if let Some(lhs_bool) = lhs_val {
                        if let Some(rhs_bool) = rhs_val {
                            // both are boolean primitives
                            return Expr::Primitive {
                                val: Value::Boolean {
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
                                val: Value::Boolean { val: true },
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
                                    val: Value::Boolean { val: true },
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
                                done_on: DataType::I32,
                                loc: None,
                            }
                        }
                    };
                }
                BinOp::EQ => {
                    let (lhs_val, rhs_val) = self.get_bool(&lhs, &rhs);
                    if let Some(res) = self.fold_bools(&lhs_val, &rhs_val, op) {
                        return res;
                    }
                    if let Some(res) = self.fold_numeric_binop(&lhs, &rhs, op, done_on, err) {
                        return res;
                    }
                    let (lhs_val, rhs_val) = self.get_str(&lhs, &rhs);
                    if let Some(res) = self.fold_strings(&lhs_val, &rhs_val, op) {
                        return res;
                    }

                    if self.is_str(&lhs, table) && self.is_str(&rhs, table) {
                        // Otherwise, replace with a call to strcmp!
                        return Expr::Call {
                            fn_target: Box::new(Expr::VarId {
                                definition: Definition::CompilerDynamic,
                                name: "strcmp".to_string(),
                                loc: None,
                            }),
                            args: vec![lhs, rhs],
                            loc: loc.clone(),
                        };
                    }
                }
                BinOp::NE => {
                    let (lhs_val, rhs_val) = self.get_bool(&lhs, &rhs);
                    if let Some(res) = self.fold_bools(&lhs_val, &rhs_val, op) {
                        return res;
                    }
                    if let Some(res) = self.fold_numeric_binop(&lhs, &rhs, op, done_on, err) {
                        return res;
                    }
                    let (lhs_val, rhs_val) = self.get_str(&lhs, &rhs);
                    if let Some(res) = self.fold_strings(&lhs_val, &rhs_val, op) {
                        return res;
                    }

                    if self.is_str(&lhs, table) && self.is_str(&rhs, table) {
                        // Otherwise, replace with a call to strcmp!
                        return Expr::UnOp {
                            op: UnOp::Not,
                            expr: Box::new(Expr::Call {
                                fn_target: Box::new(Expr::VarId {
                                    definition: Definition::CompilerDynamic,
                                    name: "strcmp".to_string(),
                                    loc: None,
                                }),
                                args: vec![lhs, rhs],
                                loc: None,
                            }),
                            done_on: DataType::I32,
                            loc: loc.clone(),
                        };
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
                | BinOp::Modulo
                | BinOp::LShift
                | BinOp::RShift
                | BinOp::BitAnd
                | BinOp::BitOr
                | BinOp::BitXor => {
                    if let Some(res) = self.fold_numeric_binop(&lhs, &rhs, op, done_on, err) {
                        return res;
                    }
                }
            }
        }

        // Cannot fold anymore
        binop.clone()
    }

    fn is_str(&mut self, expr: &Expr, table: &SymbolTable) -> bool {
        self.curr_loc = expr.loc().clone();
        match expr {
            Expr::VarId { name, .. } => {
                if let Some(Var { ty, .. }) = table.lookup_var(name, false) {
                    matches!(ty, DataType::Str)
                } else {
                    false
                }
            }
            Expr::Primitive {
                val: Value::Str { .. },
                ..
            } => true,
            Expr::Call { fn_target, .. } => {
                if let Expr::VarId { name, .. } = fn_target.as_ref() {
                    if let Some(Record::Fn { ret_ty, .. }) = table.lookup_fn(name, false) {
                        matches!(ret_ty, DataType::Str)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn fold_map_get(&mut self, expr: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        self.curr_loc = expr.loc().clone();
        if let Expr::MapGet { map, key, .. } = &expr {
            let map = self.fold_expr_inner(map, table, err);
            let key = self.fold_expr_inner(key, table, err);
            return Expr::MapGet {
                map: Box::new(map),
                key: Box::new(key),
                loc: None,
            };
        }
        expr.clone()
    }

    // similar to the logic of fold_binop
    fn fold_unop(&mut self, unop: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        self.curr_loc = unop.loc().clone();
        if let Expr::UnOp {
            op, expr, done_on, ..
        } = &unop
        {
            let expr = self.fold_expr_inner(expr, table, err);
            return match op {
                UnOp::Cast { target } => match &expr {
                    Expr::Primitive { val, .. } => {
                        let mut casted = val.clone();
                        match casted.do_explicit_cast(target) {
                            Ok(()) => Expr::Primitive {
                                val: casted,
                                loc: None,
                            },
                            Err(_) => Expr::UnOp {
                                op: UnOp::Cast {
                                    target: target.clone(),
                                },
                                expr: Box::new(expr),
                                done_on: done_on.clone(),
                                loc: None,
                            },
                        }
                    }
                    Expr::UnOp { .. }
                    | Expr::Ternary { .. }
                    | Expr::BinOp { .. }
                    | Expr::Call { .. }
                    | Expr::LibCall { .. }
                    | Expr::VarId { .. }
                    | Expr::MapGet { .. } => Expr::UnOp {
                        op: UnOp::Cast {
                            target: target.clone(),
                        },
                        expr: Box::new(expr),
                        done_on: done_on.clone(),
                        loc: None,
                    },
                },
                UnOp::Not => {
                    let expr_val = self.get_single_bool_inner(&expr);
                    if let Some(expr_bool) = expr_val {
                        Expr::Primitive {
                            val: Value::Boolean { val: !expr_bool },
                            loc: None,
                        }
                    } else {
                        return Expr::UnOp {
                            op: UnOp::Not,
                            expr: Box::new(expr),
                            done_on: done_on.clone(),
                            loc: None,
                        };
                    }
                }
                UnOp::BitwiseNot => {
                    if let Some(res) = self.fold_numeric_unop(&expr, op, done_on, err) {
                        res
                    } else {
                        Expr::UnOp {
                            op: UnOp::BitwiseNot,
                            expr: Box::new(expr),
                            done_on: done_on.clone(),
                            loc: None,
                        }
                    }
                }
            };
        }

        unop.to_owned()
    }

    fn fold_bools(
        &mut self,
        lhs_val: &Option<bool>,
        rhs_val: &Option<bool>,
        op: &BinOp,
    ) -> Option<Expr> {
        if let Some(lhs_bool) = lhs_val {
            if let Some(rhs_bool) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_bool == rhs_bool,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
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

    fn fold_numeric_binop(
        &mut self,
        lhs: &Expr,
        rhs: &Expr,
        op: &BinOp,
        done_on: &DataType,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        let (lhs_val, rhs_val) = self.get_i32s(lhs, rhs);
        if let Some(res) = self.fold_i32s_binop(&lhs_val, &rhs_val, op, err) {
            return Some(res);
        }
        let (lhs_val, rhs_val) = self.get_u32s(lhs, rhs);
        if let Some(res) = self.fold_u32s_binop(&lhs_val, &rhs_val, op, done_on, err) {
            return Some(res);
        }
        let (lhs_val, rhs_val) = self.get_i64s(lhs, rhs);
        if let Some(res) = self.fold_i64s_binop(&lhs_val, &rhs_val, op, err) {
            return Some(res);
        }
        let (lhs_val, rhs_val) = self.get_u64s(lhs, rhs);
        if let Some(res) = self.fold_u64s_binop(&lhs_val, &rhs_val, op, err) {
            return Some(res);
        }
        let (lhs_val, rhs_val) = self.get_f32s(lhs, rhs);
        if let Some(res) = self.fold_f32s_binop(&lhs_val, &rhs_val, op, err) {
            return Some(res);
        }
        let (lhs_val, rhs_val) = self.get_f64s(lhs, rhs);
        if let Some(res) = self.fold_f64s_binop(&lhs_val, &rhs_val, op, err) {
            return Some(res);
        }
        None
    }

    fn fold_i32s_binop(
        &mut self,
        lhs_val: &Option<i32>,
        rhs_val: &Option<i32>,
        op: &BinOp,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        if let Some(lhs_int) = lhs_val {
            if let Some(rhs_int) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int == rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int != rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int >= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int > rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int <= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int < rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Add => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int.wrapping_add(*rhs_int) as u32),
                        loc: None,
                    }),
                    BinOp::Subtract => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int.wrapping_sub(*rhs_int) as u32),
                        loc: None,
                    }),
                    BinOp::Multiply => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int.wrapping_mul(*rhs_int) as u32),
                        loc: None,
                    }),
                    BinOp::Divide => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_u32(lhs_int.wrapping_div(*rhs_int) as u32),
                            loc: None,
                        })
                    }
                    BinOp::Modulo => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_u32((lhs_int % rhs_int) as u32),
                            loc: None,
                        })
                    }
                    BinOp::LShift => Some(Expr::Primitive {
                        val: Value::gen_u32((lhs_int << rhs_int) as u32),
                        loc: None,
                    }),
                    BinOp::RShift => Some(Expr::Primitive {
                        val: Value::gen_u32((lhs_int >> rhs_int) as u32),
                        loc: None,
                    }),
                    BinOp::BitAnd => Some(Expr::Primitive {
                        val: Value::gen_u32((lhs_int & rhs_int) as u32),
                        loc: None,
                    }),
                    BinOp::BitOr => Some(Expr::Primitive {
                        val: Value::gen_u32((lhs_int | rhs_int) as u32),
                        loc: None,
                    }),
                    BinOp::BitXor => Some(Expr::Primitive {
                        val: Value::gen_u32((lhs_int ^ rhs_int) as u32),
                        loc: None,
                    }),
                    _ => None,
                };
            }
        }
        None
    }
    fn fold_u32s_binop(
        &mut self,
        lhs_val: &Option<u32>,
        rhs_val: &Option<u32>,
        op: &BinOp,
        done_on: &DataType,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        if let Some(lhs_int) = lhs_val {
            if let Some(rhs_int) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int == rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int != rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int >= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int > rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int <= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int < rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Add => {
                        // handle what's represented as u32s in the compiler
                        match done_on {
                            DataType::U8 => Some(Expr::Primitive {
                                val: Value::gen_u8((*lhs_int as u8).wrapping_add(*rhs_int as u8)),
                                loc: None,
                            }),
                            DataType::I8 => Some(Expr::Primitive {
                                val: Value::gen_i8((*lhs_int as i8).wrapping_add(*rhs_int as i8)),
                                loc: None,
                            }),
                            DataType::U16 => Some(Expr::Primitive {
                                val: Value::gen_u16(
                                    (*lhs_int as u16).wrapping_add(*rhs_int as u16),
                                ),
                                loc: None,
                            }),
                            DataType::I16 => Some(Expr::Primitive {
                                val: Value::gen_i16(
                                    (*lhs_int as i16).wrapping_add(*rhs_int as i16),
                                ),
                                loc: None,
                            }),
                            DataType::U32 => Some(Expr::Primitive {
                                val: Value::gen_u32(lhs_int.wrapping_add(*rhs_int)),
                                loc: None,
                            }),
                            _ => unreachable!(),
                        }
                    }
                    BinOp::Subtract => {
                        // handle what's represented as u32s in the compiler
                        match done_on {
                            DataType::U8 => Some(Expr::Primitive {
                                val: Value::gen_u8((*lhs_int as u8).wrapping_sub(*rhs_int as u8)),
                                loc: None,
                            }),
                            DataType::I8 => Some(Expr::Primitive {
                                val: Value::gen_i8((*lhs_int as i8).wrapping_sub(*rhs_int as i8)),
                                loc: None,
                            }),
                            DataType::U16 => Some(Expr::Primitive {
                                val: Value::gen_u16(
                                    (*lhs_int as u16).wrapping_sub(*rhs_int as u16),
                                ),
                                loc: None,
                            }),
                            DataType::I16 => Some(Expr::Primitive {
                                val: Value::gen_i16(
                                    (*lhs_int as i16).wrapping_sub(*rhs_int as i16),
                                ),
                                loc: None,
                            }),
                            DataType::U32 => Some(Expr::Primitive {
                                val: Value::gen_u32(lhs_int.wrapping_sub(*rhs_int)),
                                loc: None,
                            }),
                            _ => unreachable!(),
                        }
                    }
                    BinOp::Multiply => {
                        // handle what's represented as u32s in the compiler
                        match done_on {
                            DataType::U8 => Some(Expr::Primitive {
                                val: Value::gen_u8((*lhs_int as u8).wrapping_mul(*rhs_int as u8)),
                                loc: None,
                            }),
                            DataType::I8 => Some(Expr::Primitive {
                                val: Value::gen_i8((*lhs_int as i8).wrapping_mul(*rhs_int as i8)),
                                loc: None,
                            }),
                            DataType::U16 => Some(Expr::Primitive {
                                val: Value::gen_u16(
                                    (*lhs_int as u16).wrapping_mul(*rhs_int as u16),
                                ),
                                loc: None,
                            }),
                            DataType::I16 => Some(Expr::Primitive {
                                val: Value::gen_i16(
                                    (*lhs_int as i16).wrapping_mul(*rhs_int as i16),
                                ),
                                loc: None,
                            }),
                            DataType::U32 => Some(Expr::Primitive {
                                val: Value::gen_u32(lhs_int.wrapping_mul(*rhs_int)),
                                loc: None,
                            }),
                            _ => unreachable!(),
                        }
                    }
                    BinOp::Divide => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        // handle what's represented as u32s in the compiler
                        match done_on {
                            DataType::U8 => Some(Expr::Primitive {
                                val: Value::gen_u8((*lhs_int as u8).wrapping_div(*rhs_int as u8)),
                                loc: None,
                            }),
                            DataType::I8 => Some(Expr::Primitive {
                                val: Value::gen_i8((*lhs_int as i8).wrapping_div(*rhs_int as i8)),
                                loc: None,
                            }),
                            DataType::U16 => Some(Expr::Primitive {
                                val: Value::gen_u16(
                                    (*lhs_int as u16).wrapping_div(*rhs_int as u16),
                                ),
                                loc: None,
                            }),
                            DataType::I16 => Some(Expr::Primitive {
                                val: Value::gen_i16(
                                    (*lhs_int as i16).wrapping_div(*rhs_int as i16),
                                ),
                                loc: None,
                            }),
                            DataType::U32 => Some(Expr::Primitive {
                                val: Value::gen_u32(lhs_int.wrapping_div(*rhs_int)),
                                loc: None,
                            }),
                            _ => unreachable!(),
                        }
                    }
                    BinOp::Modulo => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        // handle what's represented as u32s in the compiler
                        match done_on {
                            DataType::U8 => Some(Expr::Primitive {
                                val: Value::gen_u8(*lhs_int as u8 % *rhs_int as u8),
                                loc: None,
                            }),
                            DataType::I8 => Some(Expr::Primitive {
                                val: Value::gen_i8(*lhs_int as i8 % *rhs_int as i8),
                                loc: None,
                            }),
                            DataType::U16 => Some(Expr::Primitive {
                                val: Value::gen_u16(*lhs_int as u16 % *rhs_int as u16),
                                loc: None,
                            }),
                            DataType::I16 => Some(Expr::Primitive {
                                val: Value::gen_i16(*lhs_int as i16 % *rhs_int as i16),
                                loc: None,
                            }),
                            DataType::U32 => Some(Expr::Primitive {
                                val: Value::gen_u32(lhs_int % rhs_int),
                                loc: None,
                            }),
                            _ => unreachable!(),
                        }
                    }
                    BinOp::LShift => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int << rhs_int),
                        loc: None,
                    }),
                    BinOp::RShift => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int >> rhs_int),
                        loc: None,
                    }),
                    BinOp::BitAnd => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int & rhs_int),
                        loc: None,
                    }),
                    BinOp::BitOr => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int | rhs_int),
                        loc: None,
                    }),
                    BinOp::BitXor => Some(Expr::Primitive {
                        val: Value::gen_u32(lhs_int ^ rhs_int),
                        loc: None,
                    }),
                    _ => None,
                };
            }
        }
        None
    }

    fn fold_i64s_binop(
        &mut self,
        lhs_val: &Option<i64>,
        rhs_val: &Option<i64>,
        op: &BinOp,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        if let Some(lhs_int) = lhs_val {
            if let Some(rhs_int) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int == rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int != rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int >= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int > rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int <= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int < rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Add => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int.wrapping_add(*rhs_int) as u64),
                        loc: None,
                    }),
                    BinOp::Subtract => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int.wrapping_sub(*rhs_int) as u64),
                        loc: None,
                    }),
                    BinOp::Multiply => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int.wrapping_mul(*rhs_int) as u64),
                        loc: None,
                    }),
                    BinOp::Divide => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_u64(lhs_int.wrapping_div(*rhs_int) as u64),
                            loc: None,
                        })
                    }
                    BinOp::Modulo => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_u64((lhs_int % rhs_int) as u64),
                            loc: None,
                        })
                    }
                    BinOp::LShift => Some(Expr::Primitive {
                        val: Value::gen_u64((lhs_int << rhs_int) as u64),
                        loc: None,
                    }),
                    BinOp::RShift => Some(Expr::Primitive {
                        val: Value::gen_u64((lhs_int >> rhs_int) as u64),
                        loc: None,
                    }),
                    BinOp::BitAnd => Some(Expr::Primitive {
                        val: Value::gen_u64((lhs_int & rhs_int) as u64),
                        loc: None,
                    }),
                    BinOp::BitOr => Some(Expr::Primitive {
                        val: Value::gen_u64((lhs_int | rhs_int) as u64),
                        loc: None,
                    }),
                    BinOp::BitXor => Some(Expr::Primitive {
                        val: Value::gen_u64((lhs_int ^ rhs_int) as u64),
                        loc: None,
                    }),
                    _ => None,
                };
            }
        }
        None
    }
    fn fold_u64s_binop(
        &mut self,
        lhs_val: &Option<u64>,
        rhs_val: &Option<u64>,
        op: &BinOp,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        if let Some(lhs_int) = lhs_val {
            if let Some(rhs_int) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int == rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int != rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int >= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int > rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int <= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int < rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Add => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int.wrapping_add(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Subtract => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int.wrapping_sub(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Multiply => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int.wrapping_mul(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Divide => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_u64(lhs_int.wrapping_div(*rhs_int)),
                            loc: None,
                        })
                    }
                    BinOp::Modulo => {
                        if *rhs_int == 0 {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_u64(lhs_int % rhs_int),
                            loc: None,
                        })
                    }
                    BinOp::LShift => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int << rhs_int),
                        loc: None,
                    }),
                    BinOp::RShift => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int >> rhs_int),
                        loc: None,
                    }),
                    BinOp::BitAnd => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int & rhs_int),
                        loc: None,
                    }),
                    BinOp::BitOr => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int | rhs_int),
                        loc: None,
                    }),
                    BinOp::BitXor => Some(Expr::Primitive {
                        val: Value::gen_u64(lhs_int ^ rhs_int),
                        loc: None,
                    }),
                    _ => None,
                };
            }
        }
        None
    }

    fn fold_f32s_binop(
        &mut self,
        lhs_val: &Option<f32>,
        rhs_val: &Option<f32>,
        op: &BinOp,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        if let Some(lhs_int) = lhs_val {
            if let Some(rhs_int) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int == rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int != rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int >= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int > rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int <= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int < rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Add => Some(Expr::Primitive {
                        val: Value::gen_f32(lhs_int.add(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Subtract => Some(Expr::Primitive {
                        val: Value::gen_f32(lhs_int.sub(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Multiply => Some(Expr::Primitive {
                        val: Value::gen_f32(lhs_int.mul(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Divide => {
                        if rhs_int.eq(&0f32) {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_f32(lhs_int.div(*rhs_int)),
                            loc: None,
                        })
                    }
                    BinOp::Modulo => {
                        if rhs_int.eq(&0f32) {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_f32(lhs_int.rem(rhs_int)),
                            loc: None,
                        })
                    }
                    BinOp::LShift
                    | BinOp::RShift
                    | BinOp::BitAnd
                    | BinOp::BitOr
                    | BinOp::BitXor => unreachable!(),
                    _ => None,
                };
            }
        }
        None
    }

    fn fold_f64s_binop(
        &mut self,
        lhs_val: &Option<f64>,
        rhs_val: &Option<f64>,
        op: &BinOp,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        if let Some(lhs_int) = lhs_val {
            if let Some(rhs_int) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int == rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int != rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int >= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::GT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int > rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LE => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int <= rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::LT => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_int < rhs_int,
                        },
                        loc: None,
                    }),
                    BinOp::Add => Some(Expr::Primitive {
                        val: Value::gen_f64(lhs_int.add(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Subtract => Some(Expr::Primitive {
                        val: Value::gen_f64(lhs_int.sub(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Multiply => Some(Expr::Primitive {
                        val: Value::gen_f64(lhs_int.mul(*rhs_int)),
                        loc: None,
                    }),
                    BinOp::Divide => {
                        if rhs_int.eq(&0f64) {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_f64(lhs_int.div(*rhs_int)),
                            loc: None,
                        })
                    }
                    BinOp::Modulo => {
                        if rhs_int.eq(&0f64) {
                            err.div_by_zero(self.curr_loc.clone())
                        }
                        Some(Expr::Primitive {
                            val: Value::gen_f64(lhs_int.rem(rhs_int)),
                            loc: None,
                        })
                    }
                    BinOp::LShift
                    | BinOp::RShift
                    | BinOp::BitAnd
                    | BinOp::BitOr
                    | BinOp::BitXor => unreachable!(),
                    _ => None,
                };
            }
        }
        None
    }

    fn fold_numeric_unop(
        &mut self,
        expr: &Expr,
        op: &UnOp,
        _done_on: &DataType,
        err: &mut ErrorGen,
    ) -> Option<Expr> {
        let val = self.get_i32(expr);
        if let Some(res) = self.fold_i32_unop(&val, op, err) {
            return Some(res);
        }
        let val = self.get_u32(expr);
        if let Some(res) = self.fold_u32_unop(&val, op, err) {
            return Some(res);
        }
        let val = self.get_i64(expr);
        if let Some(res) = self.fold_i64_unop(&val, op, err) {
            return Some(res);
        }
        let val = self.get_u64(expr);
        if let Some(res) = self.fold_u64_unop(&val, op, err) {
            return Some(res);
        }
        let val = self.get_f32(expr);
        if let Some(res) = self.fold_f32_unop(&val, op, err) {
            return Some(res);
        }
        let val = self.get_f64(expr);
        if let Some(res) = self.fold_f64_unop(&val, op, err) {
            return Some(res);
        }
        None
    }

    fn fold_i32_unop(&mut self, val: &Option<i32>, op: &UnOp, _err: &mut ErrorGen) -> Option<Expr> {
        if let Some(val) = val {
            return match op {
                UnOp::BitwiseNot => Some(Expr::Primitive {
                    val: Value::gen_i32(!val),
                    loc: None,
                }),
                _ => None,
            };
        }
        None
    }

    fn fold_u32_unop(&mut self, val: &Option<u32>, op: &UnOp, _err: &mut ErrorGen) -> Option<Expr> {
        if let Some(val) = val {
            return match op {
                UnOp::BitwiseNot => Some(Expr::Primitive {
                    val: Value::gen_u32(!val),
                    loc: None,
                }),
                _ => None,
            };
        }
        None
    }

    fn fold_i64_unop(&mut self, val: &Option<i64>, op: &UnOp, _err: &mut ErrorGen) -> Option<Expr> {
        if let Some(val) = val {
            return match op {
                UnOp::BitwiseNot => Some(Expr::Primitive {
                    val: Value::gen_i64(!val),
                    loc: None,
                }),
                _ => None,
            };
        }
        None
    }

    fn fold_u64_unop(&mut self, val: &Option<u64>, op: &UnOp, _err: &mut ErrorGen) -> Option<Expr> {
        if let Some(val) = val {
            return match op {
                UnOp::BitwiseNot => Some(Expr::Primitive {
                    val: Value::gen_u64(!val),
                    loc: None,
                }),
                _ => None,
            };
        }
        None
    }

    fn fold_f32_unop(&mut self, val: &Option<f32>, op: &UnOp, _err: &mut ErrorGen) -> Option<Expr> {
        if let Some(_val) = val {
            return match op {
                UnOp::BitwiseNot => unreachable!(),
                _ => None,
            };
        }
        None
    }

    fn fold_f64_unop(&mut self, val: &Option<f64>, op: &UnOp, _err: &mut ErrorGen) -> Option<Expr> {
        if let Some(_val) = val {
            return match op {
                UnOp::BitwiseNot => unreachable!(),
                _ => None,
            };
        }
        None
    }

    fn fold_strings(
        &mut self,
        lhs_val: &Option<String>,
        rhs_val: &Option<String>,
        op: &BinOp,
    ) -> Option<Expr> {
        if let Some(lhs_str) = lhs_val {
            if let Some(rhs_str) = rhs_val {
                return match op {
                    BinOp::EQ => Some(Expr::Primitive {
                        val: Value::Boolean {
                            val: lhs_str == rhs_str,
                        },
                        loc: None,
                    }),
                    BinOp::NE => Some(Expr::Primitive {
                        val: Value::Boolean {
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

    fn fold_ternary(&mut self, ternary: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        self.curr_loc = ternary.loc().clone();
        match ternary {
            Expr::Ternary {
                cond,
                conseq,
                alt,
                ty,
                ..
            } => {
                let cond = self.fold_expr_inner(cond, table, err);
                let conseq = self.fold_expr_inner(conseq, table, err);
                let alt = self.fold_expr_inner(alt, table, err);

                // check if the condition folds to true/false!
                let cond_val = self.get_single_bool_inner(&cond);
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
                        ty: ty.clone(),
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

    fn fold_call(&mut self, call: &Expr, _table: &SymbolTable) -> Expr {
        self.curr_loc = call.loc().clone();
        call.clone()
    }
    fn fold_var_id(&mut self, var_id: &Expr, table: &SymbolTable) -> Expr {
        self.curr_loc = var_id.loc().clone();
        if let Expr::VarId { name, .. } = &var_id {
            let Some(Var { value, .. }) = table.lookup_var(name, false) else {
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
    fn fold_primitive(
        &mut self,
        primitive: &Expr,
        table: &SymbolTable,
        err: &mut ErrorGen,
    ) -> Expr {
        self.curr_loc = primitive.loc().clone();
        match primitive {
            Expr::Primitive {
                val: Value::Tuple { vals, ty },
                loc,
            } => {
                let mut folded_vals = vec![];

                for val in vals.iter() {
                    folded_vals.push(self.fold_expr_inner(val, table, err))
                }

                Expr::Primitive {
                    val: Value::Tuple {
                        vals: folded_vals,
                        ty: ty.clone(),
                    },
                    loc: loc.clone(),
                }
            }
            _ => primitive.clone(),
        }
    }
    fn get_single_bool_inner(&mut self, expr: &Expr) -> Option<bool> {
        self.curr_loc = expr.loc().clone();
        match expr {
            Expr::Primitive {
                val: Value::Boolean { val, .. },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    fn get_bool(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<bool>, Option<bool>) {
        self.curr_loc = lhs.loc().clone();
        let lhs_val = self.get_single_bool_inner(lhs);
        self.curr_loc = rhs.loc().clone();
        let rhs_val = self.get_single_bool_inner(rhs);
        (lhs_val, rhs_val)
    }
    fn get_i32s(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<i32>, Option<i32>) {
        (self.get_i32(lhs), self.get_i32(rhs))
    }
    fn get_i32(&mut self, expr: &Expr) -> Option<i32> {
        match expr {
            Expr::Primitive {
                val:
                    Value::Number {
                        val: NumLit::I32 { val },
                        ..
                    },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    fn get_u32s(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<u32>, Option<u32>) {
        (self.get_u32(lhs), self.get_u32(rhs))
    }
    fn get_u32(&mut self, expr: &Expr) -> Option<u32> {
        match expr {
            Expr::Primitive {
                val:
                    Value::Number {
                        val: NumLit::U32 { val },
                        ..
                    },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    fn get_i64s(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<i64>, Option<i64>) {
        (self.get_i64(lhs), self.get_i64(rhs))
    }
    fn get_i64(&mut self, expr: &Expr) -> Option<i64> {
        match expr {
            Expr::Primitive {
                val:
                    Value::Number {
                        val: NumLit::I64 { val },
                        ..
                    },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    fn get_u64s(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<u64>, Option<u64>) {
        (self.get_u64(lhs), self.get_u64(rhs))
    }
    fn get_u64(&mut self, expr: &Expr) -> Option<u64> {
        match expr {
            Expr::Primitive {
                val:
                    Value::Number {
                        val: NumLit::U64 { val },
                        ..
                    },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    fn get_f32s(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<f32>, Option<f32>) {
        (self.get_f32(lhs), self.get_f32(rhs))
    }
    fn get_f32(&mut self, expr: &Expr) -> Option<f32> {
        match expr {
            Expr::Primitive {
                val:
                    Value::Number {
                        val: NumLit::F32 { val },
                        ..
                    },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    fn get_f64s(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<f64>, Option<f64>) {
        (self.get_f64(lhs), self.get_f64(rhs))
    }
    fn get_f64(&mut self, expr: &Expr) -> Option<f64> {
        match expr {
            Expr::Primitive {
                val:
                    Value::Number {
                        val: NumLit::F64 { val },
                        ..
                    },
                ..
            } => Some(*val),
            _ => None,
        }
    }
    fn get_str(&mut self, lhs: &Expr, rhs: &Expr) -> (Option<String>, Option<String>) {
        self.curr_loc = lhs.loc().clone();
        let lhs_val = match &lhs {
            Expr::Primitive {
                val: Value::Str { val: lhs_val, .. },
                ..
            } => Some(lhs_val.clone()),
            _ => None,
        };
        self.curr_loc = rhs.loc().clone();
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

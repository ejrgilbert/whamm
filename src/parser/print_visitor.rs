use crate::parser::types as parser_types;
use parser_types::{WhammVisitor};

use std::cmp;
use std::collections::HashMap;
use crate::parser::types::{DataType, Whammy, Whamm, Expr, Event, Package, Op, Probe, Provider, Statement, Value, Global};

const NL: &str = "\n";

pub struct AsStrVisitor {
    pub indent: i32
}
impl AsStrVisitor {
    fn increase_indent(&mut self) {
        self.indent += 1;
    }

    fn decrease_indent(&mut self) {
        self.indent -= 1;
    }

    fn get_indent(&self) -> String {
        "--".repeat(cmp::max(0, self.indent as usize))
    }

    fn visit_globals(&mut self, globals: &HashMap<String, Global>) -> String {
        let mut s = "".to_string();
        for (name, global) in globals.iter() {
            s += &format!("{}{} := ", self.get_indent(), name);
            match &global.value {
                Some(v) => s += &format!("{}{}", self.visit_value(v), NL),
                None => s += &format!("None{}", NL)
            }
        }
        s
    }
}

impl WhammVisitor<String> for AsStrVisitor {
    fn visit_whamm(&mut self, whamm: &Whamm) -> String {
        let mut s = "".to_string();

        // print fns
        if whamm.fns.len() > 0 {
            s += &format!("Whamm events:{}", NL);
            self.increase_indent();
            for f in whamm.fns.iter() {
                s += &format!("{}{}", self.visit_fn(f), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if whamm.globals.len() > 0 {
            s += &format!("Whamm globals:{}", NL);
            self.increase_indent();
            for (name, global) in whamm.globals.iter() {
                s += &format!("{}{} := ", self.get_indent(), name);
                match &global.value {
                    Some(v) => s += &format!("{}{}", self.visit_value(v), NL),
                    None => s += &format!("None{}", NL)
                }
            }
            self.decrease_indent();
        }

        s += &format!("Whammys:{}", NL);
        self.increase_indent();
        for whammy in whamm.whammys.iter() {
            s += &format!("{} `{}`:{}", self.get_indent(), whammy.name, NL);
            self.increase_indent();
            s += &format!("{}", self.visit_whammy(whammy));
            self.decrease_indent();
        }
        self.decrease_indent();

        s
    }

    fn visit_whammy(&mut self, whammy: &Whammy) -> String {
        let mut s = "".to_string();

        // print fns
        if whammy.fns.len() > 0 {
            s += &format!("{} whammy events:{}", self.get_indent(), NL);
            self.increase_indent();
            for f in whammy.fns.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(f), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if whammy.globals.len() > 0 {
            s += &format!("{} whammy globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_globals(&whammy.globals);
            self.decrease_indent();
        }

        // print providers
        s += &format!("{} whammy providers:{}", self.get_indent(), NL);
        for (name, provider) in whammy.providers.iter() {
            self.increase_indent();
            s += &format!("{} `{}` {{{}", self.get_indent(), name, NL);

            self.increase_indent();
            s += &format!("{}", self.visit_provider(provider));
            self.decrease_indent();

            s += &format!("{} }}{}", self.get_indent(), NL);
            self.decrease_indent();
        }

        s
    }

    fn visit_provider(&mut self, provider: &Provider) -> String {
        let mut s = "".to_string();

        // print fns
        if provider.fns.len() > 0 {
            s += &format!("{} events:{}", self.get_indent(), NL);
            self.increase_indent();
            for f in provider.fns.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(f), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if provider.globals.len() > 0 {
            s += &format!("{} globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_globals(&provider.globals);
            self.decrease_indent();
        }

        // print packages
        if provider.packages.len() > 0 {
            s += &format!("{} packages:{}", self.get_indent(), NL);
            for (name, package) in provider.packages.iter() {
                self.increase_indent();
                s += &format!("{} `{}` {{{}", self.get_indent(), name, NL);

                self.increase_indent();
                s += &format!("{}", self.visit_package(package));
                self.decrease_indent();

                s += &format!("{} }}{}", self.get_indent(), NL);
                self.decrease_indent();
            }
        }

        s
    }

    fn visit_package(&mut self, package: &Package) -> String {
        let mut s = "".to_string();

        // print fns
        if package.fns.len() > 0 {
            s += &format!("{} package fns:{}", self.get_indent(), NL);
            self.increase_indent();
            for f in package.fns.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(f), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if package.globals.len() > 0 {
            s += &format!("{} package globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_globals(&package.globals);
            self.decrease_indent();
        }

        // print events
        s += &format!("{} package events:{}", self.get_indent(), NL);
        for (name, event) in package.events.iter() {
            self.increase_indent();
            s += &format!("{} `{}` {{{}", self.get_indent(), name, NL);

            self.increase_indent();
            s += &format!("{}", self.visit_event(event));
            self.decrease_indent();

            s += &format!("{} }}{}", self.get_indent(), NL);
            self.decrease_indent();
        }

        s
    }

    fn visit_event(&mut self, event: &Event) -> String {
        let mut s = "".to_string();

        // print fns
        if event.fns.len() > 0 {
            s += &format!("{} event fns:{}", self.get_indent(), NL);
            self.increase_indent();
            for f in event.fns.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(f), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if event.globals.len() > 0 {
            s += &format!("{} event globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_globals(&event.globals);
            self.decrease_indent();
        }

        // print probes
        if event.probe_map.len() > 0 {
            s += &format!("{} event probe_map:{}", self.get_indent(), NL);
            for (name, probes) in event.probe_map.iter() {
                self.increase_indent();
                s += &format!("{} {}: ", self.get_indent(), name);

                s += &format!("(");
                for probe in probes.iter() {
                    self.visit_probe(probe);
                }
                s += &format!("){}", NL);
                self.decrease_indent();
            }
        }

        s
    }

    fn visit_probe(&mut self, probe: &Probe) -> String {
        let mut s = "".to_string();

        s += &format!("{} `{}` probe {{{}", self.get_indent(), probe.name, NL);
        self.increase_indent();

        // print fns
        if probe.fns.len() > 0 {
            s += &format!("{} probe fns:{}", self.get_indent(), NL);
            self.increase_indent();
            for f in probe.fns.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(f), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if probe.globals.len() > 0 {
            s += &format!("{} probe globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_globals(&probe.globals);
            self.decrease_indent();
        }

        // print predicate
        s += &format!("{} `predicate`:{}", self.get_indent(), NL);
        self.increase_indent();
        match &probe.predicate {
            Some(pred) => s += &format!("{} / {} /{}", self.get_indent(), self.visit_expr(pred), NL),
            None => s += &format!("{} / None /{}", self.get_indent(), NL)
        }
        self.decrease_indent();

        // print body
        s += &format!("{} `body`:{}", self.get_indent(), NL);
        self.increase_indent();
        match &probe.body {
            Some(b) => {
                for stmt in b {
                    s += &format!("{} {};{}", self.get_indent(), self.visit_stmt(stmt), NL)
                }
            },
            None => s += &format!("{{}}")
        }
        self.decrease_indent();

        self.decrease_indent();
        s += &format!("{} }}{}", self.get_indent(), NL);

        s
    }

    // fn visit_predicate(&mut self, _predicate: &Expr) -> String {
    //     unimplemented!()
    // }

    fn visit_fn(&mut self, f: &parser_types::Fn) -> String {
        let mut s = "".to_string();

        // print name
        s += &format!("{} {} (", self.get_indent(), f.name);

        // print params
        for param in f.params.iter() {
            s += &format!("{}, ", self.visit_formal_param(param));
        }
        s += &format!(")");

        // print return type
        match &f.return_ty {
            Some(ty) => {
                s += &format!(" -> {}", self.visit_datatype(ty));
            },
            _ => {}
        }
        s += &format!(" {{{}", NL);

        // print body
        self.increase_indent();
        match &f.body {
            Some(stmts) => {
                for stmt in stmts.iter() {
                    s += &format!("{}{}{}", self.get_indent(), self.visit_stmt(stmt), NL);
                }
            },
            _ => {}
        }
        self.decrease_indent();
        s += &format!("{} }}{}", self.get_indent(), NL);

        s
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> String {
        format!("{}: {}", self.visit_expr(&param.0), self.visit_datatype(&param.1))
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Assign {var_id, expr} => {
                format!("{} = {}", self.visit_expr(var_id), self.visit_expr(expr))
            },
            Statement::Expr {expr} => {
                self.visit_expr(expr)
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::BinOp {lhs, op, rhs} => {
                let mut s = "".to_string();
                s += &format!("{} {} {}",
                    self.visit_expr(lhs),
                    self.visit_op(op),
                    self.visit_expr(rhs)
                );
                s
            },
            Expr::Call {fn_target, args} => {
                let mut s = "".to_string();
                s += &format!("{}(", self.visit_expr(fn_target));
                match args {
                    Some(args) => {
                        for arg in args {
                            s += &format!("{}, ", self.visit_expr(arg));
                        }
                    },
                    _ => {}
                }
                s += &format!(")");
                s
            },
            Expr::VarId {name} => {
                format!("{}", name)
            }
            Expr::Primitive {val} => {
                self.visit_value(val)
            }
        }
    }

    fn visit_op(&mut self, op: &Op) -> String {
        match op {
            Op::And => "&&",
            Op::Or => "||",
            Op::EQ => "==",
            Op::NE => "!=",
            Op::GE => ">=",
            Op::GT => ">",
            Op::LE => "<=",
            Op::LT => "<",
            Op::Add => "+",
            Op::Subtract => "-",
            Op::Multiply => "*",
            Op::Divide => "/",
            Op::Modulo => "%",
        }.parse().unwrap()
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> String {
        match datatype {
            DataType::Integer => {
                "int".to_string()
            },
            DataType::Boolean => {
                "bool".to_string()
            },
            DataType::Null => {
                "null".to_string()
            },
            DataType::Str => {
                "str".to_string()
            },
            DataType::Tuple {..} => {
                "tuple".to_string()
            },
        }
    }

    fn visit_value(&mut self, value: &Value) -> String {
        match value {
            Value::Boolean { ty: _ty, val} => {
                let mut s = "".to_string();
                s += &format!("{}", val);
                s
            },
            Value::Integer { ty: _ty, val} => {
                let mut s = "".to_string();
                s += &format!("{}", val);
                s
            },
            Value::Str {ty: _ty, val, addr: _addr} => {
                let mut s = "".to_string();
                s += &format!("\"{}\"", val);
                s
            },
            Value::Tuple {ty: _ty, vals} => {
                let mut s = "".to_string();
                s += &format!("(");
                for v in vals.iter() {
                    s += &format!("{}, ", self.visit_expr(v));
                }
                s += &format!(")");
                s
            }
        }
    }
}
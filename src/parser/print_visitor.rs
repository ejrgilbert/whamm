use crate::parser::types as parser_types;
use parser_types::{WhammVisitor};

use std::cmp;
use std::collections::HashMap;
use crate::parser::types::{DataType, MMScript, Whamm, Expr, Function, Module, Op, Probe, Provider, Statement, Value};

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

    fn visit_globals(&mut self, globals: &HashMap<String, (DataType, Expr, Option<Value>)>) -> String {
        let mut s = "".to_string();
        for (name, (_ty, _var_id, val)) in globals.iter() {
            s += &format!("{}{} := ", self.get_indent(), name);
            match val {
                Some(v) => s += &format!("{}{NL}", self.visit_value(v)),
                None => s += &format!("None{NL}")
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
            s += &format!("Whamm functions:{NL}");
            self.increase_indent();
            for f in whamm.fns.iter() {
                s += &format!("{}{NL}", self.visit_fn(f));
            }
            self.decrease_indent();
        }

        // print globals
        if whamm.globals.len() > 0 {
            s += &format!("Whamm globals:{NL}");
            self.increase_indent();
            for (name, (_ty, _var_id, val)) in whamm.globals.iter() {
                s += &format!("{}{} := ", self.get_indent(), name);
                match val {
                    Some(v) => s += &format!("{}{NL}", self.visit_value(v)),
                    None => s += &format!("None{NL}")
                }
            }
            self.decrease_indent();
        }

        s += &format!("Whamm mmscripts:{NL}");
        self.increase_indent();
        for mmscript in whamm.mmscripts.iter() {
            s += &format!("{} `{}`:{NL}", self.get_indent(), mmscript.name);
            self.increase_indent();
            s += &format!("{}", self.visit_mmscript(mmscript));
            self.decrease_indent();
        }
        self.decrease_indent();

        s
    }

    fn visit_mmscript(&mut self, mmscript: &MMScript) -> String {
        let mut s = "".to_string();

        // print fns
        if mmscript.fns.len() > 0 {
            s += &format!("{} mmscript functions:{NL}", self.get_indent());
            self.increase_indent();
            for f in mmscript.fns.iter() {
                s += &format!("{}{}{NL}", self.get_indent(), self.visit_fn(f));
            }
            self.decrease_indent();
        }

        // print globals
        if mmscript.globals.len() > 0 {
            s += &format!("{} mmscript globals:{NL}", self.get_indent());
            self.increase_indent();
            self.visit_globals(&mmscript.globals);
            self.decrease_indent();
        }

        // print providers
        s += &format!("{} mmscript providers:{NL}", self.get_indent());
        for (name, provider) in mmscript.providers.iter() {
            self.increase_indent();
            s += &format!("{} `{name}` {{{NL}", self.get_indent());

            self.increase_indent();
            s += &format!("{}", self.visit_provider(provider));
            self.decrease_indent();

            s += &format!("{} }}{NL}", self.get_indent());
            self.decrease_indent();
        }

        s
    }

    fn visit_provider(&mut self, provider: &Provider) -> String {
        let mut s = "".to_string();

        // print fns
        if provider.fns.len() > 0 {
            s += &format!("{} functions:{NL}", self.get_indent());
            self.increase_indent();
            for f in provider.fns.iter() {
                s += &format!("{}{}{NL}", self.get_indent(), self.visit_fn(f));
            }
            self.decrease_indent();
        }

        // print globals
        if provider.globals.len() > 0 {
            s += &format!("{} globals:{NL}", self.get_indent());
            self.increase_indent();
            self.visit_globals(&provider.globals);
            self.decrease_indent();
        }

        // print modules
        if provider.modules.len() > 0 {
            s += &format!("{} modules:{NL}", self.get_indent());
            for (name, module) in provider.modules.iter() {
                self.increase_indent();
                s += &format!("{} `{name}` {{{NL}", self.get_indent());

                self.increase_indent();
                s += &format!("{}", self.visit_module(module));
                self.decrease_indent();

                s += &format!("{} }}{NL}", self.get_indent());
                self.decrease_indent();
            }
        }

        s
    }

    fn visit_module(&mut self, module: &Module) -> String {
        let mut s = "".to_string();

        // print fns
        if module.fns.len() > 0 {
            s += &format!("{} module fns:{NL}", self.get_indent());
            self.increase_indent();
            for f in module.fns.iter() {
                s += &format!("{}{}{NL}", self.get_indent(), self.visit_fn(f));
            }
            self.decrease_indent();
        }

        // print globals
        if module.globals.len() > 0 {
            s += &format!("{} module globals:{NL}", self.get_indent());
            self.increase_indent();
            self.visit_globals(&module.globals);
            self.decrease_indent();
        }

        // print functions
        s += &format!("{} module functions:{NL}", self.get_indent());
        for (name, function) in module.functions.iter() {
            self.increase_indent();
            s += &format!("{} `{name}` {{{NL}", self.get_indent());

            self.increase_indent();
            s += &format!("{}", self.visit_function(function));
            self.decrease_indent();

            s += &format!("{} }}{NL}", self.get_indent());
            self.decrease_indent();
        }

        s
    }

    fn visit_function(&mut self, function: &Function) -> String {
        let mut s = "".to_string();

        // print fns
        if function.fns.len() > 0 {
            s += &format!("{} function fns:{NL}", self.get_indent());
            self.increase_indent();
            for f in function.fns.iter() {
                s += &format!("{}{}{NL}", self.get_indent(), self.visit_fn(f));
            }
            self.decrease_indent();
        }

        // print globals
        if function.globals.len() > 0 {
            s += &format!("{} function globals:{NL}", self.get_indent());
            self.increase_indent();
            self.visit_globals(&function.globals);
            self.decrease_indent();
        }

        // print probes
        if function.probe_map.len() > 0 {
            s += &format!("{} function probe_map:{NL}", self.get_indent());
            for (name, probes) in function.probe_map.iter() {
                self.increase_indent();
                s += &format!("{} {name}: ", self.get_indent());

                s += &format!("(");
                for probe in probes.iter() {
                    self.visit_probe(probe);
                }
                s += &format!("){NL}");
                self.decrease_indent();
            }
        }

        s
    }

    fn visit_probe(&mut self, probe: &Probe) -> String {
        let mut s = "".to_string();

        s += &format!("{} `{}` probe {{{NL}", self.get_indent(), probe.name);
        self.increase_indent();

        // print fns
        if probe.fns.len() > 0 {
            s += &format!("{} probe fns:{NL}", self.get_indent());
            self.increase_indent();
            for f in probe.fns.iter() {
                s += &format!("{}{}{NL}", self.get_indent(), self.visit_fn(f));
            }
            self.decrease_indent();
        }

        // print globals
        if probe.globals.len() > 0 {
            s += &format!("{} probe globals:{NL}", self.get_indent());
            self.increase_indent();
            self.visit_globals(&probe.globals);
            self.decrease_indent();
        }

        // print predicate
        s += &format!("{} `predicate`:{NL}", self.get_indent());
        self.increase_indent();
        match &probe.predicate {
            Some(pred) => s += &format!("{} / {} /{NL}", self.get_indent(), self.visit_expr(pred)),
            None => s += &format!("{} / None /{NL}", self.get_indent())
        }
        self.decrease_indent();

        // print body
        s += &format!("{} `body`:{NL}", self.get_indent());
        self.increase_indent();
        match &probe.body {
            Some(b) => {
                for stmt in b {
                    s += &format!("{} {};{NL}", self.get_indent(), self.visit_stmt(stmt))
                }
            },
            None => s += &format!("{{}}")
        }
        self.decrease_indent();

        self.decrease_indent();
        s += &format!("{} }}{NL}", self.get_indent());

        s
    }

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
        s += &format!(" {{{NL}");

        // print body
        self.increase_indent();
        match &f.body {
            Some(stmts) => {
                for stmt in stmts.iter() {
                    s += &format!("{}{}{NL}", self.get_indent(), self.visit_stmt(stmt));
                }
            },
            _ => {}
        }
        self.decrease_indent();
        s += &format!("{} }}{NL}", self.get_indent());

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
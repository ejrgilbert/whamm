use crate::parser::types as parser_types;
use parser_types::WhammVisitor;

use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Expr, Global, ProvidedFunction, ProvidedGlobal, Script, Statement,
    UnOp, Value, Whamm,
};
use std::cmp;
use std::collections::HashMap;

const NL: &str = "\n";

pub struct AsStrVisitor {
    pub indent: i32,
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
                None => s += &format!("None{}", NL),
            }
        }
        s
    }

    fn visit_provided_globals(&mut self, globals: &HashMap<String, ProvidedGlobal>) -> String {
        let mut s = "".to_string();
        for (name, ProvidedGlobal { global, .. }) in globals.iter() {
            s += &format!("{}{} := ", self.get_indent(), name);
            match &global.value {
                Some(v) => s += &format!("{}{}", self.visit_value(v), NL),
                None => s += &format!("None{}", NL),
            }
        }
        s
    }
}

impl WhammVisitor<'_, String> for AsStrVisitor {
    fn visit_whamm(&mut self, whamm: &Whamm) -> String {
        let mut s = "".to_string();

        // print fns
        if !whamm.fns.is_empty() {
            s += &format!("Whamm functions:{}", NL);
            self.increase_indent();
            for ProvidedFunction { function, .. } in whamm.fns.iter() {
                s += &format!("{}{}", self.visit_fn(function), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if !whamm.globals.is_empty() {
            s += &format!("Whamm globals:{}", NL);
            self.increase_indent();
            for (name, ProvidedGlobal { global, .. }) in whamm.globals.iter() {
                s += &format!("{}{} := ", self.get_indent(), name);
                match &global.value {
                    Some(v) => s += &format!("{}{}", self.visit_value(v), NL),
                    None => s += &format!("None{}", NL),
                }
            }
            self.decrease_indent();
        }

        s += &format!("Scripts:{}", NL);
        self.increase_indent();
        for script in whamm.scripts.iter() {
            s += &format!("{} `{}`:{}", self.get_indent(), script.name, NL);
            self.increase_indent();
            s += &self.visit_script(script).to_string();
            self.decrease_indent();
        }
        self.decrease_indent();

        s
    }

    fn visit_script(&mut self, script: &Script) -> String {
        let mut s = "".to_string();

        // print fns
        if !script.fns.is_empty() {
            s += &format!("{} user defined functions:{}", self.get_indent(), NL);
            self.increase_indent();
            for f in script.fns.iter() {
                s += &format!("{}{}{}", "", self.visit_fn(f), NL);
            }
            self.decrease_indent();
        }

        // print globals
        if !script.globals.is_empty() {
            s += &format!("{} script globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_globals(&script.globals);
            self.decrease_indent();
        }
        //print global statments

        if !script.global_stmts.is_empty() {
            s += &format!("{} script global statements:{}", self.get_indent(), NL);
            self.increase_indent();
            for stmt in script.global_stmts.iter() {
                s += &format!("{} {};{}", self.get_indent(), self.visit_stmt(stmt), NL);
            }
            self.decrease_indent();
        }

        // print rules
        s += &format!("{} script rules:{}", self.get_indent(), NL);
        for (name, provider) in script.providers.iter() {
            self.increase_indent();
            s += &format!("{} `{}` {{{}", self.get_indent(), name, NL);

            self.increase_indent();
            s += &self.visit_provider(provider).to_string();
            self.decrease_indent();

            s += &format!("{} }}{}", self.get_indent(), NL);
            self.decrease_indent();
        }

        s
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> String {
        let mut s = "".to_string();

        // print fns
        let functions = provider.get_provided_fns();
        if !functions.is_empty() {
            s += &format!("{} events:{}", self.get_indent(), NL);
            self.increase_indent();
            for ProvidedFunction { function, .. } in functions.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(function), NL);
            }
            self.decrease_indent();
        }

        // print globals
        let globals = provider.get_provided_globals();
        if !globals.is_empty() {
            s += &format!("{} globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_provided_globals(globals);
            self.decrease_indent();
        }

        // print packages
        if provider.has_packages() {
            s += &format!("{} packages:{}", self.get_indent(), NL);
            for package in provider.packages() {
                self.increase_indent();
                s += &format!("{} `{}` {{{}", self.get_indent(), package.name(), NL);

                self.increase_indent();
                s += &self.visit_package(package).to_string();
                self.decrease_indent();

                s += &format!("{} }}{}", self.get_indent(), NL);
                self.decrease_indent();
            }
        }

        s
    }

    fn visit_package(&mut self, package: &dyn Package) -> String {
        let mut s = "".to_string();

        // print fns
        let functions = package.get_provided_fns();
        if !functions.is_empty() {
            s += &format!("{} package fns:{}", self.get_indent(), NL);
            self.increase_indent();
            for ProvidedFunction { function, .. } in functions.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(function), NL);
            }
            self.decrease_indent();
        }

        // print globals
        let globals = package.get_provided_globals();
        if !globals.is_empty() {
            s += &format!("{} package globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_provided_globals(&globals);
            self.decrease_indent();
        }

        // print events
        s += &format!("{} package events:{}", self.get_indent(), NL);
        for event in package.events() {
            self.increase_indent();
            s += &format!("{} `{}` {{{}", self.get_indent(), event.name(), NL);

            self.increase_indent();
            s += &self.visit_event(event).to_string();
            self.decrease_indent();

            s += &format!("{} }}{}", self.get_indent(), NL);
            self.decrease_indent();
        }

        s
    }

    fn visit_event(&mut self, event: &dyn Event) -> String {
        let mut s = "".to_string();

        // print fns
        let functions = event.get_provided_fns();
        if !functions.is_empty() {
            s += &format!("{} event fns:{}", self.get_indent(), NL);
            self.increase_indent();
            for ProvidedFunction { function, .. } in functions.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(function), NL);
            }
            self.decrease_indent();
        }

        // print globals
        let globals = event.get_provided_globals();
        if !globals.is_empty() {
            s += &format!("{} event globals:{}", self.get_indent(), NL);
            self.increase_indent();
            s += &self.visit_provided_globals(globals);
            self.decrease_indent();
        }

        // print probes
        let probes = event.probes();
        if !probes.is_empty() {
            s += &format!("{} event probe_map:{}", self.get_indent(), NL);
            for (name, probes) in probes.iter() {
                self.increase_indent();
                s += &format!("{} {}: ", self.get_indent(), name);

                s += &format!("({}", NL);
                s += &probes
                    .iter()
                    .map(|probe| self.visit_probe(probe))
                    .collect::<String>();
                s += &format!("){}", NL);
                self.decrease_indent();
            }
        }

        s
    }

    fn visit_probe(&mut self, probe: &Box<dyn Probe>) -> String {
        let mut s = "".to_string();

        s += &format!(
            "{} `{}` probe {{{}",
            self.get_indent(),
            probe.mode_name(),
            NL
        );
        self.increase_indent();

        // print fns
        let functions = probe.get_mode_provided_fns();
        if !functions.is_empty() {
            s += &format!("{} probe fns:{}", self.get_indent(), NL);
            self.increase_indent();
            for ProvidedFunction { function, .. } in functions.iter() {
                s += &format!("{}{}{}", self.get_indent(), self.visit_fn(function), NL);
            }
            self.decrease_indent();
        }

        // print globals
        let globals = probe.get_mode_provided_globals();
        if !globals.is_empty() {
            s += &format!("{} probe globals:{}", self.get_indent(), NL);
            self.increase_indent();
            self.visit_provided_globals(globals);
            self.decrease_indent();
        }

        // print predicate
        s += &format!("{} `predicate`:{}", self.get_indent(), NL);
        self.increase_indent();
        match probe.predicate() {
            Some(pred) => {
                let expr = self.visit_expr(pred);
                expr.split("&&")
                    .for_each(|line| s += &format!("{}&& {}{}", self.get_indent(), line, NL));
            }
            None => s += &format!("{} / None /{}", self.get_indent(), NL),
        }
        self.decrease_indent();

        // print body
        s += &format!("{} `body`:{}", self.get_indent(), NL);
        self.increase_indent();
        match probe.body() {
            Some(b) => {
                for stmt in b {
                    s += &format!("{} {};{}", self.get_indent(), self.visit_stmt(stmt), NL)
                }
            }
            None => s += "{}",
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
        s += &format!("{} {} (", self.get_indent(), f.name.name);

        // print params
        s += &f
            .params
            .iter()
            .map(|arg| self.visit_formal_param(arg))
            .collect::<Vec<String>>()
            .join(", ");

        s += ")";

        // print return type
        if let Some(ty) = &f.return_ty {
            s += &format!(" -> {}", self.visit_datatype(ty));
        }
        s += &format!(" {{{}", NL);

        // print body
        self.increase_indent();
        s += &self.visit_block(&f.body);
        self.decrease_indent();
        s += &format!("{} }}{}", self.get_indent(), NL);

        s
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> String {
        format!(
            "{}: {}",
            self.visit_expr(&param.0),
            self.visit_datatype(&param.1)
        )
    }

    fn visit_block(&mut self, block: &Block) -> String {
        let mut s = "".to_string();
        for stmt in block.stmts.iter() {
            s += &format!(
                "{} {}{}{}",
                self.get_indent(),
                self.visit_stmt(stmt),
                ";",
                NL
            );
        }
        s
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Decl { ty, var_id, .. } => {
                format!("{} {}", self.visit_datatype(ty), self.visit_expr(var_id))
            }
            Statement::Assign { var_id, expr, .. } => {
                format!("{} = {}", self.visit_expr(var_id), self.visit_expr(expr))
            }
            Statement::Expr { expr, .. } => self.visit_expr(expr),
            Statement::Return { expr, .. } => {
                format!("return {}", self.visit_expr(expr))
            }
            Statement::If {
                cond, conseq, alt, ..
            } => {
                let mut s = "".to_string();
                s += &format!("if ({}) {{{}", self.visit_expr(cond), NL);
                self.increase_indent();
                s += &self.visit_block(conseq);
                self.decrease_indent();
                s += &format!("{} }}", self.get_indent());
                s += &format!(" else {{{}", NL);
                self.increase_indent();
                s += &self.visit_block(alt);
                self.decrease_indent();
                s += &format!("{} }}", self.get_indent());
                s
            }
            Statement::SetMap { map, key, val, .. } => {
                format!(
                    "(map) {}[{}] = {}",
                    self.visit_expr(map),
                    self.visit_expr(key),
                    self.visit_expr(val)
                )
            }
            Statement::ReportDecl { decl, .. } => {
                format!("{} {}", "report", self.visit_stmt(decl))
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                let mut s = "".to_string();
                s += &format!(
                    "{} ? {} : {}",
                    self.visit_expr(cond),
                    self.visit_expr(conseq),
                    self.visit_expr(alt)
                );
                s
            }
            Expr::BinOp { lhs, op, rhs, .. } => {
                let mut s = "".to_string();
                s += &format!(
                    "{} {} {}",
                    self.visit_expr(lhs),
                    self.visit_binop(op),
                    self.visit_expr(rhs)
                );
                s
            }
            Expr::Call {
                fn_target, args, ..
            } => {
                let mut s = "".to_string();
                s += &format!("{}(", self.visit_expr(fn_target));
                if let Some(args) = args {
                    s += &args
                        .iter()
                        .map(|arg| self.visit_expr(arg))
                        .collect::<Vec<String>>()
                        .join(", ");
                }
                s += ")";
                s
            }
            Expr::VarId { name, .. } => name.to_string(),
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::UnOp { op, expr, .. } => {
                let mut s = "".to_string();
                s += &format!("{}{}", self.visit_unop(op), self.visit_expr(expr));
                s
            }
            Expr::GetMap { map, key, .. } => {
                let mut s = "".to_string();
                s += &format!("(map) {}[{}]", self.visit_expr(map), self.visit_expr(key));
                s
            }
        }
    }

    fn visit_unop(&mut self, op: &UnOp) -> String {
        match op {
            UnOp::Not => "!",
        }
        .parse()
        .unwrap()
    }

    fn visit_binop(&mut self, op: &BinOp) -> String {
        match op {
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::EQ => "==",
            BinOp::NE => "!=",
            BinOp::GE => ">=",
            BinOp::GT => ">",
            BinOp::LE => "<=",
            BinOp::LT => "<",
            BinOp::Add => "+",
            BinOp::Subtract => "-",
            BinOp::Multiply => "*",
            BinOp::Divide => "/",
            BinOp::Modulo => "%",
        }
        .parse()
        .unwrap()
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> String {
        match datatype {
            DataType::I32 => "i32".to_string(),
            DataType::U32 => "u32".to_string(),
            DataType::Boolean => "bool".to_string(),
            DataType::Null => "null".to_string(),
            DataType::Str => "str".to_string(),
            DataType::Tuple { ty_info } => {
                let mut s = "".to_string();
                s += "(";
                s += &ty_info
                    .iter()
                    .map(|ty| self.visit_datatype(ty))
                    .collect::<Vec<String>>()
                    .join(", ");
                s += ")";
                s
            }
            DataType::Map { key_ty, val_ty, .. } => format!(
                "map<{}, {}>",
                self.visit_datatype(key_ty),
                self.visit_datatype(val_ty)
            ),
            DataType::AssumeGood => "unknown".to_string(),
        }
    }

    fn visit_value(&mut self, value: &Value) -> String {
        match value {
            Value::Boolean { ty: _ty, val } => {
                let mut s = "".to_string();
                s += &format!("{}", val);
                s
            }
            Value::Integer { ty: _ty, val } => {
                let mut s = "".to_string();
                s += &format!("{}", val);
                s
            }
            Value::Str {
                ty: _ty,
                val,
                addr: _addr,
            } => {
                let mut s = "".to_string();
                s += &format!("\"{}\"", val);
                s
            }
            Value::Tuple { ty: _ty, vals } => {
                let mut s = "".to_string();
                s += "(";
                s += &vals
                    .iter()
                    .map(|v| self.visit_expr(v))
                    .collect::<Vec<String>>()
                    .join(", ");
                s += ")";
                s
            }
        }
    }
}

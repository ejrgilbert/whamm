use pest::error::LineColLocation;
use std::collections::HashMap;
use termcolor::{Buffer, ColorChoice, WriteColor};

use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{green, grey_italics, long_line, magenta, white, yellow};
use crate::parser::rules::{
    print_provider_docs, provider_factory, Event, Package, Probe, Provider, WhammProvider,
};
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use termcolor::BufferWriter;
use walrus::DataId;

#[derive(Parser)]
#[grammar = "./parser/whamm.pest"] // Path relative to base `src` dir
pub struct WhammParser;

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            .op(Op::infix(and, Left) | Op::infix(or, Left)) // LOGOP
            .op(Op::infix(eq, Left)                         // RELOP
                | Op::infix(ne, Left)
                | Op::infix(ge, Left)
                | Op::infix(gt, Left)
                | Op::infix(le, Left)
                | Op::infix(lt, Left)
            ).op(Op::infix(add, Left) | Op::infix(subtract, Left)) // SUMOP
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left)) // MULOP
            .op(Op::prefix(neg))
    };
}

const UNEXPECTED_ERR_MSG: &str =
    "WhammParser: Looks like you've found a bug...please report this behavior! Exiting now...";

// ===============
// ==== Types ====
// ===============

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Location {
    /// Line/column within the input string
    pub line_col: LineColLocation,
    pub path: Option<String>,
}
impl Location {
    pub fn from(loc0: &LineColLocation, loc1: &LineColLocation, path: Option<String>) -> Self {
        let pos0 = match loc0 {
            LineColLocation::Pos(pos0) => pos0,
            LineColLocation::Span(span0, ..) => span0,
        };

        let pos1 = match loc1 {
            LineColLocation::Pos(pos0) => pos0,
            LineColLocation::Span(.., span1) => span1,
        };

        Location {
            line_col: LineColLocation::Span(*pos0, *pos1),
            path,
        }
    }

    pub fn span_between(loc0: &Location, loc1: &Location) -> LineColLocation {
        let pos0 = match &loc0.line_col {
            LineColLocation::Pos(pos0) | LineColLocation::Span(pos0, ..) => *pos0,
        };

        let pos1 = match &loc1.line_col {
            LineColLocation::Pos(end1) | LineColLocation::Span(.., end1) => *end1,
        };

        LineColLocation::Span(pos0, pos1)
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::I32, DataType::I32)
            | (DataType::Boolean, DataType::Boolean)
            | (DataType::Null, DataType::Null)
            | (DataType::Str, DataType::Str)
            | (_, DataType::AssumeGood)
            | (DataType::AssumeGood, _) => true,
            (DataType::Tuple { ty_info: ty_info0 }, DataType::Tuple { ty_info: ty_info1 }) => {
                ty_info0.len() == ty_info1.len()
                    && ty_info0
                        .iter()
                        .zip(ty_info1.iter())
                        .all(|(ty0, ty1)| ty0 == ty1)
            }
            (
                DataType::Map {
                    key_ty: key_ty0,
                    val_ty: val_ty0,
                },
                DataType::Map {
                    key_ty: key_ty1,
                    val_ty: val_ty1,
                },
            ) => key_ty0 == key_ty1 && val_ty0 == val_ty1,
            _ => false,
        }
    }
}

impl Eq for DataType {}

#[derive(Clone, Debug)]
pub enum DataType {
    I32,
    U32,
    Boolean,
    Null,
    Str,
    Tuple {
        ty_info: Vec<Box<DataType>>,
    },
    Map {
        key_ty: Box<DataType>,
        val_ty: Box<DataType>,
    },
    AssumeGood,
}
impl DataType {
    pub fn print(&self, buffer: &mut Buffer) {
        match self {
            DataType::I32 => {
                yellow(true, "i32".to_string(), buffer);
            }
            DataType::U32 => {
                yellow(true, "u32".to_string(), buffer);
            }
            DataType::Boolean => {
                yellow(true, "bool".to_string(), buffer);
            }
            DataType::Null => {
                yellow(true, "null".to_string(), buffer);
            }
            DataType::Str => {
                yellow(true, "str".to_string(), buffer);
            }
            DataType::Tuple { ty_info } => {
                white(true, "(".to_string(), buffer);
                let mut is_first = true;
                for ty in ty_info.iter() {
                    if !is_first {
                        white(true, ", ".to_string(), buffer);
                    }
                    ty.print(buffer);
                    is_first = false;
                }

                white(true, ")".to_string(), buffer);
            }
            DataType::Map { key_ty, val_ty } => {
                yellow(true, "map".to_string(), buffer);
                white(true, "<".to_string(), buffer);
                key_ty.print(buffer);
                white(true, ", ".to_string(), buffer);
                val_ty.print(buffer);
                white(true, ">".to_string(), buffer);
            }
            DataType::AssumeGood => {
                yellow(true, "unknown".to_string(), buffer);
            }
        }
    }
}

// Values
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Integer {
        ty: DataType,
        val: i32,
    },
    Str {
        ty: DataType,
        val: String,

        // Used by emitter to store this string's address/len in Wasm memory
        // DataId: Walrus ID to reference data segment
        // u32: address of data in memory
        // usize:  the length of the string in memory
        addr: Option<(DataId, u32, usize)>,
    },
    Tuple {
        ty: DataType,
        vals: Vec<Expr>,
    },
    Boolean {
        ty: DataType,
        val: bool,
    },
}
#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub loc: Option<Location>,
}
impl Block {
    pub fn loc(&self) -> &Option<Location> {
        &self.loc
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        self.loc().as_ref().map(|loc| loc.line_col.clone())
    }
}

// Statements
#[derive(Clone, Debug)]
pub enum Statement {
    SaveDecl {
        decl: Box<Statement>,
        loc: Option<Location>,
    },
    Decl {
        ty: DataType,
        var_id: Expr, // should be VarId
        loc: Option<Location>,
    },

    Assign {
        var_id: Expr, // Should be VarId
        expr: Expr,
        loc: Option<Location>,
    },
    SetMap {
        map: Expr, // Should be VarId
        key: Expr,
        val: Expr,
        loc: Option<Location>,
    },
    Expr {
        expr: Expr,
        loc: Option<Location>,
    },
    Return {
        expr: Expr,
        loc: Option<Location>,
    },
    If {
        cond: Expr,
        conseq: Block,
        alt: Block,
        loc: Option<Location>,
    },
}
impl Statement {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Statement::Decl { loc, .. }
            | Statement::If { loc, .. }
            | Statement::Return { loc, .. }
            | Statement::Assign { loc, .. }
            | Statement::SetMap { loc, .. }
            | Statement::SaveDecl { loc, .. }
            | Statement::Expr { loc, .. } => loc,
        }
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        self.loc().as_ref().map(|loc| loc.line_col.clone())
    }
    pub fn dummy() -> Self {
        Self::Expr {
            expr: Expr::Primitive {
                val: Value::Integer {
                    ty: DataType::I32,
                    val: 0,
                },
                loc: None,
            },
            loc: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    UnOp {
        // Type is based on the outermost `op`
        op: UnOp,
        expr: Box<Expr>,
        loc: Option<Location>,
    },
    Ternary {
        cond: Box<Expr>,
        conseq: Box<Expr>,
        alt: Box<Expr>,
        loc: Option<Location>,
    },
    BinOp {
        // Type is based on the outermost `op` (if arithmetic op, also based on types of lhs/rhs due to doubles)
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
        loc: Option<Location>,
    },
    Call {
        // Type is fn_target.return_ty, should be VarId
        fn_target: Box<Expr>,
        args: Option<Vec<Box<Expr>>>,
        loc: Option<Location>,
    },
    VarId {
        is_comp_provided: bool, // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!
        name: String,
        loc: Option<Location>,
    },
    Primitive {
        // Type is val.ty
        val: Value,
        loc: Option<Location>,
    },
    GetMap {
        map: Box<Expr>,
        key: Box<Expr>,
        loc: Option<Location>,
    },
}
impl Expr {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Expr::UnOp { loc, .. }
            | Expr::Ternary { loc, .. }
            | Expr::BinOp { loc, .. }
            | Expr::Call { loc, .. }
            | Expr::VarId { loc, .. }
            | Expr::Primitive { loc, .. }
            | Expr::GetMap { loc, .. } => loc,
        }
    }
}

// Functions

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FnId {
    pub name: String,
    pub loc: Option<Location>,
}

#[derive(Clone, Debug)]
pub struct Fn {
    pub(crate) is_comp_provided: bool,
    pub(crate) name: FnId,
    pub(crate) params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) return_ty: Option<DataType>,
    pub(crate) body: Block,
}
impl Fn {
    pub fn print(&self, buffer: &mut Buffer) {
        green(true, self.name.name.to_string(), buffer);
        white(true, "(".to_string(), buffer);
        let mut is_first = true;
        for (param_name, param_ty) in self.params.iter() {
            if !is_first {
                white(true, ", ".to_string(), buffer);
            }
            if let Expr::VarId { name, .. } = param_name {
                green(true, name.to_string(), buffer);
                white(true, ": ".to_string(), buffer);
                param_ty.print(buffer);
            }
            is_first = false;
        }
        white(true, ")".to_string(), buffer);

        if let Some(return_ty) = &self.return_ty {
            white(true, " -> ".to_string(), buffer);
            return_ty.print(buffer);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Global {
    pub is_comp_provided: bool,

    pub ty: DataType,
    pub var_name: Expr, // Should be VarId
    pub value: Option<Value>,
}
impl Global {
    pub fn print(&self, buffer: &mut Buffer) {
        if let Expr::VarId { name, .. } = &self.var_name {
            green(true, name.to_string(), buffer);
        }
        white(true, ": ".to_string(), buffer);
        self.ty.print(buffer);
    }
}

pub(crate) fn print_global_vars(
    tabs: &mut usize,
    globals: &HashMap<String, ProvidedGlobal>,
    buffer: &mut Buffer,
) {
    if !globals.is_empty() {
        white(true, format!("{}GLOBALS:\n", " ".repeat(*tabs * 4)), buffer);
        *tabs += 1;
        for (.., ProvidedGlobal { docs, global, .. }) in globals.iter() {
            white(false, " ".repeat(*tabs * 4).to_string(), buffer);
            global.print(buffer);

            *tabs += 1;
            white(
                false,
                format!("\n{}{}\n", " ".repeat(*tabs * 4), docs),
                buffer,
            );
            *tabs -= 1;
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buffer);
    }
}

pub(crate) fn print_fns(tabs: &mut usize, functions: &[ProvidedFunction], buffer: &mut Buffer) {
    if !functions.is_empty() {
        white(
            true,
            format!("{}FUNCTIONS:\n", " ".repeat(*tabs * 4)),
            buffer,
        );
        *tabs += 1;
        for ProvidedFunction { docs, function, .. } in functions.iter() {
            green(true, " ".repeat(*tabs * 4).to_string(), buffer);
            function.print(buffer);
            green(true, "\n".to_string(), buffer);
            *tabs += 1;
            white(
                false,
                format!("{}{}\n", " ".repeat(*tabs * 4), docs),
                buffer,
            );
            *tabs -= 1;
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buffer);
    }
}

pub type ProvidedProbes = HashMap<
    String,
    (
        ProvidedFunctionality,
        HashMap<
            String,
            (
                ProvidedFunctionality,
                HashMap<String, (ProvidedFunctionality, Vec<(ProvidedFunctionality, String)>)>,
            ),
        >,
    ),
>;

pub struct Whamm {
    pub provided_probes: ProvidedProbes,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    pub scripts: Vec<Script>,
}
impl Default for Whamm {
    fn default() -> Self {
        Self::new()
    }
}
impl Whamm {
    pub fn new() -> Self {
        let whamm = Whamm {
            provided_probes: HashMap::new(),
            fns: Whamm::get_provided_fns(),
            globals: Whamm::get_provided_globals(),

            scripts: vec![],
        };
        whamm
    }

    fn get_provided_fns() -> Vec<ProvidedFunction> {
        let strcmp_params = vec![
            (
                Expr::VarId {
                    is_comp_provided: true,
                    name: "str_addr".to_string(),
                    loc: None,
                },
                DataType::Tuple {
                    ty_info: vec![Box::new(DataType::I32), Box::new(DataType::I32)],
                },
            ),
            (
                Expr::VarId {
                    is_comp_provided: true,
                    name: "value".to_string(),
                    loc: None,
                },
                DataType::Str,
            ),
        ];

        let strcmp = ProvidedFunction::new(
            "strcmp".to_string(),
            "Compare two wasm strings and return whether they are equivalent.".to_string(),
            strcmp_params,
            Some(DataType::Boolean),
        );

        vec![strcmp]
    }

    fn get_provided_globals() -> HashMap<String, ProvidedGlobal> {
        HashMap::new()
    }

    pub fn add_script(&mut self, mut script: Script) -> usize {
        let id = self.scripts.len();
        script.name = format!("script{}", id);
        self.scripts.push(script);

        id
    }
}

/// SpecPart are the probe ids in a probe spec
pub struct SpecPart {
    pub name: String,
    pub loc: Option<Location>,
}

pub struct ProbeSpec {
    pub provider: Option<SpecPart>,
    pub package: Option<SpecPart>,
    pub event: Option<SpecPart>,
    pub mode: Option<SpecPart>,
}
impl Default for ProbeSpec {
    fn default() -> Self {
        Self::new()
    }
}
impl ProbeSpec {
    pub fn new() -> Self {
        Self {
            provider: None,
            package: None,
            event: None,
            mode: None,
        }
    }
    pub fn add_spec_def(&mut self, part: SpecPart) {
        if self.provider.is_none() {
            self.provider = Some(part);
            return;
        }
        if self.package.is_none() {
            self.package = Some(part);
            return;
        }
        if self.event.is_none() {
            self.event = Some(part);
            return;
        }
        if self.mode.is_none() {
            self.mode = Some(part);
        }
    }

    pub fn print_bold_provider(&self, buffer: &mut Buffer) {
        magenta(
            true,
            self.provider.as_ref().unwrap().name.to_string(),
            buffer,
        );
        if let Some(package_patt) = &self.package {
            white(true, format!(":{}", &package_patt.name), buffer);
            if let Some(event_patt) = &self.event {
                white(true, format!(":{}", &event_patt.name), buffer);
                if let Some(mode_patt) = &self.mode {
                    white(true, format!(":{}", &mode_patt.name), buffer);
                }
            }
        }
        white(true, "\n".to_string(), buffer);
        grey_italics(true, "matches the following rules:\n\n".to_string(), buffer);
    }

    pub fn print_bold_package(&self, buffer: &mut Buffer) {
        white(
            true,
            format!("{}:", self.provider.as_ref().unwrap().name),
            buffer,
        );
        magenta(
            true,
            self.package.as_ref().unwrap().name.to_string(),
            buffer,
        );
        if let Some(event_patt) = &self.event {
            white(true, format!(":{}", &event_patt.name), buffer);
            if let Some(mode_patt) = &self.mode {
                white(true, format!(":{}", &mode_patt.name), buffer);
            }
        }
        white(true, "\n".to_string(), buffer);
        grey_italics(
            true,
            "matches the following packages:\n\n".to_string(),
            buffer,
        );
    }

    pub fn print_bold_event(&self, buffer: &mut Buffer) {
        white(
            true,
            format!(
                "{}:{}:",
                self.provider.as_ref().unwrap().name,
                self.package.as_ref().unwrap().name
            ),
            buffer,
        );
        magenta(true, self.event.as_ref().unwrap().name.to_string(), buffer);
        if let Some(mode_patt) = &self.mode {
            white(true, format!(":{}", &mode_patt.name), buffer);
        }
        white(true, "\n".to_string(), buffer);
        grey_italics(
            true,
            "matches the following events:\n\n".to_string(),
            buffer,
        );
    }

    pub fn print_bold_mode(&self, buffer: &mut Buffer) {
        white(
            true,
            format!(
                "{}:{}:{}:",
                self.provider.as_ref().unwrap().name,
                self.package.as_ref().unwrap().name,
                self.event.as_ref().unwrap().name
            ),
            buffer,
        );
        magenta(
            true,
            format!("{}\n", self.mode.as_ref().unwrap().name),
            buffer,
        );
        grey_italics(true, "matches the following modes:\n\n".to_string(), buffer);
    }
}

pub struct Script {
    pub name: String,
    /// The rules of the probes that have been used in the Script.
    pub providers: HashMap<String, Box<dyn Provider>>,
    pub fns: Vec<Fn>,                     // User-provided
    pub globals: HashMap<String, Global>, // User-provided, should be VarId
    pub global_stmts: Vec<Statement>,
}
impl Default for Script {
    fn default() -> Self {
        Self::new()
    }
}
impl Script {
    pub fn new() -> Self {
        Script {
            name: "".to_string(),
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new(),
            global_stmts: vec![],
        }
    }

    pub fn print_info(
        &mut self,
        probe_spec: &ProbeSpec,
        print_globals: bool,
        print_functions: bool,
    ) -> Result<(), Box<WhammError>> {
        let writer = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = writer.buffer();

        // Print `whamm` info
        let mut tabs = 0;
        if print_globals || print_functions {
            white(true, "\nCORE ".to_string(), &mut buffer);
            magenta(true, "`whamm`".to_string(), &mut buffer);
            white(true, " FUNCTIONALITY\n\n".to_string(), &mut buffer);

            // Print the globals
            if print_globals {
                let globals = Whamm::get_provided_globals();
                print_global_vars(&mut tabs, &globals, &mut buffer);
            }

            // Print the functions
            if print_functions {
                let functions = Whamm::get_provided_fns();
                print_fns(&mut tabs, &functions, &mut buffer);
            }
        }

        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        let mut providers: HashMap<String, Box<dyn Provider>> = HashMap::new();
        let (matched_providers, matched_packages, matched_events, matched_modes) =
            provider_factory::<WhammProvider>(&mut providers, probe_spec, None, None, None)?;

        // Print the matched provider information
        if matched_providers {
            probe_spec.print_bold_provider(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            print_provider_docs(
                provider,
                print_globals,
                print_functions,
                &mut tabs,
                &mut buffer,
            );
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print the matched package information
        if matched_packages {
            probe_spec.print_bold_package(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            provider.print_package_docs(print_globals, print_functions, &mut tabs, &mut buffer);
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print the matched event information
        if matched_events {
            probe_spec.print_bold_event(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            provider.print_event_docs(print_globals, print_functions, &mut tabs, &mut buffer);
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print the matched mode information
        if matched_modes {
            probe_spec.print_bold_mode(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            provider.print_mode_docs(print_globals, print_functions, &mut tabs, &mut buffer);
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        writer
            .print(&buffer)
            .expect("Uh oh, something went wrong while printing to terminal");
        buffer
            .reset()
            .expect("Uh oh, something went wrong while printing to terminal");

        Ok(())
    }

    pub fn add_global_stmts(&mut self, global_statements: Vec<Statement>) {
        for stmt in global_statements.iter() {
            self.global_stmts.push(stmt.clone());
        }
    }

    /// Iterates over all the matched rules, packages, events, and probe mode names
    /// to add a copy of the user-defined Probe for each of them.
    pub fn add_probe(
        &mut self,
        probe_spec: &ProbeSpec,
        predicate: Option<Expr>,
        body: Option<Vec<Statement>>,
    ) -> Result<(), Box<WhammError>> {
        let (matched_providers, matched_packages, matched_events, matched_modes): (
            bool,
            bool,
            bool,
            bool,
        ) = provider_factory::<WhammProvider>(
            &mut self.providers,
            probe_spec,
            None,
            predicate,
            body,
        )?;

        if !matched_providers {
            return if let Some(prov_patt) = &probe_spec.provider {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified provider pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find a provider matching pattern!"
                    )),
                    None,
                )))
            };
        }

        if !matched_packages {
            return if let Some(prov_patt) = &probe_spec.package {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified package pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find a package matching pattern!"
                    )),
                    None,
                )))
            };
        }

        if !matched_events {
            return if let Some(prov_patt) = &probe_spec.event {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified event pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find an event matching pattern!"
                    )),
                    None,
                )))
            };
        }

        if !matched_modes {
            return if let Some(prov_patt) = &probe_spec.mode {
                Err(Box::new(ErrorGen::get_parse_error(
                    true,
                    Some(format!(
                        "Could not find any matches for the specified mode pattern: {}",
                        prov_patt.name
                    )),
                    Some(prov_patt.loc.as_ref().unwrap().line_col.clone()),
                    vec![],
                    vec![],
                )))
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find a mode matching pattern!"
                    )),
                    None,
                )))
            };
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProvidedFunctionality {
    pub name: String,
    pub docs: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedGlobal {
    pub name: String,
    pub docs: String,
    pub global: Global,
}
impl ProvidedGlobal {
    pub fn new(name: String, docs: String, ty: DataType) -> Self {
        Self {
            name: name.clone(),
            docs,
            global: Global {
                is_comp_provided: true,
                ty,
                var_name: Expr::VarId {
                    is_comp_provided: true,
                    name,
                    loc: None,
                },
                value: None,
            },
        }
    }
}
#[derive(Clone, Debug)]
pub struct ProvidedFunction {
    pub name: String,
    pub docs: String,
    pub function: Fn,
}
impl ProvidedFunction {
    pub fn new(
        name: String,
        docs: String,
        params: Vec<(Expr, DataType)>,
        return_ty: Option<DataType>,
    ) -> Self {
        Self {
            name: name.clone(),
            docs,
            function: Fn {
                is_comp_provided: true,
                name: FnId { name, loc: None },
                params,
                return_ty,
                body: Block {
                    stmts: vec![],
                    loc: None,
                },
            },
        }
    }
}

// =====================
// ---- Expressions ----
// =====================

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnOp {
    Not,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BinOp {
    // Logical operators
    And,
    Or,

    // Relational operators
    EQ,
    NE,
    GE,
    GT,
    LE,
    LT,

    // Highest precedence arithmetic operators
    Add,
    Subtract,

    // Next highest precedence arithmetic operators
    Multiply,
    Divide,
    Modulo,
}

// =================
// ==== Visitor ====
// =================

// TODO add a default visit implementation
// (take a look at the behavior tree visit trait) that would be good to add to
// the AST visitor as well to make the visit ordering/conventions less annoying.
/// The lifetime parameter 'a is used primarily in the `behavior/builder_visitor.rs`
/// in order to enable saving off data in the `Whamm` struct while it is being visited
/// in some other data structure.
/// The lifetime is necessary to mark where the pointers are actually pointing to!
pub trait WhammVisitor<'a, T> {
    fn visit_whamm(&mut self, whamm: &'a Whamm) -> T;
    fn visit_script(&mut self, script: &'a Script) -> T;
    fn visit_provider(&mut self, provider: &'a Box<dyn Provider>) -> T;
    fn visit_package(&mut self, package: &'a dyn Package) -> T;
    fn visit_event(&mut self, event: &'a dyn Event) -> T;
    fn visit_probe(&mut self, probe: &'a Box<dyn Probe>) -> T;
    // fn visit_predicate(&mut self, predicate: &Expr) -> T;
    fn visit_fn(&mut self, f: &'a Fn) -> T;
    fn visit_formal_param(&mut self, param: &'a (Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &'a Block) -> T;
    fn visit_stmt(&mut self, stmt: &'a Statement) -> T;
    fn visit_expr(&mut self, expr: &'a Expr) -> T;
    fn visit_unop(&mut self, unop: &'a UnOp) -> T;
    fn visit_binop(&mut self, binop: &'a BinOp) -> T;
    fn visit_datatype(&mut self, datatype: &'a DataType) -> T;
    fn visit_value(&mut self, val: &'a Value) -> T;
}

/// To support setting constant-provided global vars
pub trait WhammVisitorMut<T> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> T;
    fn visit_script(&mut self, script: &mut Script) -> T;
    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) -> T;
    fn visit_package(&mut self, package: &mut dyn Package) -> T;
    fn visit_event(&mut self, event: &mut dyn Event) -> T;
    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> T;
    // fn visit_predicate(&mut self, predicate: &mut Expr) -> T;
    fn visit_fn(&mut self, f: &mut Fn) -> T;
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_stmt(&mut self, stmt: &mut Statement) -> T;
    fn visit_expr(&mut self, expr: &mut Expr) -> T;
    fn visit_unop(&mut self, unop: &mut UnOp) -> T;
    fn visit_binop(&mut self, op: &mut BinOp) -> T;
    fn visit_datatype(&mut self, datatype: &mut DataType) -> T;
    fn visit_value(&mut self, val: &mut Value) -> T;
}

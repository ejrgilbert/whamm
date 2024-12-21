#![allow(clippy::borrowed_box)]

use pest::error::LineColLocation;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use termcolor::{Buffer, ColorChoice, WriteColor};

use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{green, grey_italics, long_line, magenta, white, yellow};
use crate::parser::rules::{
    print_provider_docs, provider_factory, Event, Package, Probe, Provider, WhammProvider,
};
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use termcolor::BufferWriter;

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

#[derive(Clone, Debug, Eq)]
pub enum DataType {
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
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
impl Hash for DataType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // use any distinct number as an enum variant identifier
        match self {
            DataType::U32 => {
                state.write_u8(1);
            }
            DataType::I32 => {
                state.write_u8(2);
            }
            DataType::F32 => {
                state.write_u8(3);
            }
            DataType::U64 => {
                state.write_u8(4);
            }
            DataType::I64 => {
                state.write_u8(5);
            }
            DataType::F64 => {
                state.write_u8(6);
            }
            DataType::Boolean => {
                state.write_u8(7);
            }
            DataType::Null => {
                state.write_u8(8);
            }
            DataType::Str => {
                state.write_u8(9);
            }
            DataType::Tuple { ty_info } => {
                for ty in ty_info {
                    state.write_u8(10);
                    ty.hash(state);
                }
            }
            DataType::Map { key_ty, val_ty } => {
                state.write_u8(11);
                key_ty.hash(state);
                val_ty.hash(state);
            }
            DataType::AssumeGood => {
                state.write_u8(12);
            }
        }
    }
}
impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::U32, DataType::U32)
            | (DataType::I32, DataType::I32)
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
impl DataType {
    pub fn id(&self) -> i32 {
        match self {
            DataType::U32 => {
                1
            }
            DataType::I32 => {
                2
            }
            DataType::F32 => {
                3
            }
            DataType::U64 => {
                4
            }
            DataType::I64 => {
                5
            }
            DataType::F64 => {
                6
            }
            DataType::Boolean => {
                7
            }
            DataType::Null => {
                8
            }
            DataType::Str => {
                9
            }
            DataType::Tuple { .. } => {
                10
            }
            DataType::Map { .. } => {
                11
            }
            DataType::AssumeGood => {
                12
            }
        }
    }
    pub fn num_bytes(&self) -> Option<usize> {
        match self {
            DataType::U32 |
            DataType::I32 |
            DataType::F32 |
            DataType::Boolean |
            // We save the map ID as u32!
            DataType::Map { .. } => Some(4),
            DataType::U64 |
            DataType::I64 |
            DataType::F64 => Some(8),
            DataType::Tuple { ty_info } => {
                let mut size = 0;
                for ty in ty_info.iter() {
                    size += ty.num_bytes().unwrap_or_default();
                }
                Some(size)
            }
            DataType::Str |
            DataType::Null |
            DataType::AssumeGood => {
                // TODO -- is this okay for AssumeGood?
                // size should be determined respective to the context!
                None
            }
        }
    }

    pub fn print(&self, buffer: &mut Buffer) {
        match self {
            DataType::U32 => {
                yellow(true, "u32".to_string(), buffer);
            }
            DataType::I32 => {
                yellow(true, "i32".to_string(), buffer);
            }
            DataType::F32 => {
                yellow(true, "f32".to_string(), buffer);
            }
            DataType::U64 => {
                yellow(true, "u64".to_string(), buffer);
            }
            DataType::I64 => {
                yellow(true, "i64".to_string(), buffer);
            }
            DataType::F64 => {
                yellow(true, "f64".to_string(), buffer);
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
                yellow(true, "unknown, not type checked".to_string(), buffer);
            }
        }
    }
}

// Values
#[derive(Clone, Debug)]
pub enum Value {
    U32 {
        ty: DataType,
        val: u32,
    },
    I32 {
        ty: DataType,
        val: i32,
    },
    F32 {
        ty: DataType,
        val: f32,
    },
    U64 {
        ty: DataType,
        val: u64,
    },
    I64 {
        ty: DataType,
        val: i64,
    },
    F64 {
        ty: DataType,
        val: f64,
    },
    Boolean {
        ty: DataType,
        val: bool,
    },
    Str {
        ty: DataType,
        val: String,
    },
    Tuple {
        ty: DataType,
        vals: Vec<Expr>,
    },
    U32U32Map {
        ty: DataType,
        val: Box<HashMap<u32, u32>>,
    },
}

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub return_ty: Option<DataType>,
    pub loc: Option<Location>,
}
impl Block {
    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty()
    }
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
    // all report variables must be unshared,
    // but not all unshared variables must be reported
    UnsharedDecl {
        is_report: bool,
        decl: Box<Statement>,
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
            | Statement::UnsharedDecl { loc, .. }
            | Statement::Expr { loc, .. } => loc,
        }
    }
    pub fn line_col(&self) -> Option<LineColLocation> {
        self.loc().as_ref().map(|loc| loc.line_col.clone())
    }
    pub fn dummy() -> Self {
        Self::Expr {
            expr: Expr::Primitive {
                val: Value::I32 {
                    ty: DataType::I32,
                    val: 0,
                },
                loc: None,
            },
            loc: None,
        }
    }
}

#[derive(Clone, Debug)]
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
        ty: DataType, // populated by the type-checker (for knowing the return_ty of the emitted blocks)
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
        args: Vec<Expr>,
        loc: Option<Location>,
    },
    VarId {
        definition: Definition,
        name: String,
        loc: Option<Location>,
    },
    Primitive {
        // Type is val.ty
        val: Value,
        loc: Option<Location>,
    },
    MapGet {
        map: Box<Expr>, //This should be a VarId
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
            | Expr::MapGet { loc, .. }
            | Expr::Primitive { loc, .. } => loc,
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
    pub(crate) def: Definition,
    pub(crate) name: FnId,
    pub(crate) params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) return_ty: DataType,
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

        white(true, " -> ".to_string(), buffer);
        self.return_ty.print(buffer);
    }

    pub fn is_static(&self) -> bool {
        matches!(self.def, Definition::CompilerStatic)
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self.def, Definition::CompilerDynamic)
    }

    pub fn is_from_user(&self) -> bool {
        matches!(self.def, Definition::User)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Definition {
    User,
    CompilerStatic,
    CompilerDynamic,
}
impl Definition {
    pub fn is_comp_provided(&self) -> bool {
        matches!(self, Definition::CompilerStatic) || matches!(self, Definition::CompilerDynamic)
    }
}

#[derive(Clone, Debug)]
pub struct Global {
    pub def: Definition,
    pub report: bool,
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

    pub fn is_static(&self) -> bool {
        matches!(self.def, Definition::CompilerStatic)
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self.def, Definition::CompilerDynamic)
    }

    pub fn is_from_user(&self) -> bool {
        matches!(self.def, Definition::User)
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
        Whamm {
            provided_probes: HashMap::new(),
            fns: Whamm::get_provided_fns(),
            globals: Whamm::get_provided_globals(),

            scripts: vec![],
        }
    }

    fn get_provided_fns() -> Vec<ProvidedFunction> {
        let strcmp_params = vec![
            (
                Expr::VarId {
                    definition: Definition::CompilerStatic,
                    name: "str_addr".to_string(),
                    loc: None,
                },
                DataType::Tuple {
                    ty_info: vec![Box::new(DataType::I32), Box::new(DataType::I32)],
                },
            ),
            (
                Expr::VarId {
                    definition: Definition::CompilerStatic,
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
            DataType::Boolean,
            false,
        );

        vec![strcmp]
    }

    fn get_provided_globals() -> HashMap<String, ProvidedGlobal> {
        HashMap::new()
    }

    pub fn add_script(&mut self, mut script: Script) -> usize {
        let id = self.scripts.len();
        script.id = self.scripts.len() as u8;
        self.scripts.push(script);

        id
    }
}

/// RulePart are the probe ids in a probe rule
#[derive(Clone, Debug)]
pub struct RulePart {
    pub name: String,
    pub loc: Option<Location>,
}

#[derive(Clone, Debug)]
pub struct ProbeRule {
    pub provider: Option<RulePart>,
    pub package: Option<RulePart>,
    pub event: Option<RulePart>,
    pub mode: Option<RulePart>,
}
impl Default for ProbeRule {
    fn default() -> Self {
        Self::new()
    }
}
impl ProbeRule {
    pub fn new() -> Self {
        Self {
            provider: None,
            package: None,
            event: None,
            mode: None,
        }
    }
    pub fn full_name(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            &self.provider.as_ref().unwrap().name,
            &self.package.as_ref().unwrap().name,
            &self.event.as_ref().unwrap().name,
            &self.mode.as_ref().unwrap().name
        )
    }
    pub fn add_rule_def(&mut self, part: RulePart) {
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
                "    {}:{}:{}:",
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
        grey_italics(
            true,
            "    matches the following modes for the parent event:\n\n".to_string(),
            buffer,
        );
    }
}

pub struct Script {
    pub id: u8,
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
            id: u8::MAX,
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new(),
            global_stmts: vec![],
        }
    }

    pub fn print_info(
        &mut self,
        probe_rule: &ProbeRule,
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
        let (matched_providers, matched_packages, matched_events, _matched_modes) =
            provider_factory::<WhammProvider>(&mut providers, probe_rule, None, None, None, true)?;

        // Print the matched provider information
        if matched_providers {
            probe_rule.print_bold_provider(&mut buffer);
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
            probe_rule.print_bold_package(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            provider.print_package_docs(print_globals, print_functions, &mut tabs, &mut buffer);
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // Print the matched event information
        if matched_events {
            probe_rule.print_bold_event(&mut buffer);
        }
        for (.., provider) in providers.iter() {
            provider.print_event_and_mode_docs(
                probe_rule,
                print_globals,
                print_functions,
                &mut tabs,
                &mut buffer,
            );
        }
        long_line(&mut buffer);
        white(true, "\n\n".to_string(), &mut buffer);

        // // Print the matched mode information
        // if matched_modes {
        //     probe_rule.print_bold_mode(&mut buffer);
        // }
        // for (.., provider) in providers.iter() {
        //     provider.print_mode_docs(print_globals, print_functions, &mut tabs, &mut buffer);
        // }
        // long_line(&mut buffer);
        // white(true, "\n\n".to_string(), &mut buffer);

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
        probe_rule: &ProbeRule,
        predicate: Option<Expr>,
        body: Option<Block>,
    ) -> Result<(), Box<WhammError>> {
        let (matched_providers, matched_packages, matched_events, matched_modes): (
            bool,
            bool,
            bool,
            bool,
        ) = provider_factory::<WhammProvider>(
            &mut self.providers,
            probe_rule,
            None,
            predicate,
            body,
            false,
        )?;

        if !matched_providers {
            return if let Some(prov_patt) = &probe_rule.provider {
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
            return if let Some(prov_patt) = &probe_rule.package {
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
            return if let Some(prov_patt) = &probe_rule.event {
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
            return if let Some(prov_patt) = &probe_rule.mode {
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
#[derive(Clone, Debug)]
pub struct ProvidedGlobal {
    pub name: String,
    pub docs: String,
    pub global: Global,
}
impl ProvidedGlobal {
    pub fn new(name: String, docs: String, ty: DataType, is_static: bool) -> Self {
        Self {
            name: name.clone(),
            docs,
            global: Global {
                def: if is_static {
                    Definition::CompilerStatic
                } else {
                    Definition::CompilerDynamic
                },
                report: false,
                ty,
                var_name: Expr::VarId {
                    definition: if is_static {
                        Definition::CompilerStatic
                    } else {
                        Definition::CompilerDynamic
                    },
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
        return_ty: DataType,
        is_static: bool,
    ) -> Self {
        Self {
            name: name.clone(),
            docs,
            function: Fn {
                def: if is_static {
                    Definition::CompilerStatic
                } else {
                    Definition::CompilerDynamic
                },
                name: FnId { name, loc: None },
                params,
                return_ty,
                body: Block {
                    stmts: vec![],
                    return_ty: None,
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
pub trait WhammVisitor<T> {
    fn visit_whamm(&mut self, whamm: &Whamm) -> T;
    fn visit_script(&mut self, script: &Script) -> T;
    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> T;
    fn visit_package(&mut self, package: &dyn Package) -> T;
    fn visit_event(&mut self, event: &dyn Event) -> T;
    fn visit_probe(&mut self, probe: &Box<dyn Probe>) -> T;
    // fn visit_predicate(&mut self, predicate: &Expr) -> T;
    fn visit_fn(&mut self, f: &Fn) -> T;
    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_stmt(&mut self, stmt: &Statement) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_unop(&mut self, unop: &UnOp) -> T;
    fn visit_binop(&mut self, binop: &BinOp) -> T;
    fn visit_datatype(&mut self, datatype: &DataType) -> T;
    fn visit_value(&mut self, val: &Value) -> T;
}

/// To support setting constant-provided global vars
pub trait WhammVisitorMut<T> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> T;
    fn visit_script(&mut self, script: &mut Script) -> T;
    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) -> T;
    fn visit_package(&mut self, package: &mut dyn Package) -> T;
    fn visit_event(&mut self, event: &mut dyn Event) -> T;
    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> T;
    fn visit_fn(&mut self, f: &mut Fn) -> T;
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &mut Block) -> T;
    fn visit_stmt(&mut self, stmt: &mut Statement) -> T;
    fn visit_expr(&mut self, expr: &mut Expr) -> T;
    fn visit_unop(&mut self, unop: &mut UnOp) -> T;
    fn visit_binop(&mut self, op: &mut BinOp) -> T;
    fn visit_datatype(&mut self, datatype: &mut DataType) -> T;
    fn visit_value(&mut self, val: &mut Value) -> T;
}

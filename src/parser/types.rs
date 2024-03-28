use std::any::Any;
use std::cmp;
use std::collections::HashMap;
use glob::Pattern;
use pest::iterators::Pair;

use pest_derive::Parser;
use pest::pratt_parser::PrattParser;

use log::{trace};

#[derive(Parser)]
#[grammar = "./parser/dtrace.pest"] // Path relative to base `src` dir
pub struct DtraceParser;

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
    };
}

// ============================
// ===== Helper Functions =====
// ============================

const NL: &str = "\n";

fn increase_indent(i: &mut i32) {
    *i += 1;
}

fn decrease_indent(i: &mut i32) {
    *i -= 1;
}

fn get_indent(i: &mut i32) -> String {
    "--".repeat(cmp::max(0, *i as usize))
}

// ===============
// ==== Types ====
// ===============

pub enum DataType {
    Integer,
    Boolean,
    Null,
    Str,
    Tuple
}
impl DataType {
    fn as_str(&self) -> String {
        match self {
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
            DataType::Tuple => {
                "tuple".to_string()
            },
        }
    }
}

// Values
pub trait Value {
    fn as_str(&self, indent: &mut i32) -> String;
}
pub struct Integer {
    pub ty: DataType,
    pub val: i32,
}
impl Expression for Integer {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_str(&self, _indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("{}", self.val);
        s
    }
}
impl Value for Integer {
    fn as_str(&self, _indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("{}", self.val);
        s
    }
}
impl Integer {
    pub fn new(val: i32) -> Self {
        Integer {
            ty: DataType::Integer,
            val
        }
    }
}

pub struct Str {
    ty: DataType,
    val: String,
}
impl Expression for Str {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_str(&self, _indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("\"{}\"", self.val);
        s
    }
}
impl Value for Str {
    fn as_str(&self, _indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("\"{}\"", self.val);
        s
    }
}
impl Str {
    pub fn new(val: String) -> Self {
        Str {
            ty: DataType::Str,
            val
        }
    }
}

pub struct Tuple {
    ty: DataType,
    val: Vec<Box<dyn Expression>>,
}
impl Expression for Tuple {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("(");
        for v in self.val.iter() {
            s += &format!("{}, ", (*v).as_str(indent));
        }
        s += &format!(")");
        s
    }
}
impl Value for Tuple {
    fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("(");
        for v in self.val.iter() {
            s += &format!("{}, ", (*v).as_str(indent));
        }
        s += &format!(")");
        s
    }
}
impl Tuple {
    pub fn new(val: Vec<Box<dyn Expression>>) -> Self {
        Tuple {
            ty: DataType::Tuple,
            val
        }
    }
}

// IDs
trait ID {}
pub struct VarId {
    name: String,
}
impl Expression for VarId {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_str(&self, _indent: &mut i32) -> String {
        format!("{}", self.name)
    }
}
impl ID for VarId {}
impl VarId {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        trace!("Entering ID");
        trace!("Exiting ID");
        VarId {
            name: pair.as_str().parse().unwrap()
        }
    }
}

// struct ProbeId {
//     name: String,
// }
// impl ID for ProbeId {}
// impl ProbeId {
//     pub fn from_pair(pair: Pair<Rule>) -> Self {
//         trace!("Entering PROBE_ID");
//         let name: String = pair.as_str().parse().unwrap();
//
//         trace!("Exiting PROBE_ID");
//         ProbeId {
//             name
//         }
//     }
//
//     fn as_str(&self, _indent: &i32) -> String {
//         format!("{}", self.name)
//     }
// }

// Statements
pub trait Statement {
    fn as_str(&self, indent: &mut i32) -> String;
}
pub struct Assign {
    pub var_id: VarId,
    pub expr: Box<dyn Expression>
}
impl Statement for Assign {
    fn as_str(&self, indent: &mut i32) -> String {
        format!("{} = {}", self.var_id.as_str(indent), self.expr.as_str(indent))
    }
}

pub struct Call {
    pub fn_target: VarId,
    pub args: Option<Vec<Box<dyn Expression>>>
}
// This can be in the context of an expression OR a complete statement!
impl Expression for Call {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("{}(", &self.fn_target.as_str(indent));
        match &self.args {
            Some(args) => {
                for arg in args {
                    s += &format!("{}, ", (*arg).as_str(indent));
                }
            },
            _ => {}
        }
        s += &format!(")");
        s
    }
}
impl Statement for Call {
    fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("{}(", &self.fn_target.as_str(indent));
        match &self.args {
            Some(args) => {
                for arg in args {
                    s += &format!("{}, ", (*arg).as_str(indent));
                }
            },
            _ => {}
        }
        s += &format!(")");
        s
    }
}

// Expressions
pub trait Expression {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_str(&self, indent: &mut i32) -> String;
}
pub struct BinOp {
    pub lhs: Box<dyn Expression>,
    pub op: Op,
    pub rhs: Box<dyn Expression>,
}
impl Expression for BinOp {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();
        s += &format!("{} {} {}",
            self.lhs.as_str(indent),
            self.op.as_str(),
            self.rhs.as_str(indent)
        );
        s
    }
}

// Functions
pub struct Fn {
    name: String,
    params: Option<Vec<DataType>>,
    return_ty: Option<DataType>,
    body: Option<Vec<Box<dyn Statement>>>
}
impl Fn {
    pub fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();

        // print name
        s += &format!("{} {} (", get_indent(indent), &self.name);

        // print params
        match &self.params {
            Some(ps) => {
                for p in ps.iter() {
                    s += &format!("{}, ", p.as_str());
                }
            },
            _ => {}
        }
        s += &format!(")");

        // print return type
        match &self.return_ty {
            Some(ty) => {
                s += &format!(" -> {}", ty.as_str());
            },
            _ => {}
        }
        s += &format!(" {{{NL}");

        // print body
        increase_indent(indent);
        match &self.body {
            Some(stmts) => {
                for stmt in stmts.iter() {
                    s += &format!("{}{}{NL}", get_indent(indent),( **stmt).as_str(indent));
                }
            },
            _ => {}
        }
        decrease_indent(indent);
        s += &format!("{} }}{NL}", get_indent(indent));

        s
    }
}

pub struct Dtrace {
    pub provided_probes: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
    fns: Vec<Fn>,                                    // Comp-provided
    globals: HashMap<VarId, Option<Box<dyn Value>>>, // Comp-provided

    pub dscripts: Vec<Dscript>
}
impl Dtrace {
    pub fn new() -> Self {
        let strcmp_fn = Fn {
            name: "strcmp".to_string(),
            params: Some(vec![
                DataType::Tuple,
                DataType::Str
            ]),
            return_ty: Some(DataType::Boolean),
            body: None
        };

        let mut dtrace = Dtrace {
            provided_probes: HashMap::new(),
            fns: vec![ strcmp_fn ],
            globals: HashMap::new(),

            dscripts: vec![]
        };
        dtrace.init_provided_probes();
        dtrace
    }

    fn init_provided_probes(&mut self) {
        // A giant data structure to encode the available `providers->modules->functions->probe_types`
        self.init_core_probes();
        self.init_wasm_probes();
    }

    fn init_core_probes(&mut self) {
        // Not really any modules or functions for a core probe...just two types!
        self.provided_probes.insert("core".to_string(), HashMap::from([
            ("".to_string(), HashMap::from([
                ("".to_string(), vec![
                    "begin".to_string(),
                    "end".to_string()
                ])
            ]))
        ]));
    }

    fn init_wasm_probes(&mut self) {
        // This list of functions matches up with bytecodes supported by Walrus.
        // See: https://docs.rs/walrus/latest/walrus/ir/
        let wasm_bytecode_functions = vec![
            "Block".to_string(),
            "Loop".to_string(),
            "Call".to_string(),
            "CallIndirect".to_string(),
            "LocalGet".to_string(),
            "LocalSet".to_string(),
            "LocalTee".to_string(),
            "GlobalGet".to_string(),
            "GlobalSet".to_string(),
            "Const".to_string(),
            "Binop".to_string(),
            "Unop".to_string(),
            "Select".to_string(),
            "Unreachable".to_string(),
            "Br".to_string(),
            "BrIf".to_string(),
            "IfElse".to_string(),
            "BrTable".to_string(),
            "Drop".to_string(),
            "Return".to_string(),
            "MemorySize".to_string(),
            "MemoryGrow".to_string(),
            "MemoryInit".to_string(),
            "DataDrop".to_string(),
            "MemoryCopy".to_string(),
            "MemoryFill".to_string(),
            "Load".to_string(),
            "Store".to_string(),
            "AtomicRmw".to_string(),
            "Cmpxchg".to_string(),
            "AtomicNotify".to_string(),
            "AtomicWait".to_string(),
            "AtomicFence".to_string(),
            "TableGet".to_string(),
            "TableSet".to_string(),
            "TableGrow".to_string(),
            "TableSize".to_string(),
            "TableFill".to_string(),
            "RefNull".to_string(),
            "RefIsNull".to_string(),
            "RefFunc".to_string(),
            "V128Bitselect".to_string(),
            "I8x16Swizzle".to_string(),
            "I8x16Shuffle".to_string(),
            "LoadSimd".to_string(),
            "TableInit".to_string(),
            "ElemDrop".to_string(),
            "TableCopy".to_string()
        ];
        let wasm_bytecode_probe_types = vec![
            "before".to_string(),
            "after".to_string(),
            "alt".to_string()
        ];
        let mut wasm_bytecode_map = HashMap::new();

        // Build out the wasm_bytecode_map
        for function in wasm_bytecode_functions {
            wasm_bytecode_map.insert(function, wasm_bytecode_probe_types.clone());
        }

        self.provided_probes.insert("wasm".to_string(), HashMap::from([
            ("bytecode".to_string(), wasm_bytecode_map)
        ]));
    }

    pub fn as_str(&self) -> String {
        let mut indent = 0;
        let mut s = "".to_string();

        // print fns
        if self.fns.len() > 0 {
            s += &format!("Dtrace functions:{NL}");
            increase_indent(&mut indent);
            for f in self.fns.iter() {
                s += &format!("{}{NL}", f.as_str(&mut indent));
            }
            decrease_indent(&mut indent);
        }

        // print globals
        if self.globals.len() > 0 {
            s += &format!("Dtrace globals:{NL}");
            increase_indent(&mut indent);
            for (var_id, val) in self.globals.iter() {
                s += &format!("{}{} := ", get_indent(&mut indent), var_id.as_str(&mut indent));
                match val {
                    Some(v) => s += &format!("{}{NL}", (**v).as_str(&mut indent)),
                    None => s += &format!("None{NL}")
                }
            }
            decrease_indent(&mut indent);
        }

        s += &format!("Dtrace dscripts:{NL}");
        increase_indent(&mut indent);
        for (i, dscript) in self.dscripts.iter().enumerate() {
            s += &format!("{} `dscript{i}`:{NL}", get_indent(&mut indent));
            increase_indent(&mut indent);
            s += &format!("{}", dscript.as_str(&mut indent));
            decrease_indent(&mut indent);
        }
        decrease_indent(&mut indent);

        s
    }

    pub fn add_dscript(&mut self, dscript: Dscript) {
        self.dscripts.push(dscript);
    }
}

pub struct Dscript {
    /// The providers of the probes that have been used in the Dscript.
    /// TODO -- how to validate that these providers are available?
    pub providers: HashMap<String, Provider>,
    pub fns: Vec<Fn>,                               // User-provided
    pub globals: HashMap<VarId, Option<Box<dyn Value>>>, // User-provided

    /// The probes that have been used in the Dscript.
    /// This keeps us from having to keep multiple copies of probes across probe specs matched by
    ///     user specified glob pattern.
    /// These will be the probes available for this Function. TODO -- how to validate this?
    pub probes: Vec<Probe>,
}
impl Dscript {
    pub fn new() -> Self {
        Dscript {
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new(),
            probes: vec![],
        }
    }

    pub fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();

        // print fns
        if self.fns.len() > 0 {
            s += &format!("{} dscript functions:{NL}", get_indent(indent));
            increase_indent(indent);
            for f in self.fns.iter() {
                s += &format!("{}{}{NL}", get_indent(indent), f.as_str(indent));
            }
            decrease_indent(indent);
        }

        // print globals
        if self.globals.len() > 0 {
            s += &format!("{} dscript globals:{NL}", get_indent(indent));
            increase_indent(indent);
            for (var_id, val) in self.globals.iter() {
                s += &format!("{}{} := ", get_indent(indent), var_id.as_str(indent));
                match val {
                    Some(v) => s += &format!("{}{NL}", (**v).as_str(indent)),
                    None => s += &format!("None{NL}")
                }
            }
            decrease_indent(indent);
        }

        // print providers
        s += &format!("{} dscript providers:{NL}", get_indent(indent));
        for (name, provider) in self.providers.iter() {
            increase_indent(indent);
            s += &format!("{} `{name}` {{{NL}", get_indent(indent));

            increase_indent(indent);
            s += &format!("{}", provider.as_str(indent));
            decrease_indent(indent);

            s += &format!("{} }}{NL}", get_indent(indent));
            decrease_indent(indent);
        }

        // print probes
        s += &format!("{} dscript probes:{NL}", get_indent(indent));
        increase_indent(indent);
        for probe in self.probes.iter() {
            s += &format!("{}", probe.as_str(indent));
        }
        decrease_indent(indent);

        s
    }

    /// Iterates over all of the matched providers, modules, functions, and probe names
    /// to add a copy of the user-defined Probe for each of them.
    pub fn add_probe(&mut self, provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
                     prov_patt: &str, mod_patt: &str, func_patt: &str, nm_patt: &str,
                     predicate: Option<Box<dyn Expression>>, body: Option<Vec<Box<dyn Statement>>>) {
        // Add new probe to dscript
        let idx = self.probes.len();
        self.probes.push(Probe {
            name: nm_patt.to_string(),
            fns: vec![],
            globals: Default::default(),
            predicate,
            body,
        });

        for provider_str in Provider::get_matches(provided_probes, prov_patt).iter() {
            // Does provider exist yet?
            let provider = match self.providers.get_mut(provider_str) {
                Some(prov) => prov,
                None => {
                    // add the provider!
                    let new_prov = Provider::new(provider_str.to_lowercase().to_string());
                    self.providers.insert(provider_str.to_lowercase().to_string(), new_prov);
                    self.providers.get_mut(&provider_str.to_lowercase()).unwrap()
                }
            };
            for module_str in Module::get_matches(provided_probes,provider_str, mod_patt).iter() {
                // Does module exist yet?
                let module = match provider.modules.get_mut(module_str) {
                    Some(m) => m,
                    None => {
                        // add the module!
                        let new_mod = Module::new(module_str.to_lowercase().to_string());
                        provider.modules.insert(module_str.to_lowercase().to_string(), new_mod);
                        provider.modules.get_mut(&module_str.to_lowercase()).unwrap()
                    }
                };
                for function_str in Function::get_matches(provided_probes, provider_str, module_str, func_patt).iter() {
                    // Does function exist yet?
                    let function = match module.functions.get_mut(function_str) {
                        Some(f) => f,
                        None => {
                            // add the module!
                            let new_fn = Function::new(function_str.to_lowercase().to_string());
                            module.functions.insert(function_str.to_lowercase().to_string(), new_fn);
                            module.functions.get_mut(&function_str.to_lowercase()).unwrap()
                        }
                    };
                    for name_str in Probe::get_matches(provided_probes, provider_str, module_str, function_str, nm_patt).iter() {
                        function.insert_probe(name_str.to_string(), idx);
                    }
                }
            }
        }
    }
}

pub struct Provider {
    pub name: String,
    pub fns: Vec<Fn>,                               // Comp-provided
    pub globals: HashMap<VarId, Option<Box<dyn Value>>>, // Comp-provided

    /// The modules of the probes that have been used in the Dscript.
    /// These will be sub-modules of this Provider. TODO -- how to validate this?
    pub modules: HashMap<String, Module>
}
impl Provider {
    pub fn new(name: String) -> Self {
        Provider {
            name,
            fns: vec![],
            globals: HashMap::new(),
            modules: HashMap::new()
        }
    }

    pub fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();

        // print fns
        if self.fns.len() > 0 {
            s += &format!("{} functions:{NL}", get_indent(indent));
            increase_indent(indent);
            for f in self.fns.iter() {
                s += &format!("{}{}{NL}", get_indent(indent), f.as_str(indent));
            }
            decrease_indent(indent);
        }

        // print globals
        if self.globals.len() > 0 {
            s += &format!("{} globals:{NL}", get_indent(indent));
            increase_indent(indent);
            for (var_id, val) in self.globals.iter() {
                s += &format!("{}{} := ", get_indent(indent), var_id.as_str(indent));
                match val {
                    Some(v) => s += &format!("{}{NL}", (**v).as_str(indent)),
                    None => s += &format!("None{NL}")
                }
            }
            decrease_indent(indent);
        }

        // print modules
        if self.modules.len() > 0 {
            s += &format!("{} modules:{NL}", get_indent(indent));
            for (name, module) in self.modules.iter() {
                increase_indent(indent);
                s += &format!("{} `{name}` {{{NL}", get_indent(indent));

                increase_indent(indent);
                s += &format!("{}", module.as_str(indent));
                decrease_indent(indent);

                s += &format!("{} }}{NL}", get_indent(indent));
                decrease_indent(indent);
            }
        }

        s
    }

    /// Get the provider names that match the passed glob pattern
    pub fn get_matches(provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, prov_patt: &str) -> Vec<String> {
        let glob = Pattern::new(&prov_patt.to_lowercase()).unwrap();

        let mut matches = vec![];
        for (provider_name, _provider) in provided_probes.into_iter() {
            if glob.matches(&provider_name.to_lowercase()) {
                matches.push(provider_name.clone());
            }
        }

        matches
    }
}

pub struct Module {
    pub name: String,
    pub fns: Vec<Fn>,                               // Comp-provided
    pub globals: HashMap<VarId, Option<Box<dyn Value>>>, // Comp-provided

    /// The functions of the probes that have been used in the Dscript.
    /// These will be sub-functions of this Module. TODO -- how to validate this?
    pub functions: HashMap<String, Function>
}
impl Module {
    pub fn new(name: String) -> Self {
        Module {
            name,
            fns: vec![],
            globals: HashMap::new(),
            functions: HashMap::new()
        }
    }

    pub fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();

        // print fns
        if self.fns.len() > 0 {
            s += &format!("{} module fns:{NL}", get_indent(indent));
            increase_indent(indent);
            for f in self.fns.iter() {
                s += &format!("{}{}{NL}", get_indent(indent), f.as_str(indent));
            }
            decrease_indent(indent);
        }

        // print globals
        if self.globals.len() > 0 {
            s += &format!("{} module globals:{NL}", get_indent(indent));
            increase_indent(indent);
            for (var_id, val) in self.globals.iter() {
                s += &format!("{}{} := ", get_indent(indent), var_id.as_str(indent));
                match val {
                    Some(v) => s += &format!("{}{NL}", (**v).as_str(indent)),
                    None => s += &format!("None{NL}")
                }
            }
            decrease_indent(indent);
        }

        // print functions
        s += &format!("{} module functions:{NL}", get_indent(indent));
        for (name, function) in self.functions.iter() {
            increase_indent(indent);
            s += &format!("{} `{name}` {{{NL}", get_indent(indent));

            increase_indent(indent);
            s += &format!("{}", function.as_str(indent));
            decrease_indent(indent);

            s += &format!("{} }}{NL}", get_indent(indent));
            decrease_indent(indent);
        }

        s
    }

    /// Get the Module names that match the passed glob pattern
    pub fn get_matches(provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, provider: &str, mod_patt: &str) -> Vec<String> {
        let glob = Pattern::new(&mod_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for (mod_name, _module) in provided_probes.get(provider).unwrap().into_iter() {
            if glob.matches(&mod_name.to_lowercase()) {
                matches.push(mod_name.clone());
            }
        }

        matches
    }
}

pub struct Function {
    pub name: String,
    pub fns: Vec<Fn>,                                    // Comp-provided
    pub globals: HashMap<VarId, Option<Box<dyn Value>>>, // Comp-provided
    /// Mapping from probe type to list of indices (into `probes` in dscript above) of the probes tied to that type
    pub probe_map: HashMap<String, Vec<usize>>
}
impl Function {
    pub fn new(name: String) -> Self {
        Function {
            name,
            fns: vec![],
            globals: HashMap::new(),
            probe_map: HashMap::new()
        }
    }

    pub fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();

        // print fns
        if self.fns.len() > 0 {
            s += &format!("{} function fns:{NL}", get_indent(indent));
            increase_indent(indent);
            for f in self.fns.iter() {
                s += &format!("{}{}{NL}", get_indent(indent), f.as_str(indent));
            }
            decrease_indent(indent);
        }

        // print globals
        if self.globals.len() > 0 {
            s += &format!("{} function globals:{NL}", get_indent(indent));
            increase_indent(indent);
            for (var_id, val) in self.globals.iter() {
                s += &format!("{}{} := ", get_indent(indent), var_id.as_str(indent));
                match val {
                    Some(v) => s += &format!("{}{NL}", (**v).as_str(indent)),
                    None => s += &format!("None{NL}")
                }
            }
            decrease_indent(indent);
        }

        // print functions
        if self.probe_map.len() > 0 {
            s += &format!("{} function probe_map:{NL}", get_indent(indent));
            for (name, probe_idxs) in self.probe_map.iter() {
                increase_indent(indent);
                s += &format!("{} {name}: ", get_indent(indent));

                s += &format!("(");
                for idx in probe_idxs {
                    s += &format!("{idx}, ");
                }
                s += &format!("){NL}");
                decrease_indent(indent);
            }
        }

        s
    }

    /// Get the Function names that match the passed glob pattern
    pub fn get_matches(provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, provider: &str, module: &str, func_patt: &str) -> Vec<String> {
        let glob = Pattern::new(&func_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for (fn_name, _module) in provided_probes.get(provider).unwrap().get(module).unwrap().into_iter() {
            if glob.matches(&fn_name.to_lowercase()) {
                matches.push(fn_name.clone());
            }
        }

        matches
    }

    pub fn insert_probe(&mut self, name: String, probe_idx: usize) {
        // Does name exist yet?
        match self.probe_map.get_mut(&name) {
            Some(probe_idxs) => {
                // Add index for this probe to list
                probe_idxs.push(probe_idx);
            },
            None => {
                self.probe_map.insert(name, vec![ probe_idx ]);
            }
        };
    }
}

pub struct Probe {
    pub name: String,
    pub fns: Vec<Fn>,                                    // Comp-provided
    pub globals: HashMap<VarId, Option<Box<dyn Value>>>, // Comp-provided

    pub predicate: Option<Box<dyn Expression>>,
    pub body: Option<Vec<Box<dyn Statement>>>
}
impl Probe {
    /// Get the Probe names that match the passed glob pattern
    pub fn get_matches(provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, provider: &str, module: &str, function: &str, probe_patt: &str) -> Vec<String> {
        let glob = Pattern::new(&probe_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for p_name in provided_probes.get(provider).unwrap().get(module).unwrap().get(function).unwrap().iter() {
            if glob.matches(&p_name.to_lowercase()) {
                matches.push(p_name.clone());
            }
        }

        matches
    }

    pub fn as_str(&self, indent: &mut i32) -> String {
        let mut s = "".to_string();

        s += &format!("{} `{}` probe {{{NL}", get_indent(indent), self.name);
        increase_indent(indent);

        // print fns
        if self.fns.len() > 0 {
            s += &format!("{} probe fns:{NL}", get_indent(indent));
            increase_indent(indent);
            for f in self.fns.iter() {
                s += &format!("{}{}{NL}", get_indent(indent), f.as_str(indent));
            }
            decrease_indent(indent);
        }

        // print globals
        if self.globals.len() > 0 {
            s += &format!("{} probe globals:{NL}", get_indent(indent));
            increase_indent(indent);
            for (var_id, val) in self.globals.iter() {
                s += &format!("{}{} := ", get_indent(indent), var_id.as_str(indent));
                match val {
                    Some(v) => s += &format!("{}{NL}", (**v).as_str(indent)),
                    None => s += &format!("None{NL}")
                }
            }
            decrease_indent(indent);
        }

        // print predicate
        s += &format!("{} `predicate`:{NL}", get_indent(indent));
        increase_indent(indent);
        match &self.predicate {
            Some(pred) => s += &format!("{} / {} /{NL}", get_indent(indent), (**pred).as_str(indent)),
            None => s += &format!("{} / None /{NL}", get_indent(indent))
        }
        decrease_indent(indent);

        // print body
        s += &format!("{} `body`:{NL}", get_indent(indent));
        increase_indent(indent);
        match &self.body {
            Some(b) => {
                for stmt in b {
                    s += &format!("{} {};{NL}", get_indent(indent), (**stmt).as_str(indent))
                }
            },
            None => s += &format!("{{}}")
        }
        decrease_indent(indent);

        decrease_indent(indent);
        s += &format!("{} }}{NL}", get_indent(indent));

        s
    }
}

// EOI because it's an easier workaround than hiding the dscript rule
pub struct EOI {}

// =====================
// ---- Expressions ----
// =====================

#[derive(Debug, Clone)]
pub enum Op {
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

impl Op {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
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
        }
    }
}

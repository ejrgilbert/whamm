use std::collections::HashMap;
use glob::Pattern;

use pest_derive::Parser;
use pest::pratt_parser::PrattParser;
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
    };
}

// ===============
// ==== Types ====
// ===============

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DataType {
    Integer,
    Boolean,
    Null,
    Str,
    Tuple {
        ty_info: Option<Vec<Box<DataType>>>
    }
}

// Values
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
        addr: Option<(DataId, u32, usize)>
    },
    Tuple {
        ty: DataType,
        vals: Vec<Expr>,
    },
    Boolean {
        ty: DataType,
        val: bool
    }
}


// Statements
#[derive(Clone)]
pub enum Statement {
    Assign {
        var_id: Expr, // Should be VarId
        expr: Expr
    },

    /// Standalone `Expr` statement, which means we can write programs like this:
    /// int main() {
    ///   2 + 2;
    ///   return 0;
    /// }
    Expr {
        expr: Expr
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    BinOp {     // Type is based on the outermost `op` (if arithmetic op, also based on types of lhs/rhs due to doubles)
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Call {      // Type is fn_target.return_ty, should be VarId
        fn_target: Box<Expr>,
        args: Option<Vec<Box<Expr>>>
    },
    VarId {
        // is_comp_provided: bool, // TODO -- do I need this?
        name: String
    },
    Primitive { // Type is val.ty
        val: Value
    }
}

// impl Expr {
//     pub fn ty(&self) {
//         match self {
//             Expr::BinOp {..} => {
//                 println!("BinOp");
//             },
//             Expr::Call {..} => {
//                 println!("Call");
//             },
//             Expr::VarId {..} => {
//                 println!("VarId");
//             },
//             Expr::Primitive {..} => {
//                 println!("Primitive");
//             }
//             _ => {}
//         }
//     }
// }

// Functions
pub struct Fn {
    pub(crate) is_comp_provided: bool,
    pub(crate) name: String,
    pub(crate) params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) return_ty: Option<DataType>,
    pub(crate) body: Option<Vec<Statement>>
}

pub struct Global {
    pub is_comp_provided: bool,

    pub ty: DataType,
    pub var_name: Expr, // Should be VarId
    pub value: Option<Value>
}

pub struct Whamm {
    pub provided_probes: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
    pub(crate) fns: Vec<Fn>,              // Comp-provided
    pub globals: HashMap<String, Global>, // Comp-provided

    pub whammys: Vec<Whammy>
}
impl Whamm {
    pub fn new() -> Self {
        let mut whamm = Whamm {
            provided_probes: HashMap::new(),
            fns: Whamm::get_provided_fns(),
            globals: Whamm::get_provided_globals(),

            whammys: vec![]
        };
        whamm.init_provided_probes();
        whamm
    }

    fn get_provided_fns() -> Vec<Fn> {
        let params = vec![
            (
                Expr::VarId {
                    name: "str_addr".to_string()
                },
                DataType::Tuple {
                    ty_info: Some(vec![
                        Box::new(DataType::Integer),
                        Box::new(DataType::Integer)
                    ]),
                }
            ),
            (
                Expr::VarId {
                    name: "value".to_string()
                },
                DataType::Str
            )
        ];
        let strcmp_fn = Fn {
            is_comp_provided: true,
            name: "strcmp".to_string(),
            params,
            return_ty: Some(DataType::Boolean),
            body: None
        };
        vec![ strcmp_fn ]
    }

    fn get_provided_globals() -> HashMap<String, Global> {
        HashMap::new()
    }

    fn init_provided_probes(&mut self) {
        // A giant data structure to encode the available `providers->packages->events->probe_types`
        self.init_core_probes();
        self.init_wasm_probes();
    }

    fn init_core_probes(&mut self) {
        // Not really any packages or events for a core probe...just two types!
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
        // This list of events matches up with bytecodes supported by Walrus.
        // See: https://docs.rs/walrus/latest/walrus/ir/
        let wasm_bytecode_events = vec![
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
        for event in wasm_bytecode_events {
            wasm_bytecode_map.insert(event, wasm_bytecode_probe_types.clone());
        }

        self.provided_probes.insert("wasm".to_string(), HashMap::from([
            ("bytecode".to_string(), wasm_bytecode_map)
        ]));
    }
    pub fn add_whammy(&mut self, mut whammy: Whammy) -> usize {
        let id = self.whammys.len();
        whammy.name = format!("whammy{}", id);
        self.whammys.push(whammy);

        id
    }
}

pub struct Whammy {
    pub name: String,
    /// The providers of the probes that have been used in the Whammy.
    pub providers: HashMap<String, Provider>,
    pub fns: Vec<Fn>,                     // User-provided
    pub globals: HashMap<String, Global>, // User-provided, should be VarId
}
impl Whammy {
    pub fn new() -> Self {
        Whammy {
            name: "".to_string(),
            providers: HashMap::new(),
            fns: vec![],
            globals: HashMap::new()
        }
    }

    /// Iterates over all of the matched providers, packages, events, and probe names
    /// to add a copy of the user-defined Probe for each of them.
    pub fn add_probe(&mut self, provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
                     prov_patt: &str, mod_patt: &str, func_patt: &str, nm_patt: &str,
                     predicate: Option<Expr>, body: Option<Vec<Statement>>) {
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
            for package_str in Package::get_matches(provided_probes,provider_str, mod_patt).iter() {
                // Does package exist yet?
                let package = match provider.packages.get_mut(package_str) {
                    Some(m) => m,
                    None => {
                        // add the package!
                        let new_mod = Package::new(package_str.to_lowercase().to_string());
                        provider.packages.insert(package_str.to_lowercase().to_string(), new_mod);
                        provider.packages.get_mut(&package_str.to_lowercase()).unwrap()
                    }
                };
                for event_str in Event::get_matches(provided_probes, provider_str, package_str, func_patt).iter() {
                    // Does event exist yet?
                    let event = match package.events.get_mut(event_str) {
                        Some(f) => f,
                        None => {
                            // add the package!
                            let new_fn = Event::new(event_str.to_lowercase().to_string());
                            package.events.insert(event_str.to_lowercase().to_string(), new_fn);
                            package.events.get_mut(&event_str.to_lowercase()).unwrap()
                        }
                    };
                    for name_str in Probe::get_matches(provided_probes, provider_str, package_str, event_str, nm_patt).iter() {
                        event.insert_probe(name_str.to_string(), Probe::new(nm_patt.to_string(), predicate.clone(), body.clone()));
                    }
                }
            }
        }
    }
}

pub struct Provider {
    pub name: String,
    pub fns: Vec<Fn>,                     // Comp-provided
    pub globals: HashMap<String, Global>, // Comp-provided

    /// The packages of the probes that have been used in the Whammy.
    /// These will be sub-packages of this Provider.
    pub packages: HashMap<String, Package>
}
impl Provider {
    pub fn new(name: String) -> Self {
        let fns = Provider::get_provided_fns(&name);
        let globals = Provider::get_provided_globals(&name);
        Provider {
            name,
            fns,
            globals,
            packages: HashMap::new()
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<Fn> {
        vec![]
    }

    fn get_provided_globals(_name: &String) -> HashMap<String, Global> {
        HashMap::new()
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

pub struct Package {
    pub name: String,
    pub fns: Vec<Fn>,                     // Comp-provided
    pub globals: HashMap<String, Global>, // Comp-provided

    /// The events of the probes that have been used in the Whammy.
    /// These will be sub-events of this Package.
    pub events: HashMap<String, Event>
}
impl Package {
    pub fn new(name: String) -> Self {
        let fns = Package::get_provided_fns(&name);
        let globals = Package::get_provided_globals(&name);
        Package {
            name,
            fns,
            globals,
            events: HashMap::new()
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<Fn> {
        vec![]
    }

    fn get_provided_globals(_name: &String) -> HashMap<String, Global> {
        HashMap::new()
    }

    /// Get the Package names that match the passed glob pattern
    pub fn get_matches(provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, provider: &str, mod_patt: &str) -> Vec<String> {
        let glob = Pattern::new(&mod_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for (mod_name, _package) in provided_probes.get(provider).unwrap().into_iter() {
            if glob.matches(&mod_name.to_lowercase()) {
                matches.push(mod_name.clone());
            }
        }

        matches
    }
}

pub struct Event {
    pub name: String,
    pub fns: Vec<Fn>,                     // Comp-provided
    pub globals: HashMap<String, Global>, // Comp-provided
    pub probe_map: HashMap<String, Vec<Probe>>
}
impl Event {
    pub fn new(name: String) -> Self {
        let fns = Event::get_provided_fns(&name);
        let globals = Event::get_provided_globals(&name);
        Event {
            name,
            fns,
            globals,
            probe_map: HashMap::new()
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<Fn> {
        vec![]
    }

    fn get_provided_globals(name: &String) -> HashMap<String, Global> {
        let mut globals = HashMap::new();
        if name.to_lowercase() == "call" {
            // Add in provided globals for the "call" event
            globals.insert("target_fn_type".to_string(),Global {
                is_comp_provided: true,
                ty: DataType::Str,
                var_name: Expr::VarId {
                    name: "target_fn_type".to_string(),
                },
                value: None
            });
            globals.insert("target_imp_module".to_string(),Global {
                is_comp_provided: true,
                ty: DataType::Str,
                var_name: Expr::VarId {
                    name: "target_imp_module".to_string(),
                },
                value: None
            });
            globals.insert("target_imp_name".to_string(),Global {
                is_comp_provided: true,
                ty: DataType::Str,
                var_name: Expr::VarId {
                    name: "target_imp_name".to_string(),
                },
                value: None
            });
            globals.insert("new_target_fn_name".to_string(),Global {
                is_comp_provided: true,
                ty: DataType::Str,
                var_name: Expr::VarId {
                    name: "new_target_fn_name".to_string(),
                },
                value: None
            });
        }

        globals
    }

    /// Get the Event names that match the passed glob pattern
    pub fn get_matches(provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, provider: &str, package: &str, func_patt: &str) -> Vec<String> {
        let glob = Pattern::new(&func_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for (fn_name, _package) in provided_probes.get(provider).unwrap().get(package).unwrap().into_iter() {
            if glob.matches(&fn_name.to_lowercase()) {
                matches.push(fn_name.clone());
            }
        }

        matches
    }

    pub fn insert_probe(&mut self, name: String, probe: Probe) {
        // Does name exist yet?
        match self.probe_map.get_mut(&name) {
            Some(probes) => {
                // Add probe to list
                probes.push(probe);
            },
            None => {
                self.probe_map.insert(name, vec![ probe ]);
            }
        };
    }
}

pub struct Probe {
    pub name: String,
    pub fns: Vec<Fn>,                     // Comp-provided
    pub globals: HashMap<String, Global>, // Comp-provided

    pub predicate: Option<Expr>,
    pub body: Option<Vec<Statement>>
}
impl Probe {
    pub fn new(name: String, predicate: Option<Expr>, body: Option<Vec<Statement>>) -> Self {
        let fns = Probe::get_provided_fns(&name);
        let globals = Probe::get_provided_globals(&name);
        Probe {
            name,
            fns,
            globals,

            predicate,
            body
        }
    }

    fn get_provided_fns(_name: &String) -> Vec<Fn> {
        vec![]
    }

    fn get_provided_globals(_name: &String) -> HashMap<String, Global> {
        HashMap::new()
    }

    /// Get the Probe names that match the passed glob pattern
    pub fn get_matches(provided_probes: &HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>, provider: &str, package: &str, event: &str, probe_patt: &str) -> Vec<String> {
        let glob = Pattern::new(&probe_patt.to_lowercase()).unwrap();

        let mut matches = vec![];

        for p_name in provided_probes.get(provider).unwrap().get(package).unwrap().get(event).unwrap().iter() {
            if glob.matches(&p_name.to_lowercase()) {
                matches.push(p_name.clone());
            }
        }

        matches
    }
}

// =====================
// ---- Expressions ----
// =====================

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

// =================
// ==== Visitor ====
// =================

pub trait WhammVisitor<T> {
    fn visit_whamm(&mut self, whamm: &Whamm) -> T;
    fn visit_whammy(&mut self, whammy: &Whammy) -> T;
    fn visit_provider(&mut self, provider: &Provider) -> T;
    fn visit_package(&mut self, package: &Package) -> T;
    fn visit_event(&mut self, event: &Event) -> T;
    fn visit_probe(&mut self, probe: &Probe) -> T;
    // fn visit_predicate(&mut self, predicate: &Expr) -> T;
    fn visit_fn(&mut self, f: &Fn) -> T;
    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> T;
    fn visit_stmt(&mut self, stmt: &Statement) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_op(&mut self, op: &Op) -> T;
    fn visit_datatype(&mut self, datatype: &DataType) -> T;
    fn visit_value(&mut self, val: &Value) -> T;
}

/// To support setting constant-provided global vars
pub trait WhammVisitorMut<T> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> T;
    fn visit_whammy(&mut self, whammy: &mut Whammy) -> T;
    fn visit_provider(&mut self, provider: &mut Provider) -> T;
    fn visit_package(&mut self, package: &mut Package) -> T;
    fn visit_event(&mut self, event: &mut Event) -> T;
    fn visit_probe(&mut self, probe: &mut Probe) -> T;
    // fn visit_predicate(&mut self, predicate: &mut Expr) -> T;
    fn visit_fn(&mut self, f: &mut Fn) -> T;
    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> T;
    fn visit_stmt(&mut self, stmt: &mut Statement) -> T;
    fn visit_expr(&mut self, expr: &mut Expr) -> T;
    fn visit_op(&mut self, op: &mut Op) -> T;
    fn visit_datatype(&mut self, datatype: &mut DataType) -> T;
    fn visit_value(&mut self, val: &mut Value) -> T;
}
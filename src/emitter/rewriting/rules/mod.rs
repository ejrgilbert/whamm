use crate::emitter::rewriting::rules::core::CorePackage;
use crate::emitter::rewriting::rules::wasm::WasmPackage;
use crate::generator::ast::Probe;
use crate::generator::rewriting::simple_ast::SimpleAstProbes;
use crate::parser::rules::core::WhammModeKind;
use crate::parser::rules::{FromStr, WhammProviderKind};
use crate::parser::types::{Block, DataType, Definition, Expr, NumLit, RulePart, Statement, Value};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::Location;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use orca_wasm::ir::id::FunctionID;
use wasmparser::Operator;

mod core;
pub mod wasm;

/// A function that can be used to generate these emitter rule types
/// from the SimpleAstProbes type created by the behavior tree builder.
/// See the documentation for this type for why this works when retaining
/// composable instrumentation ordering.
/// The design decision for generating emitter types from this new AST representation
/// is motivated by the constraints of the Rust type system. The following is other
/// designs that were considered and why they are not possible in the Rust PL.
///
/// 1. Add `From` trait to parser Provider/Package/Event/Mode types to translate to emitter variations
///    - Will not work since the `From` implementation would be tied to the structs implementing the
///      underlying Provider/Package/Event/Mode traits. From the AST perspective, all we know is that
///      we have a `dyn Provider|Package|Event|Mode`, not a specific implementation of it.
/// 2. Explicitly visit the AST to generate corresponding emitter variations
///    - This is the same problem as #1.
/// 3. Match on the `*Kind` enum variants instead of String names
///    - This isn't doable since we have specific `*Kind` enums per Provider/Package/Event/Mode trait.
///      So, we can't add a new function `get_kind(&self) -> *Kind` to the trait since we can't tie
///      the return type to a specific `*Kind` enum.
/// 4. Add `ProcessLoc` trait directly to parser Provider/Package/Event/Mode types
///    - This is the same issue as #1.
///
/// All this being said, the best design we have here is to basically create a new factory pattern
/// that iterates over the SimpleAstProbes built by the behavior tree builder to match Provider/Package/Event/Mode
/// names to the corresponding emitter variation.
/// This will keep the ordering guarantees for composable instrumentation by construction of the type and
/// enable us to work around the annoying Rust type system constraints. This will also keep the emitter logic
/// separate from the parser/verifier/behavior tree logic and keep this emitter logic specific to the bytecode
/// rewriting injection strategy.
pub fn provider_factory<P: Provider + FromStr>(ast: &SimpleAstProbes) -> Vec<Box<P>> {
    // Track the added provider hierarchies.
    // When visiting the next provider hierarchy it will be added (if not already there)
    // OR the new hierarchy will be appended within its respectful location in the already-existing one.
    // This is relevant when considering multiple scripts!
    let mut providers: Vec<Box<P>> = vec![];
    ast.iter().for_each(|(provider_name, packages)| {
        let mut provider = P::from_str(provider_name);
        provider.add_packages(packages);

        providers.push(Box::new(provider));
    });

    providers
}
/// Splits out the logic to add new packages to a provider
fn package_factory<P: Package + FromStr + 'static>(
    ast_packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>,
) -> Vec<Box<dyn Package>> {
    let mut packages: Vec<Box<dyn Package>> = vec![];
    ast_packages.iter().for_each(|(package_name, events)| {
        let mut package = P::from_str(package_name);
        package.add_events(events);

        packages.push(Box::new(package));
    });
    packages
}
/// Splits out the logic to add new events to a package
fn event_factory<E: Event + FromStr + 'static>(
    ast_events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>,
) -> Vec<Box<dyn Event>> {
    let mut events: Vec<Box<dyn Event>> = vec![];
    ast_events.iter().for_each(|(event_name, probes)| {
        let mut event = E::from_str(event_name);
        event.add_probes(probes);

        events.push(Box::new(event));
    });
    events
}
fn probe_factory(
    ast_probes: &HashMap<WhammModeKind, Vec<Probe>>,
) -> HashMap<WhammModeKind, Vec<Probe>> {
    ast_probes
        .iter()
        .map(|(name, probe_list)| {
            // it would be nice to not have to do this iteration, but I don't know of another way...
            let mut new_list = vec![];
            probe_list.iter().for_each(|probe| {
                new_list.push(probe.to_owned());
            });

            (name.clone(), new_list)
        })
        .collect()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Arg {
    pub name: String,
    pub ty: Option<OrcaType>,
}
impl Arg {
    fn new(name: String, ty: Option<OrcaType>) -> Self {
        Self { name, ty }
    }
}

#[derive(Clone, Debug)]
pub struct ProbeRule {
    pub provider: Option<RulePart>,
    pub package: Option<RulePart>,
    pub event: Option<RulePart>,
    pub mode: Option<WhammModeKind>,
}
impl Display for ProbeRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let curr_provider = match &self.provider {
            Some(provider) => provider.name.clone(),
            None => "".to_string(),
        };
        let curr_package = match &self.package {
            Some(package) => package.name.clone(),
            None => "".to_string(),
        };
        let curr_event = match &self.event {
            Some(event) => event.name.clone(),
            None => "".to_string(),
        };
        let curr_mode = match &self.mode {
            Some(mode) => mode.name().clone(),
            None => "".to_string(),
        };
        write!(
            f,
            "{}:{}:{}:{}",
            curr_provider, curr_package, curr_event, curr_mode
        )
    }
}

#[derive(Default, Debug)]
pub struct LocInfo<'a> {
    /// static information to be saved in symbol table
    pub static_data: HashMap<String, Option<Value>>,
    /// dynamic information to be defined at the probe location
    pub dynamic_data: HashMap<String, Block>,
    /// dynamic information corresponding to the operands of this location
    pub(crate) args: Vec<Arg>,
    pub num_alt_probes: usize,
    /// the probes that were matched for this instruction
    /// note the Script ID is contained in SimpleProbe
    pub probes: Vec<(ProbeRule, &'a Probe)>,
}
impl<'a> LocInfo<'a> {
    fn new() -> Self {
        Self::default()
    }
    fn has_match(&self) -> bool {
        !self.probes.is_empty()
    }
    fn add_probes(&mut self, base_rule: ProbeRule, probes: &'a HashMap<WhammModeKind, Vec<Probe>>) {
        probes.iter().for_each(|(probe_mode, probes)| {
            let mut rule = base_rule.clone();
            rule.mode = Some(probe_mode.clone());

            if matches!(probe_mode, WhammModeKind::Alt) {
                // this is an alt probe, mark it with the number!
                self.num_alt_probes += probes.len();
            }
            probes.iter().for_each(|probe| {
                self.probes.push((rule.clone(), probe));
            });
        })
    }
    fn add_dynamic_value(&mut self, name: String, val: Value) {
        let var_id = Expr::VarId {
            definition: Definition::CompilerDynamic,
            name: name.clone(),
            loc: None,
        };
        match &val {
            Value::Number {
                val: NumLit::U8 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_u8(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I8 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_i8(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::U16 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_u16(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I16 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_i16(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::U32 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_u32(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I32 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::I32,
                Expr::Primitive {
                    val: Value::gen_i32(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::F32 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::F32,
                Expr::Primitive {
                    val: Value::gen_f32(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::U64 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U64,
                Expr::Primitive {
                    val: Value::gen_u64(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I64 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::I64,
                Expr::Primitive {
                    val: Value::gen_i64(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::F64 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::I64,
                Expr::Primitive {
                    val: Value::gen_f64(*val),
                    loc: None,
                },
            ),
            Value::Boolean { val, .. } => self.add_dynamic_assign(
                name,
                DataType::Boolean,
                Expr::Primitive {
                    val: Value::Boolean { val: *val },
                    loc: None,
                },
            ),
            Value::Str { val, .. } => self.add_dynamic_assign(
                name,
                DataType::Str,
                Expr::Primitive {
                    val: Value::Str { val: val.clone() },
                    loc: None,
                },
            ),
            Value::Tuple { vals, ty } => self.add_dynamic_assign(
                name,
                ty.clone(),
                Expr::Primitive {
                    val: Value::Tuple {
                        ty: ty.clone(),
                        vals: vals.clone(),
                    },
                    loc: None,
                },
            ),
            Value::U32U32Map { val: map_val } => {
                // create a declaration
                let decl = Statement::Decl {
                    ty: val.ty(),
                    var_id: var_id.clone(),
                    loc: None,
                };
                // create assignments
                let mut stmts = vec![decl];
                for (key, val) in map_val.iter() {
                    stmts.push(Statement::SetMap {
                        map: var_id.clone(),
                        key: Expr::Primitive {
                            val: Value::gen_u32(*key),
                            loc: None,
                        },
                        val: Expr::Primitive {
                            val: Value::gen_u32(*val),
                            loc: None,
                        },
                        loc: None,
                    });
                }
                self.add_dynamic_block(
                    name,
                    Block {
                        stmts,
                        return_ty: None,
                        loc: None,
                    },
                );
            }
        };
    }
    fn add_dynamic_assign(&mut self, name: String, ty: DataType, expr: Expr) {
        let var_id = Expr::VarId {
            definition: Definition::CompilerDynamic,
            name: name.clone(),
            loc: None,
        };

        // create a declaration
        let decl = Statement::Decl {
            ty,
            var_id: var_id.clone(),
            loc: None,
        };
        // create an assignment
        let assign = Statement::Assign {
            var_id: var_id.clone(),
            expr,
            loc: None,
        };

        self.add_dynamic_block(
            name,
            Block {
                stmts: vec![decl, assign],
                return_ty: None,
                loc: None,
            },
        );
    }
    fn add_dynamic_block(&mut self, name: String, block: Block) {
        self.dynamic_data.insert(name, block);
    }
    fn append(&mut self, other: &mut Self) {
        // handle static_data
        self.static_data.extend(other.static_data.to_owned());

        // handle dynamic_data
        self.dynamic_data.extend(other.dynamic_data.to_owned());

        // handle args
        if !self.args.is_empty() {
            if !other.args.is_empty() {
                // assert that args are equivalent
                if !self.args.iter().all(|item| other.args.contains(item)) {
                    panic!("Emitter rules found different values for instruction args, please report this bug!");
                }
            }
            // just keep self args the way it is (other clearly doesn't populate them)
        } else {
            // just set to the other's args
            self.args = other.args.to_owned()
        }

        // handle num_alt_probes
        self.num_alt_probes += other.num_alt_probes;

        // handle probes
        self.probes.append(&mut other.probes);
    }
}

pub trait Provider {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info(&self, app_wasm: &Module, loc: Location, instr: &Operator) -> Option<LocInfo>;
    fn add_packages(
        &mut self,
        ast_packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>,
    );
}
pub trait Package {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info(&self, app_wasm: &Module,
                    curr_fid: &FunctionID, instr: &Operator) -> Option<LocInfo>;
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>);
}
pub trait Event {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info(&self, app_wasm: &Module,
                    fid: &FunctionID, instr: &Operator) -> Option<LocInfo>;
    fn add_probes(&mut self, ast_probes: &HashMap<WhammModeKind, Vec<Probe>>);
}

pub struct WhammProvider {
    kind: WhammProviderKind,
    /// The packages of the probes that have been used in the Script.
    pub packages: Vec<Box<dyn Package>>,
}
impl FromStr for WhammProvider {
    fn from_str(name: &str) -> Self {
        match name {
            "core" => Self::core(),
            "wasm" => Self::wasm(),
            _ => panic!("unsupported WhammProvider: {name}"),
        }
    }
}
impl WhammProvider {
    fn core() -> Self {
        Self {
            kind: WhammProviderKind::Core,
            packages: vec![],
        }
    }
    fn wasm() -> Self {
        Self {
            kind: WhammProviderKind::Wasm,
            packages: vec![],
        }
    }
}
impl Provider for WhammProvider {
    fn get_loc_info(&self, app_wasm: &Module, loc: Location, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();
        let (fid, pc, fname) = match loc {
            Location::Module {
                func_idx,
                instr_idx,
            }
            | Location::Component {
                func_idx,
                instr_idx,
                ..
            } => {
                let mut fname = String::default();
                let name = app_wasm.functions.get_name(func_idx).as_ref();
                if let Some(name) = name {
                    fname = name.clone();
                }
                (func_idx, instr_idx, fname)
            }
        };

        match self.kind {
            WhammProviderKind::Wasm => {
                // if *fid == 30 {
                //     println!("we're here!!")
                // }
                loc_info
                    .static_data
                    .insert("fid".to_string(), Some(Value::gen_u32(*fid)));

                loc_info
                    .static_data
                    .insert("fname".to_string(), Some(Value::Str { val: fname.clone() }));

                // Don't think we need this right now...
                // loc_info.static_data.insert(
                //     "wasm_bytecode_loc".to_string(),
                //     Some(Value::U32 {
                //         ty: DataType::U32,
                //         val: pc,
                //     }),
                // );

                loc_info
                    .static_data
                    .insert("pc".to_string(), Some(Value::gen_u32(pc as u32)));
            }
            WhammProviderKind::Core => {
                // nothing to add
            }
        }

        // Make sure we have arg symbol data to save off params in the behavior tree for all cases!
        // loc_info.args = OpcodeEvent::get_ty_info_for_instr(app_wasm, &fid, instr).0;

        // Get location info from the rest of the configured rules
        self.packages.iter().for_each(|package| {
            if let Some(mut other_loc_info) = package.get_loc_info(app_wasm, &fid, instr) {
                loc_info.append(&mut other_loc_info);
            }
        });

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }
    fn add_packages(
        &mut self,
        ast_packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>,
    ) {
        let packages = match self.kind {
            WhammProviderKind::Core => package_factory::<CorePackage>(ast_packages),
            WhammProviderKind::Wasm => package_factory::<WasmPackage>(ast_packages),
        };
        self.packages = packages;
    }
}

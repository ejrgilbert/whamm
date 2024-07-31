use crate::emitter::rewriting::rules::core::CorePackage;
use crate::emitter::rewriting::rules::wasm::{OpcodeEvent, WasmPackage};
use crate::parser::rules::WhammProviderKind;
use crate::parser::types::{ProbeSpec, SpecPart, Value};
use orca::Component;
use std::collections::HashMap;

use crate::generator::simple_ast::{SimpleAstProbes, SimpleProbe};
use orca::ir::module::Module;
use orca::ir::types::DataType as OrcaType;
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
    ast_packages: &HashMap<String, HashMap<String, HashMap<String, Vec<SimpleProbe>>>>,
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
    ast_events: &HashMap<String, HashMap<String, Vec<SimpleProbe>>>,
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
    ast_probes: &HashMap<String, Vec<SimpleProbe>>,
) -> HashMap<String, Vec<SimpleProbe>> {
    ast_probes
        .iter()
        .map(|(name, probe_list)| {
            // it would be nice to not have to do this iteration, but I don't know of another way...
            let mut new_list = vec![];
            probe_list.iter().for_each(|probe| {
                new_list.push(probe.to_owned());
            });

            (name.to_owned(), new_list)
        })
        .collect()
}

pub trait FromStr {
    fn from_str(name: &str) -> Self;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Arg {
    pub name: String,
    pub ty: OrcaType,
}
impl Arg {
    fn new(name: String, ty: OrcaType) -> Self {
        Self { name, ty }
    }
}

#[derive(Default, Debug)]
pub struct LocInfo<'a> {
    /// static information to be saved in symbol table
    pub static_data: HashMap<String, Option<Value>>,
    /// dynamic information corresponding to the operands of this location
    pub(crate) args: Vec<Arg>,
    pub num_alt_probes: usize,
    /// the probes that were matched for this instruction
    /// note the Script ID is contained in SimpleProbe
    pub probes: Vec<(ProbeSpec, &'a SimpleProbe)>,
}
impl<'a> LocInfo<'a> {
    fn new() -> Self {
        Self::default()
    }
    fn has_match(&self) -> bool {
        !self.probes.is_empty()
    }
    fn add_probes(&mut self, base_spec: ProbeSpec, probes: &'a HashMap<String, Vec<SimpleProbe>>) {
        probes.iter().for_each(|(probe_mode, probes)| {
            let mut spec = base_spec.clone();
            spec.mode = Some(SpecPart {
                name: probe_mode.clone(),
                loc: None,
            });

            if probe_mode == "alt" {
                // this is an alt probe, mark it with the number!
                self.num_alt_probes += probes.len();
            }
            probes.iter().for_each(|probe| {
                self.probes.push((spec.clone(), probe));
            });
        })
    }
    fn append(&mut self, other: &mut Self) {
        // handle static_data
        self.static_data.extend(other.static_data.to_owned());

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
    fn get_loc_info_module(&self, app_wasm: &Module, instr: &Operator) -> Option<LocInfo>;
    fn get_loc_info_comp(&self, app_wasm: &Component, instr: &Operator) -> Option<LocInfo>;
    fn add_packages(
        &mut self,
        ast_packages: &HashMap<String, HashMap<String, HashMap<String, Vec<SimpleProbe>>>>,
    );
}
pub trait Package {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info_module(&self, app_wasm: &Module, instr: &Operator) -> Option<LocInfo>;
    fn get_loc_info_comp(&self, app_wasm: &Component, instr: &Operator) -> Option<LocInfo>;
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<String, Vec<SimpleProbe>>>);
}
pub trait Event {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info_module(&self, app_wasm: &Module, instr: &Operator) -> Option<LocInfo>;
    fn get_loc_info_component(&self, app_wasm: &Component, instr: &Operator) -> Option<LocInfo>;
    fn get_loc_info<'a>(&'a self, instr: &Operator) -> Option<LocInfo>;
    fn add_probes(&mut self, ast_probes: &HashMap<String, Vec<SimpleProbe>>);
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
    fn get_loc_info_module(&self, app_wasm: &Module, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();
        match self.kind {
            WhammProviderKind::Core | WhammProviderKind::Wasm => {
                // nothing to add
            }
        }

        // Make sure we have arg symbol data to save off params in the behavior tree for all cases!
        loc_info.args = OpcodeEvent::get_ty_info_for_instr_module(app_wasm, instr).0;

        // Get location info from the rest of the configured rules
        self.packages.iter().for_each(|package| {
            if let Some(mut other_loc_info) = package.get_loc_info_module(app_wasm, instr) {
                loc_info.append(&mut other_loc_info);
            }
        });

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }

    fn get_loc_info_comp(&self, app_wasm: &Component, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();
        match self.kind {
            WhammProviderKind::Core | WhammProviderKind::Wasm => {
                // nothing to add
            }
        }

        // Make sure we have arg symbol data to save off params in the behavior tree for all cases!
        loc_info.args = OpcodeEvent::get_ty_info_for_instr_comp(app_wasm, instr).0;

        // Get location info from the rest of the configured rules
        self.packages.iter().for_each(|package| {
            if let Some(mut other_loc_info) = package.get_loc_info_comp(app_wasm, instr) {
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
        ast_packages: &HashMap<String, HashMap<String, HashMap<String, Vec<SimpleProbe>>>>,
    ) {
        let packages = match self.kind {
            WhammProviderKind::Core => package_factory::<CorePackage>(ast_packages),
            WhammProviderKind::Wasm => package_factory::<WasmPackage>(ast_packages),
        };
        self.packages = packages;
    }
}

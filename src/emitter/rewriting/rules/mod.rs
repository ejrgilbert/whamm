use crate::parser::rules::{Probe, Provider as AstProvider, WhammProvider as AstWhammProvider, WhammProviderKind};
use crate::verifier::types::Record;
use std::collections::HashMap;
use walrus::ir::Instr;
use walrus::ValType;
use crate::behavior::builder_visitor::SimpleAstProbes;

mod core;
pub mod wasm;

pub fn wrap() {
    from::<AstWhammProvider>(&vec![]);
}

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
pub fn from<P: AstProvider + Into<Box<dyn Provider>>>(ast: &SimpleAstProbes) -> Vec<WhammProvider> {
    // Track the added provider hierarchies.
    // When visiting the next provider hierarchy it will be added (if not already there)
    // OR the new hierarchy will be appended within its respectful location in the already-existing one.
    // This is relevant when considering multiple scripts!
    let new_providers: HashMap<WhammProviderKind, Vec<WhammProvider>> = HashMap::new();
    for script in scripts.iter() {
        for (_name, provider) in script.providers.iter() {
            
        }
    }
    
    todo!()
}
impl Into<Box<dyn Provider>> for AstWhammProvider {
    fn into(self) -> Box<dyn Provider> {
        match self.kind {
            WhammProviderKind::Core => {
                todo!()
            }
            WhammProviderKind::Wasm => {
                todo!()
            }
        }
    }
}

pub struct LocInfo<'a> {
    /// static information to be saved in symbol table
    static_data: HashMap<String, Record>,
    /// dynamic information corresponding to the operands of this location
    dynamic_data: Vec<ValType>,
    /// the probes that were matched for this instruction
    probes: Vec<&'a dyn Probe>,
}

pub trait ProcessLoc {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info(&self, app_wasm: &walrus::Module, instr: &Instr, instr_name: &str) -> LocInfo;
}

pub trait Provider {}
pub trait Package {}
pub trait Event {}

pub struct WhammProvider {
    kind: WhammProviderKind,
    /// The packages of the probes that have been used in the Script.
    pub packages: Vec<Box<dyn Package>>,
}
impl Provider for WhammProvider {}
impl ProcessLoc for WhammProvider {
    fn get_loc_info(
        &self,
        _app_wasm: &walrus::Module,
        _instr: &Instr,
        _instr_name: &str,
    ) -> LocInfo {
        match self.kind {
            WhammProviderKind::Core => {
                todo!()
            }
            WhammProviderKind::Wasm => {
                todo!()
            }
        }
    }
}
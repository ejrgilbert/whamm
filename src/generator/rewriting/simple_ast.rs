use crate::parser::types as parser_types;
use parser_types::Statement;
use std::collections::HashMap;

use crate::generator::ast::{Probe, Script, WhammParam};
use crate::parser::provider_handler::ModeKind;

/// This is a structure that saves a simplified variation of the activated
/// probe rules.
/// Note that this does not explicitly represent a "Script" (which was used in the
/// previous AST representation to keep instrumentation "units" together). This
/// design is important to support composable instrumentation that imposes an ordering
/// to the injected code. If multiple scripts probe the same point in an application,
/// the injection ordering should follow the ordering of the scripts as specified by the
/// user.
/// While this is not explicitly retained, there is still a convention that is followed by
/// construction of the BehaviorTree AST visitation logic that imposes this requirement. Since
/// the AST scripts are followed in-order (they're saved to an ordered Vec type), the collapsing
/// into this new AST representation retains the intended order as well.
/// Consider the following example (where <script0_body0> is used to refer to the contents of the probe for readability):
/// Script0 {
///     wasm:opcode:call:before {<script0_body0>}
///     wasm:opcode:call:before {<script0_body1>}
///     wasm:opcode:call:after {<script0_body2>}
/// }
/// Script1 {
///     wasm:opcode:call:before {<script1_body0>}
/// }
///
/// This will translate to the following structure in the `SimpleAstProbes` type:
/// {
///     "wasm" -> {
///         "opcode" -> {
///             "call" -> {
///                 "before" -> {
///                     <script0_body0>,
///                     <script0_body1>,
///                     <script1_body0>,
///                 }
///                 "after" -> {
///                     <script0_body2>
///                 }
///             }
///         }
///     }
/// }
/// The code generator will then take this and iterate over bodies to be injected at each probed point
/// in the application code in the same order as the scripts were passed to the `whamm!` tool.
///
/// Note: The Probes here are owned by this new AST structure. This is to simplify some logic in the
/// emitter. Holding on to a reference to the original AST complicates the lifetimes of building a new
/// AST representation since we're heavily reliant on traits. Since traits must have a static lifetime,
/// mixing that with a non-static lifetimes keeps from having simple factory code.
/// As a workaround, we know that the original AST isn't really needed at this point, so we have the new
/// AST representation own the Probes instead!
///
/// Note: This AST representation will only be used for bytecode rewriting, not when targeting wei.

#[derive(Default)]
pub struct SimpleAST {
    pub global_stmts: Vec<Statement>,
    pub provs: HashMap<String, SimpleProv>,
}
impl SimpleAST {
    pub fn new(ast: Vec<Script>) -> Self {
        let mut s = Self::default();

        for Script {
            global_stmts,
            probes,
            ..
        } in ast.iter()
        {
            s.global_stmts.extend(global_stmts.to_owned());
            for probe in probes.iter() {
                s.add_probe(
                    &probe.rule.provider.name,
                    &probe.rule.package.name,
                    &probe.rule.event.name,
                    &probe.rule.mode.name,
                    probe.to_owned(),
                );
            }
        }

        s
    }
    fn add_probe(
        &mut self,
        provider_name: &str,
        package_name: &str,
        event_name: &str,
        mode_name: &str,
        probe: Probe,
    ) {
        self.provs
            .entry(provider_name.to_string())
            .and_modify(|provider| {
                provider.add_probe(package_name, event_name, mode_name, probe.to_owned());
            })
            .or_insert(SimpleProv::new(package_name, event_name, mode_name, probe));
    }
}
#[derive(Default)]
pub struct SimpleProv {
    pub pkgs: HashMap<String, SimplePkg>,
    all_params: Option<Vec<WhammParam>>,
}
impl SimpleProv {
    fn new(package_name: &str, event_name: &str, mode_name: &str, probe: Probe) -> Self {
        let mut s = Self::default();
        s.add_probe(package_name, event_name, mode_name, probe);
        s
    }
    fn add_probe(&mut self, package_name: &str, event_name: &str, mode_name: &str, probe: Probe) {
        self.pkgs
            .entry(package_name.to_string())
            .and_modify(|pkg| {
                pkg.add_probe(event_name, mode_name, probe.to_owned());
            })
            .or_insert(SimplePkg::new(event_name, mode_name, probe));
    }
    pub fn all_params(&mut self) -> &Vec<WhammParam> {
        self.all_params.get_or_insert_with(|| {
            let mut ps = vec![];
            for pkg in self.pkgs.values_mut() {
                ps.extend(pkg.all_params().clone());
            }
            ps
        })
    }
}
#[derive(Default)]
pub struct SimplePkg {
    pub evts: HashMap<String, SimpleEvt>,
    all_params: Option<Vec<WhammParam>>,
}
impl SimplePkg {
    fn new(event_name: &str, mode_name: &str, probe: Probe) -> Self {
        let mut s = Self::default();
        s.add_probe(event_name, mode_name, probe);
        s
    }
    fn add_probe(&mut self, event_name: &str, mode_name: &str, probe: Probe) {
        self.evts
            .entry(event_name.to_string())
            .and_modify(|evt| {
                evt.add_probe(mode_name, probe.to_owned());
            })
            .or_insert(SimpleEvt::new(mode_name, probe));
    }
    pub fn all_params(&mut self) -> &Vec<WhammParam> {
        self.all_params.get_or_insert_with(|| {
            let mut ps = vec![];
            for evt in self.evts.values_mut() {
                ps.extend(evt.all_params().clone());
            }
            ps
        })
    }
}
#[derive(Default)]
pub struct SimpleEvt {
    pub modes: HashMap<ModeKind, Vec<Probe>>,
    all_params: Option<Vec<WhammParam>>,
}
impl SimpleEvt {
    fn new(mode_name: &str, probe: Probe) -> Self {
        let mut s = Self::default();
        s.add_probe(mode_name, probe);
        s
    }
    fn add_probe(&mut self, mode_name: &str, probe: Probe) {
        let mode_kind = ModeKind::from(mode_name.to_string());
        self.modes
            .entry(mode_kind)
            .and_modify(|probes| {
                probes.push(probe.clone());
            })
            .or_insert(vec![probe]);
    }
    pub fn all_params(&mut self) -> &Vec<WhammParam> {
        self.all_params.get_or_insert_with(|| {
            let mut ps = vec![];
            for probes in self.modes.values() {
                for p in probes.iter() {
                    ps.extend(p.metadata.pred_args.params.clone());
                    ps.extend(p.metadata.body_args.params.clone());
                }
            }
            ps
        })
    }
}

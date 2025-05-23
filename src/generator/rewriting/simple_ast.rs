use crate::parser::types as parser_types;
use parser_types::Statement;
use std::collections::HashMap;

use crate::generator::ast::{Probe, Script};
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
/// Note: This AST representation will only be used for bytecode rewriting, not when targeting Wizard.
pub type SimpleAstProbes =
    HashMap<String, HashMap<String, HashMap<String, HashMap<ModeKind, Vec<Probe>>>>>;

#[derive(Default)]
pub struct SimpleAST {
    pub global_stmts: Vec<Statement>,
    pub probes: SimpleAstProbes,
}

pub fn build_simple_ast(ast: Vec<Script>) -> SimpleAST {
    let mut simple_ast = SimpleAST::default();

    for Script {
        global_stmts,
        probes,
        ..
    } in ast.iter()
    {
        simple_ast.global_stmts.extend(global_stmts.to_owned());
        for probe in probes.iter() {
            add_probe_to_ast(
                &mut simple_ast,
                &probe.rule.provider.name,
                &probe.rule.package.name,
                &probe.rule.event.name,
                &probe.rule.mode.name,
                probe.to_owned(),
            )
        }
    }

    simple_ast
}

fn add_probe_to_ast(
    ast: &mut SimpleAST,
    provider_name: &String,
    package_name: &String,
    event_name: &String,
    mode_name: &String,
    probe: Probe,
) {
    if let Some(provider) = ast.probes.get_mut(provider_name) {
        if let Some(package) = provider.get_mut(package_name) {
            if let Some(event) = package.get_mut(event_name) {
                let mode_kind = ModeKind::from(mode_name.clone());
                if let Some(probes) = event.get_mut(&mode_kind) {
                    probes.push(probe);
                } else {
                    event.insert(mode_kind, vec![probe]);
                }
            } else {
                package.insert(event_name.clone(), HashMap::new());
                add_probe_to_ast(
                    ast,
                    provider_name,
                    package_name,
                    event_name,
                    mode_name,
                    probe,
                );
            }
        } else {
            provider.insert(package_name.clone(), HashMap::new());
            add_probe_to_ast(
                ast,
                provider_name,
                package_name,
                event_name,
                mode_name,
                probe,
            );
        }
    } else {
        ast.probes.insert(provider_name.clone(), HashMap::new());
        add_probe_to_ast(
            ast,
            provider_name,
            package_name,
            event_name,
            mode_name,
            probe,
        );
    }
}

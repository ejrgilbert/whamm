use crate::common::rule_tracker::RuleTracker;
use crate::parser::provider_handler::{Package, Provider};
use crate::parser::types::WhammVisitor;

/// Supertrait for immutable analysis visitors (`WhammVisitor<()>`).
///
/// Provides `do_visit_provider` and `do_visit_package` helpers that handle the
/// shared scope-entry / rule-tracking / child-traversal pattern, while letting
/// the implementor inject custom logic via `before_children_*` hooks.
///
/// Usage: implement this trait on your visitor struct, override `before_children_*`
/// as needed, then delegate from your `WhammVisitor<()>` `visit_provider` /
/// `visit_package` impls to the corresponding `do_visit_*` helper.
pub trait AnalysisVisitor {
    fn enter_named_scope(&mut self, name: &str);
    fn exit_scope(&mut self);
    fn get_rule_tracker_mut(&mut self) -> &mut RuleTracker;

    /// Called after entering scope and pushing the rule, before visiting children.
    fn before_children_provider(&mut self, _provider: &Provider) {}
    /// Called after entering scope and pushing the rule, before visiting children.
    fn before_children_package(&mut self, _package: &Package) {}

    fn do_visit_provider(&mut self, provider: &Provider)
    where
        Self: WhammVisitor<()> + Sized,
    {
        let name = provider.def.name.clone();
        self.enter_named_scope(&name);
        self.get_rule_tracker_mut().push(&name);
        self.before_children_provider(provider);
        provider.packages.values().for_each(|package| {
            self.visit_package(package);
        });
        self.exit_scope();
        self.get_rule_tracker_mut().pop();
    }

    fn do_visit_package(&mut self, package: &Package)
    where
        Self: WhammVisitor<()> + Sized,
    {
        let name = format!(":{}", package.def.name);
        self.enter_named_scope(&package.def.name.clone());
        self.get_rule_tracker_mut().push(&name);
        self.before_children_package(package);
        package.events.values().for_each(|event| {
            self.visit_event(event);
        });
        self.exit_scope();
        self.get_rule_tracker_mut().pop();
    }
}

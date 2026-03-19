use crate::common::rule_tracker::RuleTracker;
use crate::parser::provider_handler::{Event, Package, Provider};
use crate::parser::types::{DataType, WhammVisitorMut};

/// Supertrait for visitors that type-check (`WhammVisitorMut<Option<DataType>>`).
///
/// Provides `do_visit_provider`, `do_visit_package`, and `do_visit_event` helpers
/// that handle the shared scope-entry / rule-tracking / child-traversal pattern,
/// while letting the implementor inject custom logic via `before_children_*` hooks.
///
/// Usage: implement this trait on your visitor struct, override `before_children_*`
/// as needed, then delegate from your `WhammVisitorMut` `visit_provider` /
/// `visit_package` / `visit_event` impls to the corresponding `do_visit_*` helper.
pub trait VerifierVisitorTyped {
    fn enter_named_scope(&mut self, name: &str);
    fn exit_scope(&mut self);
    fn get_rule_tracker_mut(&mut self) -> &mut RuleTracker;

    /// Called after entering scope and pushing the rule, before visiting children.
    fn before_children_provider(&mut self, _provider: &mut Provider) {}
    /// Called after entering scope and pushing the rule, before visiting children.
    fn before_children_package(&mut self, _package: &mut Package) {}
    /// Called after entering scope and pushing the rule, before visiting children.
    fn before_children_event(&mut self, _event: &mut Event) {}

    fn do_visit_provider(&mut self, provider: &mut Provider) -> Option<DataType>
    where
        Self: WhammVisitorMut<Option<DataType>> + Sized,
    {
        let name = provider.def.name.clone();
        self.enter_named_scope(&name);
        self.get_rule_tracker_mut().push(&name);
        self.before_children_provider(provider);
        provider.packages.values_mut().for_each(|package| {
            self.visit_package(package);
        });
        self.exit_scope();
        self.get_rule_tracker_mut().pop();
        None
    }

    fn do_visit_package(&mut self, package: &mut Package) -> Option<DataType>
    where
        Self: WhammVisitorMut<Option<DataType>> + Sized,
    {
        let name = format!(":{}", package.def.name);
        self.enter_named_scope(&package.def.name.clone());
        self.get_rule_tracker_mut().push(&name);
        self.before_children_package(package);
        package.events.values_mut().for_each(|event| {
            self.visit_event(event);
        });
        self.exit_scope();
        self.get_rule_tracker_mut().pop();
        None
    }

    fn do_visit_event(&mut self, event: &mut Event) -> Option<DataType>
    where
        Self: WhammVisitorMut<Option<DataType>> + Sized,
    {
        let name = format!(":{}", event.def.name);
        self.enter_named_scope(&event.def.name.clone());
        self.get_rule_tracker_mut().push(&name);
        self.before_children_event(event);
        event.probes.values_mut().for_each(|probes| {
            probes.iter_mut().for_each(|probe| {
                self.visit_probe(probe);
            });
        });
        self.exit_scope();
        self.get_rule_tracker_mut().pop();
        None
    }
}

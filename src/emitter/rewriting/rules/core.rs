use std::collections::HashMap;
use crate::emitter::rewriting::rules::{Event, event_factory, FromStr, LocInfo, Package, ProcessLoc};
use crate::parser::rules::core::{CoreEventKind, CorePackageKind};
use walrus::ir::Instr;
use crate::parser::rules::Probe;

pub struct CorePackage<'a> {
    kind: CorePackageKind,
    pub events: Vec<Box<dyn Event<'a>>>,
}
impl FromStr for CorePackage<'_> {
    fn from_str(name: &String) -> Self {
        match name.as_str() {
            "default" => Self::default(),
            _ => panic!("unsupported CorePackage: {name}"),
        }
    }
}
impl CorePackage<'_> {
    fn default() -> Self {
        Self {
            kind: CorePackageKind::Default,
            events: vec![]
        }
    }
}
impl<'a> Package<'a> for CorePackage<'a> {
    fn get_events_mut(&mut self) -> &mut Vec<Box<dyn Event<'a>>> {
        &mut self.events
    }
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<String, Vec<Box<&'a dyn Probe>>>>) {
        match self.kind {
            CorePackageKind::Default => {
                event_factory::<CoreEvent>(self as &mut dyn Package, ast_events);
            }
        }
    }
}
impl ProcessLoc for CorePackage<'_> {
    fn get_loc_info(
        &self,
        _app_wasm: &walrus::Module,
        _instr: &Instr,
        _instr_name: &str,
    ) -> LocInfo {
        match self.kind {
            CorePackageKind::Default => {
                todo!()
            }
        }
    }
}

pub struct CoreEvent<'a> {
    kind: CoreEventKind,
    probes: HashMap<String, Vec<Box<&'a dyn Probe>>>,
}
impl FromStr for CoreEvent<'_> {
    fn from_str(name: &String) -> Self {
        match name.as_str() {
            "default" => Self::default(),
            _ => panic!("unsupported CoreEvent: {name}")
        }
    }
}
impl CoreEvent<'_> {
    // ======================
    // ---- Constructors ----
    // ======================
    fn default() -> Self {
        Self {
            kind: CoreEventKind::Default,
            probes: HashMap::new()
        }
    }
}
impl<'a> Event<'a> for CoreEvent<'a> {
    fn add_probes(&mut self, probes: &HashMap<String, Vec<Box<&'a dyn Probe>>>) {
        self.probes = probes.to_owned()
    }
}
impl ProcessLoc for CoreEvent<'_> {
    fn get_loc_info(
        &self,
        _app_wasm: &walrus::Module,
        _instr: &Instr,
        _instr_name: &str,
    ) -> LocInfo {
        match self.kind {
            CoreEventKind::Default => {
                todo!()
            }
        }
    }
}
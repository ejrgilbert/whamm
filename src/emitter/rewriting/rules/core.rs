use std::collections::HashMap;
use crate::emitter::rewriting::rules::{Event, event_factory, FromStr, LocInfo, Package, probe_factory};
use crate::parser::rules::core::{CoreEventKind, CorePackageKind};
use walrus::ir::Instr;
use crate::behavior::builder_visitor::SimpleProbe;

pub struct CorePackage {
    kind: CorePackageKind,
    pub events: Vec<Box<dyn Event>>,
}
impl FromStr for CorePackage {
    fn from_str(name: &String) -> Self {
        match name.as_str() {
            "default" => Self::default(),
            _ => panic!("unsupported CorePackage: {name}"),
        }
    }
}
impl CorePackage {
    fn default() -> Self {
        Self {
            kind: CorePackageKind::Default,
            events: vec![]
        }
    }
}
impl Package for CorePackage {
    fn get_loc_info(
        &self,
        _instr: &Instr,
        _instr_name: &str,
    ) -> Option<LocInfo> {
        match self.kind {
            CorePackageKind::Default => {
                todo!()
            }
        }
    }
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<String, Vec<SimpleProbe>>>) {
        let events = match self.kind {
            CorePackageKind::Default => {
                event_factory::<CoreEvent>(ast_events)
            }
        };
        self.events = events;
    }
}

pub struct CoreEvent {
    kind: CoreEventKind,
    probes: HashMap<String, Vec<SimpleProbe>>,
}
impl FromStr for CoreEvent {
    fn from_str(name: &String) -> Self {
        match name.as_str() {
            "default" => Self::default(),
            _ => panic!("unsupported CoreEvent: {name}")
        }
    }
}
impl CoreEvent {
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
impl Event for CoreEvent {
    fn get_loc_info(
        &self,
        _instr: &Instr,
        _instr_name: &str,
    ) -> Option<LocInfo> {
        match self.kind {
            CoreEventKind::Default => {
                todo!()
            }
        }
    }
    fn add_probes(&mut self, probes: &HashMap<String, Vec<SimpleProbe>>) {
        self.probes = probe_factory(probes);
    }
}
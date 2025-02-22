use crate::emitter::rewriting::rules::{
    event_factory, probe_factory, Event, FromStr, LocInfo, Package,
};
use crate::parser::rules::core::{CoreEventKind, CorePackageKind, WhammModeKind};
use std::collections::HashMap;
use orca_wasm::ir::id::FunctionID;
use crate::generator::ast::Probe;
use orca_wasm::ir::module::Module;
use wasmparser::Operator;

pub struct CorePackage {
    kind: CorePackageKind,
    pub events: Vec<Box<dyn Event>>,
}
impl FromStr for CorePackage {
    fn from_str(name: &str) -> Self {
        match name {
            "default" => Self::default(),
            _ => panic!("unsupported CorePackage: {name}"),
        }
    }
}
impl CorePackage {
    fn default() -> Self {
        Self {
            kind: CorePackageKind::Default,
            events: vec![],
        }
    }
}
impl Package for CorePackage {
    fn get_loc_info(&self, app_wasm: &Module, fid: &FunctionID, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();
        match self.kind {
            CorePackageKind::Default => {
                // nothing to add
            }
        }

        // Get location info from the rest of the configured rules
        self.events.iter().for_each(|event| {
            if let Some(mut other_loc_info) = event.get_loc_info(app_wasm, fid, instr) {
                loc_info.append(&mut other_loc_info);
            }
        });

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>) {
        let events = match self.kind {
            CorePackageKind::Default => event_factory::<CoreEvent>(ast_events),
        };
        self.events = events;
    }
}

pub struct CoreEvent {
    kind: CoreEventKind,
    probes: HashMap<WhammModeKind, Vec<Probe>>,
}
impl FromStr for CoreEvent {
    fn from_str(name: &str) -> Self {
        match name {
            "default" => Self::default(),
            _ => panic!("unsupported CoreEvent: {name}"),
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
            probes: HashMap::new(),
        }
    }
}
impl Event for CoreEvent {
    fn get_loc_info(&self, _app_wasm: &Module, _curr_fid: &FunctionID, _instr: &Operator) -> Option<LocInfo> {
        let loc_info = LocInfo::new();
        match self.kind {
            CoreEventKind::Default => {
                // nothing to add
            }
        }

        // Get location info from the rest of the configured rules
        self.probes.iter().for_each(|(_probe_mode, probes)| {
            probes.iter().for_each(|_probe| {
                // TODO -- how to handle before/after probes?
                //   this is weird because we really want to check if there's a start fn...if not inject one.
                //   This is a different paradigm than we currently have (visit the app_wasm and inject on matches)
                todo!()
            });
        });

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }
    fn add_probes(&mut self, probes: &HashMap<WhammModeKind, Vec<Probe>>) {
        self.probes = probe_factory(probes);
    }
}

use std::collections::{HashMap, HashSet};

pub struct ReportMetadata {
    pub map_metadata: HashMap<i32, Metadata>,
    pub variable_metadata: HashMap<usize, Metadata>,
    pub all_metadata: HashSet<Metadata>,
}
impl ReportMetadata {
    pub fn new() -> Self {
        ReportMetadata {
            map_metadata: HashMap::new(),
            variable_metadata: HashMap::new(),
            all_metadata: HashSet::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Metadata {
    Global {
        name: String,
        script_id: i32,
    },
    Local {
        name: String,
        script_id: i32,
        bytecode_loc: i32,
        probe_id: i32,
    },
}
use std::collections::{HashMap, HashSet};

pub struct ReportVarMetadata {
    //MapID -> Metadata
    pub map_metadata: HashMap<i32, Metadata>,
    //GID -> Metadata
    pub variable_metadata: HashMap<usize, Metadata>,
    pub all_metadata: HashSet<Metadata>,
    pub curr_location: LocationData,
}
impl ReportVarMetadata {
    pub fn new() -> Self {
        ReportVarMetadata {
            map_metadata: HashMap::new(),
            variable_metadata: HashMap::new(),
            all_metadata: HashSet::new(),
            curr_location: LocationData::Global {
                script_id: "0".to_string(),
            },
        }
    }
    pub fn set_loc(&mut self, script_id: String, bytecode_loc: i32, probe_id: String) {
        self.curr_location = LocationData::Local {
            script_id,
            bytecode_loc,
            probe_id,
        };
    }
    pub fn put_global_metadata(&mut self, gid: usize, name: String, script_id: String) {
        let metadata = Metadata::Global { name, script_id };
        self.variable_metadata.insert(gid, metadata.clone());
        if !self.all_metadata.insert(metadata) {
            panic!("Duplicate metadata entry!");
        }
    }
    pub fn put_local_metadata(
        &mut self,
        gid: usize,
        name: String,
        script_id: String,
        bytecode_loc: i32,
        probe_id: String,
    ) {
        let metadata = Metadata::Local {
            name,
            script_id,
            bytecode_loc,
            probe_id,
        };
        self.variable_metadata.insert(gid, metadata.clone());
        if !self.all_metadata.insert(metadata) {
            panic!("Duplicate metadata entry!");
        }
    }
    pub fn print_metadata(&self) {
        if self.all_metadata.is_empty() {
            return;
        }
        println!("Metadata:");
        for (key, value) in &self.variable_metadata {
            println!("GID: {} -> {:?}", key, value);
        }
        for (key, value) in &self.map_metadata {
            println!("MapID: {} -> {:?}", key, value);
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Metadata {
    Global {
        name: String,
        script_id: String,
    },
    Local {
        name: String,
        script_id: String,
        bytecode_loc: i32,
        probe_id: String,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocationData {
    Global {
        script_id: String,
    },
    Local {
        script_id: String,
        bytecode_loc: i32,
        probe_id: String,
    },
}

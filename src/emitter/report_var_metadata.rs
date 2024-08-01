use std::collections::{HashMap, HashSet};

use log::info;

use crate::common::error::{ErrorGen, WhammError};

pub struct ReportVarMetadata {
    //MapID -> Metadata
    pub map_metadata: HashMap<i32, Metadata>,
    //GID -> Metadata
    pub variable_metadata: HashMap<usize, Metadata>,
    pub all_metadata: HashSet<Metadata>,
    pub curr_location: LocationData,
    pub used_i32_gids: Vec<usize>,
    pub available_i32_gids: Vec<usize>,
    pub flush_soon: bool,
}
impl Default for ReportVarMetadata {
    fn default() -> Self {
        Self::new()
    }
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
            available_i32_gids: vec![],
            used_i32_gids: vec![],
            flush_soon: false,
        }
    }
    pub fn set_loc(
        &mut self,
        script_id: String,
        bytecode_loc: (i32, i32),
        probe_id: String,
        num_reports: i32,
    ) {
        self.curr_location = LocationData::Local {
            script_id,
            bytecode_loc,
            probe_id,
            num_reports,
        };
    }
    pub fn put_global_metadata(
        &mut self,
        gid: usize,
        name: String,
    ) -> Result<bool, Box<WhammError>> {
        let script_id = match &self.curr_location {
            LocationData::Global { script_id } => script_id.clone(),
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "Expected global location data, but got: {:?}",
                        self.curr_location
                    )),
                    None,
                )))
            }
        };
        let metadata = Metadata::Global {
            name: name.clone(),
            script_id,
        };
        self.variable_metadata.insert(gid, metadata.clone());
        if !self.all_metadata.insert(metadata) {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("Duplicate metadata for map with name: {}", name)),
                None,
            )));
        }
        Ok(true)
    }
    pub fn put_local_metadata(
        &mut self,
        gid: usize,
        name: String,
    ) -> Result<bool, Box<WhammError>> {
        let script_id;
        let bytecode_loc;
        let probe_id;
        match &self.curr_location {
            LocationData::Local {
                script_id: s,
                bytecode_loc: b,
                probe_id: p,
                ..
            } => {
                script_id = s.clone();
                bytecode_loc = *b;
                probe_id = p.clone();
            }
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "Expected local location data, but got: {:?}",
                        self.curr_location
                    )),
                    None,
                )))
            }
        };
        let metadata = Metadata::Local {
            name: name.clone(),
            script_id,
            bytecode_loc,
            probe_id,
        };
        self.variable_metadata.insert(gid, metadata.clone());
        if !self.all_metadata.insert(metadata) {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("Duplicate metadata with name: {}", name)),
                None,
            )));
        }
        Ok(true)
    }
    pub fn print_metadata(&self) {
        if self.all_metadata.is_empty() {
            return;
        }
        info!("Metadata:");

        // Collect and sort variable_metadata by key
        let mut sorted_variable_metadata: Vec<_> = self.variable_metadata.iter().collect();
        sorted_variable_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_variable_metadata {
            info!("GID: {} -> {:?}", key, value);
        }

        // Collect and sort map_metadata by key
        let mut sorted_map_metadata: Vec<_> = self.map_metadata.iter().collect();
        sorted_map_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_map_metadata {
            info!("MapID: {} -> {:?}", key, value);
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
        bytecode_loc: (i32, i32),
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
        bytecode_loc: (i32, i32),
        probe_id: String,
        num_reports: i32,
    },
}

pub fn convert_meta_to_string(metadata: &Metadata) -> String {
    match metadata {
        Metadata::Global { name, script_id } => format!("{}\t {}\t \t ", name, script_id),
        Metadata::Local {
            name,
            script_id,
            bytecode_loc,
            probe_id,
        } => format!(
            "{}\t {}\t ({}, {})\t {}",
            name, script_id, bytecode_loc.0, bytecode_loc.1, probe_id
        ),
    }
}

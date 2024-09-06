use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use crate::common::error::{ErrorGen, WhammError};

pub struct ReportVarMetadata {
    //MapID -> Metadata
    pub map_metadata: HashMap<u32, Metadata>,
    //GID -> Metadata
    pub variable_metadata: HashMap<u32, Metadata>,
    pub all_metadata: HashSet<Metadata>,
    pub curr_location: LocationData,
    pub used_i32_gids: Vec<u32>,
    pub available_i32_gids: Vec<u32>,
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
    pub fn put_global_metadata(&mut self, gid: u32, name: String) -> Result<bool, Box<WhammError>> {
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
    pub fn put_local_metadata(&mut self, gid: u32, name: String) -> Result<bool, Box<WhammError>> {
        if let LocationData::Local { .. } = &self.curr_location {
            let metadata = Metadata::new(name.clone(), &self.curr_location);
            self.variable_metadata.insert(gid, metadata.clone());
            if !self.all_metadata.insert(metadata) {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!("Duplicate metadata with name: {}", name)),
                    None,
                )));
            }
            Ok(true)
        } else {
            Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "Expected local location data, but got: {:?}",
                    self.curr_location
                )),
                None,
            )))
        }
    }
    pub fn print_metadata(&self) {
        if self.all_metadata.is_empty() {
            return;
        }
        println!("Metadata:");

        // Collect and sort variable_metadata by key
        let mut sorted_variable_metadata: Vec<_> = self.variable_metadata.iter().collect();
        sorted_variable_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_variable_metadata {
            println!("GID: {} -> {:?}", key, value);
        }

        // Collect and sort map_metadata by key
        let mut sorted_map_metadata: Vec<_> = self.map_metadata.iter().collect();
        sorted_map_metadata.sort_by_key(|&(key, _)| key);

        for (key, value) in sorted_map_metadata {
            println!("MapID: {} -> {:?}", key, value);
        }
    }
    pub fn mutating_map(&mut self, map_id: u32) {
        //check if the map you are changing is in map_metadata -> flush soon if it is
        if self.map_metadata.contains_key(&map_id) {
            self.flush_soon = true;
        }
    }
    pub fn mutating_var(&mut self, var_id: u32) {
        //check if the var you are changing is in variable_metadata -> flush soon if it is
        if self.variable_metadata.contains_key(&var_id) {
            self.flush_soon = true;
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
        bytecode_loc: BytecodeLoc,
        probe_id: String,
    },
}
impl From<&LocationData> for Metadata {
    fn from(loc: &LocationData) -> Self {
        match loc {
            LocationData::Local {
                script_id,
                bytecode_loc,
                probe_id,
                ..
            } => Self::Local {
                name: "".to_string(),
                script_id: script_id.clone(),
                bytecode_loc: bytecode_loc.clone(),
                probe_id: probe_id.clone(),
            },
            LocationData::Global { script_id } => Self::Global {
                name: "".to_string(),
                script_id: script_id.clone(),
            },
        }
    }
}
impl Metadata {
    pub fn new(name: String, loc: &LocationData) -> Self {
        let mut meta = Self::from(loc);
        meta.set_name(name);
        meta
    }
    pub fn set_name(&mut self, new_name: String) {
        match self {
            Self::Local { name, .. } | Self::Global { name, .. } => *name = new_name,
        }
    }
    pub fn get_csv_header() -> String {
        r#"
==================== REPORT CSV FLUSH ====================
type, id, name, script_id, (fid, pc), probe_id, value(s)"#
            .to_string()
    }
    pub fn to_csv(&self) -> String {
        let (name, script_id, bytecode_loc, probe_id) = match self {
            Metadata::Global { name, script_id } => (name.as_str(), script_id.as_str(), "", ""),
            Metadata::Local {
                name,
                script_id,
                bytecode_loc,
                probe_id,
            } => (
                name.as_str(),
                script_id.as_str(),
                &*bytecode_loc.to_string(),
                probe_id.as_str(),
            ),
        };
        format!("{},{},{},{}", name, script_id, bytecode_loc, probe_id)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocationData {
    Global {
        script_id: String,
    },
    Local {
        script_id: String,
        bytecode_loc: BytecodeLoc,
        probe_id: String,
        num_reports: i32,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BytecodeLoc {
    fid: u32,
    pc: u32,
}
impl Display for BytecodeLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.fid, self.pc)
    }
}
impl BytecodeLoc {
    pub(crate) fn new(fid: u32, pc: u32) -> Self {
        Self { fid, pc }
    }
}

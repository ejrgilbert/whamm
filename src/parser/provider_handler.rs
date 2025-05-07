use std::fs;
use glob::glob;
use serde::{Deserialize, Serialize};
use crate::parser::types::{DataType, Expr};

pub fn yml_to_providers(base_dir: &str) -> Vec<Provider> {
    let def = read_yml(base_dir);

    todo!()
}

fn read_yml(base_dir_tmp: &str) -> YmlDefinition {
    let base_dir = if base_dir_tmp.ends_with("/") {
        base_dir_tmp.strip_suffix("/").unwrap()
    } else {
        base_dir_tmp
    };

    let mut yml_files = vec![];

    // push events first (sets up the anchors)
    println!("{base_dir}/providers/packages/events/*.yaml");
    for path in glob(&format!("{base_dir}/providers/packages/events/*.yaml")).expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        println!("hi");
        println!("{unparsed_file}");
        yml_files.push(unparsed_file);
    }

    // push packages next (sets up the anchors)
    for path in glob(&format!("{base_dir}/providers/packages/*.yaml")).expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        println!("hi");
        println!("{unparsed_file}");
        yml_files.push(unparsed_file);
    }

    // finally the providers
    for path in glob(&format!("{base_dir}/providers/*.yaml")).expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        println!("hi");
        println!("{unparsed_file}");
        yml_files.push(unparsed_file);
    }

    let mut all_yml = "".to_string();
    for yml in yml_files.iter() {
        all_yml += yml;
    }

    println!("{all_yml}");

    let def: YmlDefinition =
        serde_yml::from_str(&all_yml).expect("Could not read values.");
    println!("{:?}", def);

    def
}

// ===============================
// ==== TYPES FOR PROBE RULES ====
// ===============================

#[derive(Debug)]
pub struct Provider {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<BoundFunc>,
    docs: String,
    packages: Vec<Package>
}

#[derive(Debug)]
pub struct Package {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<BoundFunc>,
    docs: String,
    events: Vec<Event>,
}

#[derive(Debug)]
pub struct Event {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<BoundFunc>,
    supported_modes: Vec<Mode>,
    req_map: bool,      // TODO: Remove this...maybe make it request a list of libraries?
    docs: String,
}

#[derive(Debug)]
pub struct BoundVar {
    name: String,
    docs: String,
    ty: DataType,
    derived_from: Option<Expr>
}

#[derive(Debug)]
pub struct BoundFunc {
    name: String,
    params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    results: DataType,
    req_args: i32,      // TODO: Remove this...it's wasm opcode specific...
    docs: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mode {
    name: String,
    docs: String
}

// =====================
// ==== IR FOR YAML ====
// =====================

#[derive(Debug, Serialize, Deserialize)]
struct YmlDefinition {
    providers: Vec<ProviderDef>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ProviderDef {
    name: String,
    bound_vars: Vec<BoundVarDef>,
    bound_fns: Vec<BoundFuncDef>,
    docs: String,
    packages: Vec<PackageDef>
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageDef {
    name: String,
    bound_vars: Vec<BoundVarDef>,
    bound_fns: Vec<BoundFuncDef>,
    docs: String,
    events: Vec<EventDef>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventDef {
    name: String,
    bound_vars: Vec<BoundVarDef>,
    bound_fns: Vec<BoundFuncDef>,
    supported_modes: Vec<ModeDef>,
    req_map: bool,      // TODO: Remove this...maybe make it request a list of libraries?
    docs: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BoundVarDef {
    name: String,
    docs: String,
    #[serde(rename = "type")]
    ty: String,
    derived_from: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct BoundFuncDef {
    name: String,
    params: String,
    results: String,
    req_args: i32,      // TODO: Remove this...it's wasm opcode specific...
    docs: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ModeDef {
    name: String,
    docs: String
}
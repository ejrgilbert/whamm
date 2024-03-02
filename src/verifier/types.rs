use log::{debug, error};

use std::any::Any;
use std::collections::HashMap;
use std::process::exit;
use std::str::FromStr;

pub struct SymbolTable {
    scopes: Vec<Scope>,
    curr_scope: usize, // Index into `scopes` that stores our current scope

    // TODO -- could split out
    records: Vec<Box<dyn Record>>,
    curr_dscript_rec: usize,
    curr_probe_rec: usize,
    curr_method_rec: usize
}

impl SymbolTable {
    pub fn new() -> Self {
        let root_scope = Scope::new(0, None);
        return SymbolTable {
            scopes: vec![root_scope],
            curr_scope: 0,
            records: vec![],
            curr_dscript_rec: 0,
            curr_probe_rec: 0,
            curr_method_rec: 0
        }
    }

    pub fn reset(&mut self) {
        for scope in self.scopes.iter_mut() {
            scope.reset();
        }
    }

    fn print_scope(&self, scope: &Scope) -> String {
        let mut res = "".to_string();

        // Print scope info
        let scope_info = format!("{}[ {} ]", scope.name, scope.ty.to_string());

        // Print all the scope's records
        for (name, record_idx) in scope.records.iter() {
            let rec = self.records.get(*record_idx).unwrap();

            let rec_info = format!("{} - {}", *rec.get_name(), (*rec.get_ty()).to_string().as_str());
            res += format!(
                "| {:<20} | {:<20} | {:<20} |",
                name, rec_info, scope_info
            ).as_str();
        }

        return res
    }

    pub fn print(&self) {
        let mut res = "".to_string();
        res += "\nPrinting the Symbol Table\n";
        res += format!("+{:-<70}+", "").as_str();

        res += format!(
            "| {:<20} | {:<20} | {:<20} |",
            "ID", "RECORD", "SCOPE"
        ).as_str();
        res += format!("+{:-<70}+", "").as_str();

        for scope in self.scopes.iter() {
            res += self.print_scope(&scope).as_str();
        }
        res += format!("+{:-<70}+", "").as_str();

        debug!("{res}");
    }

    // ---- Records ----

    pub fn get_dscript(&self, id: usize) -> &DscriptRecord {
        match self.records.get(id).unwrap().as_any().downcast_ref::<DscriptRecord>() {
            Some(d) => d,
            None => {
                error!("Something went wrong! Couldn't downcast to DscriptRecord");
                exit(1);
            }
        }
    }

    pub fn get_dscript_mut(&mut self, id: usize) -> &mut DscriptRecord {
        match self.records.get_mut(id).unwrap().as_any_mut().downcast_mut::<DscriptRecord>() {
            Some(d) => d,
            None => {
                error!("Something went wrong! Couldn't downcast to DscriptRecord");
                exit(1);
            }
        }
    }

    pub fn get_curr_dscript(&self) -> &DscriptRecord {
        self.get_dscript(self.curr_dscript_rec)
    }

    pub fn get_curr_dscript_mut(&mut self) -> &mut DscriptRecord {
        self.get_dscript_mut(self.curr_dscript_rec)
    }

    pub fn get_probe(&self, id: usize) -> &ProbeRecord {
        match self.records.get(id).unwrap().as_any().downcast_ref::<ProbeRecord>() {
            Some(d) => d,
            None => {
                error!("Something went wrong! Couldn't downcast to ProbeRecord");
                exit(1);
            }
        }
    }

    pub fn get_curr_probe(&self) -> &ProbeRecord {
        self.get_probe(self.curr_probe_rec)
    }

    pub fn get_method(&self, id: usize) -> &MethodRecord {
        match self.records.get(id).unwrap().as_any().downcast_ref::<MethodRecord>() {
            Some(d) => d,
            None => {
                error!("Something went wrong! Couldn't downcast to MethodRecord");
                exit(1);
            }
        }
    }

    pub fn get_curr_method(&self) -> &MethodRecord {
        self.get_method(self.curr_method_rec)
    }

    pub fn add_probe(&mut self, new_probe_name: String) {
        match self.lookup(&new_probe_name) {
            Some(_) => {
                error!("Duplicate probe name [ {} ]", new_probe_name);
                // TODO -- failed!
                exit(1);
            },
            None => {
                // create record
                let curr_probe = ProbeRecord::new(
                    new_probe_name.clone(),
                    Box::new(ScopeType::Probe)
                );

                // add probe to dscript scope and record
                let id = self.put(new_probe_name.clone(), Box::new(curr_probe));
                let mut dscript: &mut DscriptRecord = self.get_curr_dscript_mut();
                dscript.add_probe(new_probe_name.clone(), id);

                // enter probe scope
                self.enter_scope();

                // set scope name and type
                self.set_curr_scope_info(new_probe_name.clone(), Box::new(ScopeType::Probe));
                // NOTE -- cannot return a probe...must be pulled out in calling function!
            }
        }
    }

    pub fn add_method(&mut self, new_method_name: String) {
        match self.lookup(&new_method_name) {
            Some(_) => {
                error!("Duplicate method name [ {new_method_name} ]");
                // TODO -- failed!
                exit(1);
            },
            None => {
                // create record
                let curr_method = MethodRecord::new(
                    new_method_name.clone(),
                    Box::new(ScopeType::Method)
                );

                // add method to current scope and dscript (only dscripts have methods for now)
                let id = self.put(new_method_name.clone(), Box::new(curr_method));
                let mut dscript = self.get_curr_dscript_mut();
                dscript.add_method(new_method_name.clone(), id);

                // enter method scope
                self.enter_scope();

                // set scope name and type
                self.set_curr_scope_info(new_method_name.clone(), Box::new(ScopeType::Method));
                // NOTE -- cannot return a method...must be pulled out in calling function!
            }
        }
    }

    pub fn put(&mut self, key: String, rec: Box<dyn Record>) -> usize {
        let rec_id = self.records.len();
        self.records.push(rec);

        self.get_curr_scope_mut().put(key.clone(), rec_id);

        rec_id
    }

    fn next_parent(&self, curr_idx: usize) -> Option<&Scope> {
        match self.scopes.get(curr_idx) {
            None => None,
            Some(curr) => {
                match curr.parent {
                    None => None,
                    Some(next_i) => self.scopes.get(next_i)
                }
            }
        }
    }

    pub fn lookup(&self, key: &String) -> Option<&Box<dyn Record>> {
        let curr = self.get_curr_scope();
        match curr.lookup(key) {
            Some(res) => {
                Some(self.records.get(*res).unwrap())
            },
            None => {
                let mut res_id = None;

                // Search the parent instead
                let mut next_scope = self.next_parent(curr.id);
                while res_id.is_none() && next_scope.is_some() {
                    // Perform lookup in next_scope (moving in the chain of parent scopes)
                    res_id = next_scope.unwrap().lookup(key);

                    next_scope = self.next_parent(curr.id);
                }
                Some(self.records.get(*res_id.unwrap()).unwrap())
            }
        }
    }

    // ---- Scopes ----

    pub fn set_curr_scope_info(&mut self, name: String, ty: Box<dyn RecordType>) {
        let mut curr = self.get_curr_scope_mut();
        curr.name = name;
        curr.ty = ty;
    }

    pub fn get_curr_scope(&self) -> &Scope {
        &self.scopes.get(self.curr_scope).unwrap()
    }

    pub fn get_curr_scope_mut(&mut self) -> &mut Scope {
        self.scopes.get_mut(self.curr_scope).unwrap()
    }

    pub fn get_curr_scope_name(&self) -> &String {
        &self.get_curr_scope().name
    }

    pub fn get_curr_scope_type(&self) -> &Box<dyn RecordType> {
        &self.get_curr_scope().ty
    }

    pub fn enter_scope(&mut self) {
        let new_id = self.scopes.len();
        let mut curr = self.get_curr_scope_mut();

        if curr.has_next() {
            curr.next_child();
            return;
        }

        // Does not have next child, create it
        let new_scope = Scope::new(
            new_id,
            Some(curr.id)
        );

        // Store new scope in the current scope's children
        curr.add_child(new_id);

        // Add new scope
        self.scopes.push(new_scope);
    }

    pub fn exit_scope(&mut self) {
        match self.get_curr_scope().parent {
            Some(parent) => self.curr_scope = parent.clone(),
            None => {
                error!("Attempted to exit current scope, but there was no parent to exit into.")
            }
        }
    }
}

// === Scoping ===

pub struct Scope {
    id: usize,
    name: String,
    ty: Box<dyn RecordType>, // Should be scope type

    next: usize,
    parent: Option<usize>,
    children: Vec<usize>,

    // Indices into the Vec of records stored in the SymbolTable
    records: HashMap<String, usize>
}

impl Scope {
    pub fn new(id: usize, parent: Option<usize>) -> Self {
        return Scope {
            id,
            name: "".to_string(),
            ty: Box::new(ScopeType::Null),
            next: 0,
            parent,
            children: vec![],
            records: HashMap::new()
        }
    }

    pub fn reset(&mut self) {
        self.next = 0;
    }

    // ---- records ----

    pub fn put(&mut self, key: String, rec: usize) {
        self.records.insert(key, rec);
    }

    pub fn lookup(&self, key: &String) -> Option<&usize> {
        // is the key in the current scope?
        match self.records.get(key) {
            Some(val) => Some(val),
            None => None
        }
    }

    // ---- Scoping ----

    pub fn add_child(&mut self, id: usize) {
        self.children.push(id);
    }

    pub fn has_next(&self) -> bool {
        self.next < self.children.len()
    }

    pub fn next_child(&mut self) -> &usize {
        if !self.has_next() {
            error!("Scope::next_child() should never be called without first checking that there is one.")
        }

        let next_child = self.children.get(self.next).unwrap();
        self.next += 1;
        next_child
    }
}

// === Records ===

pub trait Record {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_name(&self) -> &String;
    fn get_ty(&self) -> &Box<dyn RecordType>;
}

pub struct DscriptRecord {
    name: String,
    ty: Box<dyn RecordType>,

    probes: HashMap<String, usize>, // e.g. user defined probes
    globals: HashMap<String, usize>, // e.g. global variables provided by dtrace
    methods: HashMap<String, usize> // e.g. strcmp would go here
}

impl Record for DscriptRecord {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_ty(&self) -> &Box<dyn RecordType> {
        &self.ty
    }
}

impl DscriptRecord {
    pub fn new(name: String, ty: Box<dyn RecordType>) -> Self {
        let probes = HashMap::new();
        let globals = HashMap::new();
        let methods = HashMap::new();
        return DscriptRecord {
            name,
            ty,
            probes,
            globals,
            methods
        }
    }

    pub fn add_probe(&mut self, name: String, record: usize) {
        self.probes.insert(name, record);
    }

    pub fn get_probe(&self, name: String) -> Option<&usize> {
        return self.probes.get(&name);
    }

    pub fn add_method(&mut self, name: String, record: usize) {
        self.methods.insert(name, record);
    }

    pub fn get_method(&self, name: String) -> Option<&usize> {
        return self.methods.get(&name);
    }

    // pub fn print_methods(&self) {
    //     let mut res = "".to_string();
    //     for (_, method) in self.methods.iter() {
    //         res += format!("\t-> {}:{}\n", method.name, method.ty.to_string()).as_str();
    //     }
    // }

    pub fn add_global(&mut self, name: String, record: usize) {
        self.globals.insert(name, record);
    }

    pub fn get_global(&self, name: String) -> Option<&usize> {
        return self.globals.get(&name);
    }

    // pub fn print_globals(&self) {
    //     let mut res = "".to_string();
    //     for (_, global) in self.globals.iter() {
    //         res += format!("\tGLOBAL: {}:{}\n", global.name, global.ty.to_string()).as_str();
    //     }
    //     debug!("{res}");
    // }
}

pub struct MethodRecord {
    pub name: String,
    pub ty: Box<dyn RecordType>,

    params: HashMap<i32, usize>,
    next_param_idx: i32,
}

impl Record for MethodRecord {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_ty(&self) -> &Box<dyn RecordType> {
        &self.ty
    }
}

impl MethodRecord {
    pub fn new(name: String, ty: Box<dyn RecordType>) -> Self {
        let params = HashMap::new();
        return MethodRecord {
            name,
            ty,
            params,
            next_param_idx: -1
        }
    }

    pub fn add_param(&mut self, param: usize) {
        self.params.insert(self.next_param_idx, param);
        self.next_param_idx += 1;
    }

    // pub fn contains_param(&self, name: String) -> bool {
    //     for (_, param) in self.params.iter() {
    //         if param.name == name {
    //             return true;
    //         }
    //     }
    //     return false;
    // }

    pub fn print_params(&self) -> String {
        let mut res = "(".to_string();
        for (i, param) in self.params.iter() {
            // TODO -- fix the printing
            res += format!("\t\t{i}:{:?}\n", param.to_string()).as_str();
        }
        res += ")";
        res
    }

    pub fn to_string(&self) -> String {
        let mut res = "".to_string();

        res += "MethodRecord [";
        res += format!("\tname: {}", self.name).as_str();
        res += format!("\tty: {}", (*self.ty).to_string()).as_str();
        res += format!("\tparams: {}", self.print_params()).as_str();
        res += format!("\tnext_param_idx: {}", self.next_param_idx).as_str();
        res += "]";

        res
    }
}

pub struct ProbeRecord {
    pub name: String,
    pub ty: Box<dyn RecordType>,

    // Populated by the provider, module, function, or name of the probe!
    // e.g. argN, target_fn_*, etc.
    locals: HashMap<String, usize>,
}

impl Record for ProbeRecord {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_ty(&self) -> &Box<dyn RecordType> {
        &self.ty
    }
}

impl ProbeRecord {
    pub fn new(name: String, ty: Box<dyn RecordType>) -> Self {
        let locals = HashMap::new();
        return ProbeRecord {
            name,
            ty,
            locals
        }
    }

    pub fn add_local(&mut self, name: String, record: usize) {
        self.locals.insert(name, record);
    }

    pub fn get_local(&self, name: String) -> Option<&usize> {
        return self.locals.get(name.as_str());
    }

    // pub fn print_locals(&self) {
    //     let mut res = "".to_string();
    //     for (_, local) in self.locals.iter() {
    //         res += format!("\tLOCAL: {}:{}\n", local.name, local.ty.to_string()).as_str();
    //     }
    //     debug!("{res}");
    // }
}

pub struct VarRecord {
    pub name: String,
    pub ty: Box<dyn RecordType>,
}

impl Record for VarRecord {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_ty(&self) -> &Box<dyn RecordType> {
        &self.ty
    }
}

impl VarRecord {
    pub fn new(name: String, ty: Box<dyn RecordType>) -> Self {
        return VarRecord {
            name,
            ty
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = "".to_string();

        res += "VarRecord [";
        res += format!("\tname: {}", self.name).as_str();
        res += format!("\tty: {}", (*self.ty).to_string()).as_str();
        res += "]";

        res
    }
}

// === Types of Records That Can Exist ===

pub trait RecordType {
    fn to_string(&self) -> String;
}

#[derive(Clone, Debug, Eq, Hash)]
pub enum ScopeType {
    Dscript,
    Probe,
    Method,
    Null
}

impl RecordType for ScopeType {
    fn to_string(&self) -> String {
        match *self {
            ScopeType::Dscript => "Dscript".to_string(),
            ScopeType::Probe => "Probe".to_string(),
            ScopeType::Method => "Method".to_string(),
            ScopeType::Null => "Null".to_string(),
        }
    }
}

impl PartialEq for ScopeType {
    #[inline]
    fn eq(&self, other: &ScopeType) -> bool {
        match *self {
            ScopeType::Dscript => match other {
                ScopeType::Dscript => true,
                _ => false,
            },
            ScopeType::Probe => match other {
                ScopeType::Probe => true,
                _ => false,
            },
            ScopeType::Method => match other {
                ScopeType::Method => true,
                _ => false,
            },
            ScopeType::Null => match other {
                ScopeType::Null => true,
                _ => false,
            }
        }
    }

    #[inline]
    fn ne(&self, other: &ScopeType) -> bool {
        match *self {
            ScopeType::Dscript => match other {
                ScopeType::Dscript => false,
                _ => true,
            },
            ScopeType::Probe => match other {
                ScopeType::Probe => false,
                _ => true,
            },
            ScopeType::Method => match other {
                ScopeType::Method => false,
                _ => true,
            },
            ScopeType::Null => match other {
                ScopeType::Null => false,
                _ => true,
            }
        }
    }
}

impl FromStr for ScopeType {
    type Err = ();

    fn from_str(input: &str) -> Result<ScopeType, ()> {
        match input.to_uppercase().as_str() {
            "DSCRIPT" => Ok(ScopeType::Dscript),
            "PROBE" => Ok(ScopeType::Probe),
            "METHOD" => Ok(ScopeType::Method),
            "NULL" => Ok(ScopeType::Null),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash)]
pub enum VarType {
    Int,
    IntArray,
    Boolean,
    Identifier
}

impl RecordType for VarType {
    fn to_string(&self) -> String {
        match *self {
            VarType::Int => "Int".to_string(),
            VarType::IntArray => "IntArray".to_string(),
            VarType::Boolean => "Boolean".to_string(),
            VarType::Identifier => "Identifier".to_string(),
        }
    }
}

impl PartialEq for VarType {
    #[inline]
    fn eq(&self, other: &VarType) -> bool {
        match *self {
            VarType::Int => match other {
                VarType::Int => true,
                _ => false,
            },
            VarType::IntArray => match other {
                VarType::IntArray => true,
                _ => false,
            },
            VarType::Boolean => match other {
                VarType::Boolean => true,
                _ => false,
            },
            VarType::Identifier => match other {
                VarType::Identifier => true,
                _ => false,
            }
        }
    }

    #[inline]
    fn ne(&self, other: &VarType) -> bool {
        match *self {
            VarType::Int => match other {
                VarType::Int => false,
                _ => true,
            },
            VarType::IntArray => match other {
                VarType::IntArray => false,
                _ => true,
            },
            VarType::Boolean => match other {
                VarType::Boolean => false,
                _ => true,
            },
            VarType::Identifier => match other {
                VarType::Identifier => false,
                _ => true,
            }
        }
    }
}

impl FromStr for VarType {
    type Err = ();

    fn from_str(input: &str) -> Result<VarType, ()> {
        match input.to_uppercase().as_str() {
            "INT" => Ok(VarType::Int),
            "INT_ARRAY" => Ok(VarType::IntArray),
            "BOOLEAN" => Ok(VarType::Boolean),
            "IDENTIFIER" => Ok(VarType::Identifier),
            _ => Err(()),
        }
    }
}
use crate::parser::types as parser_types;
use parser_types::{AstNode, CoreProbeName, WasmProbeName};
use crate::verifier::providers;


use log::{ debug, error };
use std::any::Any;
use std::collections::HashMap;
use std::env::{var, vars};
use std::process::exit;
use std::str::FromStr;
use walrus::FunctionKind;
use walrus::ir::Instr;
use crate::parser::types::Op;

pub struct SymbolTable {
    scopes: Vec<Scope>,
    curr_scope: usize, // Index into `scopes` that stores our current scope

    records: Vec<Box<dyn Record>>,
    curr_dtrace_rec: usize,
    curr_dscript_rec: usize,
    curr_probe_rec: usize,
    curr_method_rec: usize
}

impl SymbolTable {
    pub fn new() -> Self {
        // Create DtraceCore scope
        let ty = ScopeType::DtraceCore;
        let root_scope = Scope::new(0, ty.to_string(), Box::new(ty), None);

        // Create DtraceCore record
        let rec_name = "DtraceCore";
        let curr_dtrace_core = DtraceCoreRecord::new(
            rec_name.clone().to_string(),
            Box::new(ScopeType::DtraceCore)
        );

        let mut table = SymbolTable {
            scopes: vec![ root_scope ],
            curr_scope: 0,
            records: vec![],
            curr_dtrace_rec: 0,
            curr_dscript_rec: 0,
            curr_probe_rec: 0,
            curr_method_rec: 0
        };

        // add symbols from the DtraceCore provider
        table.put(rec_name.clone().to_string(), Box::new(curr_dtrace_core));

        let dtrace_provider = DtraceCoreProvider::new();
        dtrace_provider.add_symbols(&mut table);

        table
    }

    pub fn reset(&mut self) {
        for scope in self.scopes.iter_mut() {
            scope.reset();
        }
    }

    fn print_scope(&self, scope: &Scope, col_width: usize) -> String {
        let mut res = "".to_string();

        // Print scope info
        let scope_info = format!("{}[ {} ]", scope.name, scope.ty.to_string());

        // Print all the scope's records
        for (name, record_idx) in scope.records.iter() {
            let rec = self.records.get(*record_idx).unwrap();

            let rec_info = format!("{} - {}", *rec.get_name(), (*rec.get_ty()).to_string().as_str());
            res += format!(
                "| {:<width$} | {:<width$} | {:<width$} |\n",
                name, rec_info, scope_info,
                width = col_width
            ).as_str();
        }

        return res
    }

    pub fn print(&self) {
        let mut res = "".to_string();
        res += "\nPrinting the Symbol Table\n";
        let col_width: usize = 30;
        let num_cols: usize = 3;
        let extra_chars: usize = 8;

        res += format!("+{:-<width$}+\n", "", width = (col_width * num_cols) + extra_chars).as_str();

        res += format!(
            "| {:<width$} | {:<width$} | {:<width$} |\n",
            "ID: \"rec_name\"", "RECORD: \"rec_name - rec_ty\"", "SCOPE: \"scp_name[ scp_ty ]\"",
            width = col_width
        ).as_str();
        res += format!("+{:-<width$}+\n", "", width = (col_width * num_cols) + extra_chars).as_str();

        for scope in self.scopes.iter() {
            res += self.print_scope(&scope, col_width).as_str();
        }
        res += format!("+{:-<width$}+\n", "", width = (col_width * num_cols) + extra_chars).as_str();

        debug!("{res}");
    }

    // ---- Records ----

    pub fn get_dtrace_core_mut(&mut self, id: usize) -> &mut DtraceCoreRecord {
        match self.records.get_mut(id).unwrap().as_any_mut().downcast_mut::<DtraceCoreRecord>() {
            Some(d) => d,
            None => {
                error!("Something went wrong! Couldn't downcast to DtraceCoreRecord");
                exit(1);
            }
        }
    }

    pub fn get_curr_dtrace_core_mut(&mut self) -> &mut DtraceCoreRecord {
        self.get_dtrace_core_mut(self.curr_dscript_rec)
    }

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

    pub fn get_probe_mut(&mut self, id: usize) -> &mut ProbeRecord {
        match self.records.get_mut(id).unwrap().as_any_mut().downcast_mut::<ProbeRecord>() {
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

    pub fn get_curr_probe_mut(&mut self) -> &mut ProbeRecord {
        self.get_probe_mut(self.curr_probe_rec)
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

    pub fn add_dscript(&mut self, new_dscript_name: String) -> usize {
        match self.lookup(&new_dscript_name) {
            Some(_) => {
                error!("Duplicate dscript name [ {} ]", new_dscript_name);
                exit(1);
            },
            None => {
                // enter dscript scope
                self.enter_scope();

                // create record
                let curr_dscript = DscriptRecord::new(
                    new_dscript_name.clone(),
                    Box::new(ScopeType::Dscript)
                );

                // add dscript record
                let id = self.put(new_dscript_name.clone(), Box::new(curr_dscript));

                // set scope name and type
                self.curr_dscript_rec = id;
                self.set_curr_scope_info(new_dscript_name.clone(), Box::new(ScopeType::Dscript));
                // NOTE -- cannot return a probe...must be pulled out in calling function!
                id
            }
        }
    }

    pub fn add_probe(&mut self, new_probe_name: String) -> usize {
        match self.lookup(&new_probe_name) {
            Some(_) => {
                error!("Duplicate probe name [ {} ]", new_probe_name);
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
                let dscript: &mut DscriptRecord = self.get_curr_dscript_mut();
                dscript.add_probe(new_probe_name.clone(), id);

                // enter probe scope
                self.enter_scope();

                // set scope name and type
                self.curr_probe_rec = id;
                self.set_curr_scope_info(new_probe_name.clone(), Box::new(ScopeType::Probe));

                // Add symbols for probe provider
                let probe_provider = ProbeProvider::new();
                probe_provider.add_symbols(self);

                // NOTE -- cannot return a probe...must be pulled out in calling function!
                id
            }
        }
    }

    pub fn add_core_method(&mut self, new_method_name: String) -> usize {
        match self.lookup(&new_method_name) {
            Some(_) => {
                error!("Duplicate method name [ {new_method_name} ]");
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
                let dtrace_core = self.get_curr_dtrace_core_mut();
                dtrace_core.add_method(new_method_name.clone(), id);

                // enter method scope
                self.enter_scope();

                // set scope name and type
                self.curr_method_rec = id;
                self.set_curr_scope_info(new_method_name.clone(), Box::new(ScopeType::Method));

                // NOTE -- cannot return a method...must be pulled out in calling function!
                id
            }
        }
    }

    // TODO pub fn add_method(&mut self, new_method_name: String) -> usize {
    //     match self.lookup(&new_method_name) {
    //         Some(_) => {
    //             error!("Duplicate method name [ {new_method_name} ]");
    //             exit(1);
    //         },
    //         None => {
    //             // create record
    //             let curr_method = MethodRecord::new(
    //                 new_method_name.clone(),
    //                 Box::new(ScopeType::Method)
    //             );
    //
    //             // add method to current scope and dscript (only dscripts have methods for now)
    //             let id = self.put(new_method_name.clone(), Box::new(curr_method));
    //             let dscript = self.get_curr_dscript_mut();
    //             dscript.add_method(new_method_name.clone(), id);
    //
    //             // enter method scope
    //             self.enter_scope();
    //
    //             // set scope name and type
    //             self.curr_method_rec = id;
    //             self.set_curr_scope_info(new_method_name.clone(), Box::new(ScopeType::Method));
    //             // NOTE -- cannot return a method...must be pulled out in calling function!
    //             id
    //         }
    //     }
    // }

    pub fn add_probe_local(&mut self, new_local_name: String) -> usize {
        match self.lookup(&new_local_name) {
            Some(_) => {
                error!("Duplicate local name [ {new_local_name} ]");
                exit(1);
            },
            None => {
                // create record
                let new_local = VarRecord::new(
                    new_local_name.clone(),
                    Box::new(ScopeType::Var)
                );

                // add field to current scope and probe
                let id = self.put(new_local_name.clone(), Box::new(new_local));
                let probe = self.get_curr_probe_mut();
                probe.add_local(new_local_name.clone(), id);

                id
            }
        }
    }

    pub fn put(&mut self, key: String, rec: Box<dyn Record>) -> usize {
        let rec_id = self.records.len();
        self.records.push(rec);

        self.get_curr_scope_mut().unwrap().put(key.clone(), rec_id);

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
        let c = self.get_curr_scope();
        match c {
            None => None,
            Some(curr) => {
                match curr.lookup(key) {
                    Some(res) => {
                        Some(self.records.get(*res).unwrap())
                    },
                    None => {
                        let mut res_id = None;

                        // Search the parent instead
                        let mut lookup_idx = curr.id;
                        let mut next_scope = self.next_parent(lookup_idx);
                        while res_id.is_none() && next_scope.is_some() {
                            // Perform lookup in next_scope (moving in the chain of parent scopes)
                            res_id = next_scope.unwrap().lookup(key);

                            lookup_idx = next_scope.unwrap().id;
                            next_scope = self.next_parent(lookup_idx);
                        }
                        if res_id.is_none() {
                            return None;
                        }
                        Some(self.records.get(*res_id.unwrap()).unwrap())
                    }
                }
            }
        }
    }

    // ---- Scopes ----

    pub fn set_curr_scope_info(&mut self, name: String, ty: Box<dyn RecordType>) {
        let mut curr = self.get_curr_scope_mut().unwrap();
        curr.name = name;
        curr.ty = ty;
    }

    pub fn get_curr_scope(&self) -> Option<&Scope> {
        self.scopes.get(self.curr_scope)
    }

    pub fn get_curr_scope_mut(&mut self) -> Option<&mut Scope> {
        self.scopes.get_mut(self.curr_scope)
    }

    pub fn get_curr_scope_name(&self) -> &String {
        &self.get_curr_scope().unwrap().name
    }

    pub fn get_curr_scope_type(&self) -> &Box<dyn RecordType> {
        &self.get_curr_scope().unwrap().ty
    }

    pub fn enter_scope(&mut self) {
        let new_id = self.scopes.len();
        let parent = match self.get_curr_scope_mut() {
            Some(curr) => {
                if curr.has_next() {
                    curr.next_child();
                    return;
                }
                // Will need to create a new next scope
                // Store new scope in the current scope's children
                curr.add_child(new_id);
                Some(curr.id)
            },
            _ => {
                None
            }
        };

        // Does not have next child, create it
        let new_scope = Scope::new(
            new_id,
            "".to_string(),
            Box::new(ScopeType::Null),
            parent
        );

        // Add new scope
        self.scopes.push(new_scope);
        self.curr_scope = new_id;
    }

    pub fn exit_scope(&mut self) {
        match self.get_curr_scope().unwrap().parent {
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
    pub name: String,
    ty: Box<dyn RecordType>, // Should be scope type

    next: usize,
    parent: Option<usize>,
    children: Vec<usize>,

    // Indices into the Vec of records stored in the SymbolTable
    records: HashMap<String, usize>
}

impl Scope {
    pub fn new(id: usize, name: String, ty: Box<ScopeType>, parent: Option<usize>) -> Self {
        return Scope {
            id,
            name,
            ty,
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
        self.next += 1; // TODO -- check this
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

pub struct DtraceCoreRecord {
    name: String,
    ty: Box<dyn RecordType>,

    globals: HashMap<String, usize>, // e.g. global variables provided by dtrace
    methods: HashMap<String, usize> // e.g. strcmp would go here
}

impl Record for DtraceCoreRecord {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_ty(&self) -> &Box<dyn RecordType> {
        &self.ty
    }
}

impl DtraceCoreRecord {
    pub fn new(name: String, ty: Box<dyn RecordType>) -> Self {
        let globals = HashMap::new();
        let methods = HashMap::new();
        return DtraceCoreRecord {
            name,
            ty,
            globals,
            methods
        }
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

pub struct DscriptRecord {
    name: String,
    ty: Box<dyn RecordType>,

    probes: HashMap<String, usize>, // e.g. user defined probes
    // TODO methods: HashMap<String, usize> // e.g. user defined methods
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
        return DscriptRecord {
            name,
            ty,
            probes
        }
    }

    pub fn add_probe(&mut self, name: String, record: usize) {
        self.probes.insert(name, record);
    }

    pub fn get_probe(&self, name: String) -> Option<&usize> {
        return self.probes.get(&name);
    }
}

pub struct MethodRecord {
    pub name: String,
    pub ty: Box<dyn RecordType>,

    params: HashMap<i32, usize>,
    next_param_idx: i32,
    used: bool
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
            next_param_idx: -1,
            used: false
        }
    }

    pub fn add_param(&mut self, param: usize) {
        self.params.insert(self.next_param_idx, param);
        self.next_param_idx += 1;
    }

    pub fn mark_used(&mut self) {
        self.used = true;
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
        res += format!("\tused: {}", self.used).as_str();
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
    DtraceCore,
    Dscript,
    Probe,
    Method,
    Var,
    Null
}

impl RecordType for ScopeType {
    fn to_string(&self) -> String {
        match *self {
            ScopeType::DtraceCore => "DtraceCore".to_string(),
            ScopeType::Dscript => "Dscript".to_string(),
            ScopeType::Probe => "Probe".to_string(),
            ScopeType::Method => "Method".to_string(),
            ScopeType::Var => "Var".to_string(),
            ScopeType::Null => "Null".to_string(),
        }
    }
}

impl PartialEq for ScopeType {
    #[inline]
    fn eq(&self, other: &ScopeType) -> bool {
        match *self {
            ScopeType::DtraceCore => match other {
                ScopeType::DtraceCore => true,
                _ => false,
            },
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
            ScopeType::Var => match other {
                ScopeType::Var => true,
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
            ScopeType::DtraceCore => match other {
                ScopeType::DtraceCore => false,
                _ => true,
            },
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
            ScopeType::Var => match other {
                ScopeType::Var => false,
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
            "DTRACECORE" => Ok(ScopeType::DtraceCore),
            "DSCRIPT" => Ok(ScopeType::Dscript),
            "PROBE" => Ok(ScopeType::Probe),
            "METHOD" => Ok(ScopeType::Method),
            "VAR" => Ok(ScopeType::Var),
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

// ** Organized AST **

// ==================
// = Helper Methods =
// ==================

pub fn unbox(contents: &Vec<Box<AstNode>>) -> Vec<AstNode> {
    contents.into_iter().map(|item| {
        *item.clone()
    }).collect()
}

// =========
// = Types =
// =========

// OLD TYPES BELOW

pub struct AllWasmFnProbes {
    pub(crate) all_probes: HashMap<String, WasmFnProbes>
}

impl AllWasmFnProbes {
    pub fn new() -> Self {
        return AllWasmFnProbes {
            all_probes: HashMap::new()
        }
    }

    fn add_or_append(&mut self, module: &String, function: &String, probe_type: &WasmProbeName, probe: WasmProbe) {
        let mut new_fn_probes = WasmFnProbes::new(module.clone(), function.clone()); // might not be used

        self.all_probes.entry(format!("{module}.{function}"))
            .and_modify(|fn_probes| fn_probes.add_probe(module, function, probe_type, probe.clone()))
            .or_insert_with(|| {
                new_fn_probes.add_probe(module, function, probe_type, probe);
                new_fn_probes
            });
    }

    pub fn add_probe(&mut self, module: &String, function: &String, probe_type: &WasmProbeName, probe: WasmProbe) {
        self.add_or_append(module, function, probe_type, probe);
    }
}

pub struct WasmFnProbes {
    pub(crate) module: String,
    pub(crate) function: String,
    pub(crate) fn_probes: HashMap<WasmProbeName, Vec<WasmProbe>>
}

impl WasmFnProbes {
    pub fn new(module: String, function: String) -> Self {
        let mut fps = HashMap::new();
        fps.insert(WasmProbeName::Before, Vec::new());
        fps.insert(WasmProbeName::After, Vec::new());
        fps.insert(WasmProbeName::Alt, Vec::new());

        return WasmFnProbes {
            module,
            function,
            fn_probes: fps
        }
    }

    pub fn add_probe(&mut self, module: &String, function: &String, probe_type: &WasmProbeName, probe: WasmProbe) {
        if self.module != *module && self.function != *function {
            println!("ERROR: trying to add probe with mismatching clause. Expected: {}:{}. Actual: {module}:{function}.", self.module, self.function);
            return;
        }

        self.fn_probes.get_mut(&probe_type).unwrap().push(probe);
    }
}

trait Probe {}

#[derive(Clone, Debug)]
pub struct WasmProbe {
    // pub(crate) id: usize, // The id of this probe's scope (for SymbolTable lookup)
    pub(crate) predicate: Option<AstNode>,
    pub(crate) body: Option<Vec<AstNode>>
}
impl Probe for WasmProbe {}

// TODO -- CoreProbe code generation
#[derive(Clone, Debug)]
pub struct CoreProbe {
    // pub(crate) id: usize, // The id of this probe's scope (for SymbolTable lookup)
    pub(crate) name: CoreProbeName,
    pub(crate) body: Option<Vec<AstNode>>
}
impl Probe for CoreProbe {}
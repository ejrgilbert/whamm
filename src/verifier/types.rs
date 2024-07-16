use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::{DataType, FnId, Location, ProbeSpec, Value};
use std::collections::HashMap;
use std::fmt;
use walrus::{FunctionId, GlobalId, LocalId};

const UNEXPECTED_ERR_MSG: &str =
    "SymbolTable: Looks like you've found a bug...please report this behavior!";

#[derive(Debug)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    curr_scope: usize, // indexes into this::scopes

    pub records: Vec<Record>,
    pub curr_rec: usize, // indexes into this::records
}
impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
impl SymbolTable {
    pub fn new() -> Self {
        let root_scope = Scope::new(0, "".to_string(), ScopeType::Null, None);

        SymbolTable {
            scopes: vec![root_scope],
            curr_scope: 0,
            records: vec![],
            curr_rec: 0,
        }
    }

    // Scope operations

    pub fn get_curr_scope(&self) -> Option<&Scope> {
        self.scopes.get(self.curr_scope)
    }

    pub fn get_curr_scope_mut(&mut self) -> Option<&mut Scope> {
        self.scopes.get_mut(self.curr_scope)
    }

    pub fn set_curr_scope_info(&mut self, name: String, ty: ScopeType) {
        let curr = self.get_curr_scope_mut().unwrap();
        curr.name = name;
        curr.ty = ty;
    }

    pub fn reset(&mut self) {
        self.curr_scope = 0;
        for scope in self.scopes.iter_mut() {
            scope.reset();
        }
    }

    pub fn reset_children(&mut self) {
        let curr = self.get_curr_scope_mut().unwrap();
        curr.reset();
        let children = curr.children.clone();

        children.iter().for_each(|child| {
            let child_scope: &mut Scope = self.scopes.get_mut(*child).unwrap();
            child_scope.reset();
        });
    }

    pub fn enter_named_scope(&mut self, scope_name: &str) -> bool {
        let curr = self.get_curr_scope_mut().unwrap();
        let children = curr.children.clone();

        let mut new_curr_scope = None;
        let mut new_next = None;
        for (i, child_id) in children.iter().enumerate() {
            if let Some(child_scope) = self.scopes.get_mut(*child_id) {
                if child_scope.name == *scope_name {
                    new_curr_scope = Some(*child_id);
                    new_next = Some(i + 1);
                    child_scope.reset();
                }
            }
        }

        // create new instance fix Rust's compilation issue.
        let curr = self.get_curr_scope_mut().unwrap();
        if let (Some(new_curr), Some(new_next)) = (new_curr_scope, new_next) {
            curr.next = new_next;
            self.curr_scope = new_curr;
            return true;
        }

        false
    }

    pub fn enter_scope_via_spec(&mut self, script_id: &str, probe_spec: &ProbeSpec) -> bool {
        let mut is_success = true;

        is_success &= self.enter_named_scope(script_id);
        if let Some(provider) = &probe_spec.provider {
            is_success &= self.enter_named_scope(&provider.name);
            if let Some(package) = &probe_spec.package {
                is_success &= self.enter_named_scope(&package.name);
                if let Some(event) = &probe_spec.event {
                    is_success &= self.enter_named_scope(&event.name);
                    if let Some(mode) = &probe_spec.mode {
                        is_success &= self.enter_named_scope(&mode.name);
                    }
                }
            }
        }
        is_success
    }

    pub fn enter_scope(&mut self) -> Result<(), Box<WhammError>> {
        let new_id = self.scopes.len();

        let curr_scope = self.get_curr_scope_mut().unwrap();
        if curr_scope.has_next() {
            return match curr_scope.next_child() {
                Err(e) => Err(e),
                Ok(n) => {
                    self.curr_scope = *n;
                    Ok(())
                }
            };
        }
        // Will need to create a new next scope
        // Store new scope in the current scope's children
        curr_scope.add_child(new_id);

        // Does not have next child, create it
        let new_scope = Scope::new(new_id, "".to_string(), ScopeType::Null, Some(curr_scope.id));

        // Increment current scope's next child pointer
        curr_scope.next += 1;

        // Add new scope
        self.scopes.push(new_scope);
        self.curr_scope = new_id;
        Ok(())
    }

    pub fn exit_scope(&mut self) -> Result<(), Box<WhammError>> {
        match self.get_curr_scope().unwrap().parent {
            Some(parent) => self.curr_scope = parent,
            None => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{} Attempted to exit current scope, but there was no parent to exit into.",
                        UNEXPECTED_ERR_MSG
                    )),
                    None,
                )));
            }
        }
        Ok(())
    }

    // Record operations

    pub fn set_curr_script(&mut self, id: usize) {
        self.get_curr_scope_mut().unwrap().containing_script = Some(id);
    }

    pub fn remove_record(&mut self, symbol_name: &String) {
        match self.get_curr_scope_mut() {
            None => {
                // nothing to do
            }
            Some(curr) => {
                curr.records.remove(symbol_name);
            }
        }
    }

    pub fn get_record(&self, rec_id: &usize) -> Option<&Record> {
        self.records.get(*rec_id)
    }

    pub fn get_record_mut(&mut self, rec_id: &usize) -> Option<&mut Record> {
        self.records.get_mut(*rec_id)
    }

    pub fn get_curr_rec(&self) -> Option<&Record> {
        self.records.get(self.curr_rec)
    }

    pub fn get_curr_rec_mut(&mut self) -> Option<&mut Record> {
        self.records.get_mut(self.curr_rec)
    }

    pub fn put(&mut self, key: String, rec: Record) -> usize {
        let new_rec_id = self.records.len();
        match rec {
            Record::Whamm { .. }
            | Record::Script { .. }
            | Record::Provider { .. }
            | Record::Package { .. }
            | Record::Event { .. }
            | Record::Probe { .. } => {
                self.curr_rec = new_rec_id;
            }
            _ => {
                // ignore, not a record container we'd want to add to!
            }
        }

        self.records.push(rec);

        self.get_curr_scope_mut()
            .unwrap()
            .put(key.clone(), new_rec_id);

        new_rec_id
    }

    pub fn lookup(&self, key: &str) -> Option<&usize> {
        match self.get_curr_scope() {
            None => None,
            Some(curr) => {
                match curr.lookup(key) {
                    Some(rec_id) => Some(rec_id),
                    None => {
                        let mut rec_id = None;

                        // Search the parent instead
                        let mut lookup_scope = curr;
                        let mut next_parent: Option<&Scope> = match lookup_scope.parent {
                            None => None,
                            Some(p_id) => self.scopes.get(p_id),
                        };
                        while rec_id.is_none() && next_parent.is_some() {
                            // Perform lookup in next_parent (moving in the chain of parent scopes)
                            rec_id = next_parent.unwrap().lookup(key);

                            lookup_scope = next_parent.unwrap();
                            next_parent = match lookup_scope.parent {
                                None => None,
                                Some(p_id) => self.scopes.get(p_id),
                            };
                        }

                        match rec_id {
                            None => None,
                            Some(id) => Some(id),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    pub id: usize, // indexes into SymbolTable::scopes
    pub name: String,
    pub ty: ScopeType,

    parent: Option<usize>, // indexes into SymbolTable::scopes
    children: Vec<usize>,  // indexes into SymbolTable::scopes
    next: usize,           // indexes into this::children

    pub containing_script: Option<usize>, // indexes into SymbolTable::records
    records: HashMap<String, usize>,      // indexes into SymbolTable::records
}
impl Scope {
    pub fn new(id: usize, name: String, ty: ScopeType, parent: Option<usize>) -> Self {
        Scope {
            id,
            name,
            ty,

            containing_script: None,
            next: 0,
            parent,
            children: vec![],

            records: HashMap::new(),
        }
    }

    pub fn set_metadata(&mut self, name: String, ty: ScopeType) {
        self.name = name;
        self.ty = ty;
    }

    // Scoping operations

    pub fn add_child(&mut self, id: usize) {
        self.children.push(id);
    }

    pub fn has_next(&self) -> bool {
        self.next < self.children.len()
    }

    pub fn next_child(&mut self) -> Result<&usize, Box<WhammError>> {
        if !self.has_next() {
            return Err(Box::new(ErrorGen::get_unexpected_error(true,
              Some(format!("{} Scope::next_child() should never be called without first checking that there is one.", UNEXPECTED_ERR_MSG)),
              None)));
        }

        let next_child = self.children.get(self.next).unwrap();
        self.next += 1;
        Ok(next_child)
    }

    pub fn reset(&mut self) {
        self.next = 0;
    }

    // Record operations

    pub fn put(&mut self, key: String, rec: usize) {
        self.records.insert(key, rec);
    }

    /// Is the key in the current scope?
    pub fn lookup(&self, key: &str) -> Option<&usize> {
        self.records.get(key)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ScopeType {
    Whamm,
    Script,
    Provider,
    Package,
    Event,
    Probe,
    Fn,
    Null,
}
impl fmt::Display for ScopeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScopeType::Whamm { .. } => {
                write!(f, "Whamm")
            }
            ScopeType::Script { .. } => {
                write!(f, "Script")
            }
            ScopeType::Provider { .. } => {
                write!(f, "Provider")
            }
            ScopeType::Package { .. } => {
                write!(f, "Package")
            }
            ScopeType::Event { .. } => {
                write!(f, "Event")
            }
            ScopeType::Probe { .. } => {
                write!(f, "Probe")
            }
            ScopeType::Fn { .. } => {
                write!(f, "Fn")
            }
            ScopeType::Null { .. } => {
                write!(f, "Null")
            }
        }
    }
}

/// The usize values in the record fields index into the SymbolTable::records Vec.
#[derive(Debug)]
pub enum Record {
    Whamm {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        scripts: Vec<usize>,
    },
    Script {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        providers: Vec<usize>,
    },
    Provider {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        packages: Vec<usize>,
    },
    Package {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        events: Vec<usize>,
    },
    Event {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        probes: Vec<usize>,
    },
    Probe {
        mode: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
    },
    Fn {
        name: FnId,
        params: Vec<usize>,
        // given that we are assuming function that return nothing
        // returns a unit type (empty tuple)
        ret_ty: DataType,
        is_comp_provided: bool,

        /// The address of this function post-injection
        // TODO -- this representation SUCKS...specific to walrus bytecode injection...
        //         can't find another way though since I can't encode a FunctionId through the API
        //         ...maybe use type parameters?
        addr: Option<FunctionId>,
        loc: Option<Location>,
    },
    Var {
        ty: DataType,
        name: String,
        value: Option<Value>,
        is_comp_provided: bool, // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!

        /// The address of this var post-injection
        addr: Option<VarAddr>,
        loc: Option<Location>,
    },
}
impl Record {
    pub fn loc(&self) -> &Option<Location> {
        match self {
            Record::Fn { name, .. } => &name.loc,
            Record::Var { loc, .. } => loc,
            _ => &None,
        }
    }
    pub fn is_comp_provided(&self) -> bool {
        match self {
            Record::Fn {
                is_comp_provided, ..
            } => *is_comp_provided,
            Record::Var {
                is_comp_provided, ..
            } => *is_comp_provided,
            _ => true,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum VarAddr {
    Local { addr: LocalId },
    Global { addr: GlobalId },
}

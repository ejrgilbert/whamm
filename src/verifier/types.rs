use std::collections::HashMap;
use log::{ error };
use walrus::{FunctionId, GlobalId, LocalId};
use crate::parser::types::{DataType, Value};

#[derive(Debug)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    curr_scope: usize,    // indexes into this::scopes

    pub records: Vec<Record>,
    pub curr_rec: usize,    // indexes into this::records
}
impl SymbolTable {
    pub fn new() -> Self {
        let root_scope = Scope::new(0, "".to_string(), ScopeType::Null, None);

        SymbolTable {
            scopes: vec![ root_scope ],
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
        let mut curr = self.get_curr_scope_mut().unwrap();
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
        let mut curr = self.get_curr_scope_mut().unwrap();
        curr.reset();
        let children = curr.children.clone();

        children.iter().for_each(|child| {
            let child_scope: &mut Scope = self.scopes.get_mut(*child).unwrap();
            child_scope.reset();
        });
    }

    pub fn enter_named_scope(&mut self, scope_name: &String) {
        let mut curr = self.get_curr_scope_mut().unwrap();
        let children = curr.children.clone();

        let mut new_curr_scope = None;
        let mut new_next = None;
        for (i, child_id) in children.iter().enumerate() {
            let child_scope: &Scope = self.scopes.get(*child_id).unwrap();
            if child_scope.name == *scope_name {
                new_curr_scope = Some(child_id.clone());
                new_next = Some(i.clone() + 1);
            }
        };

        // create new instance fix Rust's compilation issue.
        let mut curr = self.get_curr_scope_mut().unwrap();
        if let (Some(new_curr), Some(new_next)) = (new_curr_scope, new_next) {
            curr.next = new_next;
            self.curr_scope = new_curr;
        } else {
            error!("Could not find the specified scope by name: `{scope_name}`");
        }
    }

    pub fn enter_scope(&mut self) {
        let new_id = self.scopes.len();

        let curr_scope = self.get_curr_scope_mut().unwrap();
        if curr_scope.has_next() {
            self.curr_scope = curr_scope.next_child().clone();
            return;
        }
        // Will need to create a new next scope
        // Store new scope in the current scope's children
        curr_scope.add_child(new_id);

        // Does not have next child, create it
        let new_scope = Scope::new(
            new_id,
            "".to_string(),
            ScopeType::Null,
            Some(curr_scope.id)
        );

        // Increment current scope's next child pointer
        curr_scope.next += 1;

        // Add new scope
        self.scopes.push(new_scope);
        self.curr_scope = new_id.clone();
    }

    pub fn exit_scope(&mut self) {
        match self.get_curr_scope().unwrap().parent {
            Some(parent) => self.curr_scope = parent.clone(),
            None => {
                error!("Attempted to exit current scope, but there was no parent to exit into.")
            }
        }
    }

    // Record operations

    pub fn set_curr_dscript(&mut self, id: usize) {
        self.get_curr_scope_mut().unwrap().containing_dscript = Some(id);
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
            Record::Dtrace { .. } |
            Record::Dscript { .. } |
            Record::Provider { .. } |
            Record::Module { .. } |
            Record::Function { .. } |
            Record::Probe { .. } => {
                self.curr_rec = new_rec_id.clone();
            }
            _ => {
                // ignore, not a record container we'd want to add to!
            }
        }

        self.records.push(rec);

        self.get_curr_scope_mut().unwrap().put(key.clone(), new_rec_id);

        new_rec_id
    }

    pub fn lookup(&self, key: &String) -> Option<&usize> {
        match self.get_curr_scope() {
            None => None,
            Some(curr) => {
                match curr.lookup(key) {
                    Some(rec_id) => {
                        Some(rec_id)
                    },
                    None => {
                        let mut rec_id = None;

                        // Search the parent instead
                        let mut lookup_scope = curr;
                        let mut next_parent: Option<&Scope> = match lookup_scope.parent {
                            None => None,
                            Some(p_id) => self.scopes.get(p_id)
                        };
                        while rec_id.is_none() && next_parent.is_some() {
                            // Perform lookup in next_parent (moving in the chain of parent scopes)
                            rec_id = next_parent.unwrap().lookup(key);

                            lookup_scope = next_parent.unwrap();
                            next_parent = match lookup_scope.parent {
                                None => None,
                                Some(p_id) => self.scopes.get(p_id)
                            };
                        }

                        match rec_id {
                            None => None,
                            Some(id) => Some(id)
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    pub id: usize,                       // indexes into SymbolTable::scopes
    pub name: String,
    pub ty: ScopeType,

    parent: Option<usize>,             // indexes into SymbolTable::scopes
    children: Vec<usize>,              // indexes into SymbolTable::scopes
    next: usize,                       // indexes into this::children

    pub containing_dscript: Option<usize>, // indexes into SymbolTable::records
    records: HashMap<String, usize>,   // indexes into SymbolTable::records
}
impl Scope {
    pub fn new(id: usize, name: String, ty: ScopeType, parent: Option<usize>) -> Self {
        Scope {
            id,
            name,
            ty,

            containing_dscript: None,
            next: 0,
            parent,
            children: vec![],

            records: HashMap::new()
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

    pub fn next_child(&mut self) -> &usize {
        if !self.has_next() {
            error!("Scope::next_child() should never be called without first checking that there is one.")
        }

        let next_child = self.children.get(self.next).unwrap();
        self.next += 1;
        next_child
    }

    pub fn reset(&mut self) {
        self.next = 0;
    }

    // Record operations

    pub fn put(&mut self, key: String, rec: usize) {
        self.records.insert(key, rec);
    }

    /// Is the key in the current scope?
    pub fn lookup(&self, key: &String) -> Option<&usize> {
        self.records.get(key)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ScopeType {
    Dtrace,
    Dscript,
    Provider,
    Module,
    Function,
    Probe,
    Fn,
    Null
}

/// The usize values in the record fields index into the SymbolTable::records Vec.
#[derive(Debug)]
pub enum Record {
    Dtrace {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        dscripts: Vec<usize>
    },
    Dscript {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        providers: Vec<usize>
    },
    Provider {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        modules: Vec<usize>
    },
    Module {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        functions: Vec<usize>
    },
    Function {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>,
        probes: Vec<usize>
    },
    Probe {
        name: String,
        fns: Vec<usize>,
        globals: Vec<usize>
    },
    Fn {
        name: String,
        params: Vec<usize>,

        /// The address of this function post-injection
        // TODO -- this representation SUCKS...specific to walrus bytecode injection...
        //         can't find another way though since I can't encode a FunctionId through the API
        //         ...maybe use type parameters?
        addr: Option<FunctionId>
    },
    Var {
        ty: DataType,
        name: String,
        value: Option<Value>,

        /// The address of this var post-injection
        addr: Option<VarAddr>
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum VarAddr {
    Local {
        addr: LocalId
    },
    Global {
        addr: GlobalId
    }
}
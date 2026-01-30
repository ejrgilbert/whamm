use crate::generator::ast::StackReq;
use crate::parser::types::{DataType, Definition, FnId, Location, ProbeRule, Value};
use pest::error::LineColLocation;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

const UNEXPECTED_ERR_MSG: &str =
    "SymbolTable: Looks like you've found a bug...please report this behavior!";

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    curr_scope: usize, // indexes into this::scopes

    pub records: Vec<Record>,
    pub curr_rec: usize, // indexes into this::records
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

    pub fn enter_named_scope(&mut self, scope_name: &str) -> bool {
        let curr = self.get_curr_scope_mut().unwrap();
        let children = curr.children.clone();

        let mut new_curr_scope = None;
        let mut new_next = None;
        for (i, child_id) in children.iter().enumerate() {
            if let Some(child_scope) = self.scopes.get_mut(*child_id) {
                if child_scope.name == scope_name {
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

    pub fn enter_scope_via_rule(
        &mut self,
        script_id: &str,
        probe_rule: &ProbeRule,
        scope_id: usize,
    ) -> bool {
        let mut is_success = true;

        self.reset();
        is_success &= self.enter_named_scope(script_id);
        if let Some(provider) = &probe_rule.provider {
            is_success &= self.enter_named_scope(&provider.name);
            if let Some(package) = &probe_rule.package {
                is_success &= self.enter_named_scope(&package.name);
                if let Some(event) = &probe_rule.event {
                    is_success &= self.enter_named_scope(&event.name);
                    if let Some(mode) = &probe_rule.mode {
                        is_success &= self.enter_named_scope(&mode.name);
                        is_success &= self.enter_named_scope(&scope_id.to_string());
                    }
                }
            }
        }
        is_success
    }

    pub fn enter_scope(&mut self) {
        let curr_scope = self.get_curr_scope_mut().unwrap();
        if curr_scope.has_next() {
            if let Some(n) = curr_scope.next_child() {
                self.curr_scope = *n;
            }
            return;
        }
        self.add_and_enter_new_scope()
    }

    pub fn exit_scope(&mut self) {
        match self.get_curr_scope().unwrap().parent {
            Some(parent) => self.curr_scope = parent,
            None => {
                unreachable!(
                    "{} Attempted to exit current scope, but there was no parent to exit into.",
                    UNEXPECTED_ERR_MSG
                );
            }
        }
    }

    // Used when we want to force the addition of a new scope (even when we
    // haven't visited the children)
    pub fn add_and_enter_new_scope(&mut self) {
        let new_id = self.scopes.len();

        let curr_scope = self.get_curr_scope_mut().unwrap();
        // Will need to create a new next scope
        // Store new scope in the current scope's children
        curr_scope.add_child(new_id);

        // Does not have next child, create it
        let new_scope = Scope::new(new_id, "".to_string(), ScopeType::Null, Some(curr_scope.id));

        // current scope's next pointer should be pushed to the end
        curr_scope.next = curr_scope.children.len();

        // Add new scope
        self.scopes.push(new_scope);
        self.curr_scope = new_id;
    }

    // Record operations

    pub fn set_curr_script(&mut self, id: usize) {
        self.get_curr_scope_mut().unwrap().containing_script = Some(id);
    }

    pub fn override_record_addr(
        &mut self,
        symbol_name: &str,
        var_ty: DataType,
        var_addr: Option<Vec<VarAddr>>,
    ) {
        let rec_id = match self.lookup(symbol_name) {
            Some(rec_id) => rec_id,
            _ => {
                panic!(
                    "{UNEXPECTED_ERR_MSG} \
                    `{symbol_name}` symbol does not exist in this scope!"
                );
            }
        };
        let mut rec = self.get_record_mut(rec_id);
        if let Some(Record::Var { addr, ty, .. }) = &mut rec {
            *ty = var_ty;
            *addr = var_addr;
        }
    }

    pub fn override_record_val(
        &mut self,
        symbol_name: &str,
        val: Option<Value>,
        fail_on_dne: bool,
    ) {
        let rec_id = match self.lookup(symbol_name) {
            Some(rec_id) => rec_id,
            _ => {
                if !fail_on_dne {
                    return;
                }
                let curr_scope = self.get_curr_scope();
                println!("{:#?}", curr_scope);
                panic!(
                    "{UNEXPECTED_ERR_MSG} \
                `{symbol_name}` symbol does not exist in this scope!"
                );
            }
        };
        let mut rec = self.get_record_mut(rec_id);
        if let Some(Record::Var { value, .. }) = &mut rec {
            *value = val;
        }
    }

    pub fn get_record(&self, rec_id: usize) -> Option<&Record> {
        self.records.get(rec_id)
    }

    pub fn get_record_mut(&mut self, rec_id: usize) -> Option<&mut Record> {
        self.records.get_mut(rec_id)
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
            | Record::Mode { .. }
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
    pub fn lookup_rec_with_context(&self, key: &str) -> (Option<&Record>, String) {
        if let (Some(id), context) = self.lookup_with_context(key) {
            if let Some(rec) = self.get_record(id) {
                return (Some(rec), context);
            }
        }
        (None, "".to_string())
    }
    pub fn lookup_rec(&self, key: &str) -> Option<&Record> {
        if let Some(id) = self.lookup(key) {
            if let Some(rec) = self.get_record(id) {
                return Some(rec);
            }
        }
        None
    }
    pub fn lookup_rec_mut(&mut self, key: &str) -> Option<&mut Record> {
        let id = self.lookup(key)?;
        if let Some(rec) = self.get_record_mut(id) {
            return Some(rec);
        }
        None
    }

    fn no_match(rec: &Record, exp: &str) {
        panic!("Unexpected record type. Expected {}, found: {:?}", exp, rec)
    }

    pub fn lookup_lib(&self, key: &str) -> Option<&Record> {
        if let Some(rec) = self.lookup_rec(key) {
            if matches!(rec, Record::Library { .. }) {
                Some(rec)
            } else {
                Self::no_match(rec, "Library");
                None
            }
        } else {
            None
        }
    }
    pub fn lookup_lib_mut(&mut self, key: &str) -> Option<&mut Record> {
        if let Some(rec) = self.lookup_rec_mut(key) {
            if matches!(rec, Record::Library { .. }) {
                Some(rec)
            } else {
                Self::no_match(rec, "Library");
                None
            }
        } else {
            None
        }
    }

    pub fn lookup_var_mut(&mut self, key: &str, panic_if_missing: bool) -> Option<&mut Record> {
        if let Some(rec) = self.lookup_rec_mut(key) {
            if matches!(rec, Record::Var { .. }) {
                Some(rec)
            } else {
                Self::no_match(rec, "Var");
                None
            }
        } else if panic_if_missing {
            panic!("Could not find var for: {}", key)
        } else {
            None
        }
    }
    pub fn lookup_var(&self, key: &str, fail_on_miss: bool) -> Option<&Record> {
        if let Some(rec) = self.lookup_rec(key) {
            if matches!(rec, Record::Var { .. }) {
                Some(rec)
            } else {
                Self::no_match(rec, "Var");
                None
            }
        } else {
            if fail_on_miss {
                unreachable!("Could not find var for: {}", key);
            }

            None
        }
    }
    pub fn lookup_fn_with_context(&self, key: &str) -> (Option<&Record>, String) {
        if let (Some(rec), context) = self.lookup_rec_with_context(key) {
            if matches!(rec, Record::Fn { .. }) {
                (Some(rec), context)
            } else {
                Self::no_match(rec, "Fn");
                (None, context)
            }
        } else {
            unreachable!("Could not find fn for: {}", key)
        }
    }
    pub fn lookup_fn(&self, key: &str, fail_on_miss: bool) -> Option<&Record> {
        if let Some(rec) = self.lookup_rec(key) {
            if matches!(rec, Record::Fn { .. }) {
                Some(rec)
            } else {
                Self::no_match(rec, "Fn");
                None
            }
        } else {
            if fail_on_miss {
                unreachable!("Could not find fn for: {}", key);
            }
            None
        }
    }
    pub fn lookup_fn_mut(&mut self, key: &str) -> Option<&mut Record> {
        if let Some(rec) = self.lookup_rec_mut(key) {
            if matches!(rec, Record::Fn { .. }) {
                Some(rec)
            } else {
                Self::no_match(rec, "Fn");
                None
            }
        } else {
            unreachable!("Could not find fn for: {}", key);
        }
    }

    pub fn lookup_lib_fn(&self, lib_name: &str, lib_fn_name: &str) -> Option<&Record> {
        if let Some(Record::Library { fns, .. }) = self.lookup_lib(lib_name) {
            if let Some(rec) = fns.get(lib_fn_name) {
                if let Some(rec) = self.get_record(*rec) {
                    return Some(rec);
                }
            }
        }
        None
    }

    pub fn lookup_lib_fn_mut(&mut self, lib_name: &str, lib_fn_name: &str) -> Option<&mut Record> {
        let rec_id = if let Some(Record::Library { fns, .. }) = self.lookup_lib_mut(lib_name) {
            if let Some(rec) = fns.get(lib_fn_name) {
                *rec
            } else {
                panic!("Could not find match for library function: {lib_name}.{lib_fn_name}");
            }
        } else {
            panic!("Could not find library for: {}", lib_name);
        };
        if let Some(rec) = self.get_record_mut(rec_id) {
            Some(rec)
        } else {
            panic!("Could not find match for library function: {lib_name}.{lib_fn_name}");
        }
    }

    pub fn lookup_with_context(&self, key: &str) -> (Option<usize>, String) {
        match self.get_curr_scope() {
            None => (None, "".to_string()),
            Some(curr) => {
                match curr.lookup(key) {
                    Some(rec_id) => (Some(*rec_id), self.get_scope_context(curr.id)),
                    None => {
                        let mut rec_id: Option<&usize> = None;

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
                        let context = if let Some(id) = rec_id {
                            self.get_scope_context(*id)
                        } else {
                            "".to_string()
                        };

                        (rec_id.copied(), context)
                    }
                }
            }
        }
    }
    pub fn lookup(&self, key: &str) -> Option<usize> {
        self.lookup_with_context(key).0
    }

    fn get_scope_context(&self, scope_id: usize) -> String {
        let mut context = "".to_string();
        if let Some(scope) = self.scopes.get(scope_id) {
            let rec_id: Option<&usize> = None;
            let mut curr_scope = scope;
            let mut next_parent: Option<&Scope> = match curr_scope.parent {
                None => None,
                Some(p_id) => self.scopes.get(p_id),
            };
            while rec_id.is_none() && next_parent.is_some() {
                if !context.is_empty() {
                    context += ":";
                }
                context += next_parent.unwrap().name.as_str();

                curr_scope = next_parent.unwrap();
                next_parent = match curr_scope.parent {
                    None => None,
                    Some(p_id) => self.scopes.get(p_id),
                };
            }
        }
        context
    }
}

pub fn line_col_from_loc(loc: &Option<Location>) -> Option<LineColLocation> {
    loc.as_ref()
        .map(|Location { line_col, .. }| line_col.clone())
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

    // Scoping operations

    pub fn add_child(&mut self, id: usize) {
        self.children.push(id);
    }

    pub fn has_next(&self) -> bool {
        self.next < self.children.len()
    }

    pub fn next_child(&mut self) -> Option<&usize> {
        if !self.has_next() {
            unreachable!(
                "{UNEXPECTED_ERR_MSG} Scope::next_child() should never be called without first checking that there is one."
            );
        }

        let next_child = self.children.get(self.next).unwrap();
        self.next += 1;
        Some(next_child)
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
    Mode,
    Probe,
    Fn,
    Null,
}
impl Display for ScopeType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ScopeType::Whamm => {
                write!(f, "Whamm")
            }
            ScopeType::Script => {
                write!(f, "Script")
            }
            ScopeType::Provider => {
                write!(f, "Provider")
            }
            ScopeType::Package => {
                write!(f, "Package")
            }
            ScopeType::Event => {
                write!(f, "Event")
            }
            ScopeType::Mode => {
                write!(f, "Mode")
            }
            ScopeType::Probe => {
                write!(f, "Probe")
            }
            ScopeType::Fn => {
                write!(f, "Fn")
            }
            ScopeType::Null => {
                write!(f, "Null")
            }
        }
    }
}

/// The usize values in the record fields index into the SymbolTable::records Vec.
#[derive(Debug)]
pub enum Record {
    Whamm {
        fns: Vec<usize>,
        globals: Vec<usize>,
        scripts: Vec<usize>,
    },
    Script {
        user_libs: Vec<usize>,
        fns: Vec<usize>,
        globals: Vec<usize>,
        providers: Vec<usize>,
    },
    Library {
        fns: HashMap<String, usize>,
    },
    Provider {
        fns: Vec<usize>,
        vars: Vec<usize>,
        packages: Vec<usize>,
    },
    Package {
        fns: Vec<usize>,
        vars: Vec<usize>,
        events: Vec<usize>,
    },
    Event {
        fns: Vec<usize>,
        vars: Vec<usize>,
        modes: Vec<usize>,
    },
    Mode {
        probes: Vec<usize>,
    },
    Probe {
        fns: Vec<usize>,
        vars: Vec<usize>,
    },
    LibFn {
        name: String,
        params: Vec<DataType>,
        results: Vec<DataType>,
        def: Definition,

        /// The address of this function post-injection
        addr: Option<u32>,
        loc: Option<Location>,
    },
    Fn {
        name: FnId,
        params: Vec<usize>,
        // given that we are assuming function that return nothing
        // returns a unit type (empty tuple)
        ret_ty: DataType,
        def: Definition,

        /// The address of this function post-injection
        addr: Option<u32>,
        loc: Option<Location>,

        /// Bookkeeping for bound functions that need the args defined
        /// (only tracks this for dynamic bound functions)
        req_args: StackReq,
    },
    Var {
        ty: DataType,
        value: Option<Value>,
        def: Definition,
        /// The address of this var post-injection
        addr: Option<Vec<VarAddr>>,
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
    pub fn is_comp_defined(&self) -> bool {
        match self {
            Record::Fn { def, .. } | Record::Var { def, .. } => def.is_comp_defined(),
            _ => true,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// the index of the variables (global/local) in app.wasm
/// This is the relative index that's dependent on which function/module you're in.
pub enum VarAddr {
    Local {
        addr: u32,
    },
    Global {
        addr: u32,
    },
    MapId {
        addr: u32,
    },
    MemLoc {
        // The ID of the memory that the var is stored in
        mem_id: u32,
        // The type of the data at this memory location
        // using whamm DataType rather than wasm type here
        // this enables us to store things like U8 in a single
        // byte instead of using an i32!
        ty: DataType,
        // The offset within a function's variable block...
        // This should be added to a base offset value to find
        // the true memory offset for this variable.
        var_offset: u32,
    },
}
impl Display for VarAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VarAddr::Local { addr } | VarAddr::Global { addr } | VarAddr::MapId { addr } => {
                write!(f, "{}", addr)
            }
            VarAddr::MemLoc {
                mem_id, var_offset, ..
            } => write!(f, "{}@{}", mem_id, var_offset),
        }
    }
}
impl VarAddr {
    pub fn ty(&self) -> String {
        match self {
            Self::Local { .. } => "local_id".to_string(),
            Self::Global { .. } => "global_id".to_string(),
            Self::MapId { .. } => "map_id".to_string(),
            Self::MemLoc { .. } => "memaddr".to_string(),
        }
    }
}

use std::process::exit;
use log::{debug, error, info};
use regex::Regex;
use walrus::{ActiveData, ActiveDataLocation, DataKind, FunctionBuilder, FunctionId, FunctionKind,
             ImportedFunction, InstrSeqBuilder, LocalFunction, MemoryId, ModuleData, ValType};
use walrus::ir::{BinaryOp, ExtendedLoad, Instr, InstrSeqId, LoadKind, MemArg};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Expr, Fn, Op, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait Emitter {
    fn enter_scope(&mut self);
    fn enter_named_scope(&mut self, scope_name: &String) -> bool;
    fn exit_scope(&mut self);
    fn reset_children(&mut self);

    fn has_next_instr(&self) -> bool;
    fn next_instr(&mut self) -> bool;
    fn curr_instr_is_of_type(&mut self, instr_names: &Vec<String>) -> bool;
    fn has_params(&mut self) -> bool;
    fn save_params(&mut self) -> bool;
    fn emit_params(&mut self) -> bool;
    fn define_compiler_var(&mut self, context: &String, var_name: &String) -> bool;
    // fn emit_event(&mut self, context: &str, event: &mut Event) -> bool;
    fn fold_expr(&mut self, expr: &mut Expr) -> bool;
    fn emit_expr(&mut self, expr: &mut Expr) -> bool;

    fn emit_fn(&mut self, context_name: &str, f: &Fn) -> bool;
    fn emit_formal_param(&mut self, param: &(Expr, DataType)) -> bool;
    fn emit_global(&mut self, name: String, ty: DataType, val: &Option<Value>) -> bool;
    fn remove_orig(&mut self) -> bool;
    fn emit_orig(&mut self) -> bool;
    fn emit_if(&mut self) -> bool;
    fn emit_if_else(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent expression as the condition of an if or if/else stmt
    fn emit_condition(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the consequent body of an if or if/else stmt
    fn emit_consequent(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the alternate body of an if/else stmt
    fn emit_alternate(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements in the outer block of some branching logic
    fn finish_branch(&mut self) -> bool;
    fn emit_body(&mut self, body: &mut Vec<Statement>) -> bool;
    fn has_alt_call(&mut self) -> bool; // TODO -- remove need for this
    fn emit_alt_call(&mut self) -> bool; // TODO -- remove need for this
    fn emit_stmt(&mut self, stmt: &mut Statement) -> bool;

    fn dump_to_file(&mut self, output_wasm_path: String) -> bool;
}

// =================================================================================
// ================ WasmRewritingEmitter - HELPER FUNCTIONS ========================
// Necessary to extract common logic between Emitter and InstrumentationVisitor.
// Can't pass an Emitter instance to InstrumentationVisitor due to Rust not
// allowing nested references to a common mutable object. So I can't pass the
// Emitter to the InstrumentationVisitor since I must iterate over Emitter.app_wasm
// with a construction of InstrumentationVisitor inside that loop.
// =================================================================================
// =================================================================================

fn emit_stmt(table: &mut SymbolTable, module_data: &mut ModuleData, stmt: &mut Statement,
             instr_builder: &mut InstrSeqBuilder, metadata: &mut InsertionMetadata, index: &mut usize) -> bool {
    let mut is_success = true;
    match stmt {
        Statement::Assign { var_id, expr } => {
            let folded_expr = ExprFolder::fold_expr(expr, table);
            return if let Expr::Primitive { val } = folded_expr {
                // This is a constant, just save the value to the symbol table for later use
                if let Expr::VarId { name } = var_id {
                    let var_rec_id = match table.lookup(name) {
                        Some(rec_id) => rec_id.clone(),
                        _ => {
                            error!("VarId '{}' does not exist in this scope!", name);
                            return false;
                        }
                    };
                    match table.get_record_mut(&var_rec_id) {
                        Some(Record::Var { value, .. }) => {
                            *value = Some(val);
                            true
                        },
                        Some(ty) => {
                            error!("Incorrect variable record, expected Record::Var, found: {:?}", ty);
                            false
                        },
                        None => {
                            error!("Variable symbol does not exist!");
                            false
                        }
                    }
                } else {
                    error!("Expected VarId.");
                    false
                }
            } else {
                is_success &= emit_expr(table, module_data, expr, instr_builder, metadata, index);

                return if let Expr::VarId { name } = var_id {
                    let var_rec_id = match table.lookup(name) {
                        Some(rec_id) => rec_id.clone(),
                        _ => {
                            error!("VarId '{}' does not exist in this scope!", name);
                            is_success &= false;
                            return is_success
                        }
                    };
                    match table.get_record_mut(&var_rec_id) {
                        Some(Record::Var { addr, .. }) => {
                            // this will be different based on if this is a global or local var
                            match addr {
                                Some(VarAddr::Global { addr }) => {
                                    instr_builder.instr_at(*index, walrus::ir::GlobalSet {
                                        global: addr.clone()
                                    });
                                    // update index to point to what follows our insertions
                                    *index += 1;
                                }
                                Some(VarAddr::Local { addr }) => {
                                    instr_builder.instr_at(*index, walrus::ir::LocalSet {
                                        local: addr.clone()
                                    });
                                    // update index to point to what follows our insertions
                                    *index += 1;
                                },
                                None => {
                                    // TODO No address yet, let's make a new local variable
                                    unimplemented!()
                                }
                            }
                            is_success
                        },
                        Some(ty) => {
                            error!("Incorrect variable record, expected Record::Var, found: {:?}", ty);
                            is_success &= false;
                            is_success
                        },
                        None => {
                            error!("Variable symbol does not exist!");
                            is_success &= false;
                            is_success
                        }
                    }
                } else {
                    error!("Expected VarId.");
                    is_success &= false;
                    is_success
                }
            }
        }
        Statement::Expr { expr } => {
            is_success &= emit_expr(table, module_data, expr, instr_builder, metadata, index);
        }
    }
    is_success
}

fn emit_expr(table: &mut SymbolTable, module_data: &mut ModuleData, expr: &mut Expr, instr_builder: &mut InstrSeqBuilder,
             metadata: &mut InsertionMetadata, index: &mut usize) -> bool {
    let mut is_success = true;
    match expr {
        Expr::BinOp {lhs, op, rhs} => {
            is_success &= emit_expr(table, module_data, lhs, instr_builder, metadata, index);
            is_success &= emit_expr(table, module_data, rhs, instr_builder, metadata, index);
            is_success &= emit_op(op, instr_builder, index);
        }
        Expr::Call { fn_target, args } => {
            let fn_name = match &**fn_target {
                Expr::VarId{ name } => name.clone(),
                _ => return false
            };

            // emit the arguments
            if let Some(args) = args {
                args.iter_mut().for_each(|boxed_arg| {
                    let arg = &mut **boxed_arg; // unbox
                    is_success &= emit_expr(table, module_data, arg, instr_builder, metadata, index);
                })
            }

            let fn_rec_id = match table.lookup(&fn_name) {
                Some(rec_id) => Some(rec_id.clone()),
                _ => {
                    None
                }
            };
            match fn_rec_id {
                Some(rec_id) => {
                    let fn_rec = table.get_record_mut(&rec_id);
                    match fn_rec {
                        Some(Record::Fn { addr, .. }) => {
                            if let Some(f_id) = addr {
                                instr_builder.instr_at( *index,walrus::ir::Call {
                                    func: f_id.clone()
                                });
                                // update index to point to what follows our insertions
                                *index += 1;
                            } else {
                                error!("fn_target address not in symbol table, not emitted yet...");
                                return false;
                            }
                        }
                        _ => {
                            error!("fn_target not defined in symbol table!");
                            return false;
                        }
                    }
                },
                None => {
                    // Must be defined in the Wasm
                    unimplemented!()
                }
            }
        }
        Expr::VarId { name } => {
            // TODO -- support string vars (unimplemented)
            let var_rec_id = match table.lookup(&name) {
                Some(rec_id) => rec_id.clone(),
                _ => {
                    error!("VarId '{}' does not exist in this scope!", name);
                    return false;
                }
            };
            return match table.get_record_mut(&var_rec_id) {
                Some(Record::Var { addr, .. }) => {
                    // this will be different based on if this is a global or local var
                    match addr {
                        Some(VarAddr::Global { addr }) => {
                            instr_builder.instr_at(*index, walrus::ir::GlobalGet {
                                global: addr.clone()
                            });
                            // update index to point to what follows our insertions
                            *index += 1;
                        }
                        Some(VarAddr::Local { addr }) => {
                            instr_builder.instr_at(*index, walrus::ir::LocalGet {
                                local: addr.clone()
                            });
                            // update index to point to what follows our insertions
                            *index += 1;
                        },
                        None => {
                            error!("Variable does not exist in scope: {}", name);
                            return false;
                        }
                    }
                    true
                },
                Some(ty) => {
                    error!("Incorrect variable record, expected Record::Var, found: {:?}", ty);
                    false
                },
                None => {
                    error!("Variable symbol does not exist!");
                    false
                }
            }
        }
        Expr::Primitive { val } => {
            is_success &= emit_value(table, module_data, val, instr_builder, metadata, index);
        }
    }
    is_success
}

fn emit_op(op: &Op, instr_builder: &mut InstrSeqBuilder, index: &mut usize) -> bool {
    match op {
        Op::And => {
            // we only support i32's at the moment
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32And
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::Or => {
            // we only support i32's at the moment
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32Or
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::EQ => {
            // we only support i32's at the moment
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32Eq
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::NE => {
            // we only support i32's at the moment
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32Ne
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::GE => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32GeS
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::GT => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32GtS
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::LE => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32LeS
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::LT => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32LtS
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::Add => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32Add
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::Subtract => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32Sub
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::Multiply => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32Mul
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::Divide => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32DivS
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        Op::Modulo => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at( *index,walrus::ir::Binop {
                op: BinaryOp::I32RemS
            });
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
    }
}

fn emit_value(table: &mut SymbolTable, module_data: &mut ModuleData, val: &mut Value, instr_builder: &mut InstrSeqBuilder,
              metadata: &mut InsertionMetadata, index: &mut usize) -> bool {
    let mut is_success = true;
    match val {
        Value::Integer { val, .. } => {
            instr_builder.instr_at( *index,walrus::ir::Const {
                value: walrus::ir::Value::I32(val.clone())
            });
            // update index to point to what follows our insertions
            *index += 1;
            is_success &= true;
        }
        Value::Str { val, addr, ty: _ty } => {
            let data_id = module_data.add(DataKind::Active(ActiveData {
                memory: metadata.mem_id,
                location: ActiveDataLocation::Absolute(metadata.curr_mem_offset.clone())
            }), Vec::from(val.as_bytes()));

            // save the memory addresses/lens so they can be used as appropriate
            *addr = Some((
                data_id,
                metadata.curr_mem_offset.clone(),
                val.len()
            ));

            // emit Wasm instructions for the memory address and string length
            instr_builder.instr_at( *index,walrus::ir::Const {
                value: walrus::ir::Value::I32(metadata.curr_mem_offset.clone() as i32)
            });
            // update index to point to what follows our insertions
            *index += 1;
            instr_builder.instr_at( *index,walrus::ir::Const {
                value: walrus::ir::Value::I32(val.len() as i32)
            });
            // update index to point to what follows our insertions
            *index += 1;

            // update curr_mem_offset to account for new data
            metadata.curr_mem_offset += val.len() as u32;
            is_success &= true;
        }
        Value::Tuple { vals, .. } => {
            vals.iter_mut().for_each(|val| {
                is_success &= emit_expr(table, module_data, val, instr_builder, metadata, index);
            });
        }
        Value::Boolean { val, .. } => {
            // "In a boolean context, such as a br_if condition, any non-zero value is interpreted as true and 0 is interpreted as false."
            // https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#booleans
            if *val {
                // insert true (non-zero)
                instr_builder.instr_at( *index,walrus::ir::Const {
                    value: walrus::ir::Value::I32(1)
                });
            } else {
                // insert false (zero)
                instr_builder.instr_at( *index,walrus::ir::Const {
                    value: walrus::ir::Value::I32(0)
                });
            }
            // update index to point to what follows our insertions
            *index += 1;
            is_success &= true;
        }
    }
    is_success
}



fn get_func_info(app_wasm: &walrus::Module, func: &walrus::Function) -> FuncInfo {
    match &func.kind {
        FunctionKind::Import(ImportedFunction { ty: ty_id, import: import_id }) => {
            let import = app_wasm.imports.get(*import_id);
            let ty = app_wasm.types.get(*ty_id);

            FuncInfo {
                func_kind: "import".to_string(),
                module: import.module.clone(),
                name: import.name.clone(),
                params: Vec::from(ty.params())
            }
        },
        FunctionKind::Local(LocalFunction{ args, ..}) => {
            let mut params = vec![];
            args.iter().for_each(|arg_id| {
                let arg = app_wasm.locals.get(*arg_id);
                params.push(arg.ty());
            });

            FuncInfo {
                func_kind: "local".to_string(),
                module: "".to_string(),
                name: "".to_string(),
                params: Vec::from(params)
            }
        },
        FunctionKind::Uninitialized(ty_id) => {
            let ty = app_wasm.types.get(*ty_id);

            FuncInfo {
                func_kind: "uninitialized".to_string(),
                module: "".to_string(),
                name: "".to_string(),
                params: Vec::from(ty.params())
            }
        }
    }
}

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

struct InsertionMetadata {
    // curr_event: String,
    mem_id: MemoryId,
    curr_mem_offset: u32,
}

#[derive(Debug)]
struct InstrIter {
    instr_locs: Vec<ProbeLoc>,
    curr_loc: usize
}
impl InstrIter {
    /// Build out a list of all local functions and their blocks/instruction indexes
    /// to visit while doing instrumentation.
    fn new(app_wasm: &walrus::Module) -> Self {
        // Figure out which functions to visit
        let mut instr_locs = vec![];
        for func in app_wasm.funcs.iter() {
            let func_id = func.id();
            if let Some(name) = func.name.as_ref() {
                // TODO -- get rid of this necessity (probably by removing the need to have
                //         functions already present in the app code)
                if name.starts_with("instr_") {
                    continue;
                }
            }

            if let FunctionKind::Local(local_func) = &func.kind {
                // TODO -- make sure that the id is not any of the injected function IDs (strcmp)
                Self::init_instr_locs(&mut instr_locs, local_func, &func_id, func.name.clone(),
                                           local_func.entry_block());
            }
        }
        debug!("Finished creating list of instructions to visit");
        Self {
            instr_locs,
            curr_loc: 0
        }
    }
    fn init_instr_locs(locs: &mut Vec<ProbeLoc>, func: &LocalFunction, func_id: &FunctionId,
                       func_name: Option<String>, instr_seq_id: InstrSeqId) {
        func.block(instr_seq_id)
            .iter()
            .enumerate()
            .for_each(|(index, (instr, _))| {
                let instr_as_str = &format!("{:?}", instr);
                let instr_name = instr_as_str.split("(").next().unwrap().to_lowercase();

                // add current instr
                locs.push( ProbeLoc {
                    // wasm_func_name: func_name.clone(),
                    wasm_func_id: func_id.clone(),
                    instr_seq_id,
                    index,
                    instr_name: instr_name.clone(),
                    instr: instr.clone(),
                    instr_params: None,
                    instr_created_args: vec![],
                    instr_alt_call: None,
                    // instr_symbols: HashMap::new()
                });

                // visit nested blocks
                match instr {
                    Instr::Block(block) => {
                        Self::init_instr_locs(locs, func, func_id, func_name.clone(), block.seq);
                    }
                    Instr::Loop(_loop) => {
                        Self::init_instr_locs(locs, func, func_id, func_name.clone(), _loop.seq);
                    }
                    Instr::IfElse(if_else, ..) => {
                        println!("IfElse: {:#?}", if_else);
                        Self::init_instr_locs(locs, func, func_id, func_name.clone(), if_else.consequent);
                        Self::init_instr_locs(locs, func, func_id, func_name.clone(), if_else.alternative);
                    }
                    _ => {
                        // do nothing extra for other instructions
                    }
                }
            });
    }
    fn has_next(&self) -> bool {
        self.curr_loc + 1 < self.instr_locs.len()
    }
    fn next(&mut self) -> Option<&ProbeLoc> {
        self.curr_loc += 1;
        self.curr()
    }
    fn curr(&self) -> Option<&ProbeLoc> {
        self.instr_locs.get(self.curr_loc)
    }
    fn curr_mut(&mut self) -> Option<&mut ProbeLoc> {
        self.instr_locs.get_mut(self.curr_loc)
    }
}

// Struct to store info on insertion locations for an instruction sequence.
// Note that blocks can be indefinitely nested.
#[derive(Debug)]
struct ProbeLoc {
    // wasm_func_name: Option<String>,
    wasm_func_id: FunctionId,
    instr_seq_id: InstrSeqId,
    index: usize,

    instr_name: String,
    instr: Instr,
    instr_params: Option<Vec<ValType>>,
    instr_created_args: Vec<(String, usize)>,

    // Save off the compiler-defined constants for this instruction
    // instr_symbols: HashMap<String, Record>,
    instr_alt_call: Option<FunctionId>
}
struct FuncInfo {
    func_kind: String,
    module: String,
    name: String,
    params: Vec<ValType>
}
struct EmittingInstrTracker {
    curr_seq_id: InstrSeqId,
    curr_idx: usize,

    /// The sequence ID of the main block (containing the instruction of-interest)
    main_seq_id: InstrSeqId,
    /// The current index into the main block (containing the instruction of-interest)
    main_idx: usize,

    /// The sequence ID of the outer block of an injected conditional
    outer_seq_id: Option<InstrSeqId>,
    /// The current index into the outer block of an injected conditional
    outer_idx: Option<usize>,

    /// The sequence ID of the consequent block of an injected conditional
    then_seq_id: Option<InstrSeqId>,
    /// The current index into the consequent block of an injected conditional
    then_idx: Option<usize>,

    /// The sequence ID of the alternate block of an injected conditional
    else_seq_id: Option<InstrSeqId>,
    /// The current index into the alternate block of an injected conditional
    else_idx: Option<usize>
}

pub struct WasmRewritingEmitter {
    pub app_wasm: walrus::Module,
    pub table: SymbolTable,

    // whamm! AST traversal bookkeeping
    metadata: InsertionMetadata,
    instr_iter: InstrIter,
    emitting_instr: Option<EmittingInstrTracker>,

    fn_providing_contexts: Vec<String>
}
impl WasmRewritingEmitter {
    pub fn new(app_wasm: walrus::Module, table: SymbolTable) -> Self {
        let mem_id = app_wasm.memories.iter().next()
            .expect("only single memory is supported")
            .id();
        let instr_iter = InstrIter::new(&app_wasm);

        Self {
            app_wasm,
            table,
            metadata: InsertionMetadata {
                // curr_event: "".to_string(),
                mem_id,
                curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
            },
            instr_iter,
            emitting_instr: None,
            fn_providing_contexts: vec![ "whamm".to_string() ]
        }
    }

    fn override_var_val(&mut self, rec_id: &usize, val: Option<Value>) {
        let mut rec = self.table.get_record_mut(&rec_id);
        match &mut rec {
            Some(Record::Var { value, .. }) => {
                *value = val;
            }
            _ => {}
        }
    }

    fn define_new_target_fn_name(&mut self) -> bool {
        // TODO -- change this to be an inline call() instead of setting a var
        true
    }

    fn define_target_imp_name(&mut self) -> bool {
        let var_name = "target_imp_name".to_string();

        if let Some(curr_instr) = self.instr_iter.curr() {
            if let Instr::Call(func) = &curr_instr.instr {
                let func = self.app_wasm.funcs.get(func.func);
                let func_info = get_func_info(&self.app_wasm, func);
                if func.name.as_ref().unwrap().contains("call_perform") {
                    println!("{}", func.name.as_ref().unwrap());
                }

                let rec_id = match self.table.lookup(&var_name) {
                    Some(rec_id) => rec_id.clone(),
                    _ => {
                        error!("{} symbol does not exist in this scope!", var_name);
                        return false;
                    }
                };
                self.override_var_val(&rec_id, Some(Value::Str {
                    ty: DataType::Str,
                    val: func_info.name.to_string(),
                    addr: None
                }));
            }
        }
        true
    }

    fn define_target_fn_type(&mut self) -> bool {
        let var_name = "target_fn_type".to_string();

        if let Some(curr_instr) = self.instr_iter.curr() {
            if let Instr::Call(func) = &curr_instr.instr {
                let func = self.app_wasm.funcs.get(func.func);
                let func_info = get_func_info(&self.app_wasm, func);
                // if func.name.as_ref().unwrap().contains("call_perform") {
                //     println!("{}", func.name.as_ref().unwrap());
                // }
                let rec_id = match self.table.lookup(&var_name) {
                    Some(rec_id) => rec_id.clone(),
                    _ => {
                        error!("{} symbol does not exist in this scope!", var_name);
                        return false;
                    }
                };
                self.override_var_val(&rec_id, Some(Value::Str {
                    ty: DataType::Str,
                    val: func_info.func_kind.to_string(),
                    addr: None
                }));
            }
        }
        true
    }

    fn define_target_imp_module(&mut self) -> bool {
        let var_name = "target_imp_module".to_string();
        if let Some(curr_instr) = self.instr_iter.curr() {
            if let Instr::Call(func) = &curr_instr.instr {
                let func = self.app_wasm.funcs.get(func.func);
                let func_info = get_func_info(&self.app_wasm, func);
                // if func.name.as_ref().unwrap().contains("call_perform") {
                //     println!("{}", func.name.as_ref().unwrap());
                // }
                let rec_id = match self.table.lookup(&var_name) {
                    Some(rec_id) => rec_id.clone(),
                    _ => {
                        error!("{} symbol does not exist in this scope!", var_name);
                        return false;
                    }
                };
                self.override_var_val(&rec_id, Some(Value::Str {
                    ty: DataType::Str,
                    val: func_info.module.to_string(),
                    addr: None
                }));
            }
        }
        true
    }

    fn emit_provided_fn(&mut self, context: &str, f: &Fn) -> bool {
        return if context == "whamm" && f.name == "strcmp" {
            self.emit_whamm_strcmp_fn(f)
        } else {
            error!("Provided function, but could not find a context to provide the definition, context: {}", context);
            false
        }
    }

    fn emit_whamm_strcmp_fn(&mut self, f: &Fn) -> bool {
        let strcmp_params = vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32];
        let strcmp_result = vec![ValType::I32];

        let mut strcmp = FunctionBuilder::new(&mut self.app_wasm.types, &strcmp_params, &strcmp_result);

        // create params
        let str0_offset = self.app_wasm.locals.add(ValType::I32);
        let str0_size = self.app_wasm.locals.add(ValType::I32);
        let str1_offset = self.app_wasm.locals.add(ValType::I32);
        let str1_size = self.app_wasm.locals.add(ValType::I32);

        // create locals
        let i = self.app_wasm.locals.add(ValType::I32);
        let str0_char = self.app_wasm.locals.add(ValType::I32);
        let str1_char = self.app_wasm.locals.add(ValType::I32);

        // create the body of strcmp
        strcmp
            .func_body()
            .block(None, |neq_block| {
                let neq = neq_block.id();

                neq_block.block(None, |eq_block| {
                    let eq = eq_block.id();

                    // 1. Check if sizes are equal, if not return 0
                    eq_block
                        .local_get(str0_size)
                        .local_get(str1_size)
                        .binop(BinaryOp::I32Ne)
                        .br_if(neq);

                    // 2. Check if mem offset is equal, if yes return non-zero (we are comparing the same data)
                    eq_block
                        .local_get(str0_offset)
                        .local_get(str1_offset)
                        .binop(BinaryOp::I32Eq)
                        .br_if(eq);

                    // 3. iterate over each string and check equivalence of chars, if any not equal, return 0
                    eq_block
                        .i32_const(0)
                        .local_set(i)
                        .loop_(None, |loop_| {
                            let cmp_char = loop_.id();

                            // Check if we've reached the end of the string
                            loop_
                                .local_get(i)
                                .local_get(str0_size) // (can compare with either str size, equal at this point)
                                .binop(BinaryOp::I32LtU)
                                .i32_const(0)
                                .binop(BinaryOp::I32Eq)
                                .br_if(eq); // We've reached the end without failing equality checks!

                            // get char for str0
                            loop_
                                .local_get(str0_offset)
                                .local_get(i)
                                .binop(BinaryOp::I32Add)
                                .load(
                                    self.metadata.mem_id,
                                    LoadKind::I32_8 {
                                        kind: ExtendedLoad::ZeroExtend,
                                    },
                                    MemArg {
                                        offset: 0,
                                        align: 1,
                                    },
                                )
                                .local_set(str0_char);

                            // get char for str1
                            loop_
                                .local_get(str1_offset)
                                .local_get(i)
                                .binop(BinaryOp::I32Add)
                                .load(
                                    self.metadata.mem_id,
                                    LoadKind::I32_8 {
                                        kind: ExtendedLoad::ZeroExtend,
                                    },
                                    MemArg {
                                        offset: 0,
                                        align: 1,
                                    },
                                )
                                .local_set(str1_char);

                            // compare the two chars
                            loop_
                                .local_get(str0_char)
                                .local_get(str1_char)
                                .binop(BinaryOp::I32Ne)
                                .br_if(neq); // If they are not equal, exit and return '0'

                            // Increment i and continue loop
                            loop_
                                .local_get(i)
                                .i32_const(1)
                                .binop(BinaryOp::I32Add)
                                .local_set(i)
                                .br(cmp_char);
                        })
                        // 4. Reached the end of each string without returning, return nonzero
                        .br(eq);
                })
                // they are equal, return '1'
                .i32_const(1)
                .return_();
            })
            // they are not equal, return '0'
            .i32_const(0)
            .return_();

        let strcmp_id = strcmp.finish(vec![ str0_offset, str0_size, str1_offset, str1_size ], &mut self.app_wasm.funcs);
        let rec_id = match self.table.lookup(&f.name) {
            Some(rec_id) => *rec_id,
            _ => {
                error!("strcmp fn symbol does not exist in this scope!");
                return false;
            }
        };

        return if let Some(rec) = self.table.get_record_mut(&rec_id) {
            if let Record::Fn { addr, ..} = rec {
                *addr = Some(strcmp_id);
                true
            } else {
                error!("Incorrect global variable record, expected Record::Var, found: {:?}", rec);
                false
            }
        } else {
            error!("Global variable symbol does not exist!");
            false
        };
    }
}

impl Emitter for WasmRewritingEmitter {
    fn enter_scope(&mut self) {
        self.table.enter_scope();
    }
    fn enter_named_scope(&mut self, scope_name: &String) -> bool {
        self.table.enter_named_scope(scope_name)
    }
    fn exit_scope(&mut self) {
        self.table.exit_scope();
    }
    fn reset_children(&mut self) {
        self.table.reset_children();
    }

    /// bool -> whether there is a next instruction to process
    fn has_next_instr(&self) -> bool {
        self.instr_iter.has_next()
    }

    /// bool -> whether it found a next instruction
    fn next_instr(&mut self) -> bool {
        if self.instr_iter.has_next() {
            if let Some(next) = self.instr_iter.next() {
                self.emitting_instr = Some(EmittingInstrTracker {
                    curr_seq_id: next.instr_seq_id.clone(),
                    curr_idx: next.index.clone(),
                    main_seq_id: next.instr_seq_id.clone(),
                    main_idx: next.index.clone(),
                    outer_seq_id: None,
                    outer_idx: None,
                    then_seq_id: None,
                    then_idx: None,
                    else_seq_id: None,
                    else_idx: None,
                });
                return true;
            }
        }
        false
    }

    /// bool -> whether the current instruction is one of the passed list of types
    fn curr_instr_is_of_type(&mut self, instr_names: &Vec<String>) -> bool {
        if let Some(instr) = self.instr_iter.curr() {
            return instr_names.contains(&instr.instr_name);
        }
        false
    }

    fn has_params(&mut self) -> bool {
        if let Some(curr_instr) = self.instr_iter.curr_mut() {
            if let Some(params) = &curr_instr.instr_params {
                return !params.is_empty();
            }

            // We haven't defined the params for this instr yet, let's do that
            if let Instr::Call(func) = &curr_instr.instr {
                let func = self.app_wasm.funcs.get(func.func);
                let func_info = get_func_info(&self.app_wasm, func);
                // if func.name.as_ref().unwrap().contains("call_perform") {
                //     println!("{}", func.name.as_ref().unwrap());
                // }

                curr_instr.instr_params = Some(func_info.params);
            }
            return curr_instr.instr_params.as_ref().unwrap().len() > 0;
        }
        error!("Something went wrong when trying to access the current instruction.");
        false
    }

    fn save_params(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                // No bytecodes should have been emitted in the module yet!
                // So, we can just save off the first * items in the stack as the args
                // to the call.
                let mut arg_recs = vec![]; // vec to retain order!
                if let Some(params) = &curr_loc.instr_params {
                    params.iter().enumerate().for_each(|(num, param_ty)| {
                        // create local for the param in the module
                        let arg_local_id = self.app_wasm.locals.add(*param_ty);

                        // emit a bytecode in the event to assign the ToS to this new local
                        instr_builder.instr_at( tracker.curr_idx,walrus::ir::LocalSet {
                            local: arg_local_id.clone()
                        });

                        // update index to point to what follows our insertions
                        tracker.curr_idx += 1;

                        // place in symbol table with var addr for future reference
                        let arg_name = format!("arg{}", num);
                        let id = self.table.put(arg_name.clone(), Record::Var {
                            ty: DataType::Integer, // we only support integers right now.
                            name: arg_name.clone(),
                            value: None,
                            addr: Some(VarAddr::Local {
                                addr: arg_local_id
                            })
                        });
                        arg_recs.push((arg_name, id));
                    });
                }
                curr_loc.instr_created_args = arg_recs;
            }
        }
        false
    }

    fn emit_params(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                for (_param_name, param_rec_id) in curr_loc.instr_created_args.iter() {
                    let param_rec = self.table.get_record_mut(&param_rec_id);
                    if let Some(Record::Var { addr: Some(VarAddr::Local {addr}), .. }) = param_rec {
                        instr_builder.instr_at(tracker.curr_idx, walrus::ir::LocalGet {
                            local: addr.clone()
                        });
                        tracker.curr_idx += 1;
                    } else {
                        error!("Could not emit parameters, something went wrong...");
                        exit(1);
                    }
                }
            }
        }
        false
    }

    fn define_compiler_var(&mut self, context: &String, var_name: &String) -> bool {
        let regex = Regex::new(r"whamm:whammy([0-9]+):wasm:bytecode").unwrap();
        return if let Some(_caps) = regex.captures(context) {
            match var_name.as_str() {
                "new_target_fn_name" => {
                    self.define_new_target_fn_name()
                },
                "target_imp_name" => {
                    self.define_target_imp_name()
                },
                "target_fn_type" => {
                    self.define_target_fn_type()
                },
                "target_imp_module" => {
                    self.define_target_imp_module()
                }
                _ => {
                    error!("Current context `{}` does not provide definition for variable `{}`", context, var_name);
                    false
                }
            }
        } else {
            error!("Could not find a context to provide the definition, context: {}", context);
            false
        };
    }

    fn fold_expr(&mut self, expr: &mut Expr) -> bool {
        *expr = ExprFolder::fold_expr(expr, &self.table);
        true
    }
    fn emit_expr(&mut self, expr: &mut Expr) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                return emit_expr(&mut self.table, &mut self.app_wasm.data, expr,
                                 &mut instr_builder, &mut self.metadata, &mut tracker.curr_idx);
            }
        }
        false
    }
    fn emit_fn(&mut self, context: &str, f: &Fn) -> bool {
        // figure out if this is a provided fn.
        if f.is_comp_provided {
            return if self.fn_providing_contexts.contains(&context.to_string()) {
                self.emit_provided_fn(context, f)
            } else {
                error!("Provided fn, but could not find a context to provide the definition, context: {}", context);
                false
            }
        }

        // emit non-provided fn
        // only when we're supporting user-defined fns in whammy...
        unimplemented!();
    }

    fn emit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        // only when we're supporting user-defined fns in whammy...
        unimplemented!();
    }

    fn emit_global(&mut self, name: String, _ty: DataType, _val: &Option<Value>) -> bool {
        let rec_id = match self.table.lookup(&name) {
            Some(rec_id) => rec_id.clone(),
            _ => {
                error!("Global variable symbol does not exist in this scope!");
                return false
            } // Ignore, continue to emit
        };

        let rec = self.table.get_record_mut(&rec_id);
        return match rec {
            Some(Record::Var { addr: _addr, .. }) => {
                // emit global variable and set addr in symbol table
                // only when we're supporting user-defined globals in whammy...
                unimplemented!();
            },
            Some(ty) => {
                error!("Incorrect global variable record, expected Record::Var, found: {:?}", ty);
                false
            },
            None => {
                error!("Global variable symbol does not exist!");
                return false;
            }
        }
    }

    fn remove_orig(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &self.emitting_instr {
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                instr_builder.instrs_mut().remove(tracker.curr_idx);
                return true;
            }
        }
        return false;
    }

    fn emit_orig(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &self.emitting_instr {
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                instr_builder.instr_at(tracker.curr_idx, curr_loc.instr.clone());
                return true;
            }
        }
        return false;
    }

    fn emit_if(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                instr_builder.block_at(
                    tracker.curr_idx,
                    None,
                    |outer_block| {
                        let outer_block_id = outer_block.id();
                        // create new `index` var to store current index into the of the `then` instr sequence
                        let outer_block_idx = 0 as usize;

                        // Add logic that will execute after the injected conditional to
                        // break out of the if block if it evaluates to true.
                        // If result of predicate equals 0, break out of the probe block.
                        // Will continue with the application code.
                        outer_block
                            .i32_const(0)
                            .binop(BinaryOp::I32Eq)
                            .br_if(outer_block_id);

                        // Leave block index at 0 to enable injecting conditional before the
                        // above instructions.

                        // Save the block information for future reference
                        tracker.outer_seq_id = Some(outer_block_id);
                        tracker.outer_idx = Some(outer_block_idx);
                    });

                tracker.curr_idx += 1;
                return true;
            }
        }
        false
    }

    fn emit_if_else(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                let mut outer_seq_id = None;
                let mut outer_idx = None;
                let mut then_seq_id = None;
                let mut then_idx = None;
                let mut else_seq_id = None;
                let mut else_idx = None;

                instr_builder.block_at(
                    tracker.curr_idx,
                    None,
                    |outer_block| {
                        outer_seq_id = Some(outer_block.id());
                        outer_idx = Some(0 as usize);
                        outer_block.if_else(
                            None,
                            | then | {
                                then_seq_id = Some(then.id());
                                then_idx = Some(0 as usize);
                            },
                            |else_| {
                                else_seq_id = Some(else_.id());
                                else_idx = Some(0 as usize);
                            },
                        );
                    });
                // leave outer_block_idx as 0 to enable injection of condition!

                // Save the block information for future reference
                tracker.outer_seq_id = outer_seq_id;
                tracker.outer_idx = outer_idx;
                tracker.then_seq_id = then_seq_id;
                tracker.then_idx = then_idx;
                tracker.else_seq_id = else_seq_id;
                tracker.else_idx = else_idx;
                tracker.curr_idx += 1;
                return true;
            }
        }
        false
    }

    /// Will configure the emitter to emit subsequent expression as the condition of an if or if/else stmt
    /// Then emits the passed condition at that location.
    fn emit_condition(&mut self) -> bool {
        if let Some(tracker) = &mut self.emitting_instr {
            if let Some(outer_seq_id) = &tracker.outer_seq_id {
                if let Some(outer_idx) = &tracker.outer_idx {
                    tracker.curr_seq_id = outer_seq_id.clone();
                    tracker.curr_idx = outer_idx.clone();
                }
            }
        }
        false
    }

    /// Will configure the emitter to emit subsequent statements into the consequent body of an if or if/else stmt
    fn emit_consequent(&mut self) -> bool {
        if let Some(tracker) = &mut self.emitting_instr {
            if let Some(then_seq_id) = &tracker.then_seq_id {
                if let Some(then_idx) = &tracker.then_idx {
                    tracker.curr_seq_id = then_seq_id.clone();
                    tracker.curr_idx = then_idx.clone();
                }
            }
            return true;
        }
        false
    }

    /// Will configure the emitter to emit subsequent statements into the alternate body of an if/else stmt
    fn emit_alternate(&mut self) -> bool {
        if let Some(tracker) = &mut self.emitting_instr {
            if let Some(else_seq_id) = &tracker.else_seq_id {
                if let Some(else_idx) = &tracker.else_idx {
                    tracker.curr_seq_id = else_seq_id.clone();
                    tracker.curr_idx = else_idx.clone();
                    return true;
                }
            }
        }
        false
    }

    /// Will configure the emitter to emit subsequent statements in the outer block of some branching logic
    fn finish_branch(&mut self) -> bool {
        if let Some(tracker) = &mut self.emitting_instr {
            tracker.curr_seq_id = tracker.main_seq_id;
            tracker.curr_idx = tracker.main_idx;

            tracker.outer_seq_id = None;
            tracker.outer_idx = None;
            tracker.then_seq_id = None;
            tracker.then_idx = None;
            tracker.else_seq_id = None;
            tracker.else_idx = None;
            return true;
        }
        true
    }

    fn emit_body(&mut self, body: &mut Vec<Statement>) -> bool {
        let mut is_success = true;
        body.iter_mut().for_each(|stmt| {
            is_success &= self.emit_stmt(stmt);
        });
        is_success
    }

    fn has_alt_call(&mut self) -> bool {
        // check if we should inject an alternate call!
        // At this point the body has been visited, so "new_target_fn_name" would be defined
        let rec_id = match self.table.lookup(&"new_target_fn_name".to_string()) {
            Some(rec_id) => Some(rec_id.clone()),
            None => None
        };

        if rec_id.is_none() {
            info!("`new_target_fn_name` not configured for this probe.");
            return false;
        } else {
            let (name, func_call_id) = match rec_id {
                Some(r_id) => {
                    let rec = self.table.get_record_mut(&r_id);
                    if let Some(Record::Var { value: Some(Value::Str { val, .. }), .. }) = rec {
                        (val.clone(), self.app_wasm.funcs.by_name(val))
                    } else {
                        ("".to_string(), None)
                    }
                }
                None => {
                    ("".to_string(), None)
                },
            };
            if func_call_id.is_none() {
                info!("Could not find function in app Wasm specified by `new_target_fn_name`: {}", name);
                return false;
            }
            if let Some(curr_loc) = self.instr_iter.curr_mut() {
                curr_loc.instr_alt_call = func_call_id;
            } else {
                info!("The instruction iterator has not been initialized, we've hit a bug!");
                return false;
            }
        }
        true
    }

    fn emit_alt_call(&mut self) -> bool {
        let mut is_success = true;
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {

                if let Some(alt_fn_id) = curr_loc.instr_alt_call {
                    // we need to inject an alternate call to the specified fn name!
                    let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                    let func_builder = func.builder_mut();
                    let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                    // inject call
                    instr_builder.instr_at(tracker.curr_idx, walrus::ir::Call {
                        func: alt_fn_id.clone()
                    });
                    tracker.curr_idx += 1;

                    is_success &= true;
                } else {
                    error!("Could not inject alternate call to function, something went wrong...");
                }
            }
        }
        is_success
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                let func = self.app_wasm.funcs.get_mut(curr_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                return emit_stmt(&mut self.table, &mut self.app_wasm.data, stmt,
                                 &mut instr_builder, &mut self.metadata, &mut tracker.curr_idx);
            }
        }
        false
    }

    fn dump_to_file(&mut self, output_wasm_path: String) -> bool {
        match self.app_wasm.emit_wasm_file(&output_wasm_path) {
            Ok(_ok) => {
                true
            },
            Err(err) => {
                error!("Failed to dump instrumented wasm to {} from error: {}", &output_wasm_path, err);
                false
            },
        }
    }
}

// =====================
// ==== WasiEmitter ====
// =====================
// unimplemented

// =======================
// ==== VirgilEmitter ====
// =======================
// unimplemented

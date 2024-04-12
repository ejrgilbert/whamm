use log::error;
use walrus::{FunctionBuilder, FunctionId, FunctionKind, ImportedFunction, InstrSeqBuilder, LocalFunction, ModuleLocals, ValType};
use walrus::ir::{BinaryOp, ExtendedLoad, Instr, LoadKind, MemArg};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Dscript, Dtrace, Expr, Fn, Function, Module, Op, Provider, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait Emitter {
    fn enter_scope(&mut self);
    fn exit_scope(&mut self);

    fn emit_dtrace(&mut self, dtrace: &Dtrace) -> bool;
    fn emit_dscript(&mut self, dscript: &Dscript) -> bool;
    fn emit_provider(&mut self, context: &String, provider: &mut Provider) -> bool;

    // TODO -- should emit module/function/probe be private?
    fn emit_module(&mut self, context: &String, module: &mut Module) -> bool;

    fn emit_fn(&mut self, context_name: &String, f: &Fn) -> bool;
    fn emit_formal_param(&mut self, param: &(Expr, DataType)) -> bool;
    fn emit_global(&mut self, name: String, ty: DataType, val: &Option<Value>) -> bool;
    fn emit_stmt(&mut self, stmt: &Statement) -> bool;
    fn emit_expr(&mut self, expr: &Expr) -> bool;
    fn emit_op(&mut self, op: &Op) -> bool;
    fn emit_datatype(&mut self, datatype: &DataType) -> bool;
    fn emit_value(&mut self, val: &Value) -> bool;

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
fn create_arg_vars(table: &mut SymbolTable, app_locals: &mut ModuleLocals, func_params: &Option<Vec<ValType>>, instr_builder: &mut InstrSeqBuilder, index: &mut usize) {
    // No bytecodes should have been emitted in the module yet!
    // So, we can just save off the first * items in the stack as the args
    // to the call.
    if let Some(params) = func_params {
        params.iter().enumerate().for_each(|(num, param_ty)| {
            // create local for the param in the module
            let arg_local_id = app_locals.add(*param_ty);

            // emit a bytecode in the function to assign the ToS to this new local
            instr_builder.instr_at( *index,walrus::ir::LocalSet {
                local: arg_local_id.clone()
            });

            // update index to point to what follows our insertions
            *index += 1;

            // place in symbol table with var addr for future reference
            let arg_name = format!("arg{num}");
            table.put(arg_name.clone(), Record::Var {
                ty: DataType::Integer, // we only support integers right now.
                name: arg_name,
                value: None,
                addr: Some(VarAddr::Local {
                    addr: arg_local_id
                })
            });
        });
    }
}

fn emit_body(table: &mut SymbolTable, body: &Vec<Statement>, instr_builder: &mut InstrSeqBuilder, index: &mut usize) -> bool {
    let mut is_success = true;
    body.iter().for_each(|stmt| {
        is_success &= emit_stmt(table, stmt, instr_builder, index)
    });
    is_success
}

fn emit_stmt(table: &mut SymbolTable, stmt: &Statement, instr_builder: &mut InstrSeqBuilder, index: &mut usize) -> bool {
    let mut is_success = true;
    match stmt {
        Statement::Assign { var_id, expr } => {
            let folded_expr = ExprFolder::fold_expr(expr, table);
            if let Expr::Primitive { val } = folded_expr {
                // This is a constant, just save the value to the symbol table for later use
                return if let Expr::VarId { name } = var_id {
                    let var_rec_id = match table.lookup(name) {
                        Some(rec_id) => rec_id.clone(),
                        _ => {
                            error!("VarId '{name}' does not exist in this scope!");
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
                is_success &= emit_expr(table, expr, instr_builder, index);

                return if let Expr::VarId { name } = var_id {
                    let var_rec_id = match table.lookup(name) {
                        Some(rec_id) => rec_id.clone(),
                        _ => {
                            error!("VarId '{name}' does not exist in this scope!");
                            return false;
                        }
                    };
                    match table.get_record_mut(&var_rec_id) {
                        Some(Record::Var { addr, .. }) => {
                            // this will be different based on if this is a global or local var
                            match addr {
                                Some(VarAddr::Global { addr }) => {
                                    instr_builder.instr_at( *index,walrus::ir::GlobalSet {
                                        global: addr.clone()
                                    });
                                    // update index to point to what follows our insertions
                                    *index += 1;
                                }
                                Some(VarAddr::Local { addr } ) => {
                                    instr_builder.instr_at( *index,walrus::ir::LocalSet {
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
                };
            }
        }
        Statement::Expr { expr } => {
            is_success &= emit_expr(table, expr, instr_builder, index);
        }
    }
    is_success
}

fn emit_expr(table: &mut SymbolTable, expr: &Expr, instr_builder: &mut InstrSeqBuilder, index: &mut usize) -> bool {
    let mut is_success = true;
    match expr {
        Expr::BinOp {lhs, op, rhs} => {
            is_success &= emit_expr(table, lhs, instr_builder, index);
            is_success &= emit_expr(table, rhs, instr_builder, index);
            is_success &= emit_op(op, instr_builder, index);
        }
        Expr::Call { fn_target, args } => {
            let fn_name = match &**fn_target {
                Expr::VarId{ name } => name.clone(),
                _ => return false
            };

            // emit the arguments
            if let Some(args) = args {
                args.iter().for_each(|boxed_arg| {
                    let arg = &**boxed_arg; // unbox
                    is_success &= emit_expr(table, arg, instr_builder, index);
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
                    let mut fn_rec = table.get_record_mut(&rec_id);
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
            // also, pay attention to 'arg*' names. this could be a call probe
            // if let Some(arg_idx) = name.strip_prefix("arg") {
            //     // This is a compiler-provided `arg*` variable
            //     // TODO -- this needs to exist in the symbol table. Not done custom here.
            //
            //     // lookup arg* in symbol table
            //     // exists? reference it with the local ID
            //     // DNE? create a new
            //     instr_builder.instr_at( *index,walrus::ir::LocalGet {
            //         local: addr.clone()
            //     });
            //     // update index to point to what follows our insertions
            //     *index += 1;
            // }

            let var_rec_id = match table.lookup(&name) {
                Some(rec_id) => rec_id.clone(),
                _ => {
                    error!("VarId '{name}' does not exist in this scope!");
                    return false;
                }
            };
            match table.get_record_mut(&var_rec_id) {
                Some(Record::Var { addr, .. }) => {
                    // this will be different based on if this is a global or local var
                    match addr {
                        Some(VarAddr::Global { addr }) => {
                            instr_builder.instr_at( *index,walrus::ir::GlobalGet {
                                global: addr.clone()
                            });
                            // update index to point to what follows our insertions
                            *index += 1;
                        }
                        Some(VarAddr::Local { addr } ) => {
                            instr_builder.instr_at( *index,walrus::ir::LocalGet {
                                local: addr.clone()
                            });
                            // update index to point to what follows our insertions
                            *index += 1;
                        },
                        None => {
                            // TODO could be an `arg*` variable, need to check and initialize if so.

                            error!("Variable does not exist in scope: {name}");
                            return false;
                        }
                    }
                    return true;
                },
                Some(ty) => {
                    error!("Incorrect variable record, expected Record::Var, found: {:?}", ty);
                    return false;
                },
                None => {
                    error!("Variable symbol does not exist!");
                    return false;
                }
            }
        }
        Expr::Primitive { val } => {
            is_success &= emit_value(table, val, instr_builder, index);
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

fn emit_datatype(_datatype: &DataType, _instr_builder: &InstrSeqBuilder, _index: &mut usize) -> bool {
    // don't think i actually need this
    false
}

fn emit_value(table: &mut SymbolTable, val: &Value, instr_builder: &mut InstrSeqBuilder, index: &mut usize) -> bool {
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
        Value::Str { val, .. } => {
            // TODO -- decide where to insert the string
            //         1. store in a new memory...but not supported in dfinity
            //         2. store at a memory offset
            //         3. store at address 0 (for local strings)
            //            ...but will have to keep track of what we've used and then write back what we changed.
            // TODO -- update index to point to what follows our insertions
            // NEXT
            todo!()
        }
        Value::Tuple { vals, .. } => {
            vals.iter().for_each(|val| {
                is_success &= emit_expr(table, val, instr_builder, index);
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

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

pub(crate) struct WasmRewritingEmitter {
    pub(crate) app_wasm_path: String,
    pub(crate) app_wasm: walrus::Module,
    pub(crate) table: SymbolTable,

    fn_providing_contexts: Vec<String>
}
impl WasmRewritingEmitter {
    pub fn new(app_wasm_path: String, app_wasm: walrus::Module, table: SymbolTable) -> Self {
        Self {
            app_wasm_path,
            app_wasm,
            table,
            fn_providing_contexts: vec![ "dtrace".to_string() ]
        }
    }

    fn emit_wasm_bytecode_module(&mut self, module: &mut Module) -> bool {
        // TODO -- creating a new instance of app_wasm is obscene.
        // Create yet another copy of app_wasm because I cannot for the life of me figure out how to
        // avoid this, but it still work with Rust syntax restrictions...
        let app_wasm = walrus::Module::from_file(&self.app_wasm_path.clone()).unwrap();

        let mut is_success = true;
        app_wasm.funcs.iter_local().for_each(|(id, func)| {
            // TODO -- make sure that the id is not any of the injected function IDs
            let instr_seq = func.block(func.entry_block());
            instr_seq.instrs.iter().enumerate().for_each(|(index, (instr, ..))| {
                let instr_as_str = &format!("{:?}", instr);
                if let Some(function) = module.functions.get_mut(instr_as_str) {
                    // preprocess
                    let params = self.preprocess_instr(instr, function);
                    // passing a clone of index so it can be mutated as instructions are injected
                    is_success &= self.emit_function(function, id, &params, &mut index.clone());
                }
            });
        });
        is_success
    }
    fn preprocess_instr(&mut self, instr: &Instr, function: &mut Function) -> Option<Vec<ValType>> {
        if function.name.to_lowercase() == "call" {
            if let Instr::Call(func) = &instr {
                let func = self.app_wasm.funcs.get(func.func);
                let (func_kind, module, name, params) = match &func.kind {
                    FunctionKind::Import(ImportedFunction { ty: ty_id, import: import_id }) => {
                        let func_kind = "import";
                        let import = self.app_wasm.imports.get(*import_id);
                        let ty = self.app_wasm.types.get(*ty_id);

                        (func_kind, Some(&import.module), Some(&import.name), Vec::from(ty.params()))
                    },
                    FunctionKind::Local(LocalFunction{ args, ..}) => {
                        let func_kind = "local";
                        let mut params = vec![];
                        args.iter().for_each(|arg_id| {
                            let arg = self.app_wasm.locals.get(*arg_id);
                            params.push(arg.ty());
                        });

                        (func_kind, None, None, Vec::from(params))
                    },
                    FunctionKind::Uninitialized(ty_id) => {
                        let func_kind = "uninitialized";
                        let ty = self.app_wasm.types.get(*ty_id);

                        (func_kind, None, None, Vec::from(ty.params()))
                    }
                };
                // define compiler constants
                let rec_id = match self.table.lookup(&"target_fn_type".to_string()) {
                    Some(rec_id) => rec_id.clone(),
                    _ => {
                        error!("target_fn_type symbol does not exist in this scope!");
                        return Some(params);
                    }
                };
                let mut rec = self.table.get_record_mut(&rec_id);
                match &mut rec {
                    Some(Record::Var { value, .. }) => {
                        *value = Some(Value::Str {
                            ty: DataType::Str,
                            val: func_kind.to_string(),
                        });
                    }
                    _ => {}
                }

                let tuple = function.globals.get_mut("target_fn_type").unwrap();
                tuple.2 = Some(Value::Str {
                    ty: DataType::Str,
                    val: func_kind.to_string(),
                });

                if module.is_some() {
                    let rec_id = match self.table.lookup(&"target_imp_module".to_string()) {
                        Some(rec_id) => rec_id.clone(),
                        _ => {
                            error!("target_imp_module symbol does not exist in this scope!");
                            return Some(params);
                        }
                    };
                    let mut rec = self.table.get_record_mut(&rec_id);
                    match &mut rec {
                        Some(Record::Var { value, .. }) => {
                            *value = Some(Value::Str {
                                ty: DataType::Str,
                                val: module.unwrap().to_string(),
                            });
                        }
                        _ => {}
                    }
                    let tuple = function.globals.get_mut("target_imp_module").unwrap();
                    tuple.2 = Some(Value::Str {
                        ty: DataType::Str,
                        val: module.unwrap().to_string(),
                    });
                }

                if name.is_some() {
                    let rec_id = match self.table.lookup(&"target_imp_name".to_string()) {
                        Some(rec_id) => rec_id.clone(),
                        _ => {
                            error!("target_imp_name symbol does not exist in this scope!");
                            return Some(params);
                        }
                    };
                    let mut rec = self.table.get_record_mut(&rec_id);
                    match &mut rec {
                        Some(Record::Var { value, .. }) => {
                            *value = Some(Value::Str {
                                ty: DataType::Str,
                                val: name.unwrap().to_string(),
                            });
                        }
                        _ => {}
                    }
                    let tuple = function.globals.get_mut("target_imp_name").unwrap();
                    tuple.2 = Some(Value::Str {
                        ty: DataType::Str,
                        val: name.unwrap().to_string(),
                    });
                }
                return Some(params);
            }
        }
        None
    }
    fn emit_function(&mut self, function: &mut Function, func_id: FunctionId, func_params: &Option<Vec<ValType>>, index: &mut usize) -> bool {
        self.table.enter_scope();
        let is_success = true;

        // inject probes (should be at the correct point in the `walrus::ir::VisitorMut`)
        self.emit_probes_for_fn(function, func_id, func_params, index);

        self.table.exit_scope();
        is_success
    }
    fn emit_probes_for_fn(&mut self, function: &Function, func_id: FunctionId, func_params: &Option<Vec<ValType>>, index: &mut usize) -> bool {
        let mut is_success = true;
        // 1. Inject BEFORE probes
        if let Some(res) = self.emit_probes(function, func_id, func_params, &"before".to_string(), index) {
            // Assumption: before probes push/pop from stack so it is equivalent to what it was originally
            is_success &= res;
        }
        // 2. Inject ALT probes
        if let Some(res) = self.emit_probes(function, func_id, func_params, &"alt".to_string(), index) {
            is_success &= res;
        }
        // 3. Inject AFTER probes
        if let Some(res) = self.emit_probes(function, func_id, func_params, &"after".to_string(), index) {
            // Assumption: before probes push/pop from stack so it is equivalent to what it was originally
            is_success &= res;
        }

        is_success
    }
    fn emit_probes(&mut self, function: &Function, func_id: FunctionId, func_params: &Option<Vec<ValType>>, probe_name: &String, index: &mut usize) -> Option<bool> {
        let mut is_success = true;

        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(func_id).kind.unwrap_local_mut();
        let entry_block_id = func.entry_block();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(entry_block_id);

        if let Some(probes) = function.probe_map.get(probe_name) {
            let mut removed = false;
            let mut emitted_params = false;
            probes.iter().for_each(|probe| {
                if !removed && probe_name == "alt" && probe.body.is_some() {
                    // remove the original bytecode first
                    instr_builder.instrs_mut().remove(*index);
                    // only remove the original bytecode index once!
                    removed = true;
                }

                self.table.enter_scope();
                if probe.body.is_some() && probe.predicate.is_some() {
                    // Fold predicate via constant propagation
                    let folded_pred = ExprFolder::fold_expr(&probe.predicate.as_ref().unwrap(), &self.table);

                    if let Some(pred_as_bool) = ExprFolder::get_single_bool(&folded_pred) {
                        if !pred_as_bool {
                            // predicate is FALSE, DON'T INJECT!
                            is_success &= true;
                            return;
                        }
                        // predicate is TRUE, unconditionally inject body stmts
                    } else {
                        // predicate has not been reduced to a boolean value, inject
                        if !emitted_params && function.name == "call" {
                            // save the inputs to the current bytecode
                            create_arg_vars(&mut self.table, &mut self.app_wasm.locals, func_params, &mut instr_builder, index);
                            emitted_params = true;
                        }

                        // inject predicate
                        is_success &= emit_expr(&mut self.table, &folded_pred, &mut instr_builder, index);
                    }
                }
                if probe.body.is_some() {
                    if !emitted_params && function.name == "call" {
                        // save the inputs to the current bytecode
                        create_arg_vars(&mut self.table, &mut self.app_wasm.locals, func_params, &mut instr_builder, index);
                        emitted_params = true;
                    }

                    let body = probe.body.as_ref().unwrap();
                    is_success &= emit_body(&mut self.table, &body, &mut instr_builder, index);
                }

                if probe_name == "alt" {
                    // check if we should inject an alternate call!
                    let rec_id = match self.table.lookup(&"new_target_imp_name".to_string()) {
                        Some(rec_id) => Some(rec_id.clone()),
                        None => None
                    };

                    if let Some(r_id) = rec_id {
                        if let Some(Record::Var { value: Some(value), .. }) = self.table.get_record_mut(&r_id) {
                            if let Value::Str {val, ..} = value {
                                // we need to inject an alternate call to the specified fn name!

                                // XXX -- creating a new instance of app_wasm is obscene.
                                // Create yet another copy of app_wasm because I cannot for the life of me figure out how to
                                // avoid this, but it still work with Rust syntax restrictions...
                                let app_wasm = walrus::Module::from_file(&self.app_wasm_path.clone()).unwrap();

                                let f_id = match app_wasm.funcs.by_name(&val) {
                                    Some(f_id) => Some(f_id.clone()),
                                    None => None
                                };
                                if let Some(f_id) = f_id {
                                    instr_builder.instr_at( *index,walrus::ir::Call {
                                        func: f_id.clone()
                                    });
                                    // update index to point to what follows our insertions
                                    *index += 1;
                                    is_success &= true;
                                } else {
                                    error!("Could not inject alternate call to '{val}' function, not found in Wasm module");
                                    is_success &= false;
                                }
                            }
                        }
                    }
                }
                self.table.exit_scope();
            });
            Some(is_success)
        } else {
            None
        }
    }

    fn create_arg_vars(&mut self, func_params: &Option<Vec<ValType>>, instr_builder: &mut InstrSeqBuilder, index: &mut usize) {
        // No bytecodes should have been emitted in the module yet!
        // So, we can just save off the first * items in the stack as the args
        // to the call.
        if let Some(params) = func_params {
            params.iter().enumerate().for_each(|(num, param_ty)| {
                // create local for the param in the module
                let arg_local_id = self.app_wasm.locals.add(*param_ty);

                // emit a bytecode in the function to assign the ToS to this new local
                instr_builder.instr_at( *index,walrus::ir::LocalSet {
                    local: arg_local_id.clone()
                });

                // update index to point to what follows our insertions
                *index += 1;

                // place in symbol table with var addr for future reference
                let arg_name = format!("arg{num}");
                self.table.put(arg_name.clone(), Record::Var {
                    ty: DataType::Integer, // we only support integers right now.
                    name: arg_name,
                    value: None,
                    addr: Some(VarAddr::Local {
                        addr: arg_local_id
                    })
                });
            });
        }
    }

    fn emit_provided_fn(&mut self, context: &String, f: &Fn) -> bool {
        return if context == &"dtrace".to_string() && &f.name == &"strcmp".to_string() {
            self.emit_dtrace_strcmp_fn(f)
        } else {
            error!("Provided function, but could not find a context to provide the definition");
            false
        }
    }

    fn emit_dtrace_strcmp_fn(&mut self, f: &Fn) -> bool {
        let strcmp_params = vec![walrus::ValType::I32, walrus::ValType::I32, walrus::ValType::I32, walrus::ValType::I32];
        let strcmp_result = vec![walrus::ValType::I32];

        let mut strcmp = FunctionBuilder::new(&mut self.app_wasm.types, &strcmp_params, &strcmp_result);

        // get memory id
        let memory_id = self.app_wasm.memories
            .iter()
            .next()
            .expect("only single memory is supported")
            .id();

        // create params
        let str0_offset = self.app_wasm.locals.add(walrus::ValType::I32);
        let str0_size = self.app_wasm.locals.add(walrus::ValType::I32);
        let str1_offset = self.app_wasm.locals.add(walrus::ValType::I32);
        let str1_size = self.app_wasm.locals.add(walrus::ValType::I32);

        // create locals
        let i = self.app_wasm.locals.add(walrus::ValType::I32);
        let str0_char = self.app_wasm.locals.add(walrus::ValType::I32);
        let str1_char = self.app_wasm.locals.add(walrus::ValType::I32);

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
                        .binop(BinaryOp::I32Eq)
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
                                    memory_id,
                                    LoadKind::I32_8 {
                                        kind: ExtendedLoad::SignExtend,
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
                                    memory_id,
                                    LoadKind::I32_8 {
                                        kind: ExtendedLoad::SignExtend,
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
                        .br_if(eq);
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
            Some(rec_id) => rec_id.clone(),
            _ => {
                error!("strcmp fn symbol does not exist in this scope!");
                return false;
            }
        };

        let rec = self.table.get_record_mut(&rec_id);
        return match rec {
            Some(Record::Fn { mut addr, .. }) => {
                addr = Some(strcmp_id);
                true
            },
            Some(ty) => {
                error!("Incorrect global variable record, expected Record::Var, found: {:?}", ty);
                false
            },
            None => {
                error!("Global variable symbol does not exist!");
                false
            }
        };
    }
}
/// Walrus Visitor over `app.wasm`
/// - as we get relevant info, lookup in SymbolTable for binding to globally set that value
/// - for each bytecode, do we have a probe?
///   - fold predicate with known globals. FALSE? Don't inject! NOT FALSE? inject (with remaining Expr, not folded parts)
///   - See fold Rust pattern: https://rust-unofficial.github.io/patterns/patterns/creational/fold.html
/// - now we have instrumented `app.wasm`
///   - write to app_instr.wasm
impl Emitter for WasmRewritingEmitter {
    fn enter_scope(&mut self) {
        self.table.enter_scope();
    }
    fn exit_scope(&mut self) {
        self.table.exit_scope();
    }
    fn emit_dtrace(&mut self, _dtrace: &Dtrace) -> bool {
        // nothing to do here
        true
    }
    fn emit_dscript(&mut self, _dscript: &Dscript) -> bool {
        // nothing to do here
        true
    }
    fn emit_provider(&mut self, context: &String, provider: &mut Provider) -> bool {
        let mut is_success = true;
        provider.modules.iter_mut().for_each(|(_name, module)| {
            is_success &= self.emit_module(context, module);
        });
        is_success
    }
    fn emit_module(&mut self, context: &String, module: &mut Module) -> bool {
        return if context == &"dtrace:dscript:wasm:bytecode".to_string() {
            self.emit_wasm_bytecode_module(module)
        } else {
            error!("Provided function, but could not find a context to provide the definition");
            false
        };
    }
    fn emit_fn(&mut self, context: &String, f: &Fn) -> bool {
        self.table.enter_scope();
        // figure out if this is a provided fn.
        if f.is_provided {
            return if self.fn_providing_contexts.contains(context) {
                self.emit_provided_fn(context, f)
            } else {
                error!("Provided function, but could not find a context to provide the definition");
                false
            }
        }

        // emit non-provided fn
        // only when we're supporting user-defined fns in dscript...
        unimplemented!();
    }
    fn emit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        // only when we're supporting user-defined fns in dscript...
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
                // only when we're supporting user-defined globals in dscript...
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

    fn emit_stmt(&mut self, _stmt: &Statement) -> bool {
        unimplemented!()
    }

    fn emit_expr(&mut self, _expr: &Expr) -> bool {
        unimplemented!()
    }

    fn emit_op(&mut self, _op: &Op) -> bool {
        unimplemented!()
    }

    fn emit_datatype(&mut self, _datatype: &DataType) -> bool {
        unimplemented!()
    }

    fn emit_value(&mut self, _val: &Value) -> bool {
        unimplemented!()
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

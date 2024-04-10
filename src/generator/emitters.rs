use log::error;
use walrus::{FunctionBuilder, FunctionId, FunctionKind, InstrSeqBuilder, LocalFunction};
use walrus::ir::{BinaryOp, ExtendedLoad, Instr, InstrLocId, InstrSeq, InstrSeqId, LoadKind, MemArg};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Dscript, Dtrace, Expr, Fn, Function, Module, Op, Provider, Statement, Value};
use crate::parser::types::Rule::predicate;
use crate::verifier::types::{Record, SymbolTable};
use crate::verifier::types::Record::Var;

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

fn emit_body(body: &Vec<Statement>, instr_builder: &InstrSeqBuilder, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
    let mut is_success = true;
    body.iter().for_each(|stmt| {
        is_success &= emit_stmt(stmt, instr_builder, index, instr, instr_loc)
    });
    is_success
}

fn emit_stmt(stmt: &Statement, instr_builder: &InstrSeqBuilder, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
    match stmt {
        Statement::Assign { .. } => {
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
        Statement::Expr { expr } => {
            emit_expr(expr, instr_builder, index, instr, instr_loc)
        }
    }
}

fn emit_expr(expr: &Expr, instr_builder: &InstrSeqBuilder, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
    match expr {
        Expr::BinOp { .. } => {
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
        Expr::Call { .. } => {
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
        Expr::VarId { .. } => {
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
        Expr::Primitive { val } => {
            emit_value(val, instr_builder, index, instr, instr_loc)
        }
    }
}

fn emit_op(_op: &Op, _instr_builder: &InstrSeqBuilder, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
    // don't think i actually need this
    false
}

fn emit_datatype(_datatype: &DataType, _instr_builder: &InstrSeqBuilder, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
    // don't think i actually need this
    false
}

fn emit_value(val: &Value, _instr_builder: &InstrSeqBuilder, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
    match val {
        Value::Integer { val, .. } => {
            // instr_seq.instrs.insert(index, (Instr::Const {
            //     value: walrus::ir::Value::I32(val.clone())
            // }, index))
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
        Value::Str { val, .. } => {
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
        Value::Tuple { vals, .. } => {
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
        Value::Boolean { val, .. } => {
            // TODO -- update index to point to what follows our insertions
            todo!()
        }
    }
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
            instr_seq.instrs.iter().enumerate().for_each(|(index, (instr, instr_loc))| {
                let instr_as_str = &format!("{:?}", instr);
                if let Some(function) = module.functions.get_mut(instr_as_str) {
                    // preprocess
                    self.preprocess_instr(instr, function);
                    // passing a clone of index so it can be mutated as instructions are injected
                    is_success &= self.emit_function(function, id, &mut index.clone(), instr, instr_loc);
                }
            });
        });
        is_success
    }
    fn preprocess_instr(&mut self, instr: &Instr, function: &mut Function) {
        if function.name.to_lowercase() == "call" {
            if let Instr::Call(func) = &instr {
                let func = self.app_wasm.funcs.get(func.func);
                let (func_kind, module, name) = match &func.kind {
                    FunctionKind::Import(imp) => {
                        let func_kind = "import";
                        let import = self.app_wasm.imports.get(imp.import);

                        (func_kind, Some(&import.module), Some(&import.name))
                    },
                    FunctionKind::Local(..) => {
                        let func_kind = "local";

                        (func_kind, None, None)
                    },
                    FunctionKind::Uninitialized(..) => {
                        let func_kind = "uninitialized";

                        (func_kind, None, None)
                    }
                };
                // define compiler constants
                let rec_id = match self.table.lookup(&"target_fn_type".to_string()) {
                    Some(rec_id) => rec_id.clone(),
                    _ => {
                        error!("target_fn_type symbol does not exist in this scope!");
                        return;
                    }
                };
                let mut rec = self.table.get_record_mut(&rec_id);
                match &mut rec {
                    Some(Var { value, .. }) => {
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
                            return;
                        }
                    };
                    let mut rec = self.table.get_record_mut(&rec_id);
                    match &mut rec {
                        Some(Var { value, .. }) => {
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
                            return;
                        }
                    };
                    let mut rec = self.table.get_record_mut(&rec_id);
                    match &mut rec {
                        Some(Var { value, .. }) => {
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

                //TODO: What are the inputs to the current bytecode?
                //     - save these off
            };
        }
    }
    fn emit_function(&mut self, function: &mut Function, func_id: FunctionId, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
        self.table.enter_scope();
        let is_success = true;

        // inject probes (should be at the correct point in the `walrus::ir::VisitorMut`)
        self.emit_probes_for_fn(function, func_id, index, instr, instr_loc);

        self.table.exit_scope();
        is_success
    }
    fn emit_probes_for_fn(&mut self, function: &Function, func_id: FunctionId, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> bool {
        let mut is_success = true;
        // 1. Inject BEFORE probes
        if let Some(res) = self.emit_probes(function, func_id, &"before".to_string(), index, instr, instr_loc) {
            // Assumption: before probes push/pop from stack so it is equivalent to what it was originally
            is_success &= res;
        }
        // 2. Inject ALT probes
        if let Some(res) = self.emit_probes(function, func_id, &"alt".to_string(), index, instr, instr_loc) {
            is_success &= res;
        }
        // 3. Inject AFTER probes
        if let Some(res) = self.emit_probes(function, func_id, &"after".to_string(), index, instr, instr_loc) {
            // Assumption: before probes push/pop from stack so it is equivalent to what it was originally
            is_success &= res;
        }

        is_success
    }
    fn emit_probes(&mut self, function: &Function, func_id: FunctionId, probe_name: &String, index: &mut usize, instr: &Instr, instr_loc: &InstrLocId) -> Option<bool> {
        let mut is_success = true;

        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let mut func = self.app_wasm.funcs.get_mut(func_id).kind.unwrap_local_mut();
        let mut entry_block_id = func.entry_block();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(entry_block_id);

        if let Some(probes) = function.probe_map.get(probe_name) {
            let mut removed = false;
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
                        is_success &= emit_expr(&folded_pred, &instr_builder, index, instr, instr_loc);
                    }
                }
                if probe.body.is_some() {
                    let body = probe.body.as_ref().unwrap();
                    is_success &= emit_body(&body, &instr_builder, index, instr, instr_loc);
                }
                self.table.exit_scope();
            });
            Some(is_success)
        } else {
            None
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

        let mut strcmp = walrus::FunctionBuilder::new(&mut self.app_wasm.types, &strcmp_params, &strcmp_result);

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

        // TODO -- emit non-provided fn
        //         only when we're supporting user-defined fns in dscript...
        unimplemented!();
    }
    fn emit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        // TODO -- only when we're supporting user-defined fns in dscript...
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
                // TODO -- emit global variable and set addr in symbol table
                //         only when we're supporting user-defined globals in dscript...
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
        todo!()
    }

    fn emit_expr(&mut self, _expr: &Expr) -> bool {
        todo!()
    }

    fn emit_op(&mut self, _op: &Op) -> bool {
        todo!()
    }

    fn emit_datatype(&mut self, _datatype: &DataType) -> bool {
        todo!()
    }

    fn emit_value(&mut self, _val: &Value) -> bool {
        todo!()
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
// TODO

// =======================
// ==== VirgilEmitter ====
// =======================
// TODO

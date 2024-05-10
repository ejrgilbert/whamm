use std::collections::HashMap;
use std::process::exit;
use log::{error, info, warn};
use regex::Regex;
use walrus::{ActiveData, ActiveDataLocation, DataKind, FunctionBuilder, FunctionId, FunctionKind,
             ImportedFunction, InstrLocId, InstrSeqBuilder, LocalFunction, MemoryId, ModuleData, ValType};
use walrus::ir::{BinaryOp, dfs_pre_order_mut, ExtendedLoad, Instr, InstrSeqId, LoadKind, MemArg};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Whammy, Whamm, Expr, Fn, Event, Package, Op, Probe, Provider, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait Emitter {
    fn enter_scope(&mut self);
    fn enter_named_scope(&mut self, scope_name: &String);
    fn exit_scope(&mut self);
    fn reset_children(&mut self);

    fn emit_whamm(&mut self, whamm: &Whamm) -> bool;
    fn emit_whammy(&mut self, whammy: &Whammy) -> bool;
    fn emit_provider(&mut self, context: &str, provider: &mut Provider) -> bool;
    fn emit_package(&mut self, context: &str, package: &mut Package) -> bool;
    fn emit_event(&mut self, context: &str, event: &mut Event) -> bool;
    fn emit_params(&mut self) -> bool;
    fn emit_probe(&mut self, context: &str, probe: &mut Probe) -> bool;
    fn fold_expr(&mut self, expr: &mut Expr) -> bool;

    fn emit_fn(&mut self, context_name: &str, f: &Fn) -> bool;
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

fn emit_body(table: &mut SymbolTable, module_data: &mut ModuleData, body: &mut Vec<Statement>,
             instr_builder: &mut InstrSeqBuilder, metadata: &mut InsertionMetadata, index: &mut usize) -> bool {
    let mut is_success = true;
    body.iter_mut().for_each(|stmt| {
        is_success &= emit_stmt(table, module_data, stmt, instr_builder, metadata, index)
    });
    is_success
}

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

fn _emit_datatype(_datatype: &DataType, _instr_builder: &InstrSeqBuilder, _index: &mut usize) -> bool {
    // don't think i actually need this
    false
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

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

struct InsertionMetadata {
    curr_event: String,
    mem_id: MemoryId,
    curr_mem_offset: u32,
}

// Struct to store info on insertion locations for an instruction sequence.
// Note that blocks can be indefinitely nested.
#[derive(Debug)]
struct ProbeLoc {
    wasm_func_name: Option<String>,
    wasm_func_id: FunctionId,
    instr_seq_id: InstrSeqId,
    index: usize,

    instr_name: String,
    instr: Instr,
    instr_params: Option<Vec<ValType>>,
    instr_created_args: Vec<(String, usize)>,

    // Save off the compiler-defined constants for this instruction
    instr_symbols: HashMap<String, Record>
}

fn get_probe_insert_locations(probe_locs: &mut HashMap<String, Vec<ProbeLoc>>, package: &mut Package,
                              func_id: FunctionId, func_name: Option<String>, func: &LocalFunction,
                              instr_seq_id: InstrSeqId) {
    func.block(instr_seq_id)
        .iter()
        .enumerate()
        .for_each(|(index, (instr, _))| {
            let instr_as_str = &format!("{:?}", instr);
            let instr_name = instr_as_str.split("(").next().unwrap().to_lowercase();

            if let Some(_event) = package.events.get_mut(&instr_name) {
                // This instruction might need to be probed!
                // get current probe locations for this instr type
                let loc_list = match probe_locs.get_mut(&instr_name) {
                    Some(probe_loc) => {
                        probe_loc
                    },
                    None => {
                        probe_locs.insert(instr_name.clone(), vec![]);
                        probe_locs.get_mut(&instr_name).unwrap()
                    }
                };

                // add current instr
                loc_list.push( ProbeLoc {
                    wasm_func_name: func_name.clone(),
                    wasm_func_id: func_id.clone(),
                    instr_seq_id,
                    index,
                    instr_name: instr_name.clone(),
                    instr: instr.clone(),
                    instr_params: None,
                    instr_created_args: vec![],
                    instr_symbols: HashMap::new()
                });
            }
            // visit nested blocks
            match instr {
                Instr::Block(block) => {
                    get_probe_insert_locations(probe_locs, package, func_id, func_name.clone(), func, block.seq);
                }
                Instr::Loop(_loop) => {
                    get_probe_insert_locations(probe_locs, package, func_id, func_name.clone(), func, _loop.seq);
                }
                Instr::IfElse(if_else, ..) => {
                    println!("IfElse: {:#?}", if_else);
                    get_probe_insert_locations(probe_locs, package, func_id, func_name.clone(), func, if_else.consequent);
                    get_probe_insert_locations(probe_locs, package, func_id, func_name.clone(), func, if_else.alternative);
                }
                _ => {
                    // do nothing extra for other instructions
                }
            }
        });
}

pub struct WasmRewritingEmitter {
    pub app_wasm: walrus::Module,
    pub table: SymbolTable,

    // whamm! AST traversal bookkeeping
    metadata: Option<InsertionMetadata>,
    probe_locs: HashMap<String, Vec<ProbeLoc>>, // event_name -> ProbeLoc

    fn_providing_contexts: Vec<String>
}
impl WasmRewritingEmitter {
    pub fn new(app_wasm: walrus::Module, table: SymbolTable) -> Self {
        Self {
            app_wasm,
            table,
            metadata: None,
            probe_locs: HashMap::new(),
            fn_providing_contexts: vec![ "whamm".to_string() ]
        }
    }

    fn next_instr(&mut self) {

    }

    fn setup_wasm_bytecode_package(&mut self, _context: &str, package: &mut Package) -> bool {
        // Initialize this to 4 MB
        let mem_id = self.app_wasm.memories.iter().next()
            .expect("only single memory is supported")
            .id();
        let mut curr_mem_offset: u32 = 1_052_576; // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
        let mut is_success = true;

        // Figure out which functions to visit
        let mut probe_locs = HashMap::new();
        for func in self.app_wasm.funcs.iter() {
            let id = func.id();
            if let Some(name) = func.name.as_ref() {
                // TODO -- get rid of this necessity (probably by removing the need to have
                //         functions already present in the app code)
                if name.starts_with("instr_") {
                    continue;
                }

                if name.contains("CallFuture$LT") {
                    println!("reached it!");
                }
            }

            if let FunctionKind::Local(local_func) = &func.kind {
                // TODO -- make sure that the id is not any of the injected function IDs (strcmp)
                get_probe_insert_locations(&mut probe_locs, package, id, func.name.clone(),
                                           local_func, local_func.entry_block());

                // cache and store constants with the ProbeLoc
                // self.preprocess_instrs();
            }
        }

        // Just have this find the probe insert locations, then emit_event iterates over this list!
        self.metadata = Some(InsertionMetadata {
            curr_event: "".to_string(),
            mem_id,
            curr_mem_offset
        });
        self.probe_locs = probe_locs;
        is_success
    }

    fn preprocess_instrs(&mut self) {
        // iterate over found probe insert locations and process each instruction
        // (define compiler constants and get params)
        for (instr_name, probe_locs) in self.probe_locs.iter_mut() {
            for probe_loc in probe_locs.iter_mut() {
                if instr_name.to_lowercase() == "call" {
                    if let Instr::Call(func) = &probe_loc.instr {
                        let func = self.app_wasm.funcs.get(func.func);
                        // if func.name.as_ref().unwrap().contains("ZN87") {
                        //     println!("{}", func.name.as_ref().unwrap());
                        // }
                        let (func_kind, module, name, params) = match &func.kind {
                            FunctionKind::Import(ImportedFunction { ty: ty_id, import: import_id }) => {
                                let func_kind = "import";
                                let import = self.app_wasm.imports.get(*import_id);
                                let ty = self.app_wasm.types.get(*ty_id);

                                (func_kind, import.module.clone(), import.name.clone(), Vec::from(ty.params()))
                            },
                            FunctionKind::Local(LocalFunction{ args, ..}) => {
                                let func_kind = "local";
                                let mut params = vec![];
                                args.iter().for_each(|arg_id| {
                                    let arg = self.app_wasm.locals.get(*arg_id);
                                    params.push(arg.ty());
                                });

                                (func_kind, "".to_string(), "".to_string(), Vec::from(params))
                            },
                            FunctionKind::Uninitialized(ty_id) => {
                                let func_kind = "uninitialized";
                                let ty = self.app_wasm.types.get(*ty_id);

                                (func_kind, "".to_string(), "".to_string(), Vec::from(ty.params()))
                            }
                        };
                        // save off the param types to this instruction
                        probe_loc.instr_params = Some(params);

                        // define and save off compiler constants
                        let var_name = "target_fn_type".to_string();
                        let rec_id = match self.table.lookup(&var_name) {
                            Some(rec_id) => rec_id.clone(),
                            _ => {
                                error!("{} symbol does not exist in this scope!", var_name);
                                return;
                            }
                        };
                        let rec = self.table.get_record_mut(&rec_id);
                        if let Some(Record::Var {..}) = rec {
                            let mut new_rec = rec.unwrap().clone();
                            match &mut new_rec {
                                Record::Var { value, .. } => {
                                    *value = Some(Value::Str {
                                        ty: DataType::Str,
                                        val: func_kind.to_string(),
                                        addr: None
                                    });
                                }
                                _ => {}
                            }

                            probe_loc.instr_symbols.insert(var_name.clone(), new_rec);
                        }

                        let var_name = "target_imp_module".to_string();
                        let rec_id = match self.table.lookup(&"target_imp_module".to_string()) {
                            Some(rec_id) => rec_id.clone(),
                            _ => {
                                error!("{} symbol does not exist in this scope!", var_name);
                                return;
                            }
                        };
                        let rec = self.table.get_record_mut(&rec_id);
                        if let Some(Record::Var {..}) = rec {
                            let mut new_rec = rec.unwrap().clone();
                            match &mut new_rec {
                                Record::Var { value, .. } => {
                                    *value = Some(Value::Str {
                                        ty: DataType::Str,
                                        val: module.clone(),
                                        addr: None
                                    });
                                }
                                _ => {}
                            }

                            probe_loc.instr_symbols.insert(var_name.clone(), new_rec);
                        }

                        let var_name = "target_imp_name".to_string();
                        let rec_id = match self.table.lookup(&"target_imp_name".to_string()) {
                            Some(rec_id) => rec_id.clone(),
                            _ => {
                                error!("{} symbol does not exist in this scope!", var_name);
                                return;
                            }
                        };
                        let rec = self.table.get_record_mut(&rec_id);
                        if let Some(Record::Var {..}) = rec {
                            let mut new_rec = rec.unwrap().clone();
                            match &mut new_rec {
                                Record::Var { value, .. } => {
                                    *value = Some(Value::Str {
                                        ty: DataType::Str,
                                        val: name.clone(),
                                        addr: None
                                    });
                                }
                                _ => {}
                            }

                            probe_loc.instr_symbols.insert(var_name.clone(), new_rec);
                        }
                    }
                }
            }
        }
    }

    fn emit_probe_for_loc(&mut self, probe: &mut Probe, instr_name: &String, probe_loc: &mut ProbeLoc) -> bool {
        let mut is_success = true;
        // TODO -- load symbols for this instruction

        if instr_name == "call" {
            // save off call params if instrumenting a fn call
            self.save_params(probe_loc);
        }

        // determine if I should inject a predicate.
        if let Some(predicate) = &mut probe.predicate {
            // Fold predicate via constant propagation
            self.fold_expr(predicate);
        }
        if let Some(pred_as_bool) = ExprFolder::get_single_bool(&probe.predicate.as_ref().unwrap()) {
            // predicate has been reduced to a boolean value, check true/false
            // otherwise: predicate has not been reduced to a boolean value, will need to inject the folded predicate
            if !pred_as_bool {
                // predicate is FALSE, DON'T INJECT PROBE IN GENERAL, so just return from this visit!
                info!("Predicate is false, no need to inject probe.");
                return true;
            }
            // predicate is TRUE, unconditionally inject body stmts
            probe.predicate = None;
        }

        if let Some(predicate) = &mut probe.predicate {
            if probe.name == "alt" {
                self.emit_predicate(predicate, probe_loc);

                // an alternate probe will need to emit an if/else
                // if pred { <alt_body>; Optional(<alt_call>;) } else { <original_instr> }
                let (
                    if_then_block_id,
                    mut if_then_idx,
                    else_block_id,
                    mut else_idx
                ) = self.emit_alt_body(probe, probe_loc);

                // 2. possibly emit alt call (if configured to do so)
                if instr_name == "call" {
                    is_success &= self.emit_alt_call(probe_loc, &if_then_block_id, &mut if_then_idx);

                    // This is a call instruction, emit original parameters for the original call in the `else` block
                    self.emit_params(probe_loc, &else_block_id, &mut else_idx);
                }
            } else {
                // other probe types will just need to have an if block conditional on the predicate
                // if pred { <probe_body>; }
                self.emit_predicated_body(probe, predicate, probe_loc.wasm_func_id, &probe_loc.instr_seq_id, &mut probe_loc.index);
            }
        } else {
            // No predicate, just emit the un-predicated probe body
            // <probe_body>;
            is_success &= self.emit_body(probe.body.as_mut().unwrap(), probe_loc.wasm_func_id, &probe_loc.instr_seq_id, &mut probe_loc.index);

            if instr_name == "call" && probe.name == "alt" {
                self.remove_orig_bytecode(probe, probe_loc);

                // 2. possibly emit alt call (if configured to do so)
                self.emit_alt_call(probe_loc, &probe_loc.instr_seq_id, &mut probe_loc.index);
            }
        }
        is_success
    }

    // fn create_arg_vars(&mut self, func_params: &Option<Vec<ValType>>, func_id: FunctionId,
    //                    instr_seq_id: &InstrSeqId, index: &mut usize) -> Vec<(String, usize)> {
    //
    // }

    fn emit_predicated_body(&mut self, probe: &mut Probe, predicate: &mut Expr, func_id: FunctionId,
                 instr_seq_id: &InstrSeqId, index: &mut usize) {
        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(func_id).kind.unwrap_local_mut();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(*instr_seq_id);

        instr_builder.block_at(
            *index,
            None,
            |mut probe_block| {
                let probe_block_id = probe_block.id();
                // create new `index` var to store current index into the of the `then` instr sequence
                let mut probe_block_idx = 0 as usize;

                // inject predicate
                if !emit_expr(&mut self.table, &mut self.app_wasm.data, predicate, &mut probe_block,
                              self.metadata.as_mut().unwrap(), &mut probe_block_idx) {
                    error!("Failed to inject predicate!");
                    exit(1);
                }

                // If result of predicate equals 0, break out of the probe block.
                // Will continue with the application code.
                probe_block
                    .i32_const(0)
                    .binop(BinaryOp::I32Eq)
                    .br_if(probe_block_id);

                probe_block_idx += 3; // account for the 3 instructions above!

                // At this point we know the predicate returned `true`, so we need to fire the probe body
                emit_body(&mut self.table, &mut self.app_wasm.data, probe.body.as_mut().unwrap(),
                          &mut probe_block, self.metadata.as_mut().unwrap(), &mut probe_block_idx);
            });

        *index += 1;
    }

    fn emit_predicate(&mut self, predicate: &mut Expr, probe_loc: &mut ProbeLoc) {
        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(probe_loc.wasm_func_id).kind.unwrap_local_mut();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(probe_loc.instr_seq_id);

        if !emit_expr(&mut self.table, &mut self.app_wasm.data, predicate, &mut instr_builder,
                      self.metadata.as_mut().unwrap(), &mut probe_loc.index) {
            error!("Failed to inject predicate!");
            exit(1);
        }
    }

    fn emit_body(&mut self, body: &mut Vec<Statement>, func_id: FunctionId, instr_seq_id: &InstrSeqId, index: &mut usize) -> bool {
        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(func_id).kind.unwrap_local_mut();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(*instr_seq_id);

        emit_body(&mut self.table, &mut self.app_wasm.data, body, &mut instr_builder,
                  self.metadata.as_mut().unwrap(), index)
    }

    fn remove_orig_bytecode(&mut self, probe: &mut Probe, probe_loc: &mut ProbeLoc) -> Option<(Instr, InstrLocId)> {
        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(probe_loc.wasm_func_id).kind.unwrap_local_mut();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(probe_loc.instr_seq_id);

        // remove the original instruction and store it for later use
        let mut orig_instr: Option<(Instr, InstrLocId)> = None;
        if probe.name == "alt" {
            // remove the original bytecode first
            orig_instr = Some(instr_builder.instrs_mut().remove(probe_loc.index))
        }
        orig_instr
    }

    fn save_params(&mut self, probe_loc: &mut ProbeLoc) {
        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(probe_loc.wasm_func_id).kind.unwrap_local_mut();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(probe_loc.instr_seq_id);

        // No bytecodes should have been emitted in the module yet!
        // So, we can just save off the first * items in the stack as the args
        // to the call.
        let mut arg_recs = vec![]; // vec to retain order!
        if let Some(params) = &probe_loc.instr_params {
            params.iter().enumerate().for_each(|(num, param_ty)| {
                // create local for the param in the module
                let arg_local_id = self.app_wasm.locals.add(*param_ty);

                // emit a bytecode in the event to assign the ToS to this new local
                instr_builder.instr_at( probe_loc.index,walrus::ir::LocalSet {
                    local: arg_local_id.clone()
                });

                // update index to point to what follows our insertions
                probe_loc.index += 1;

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
        probe_loc.instr_created_args = arg_recs;
    }

    fn emit_params(&mut self, probe_loc: &ProbeLoc, instr_seq_id: &InstrSeqId, index: &mut usize) {
        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(probe_loc.wasm_func_id).kind.unwrap_local_mut();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(*instr_seq_id);

        for (_param_name, param_rec_id) in probe_loc.instr_created_args.iter() {
            let param_rec = self.table.get_record_mut(&param_rec_id);
            if let Some(Record::Var { addr: Some(VarAddr::Local {addr}), .. }) = param_rec {
                instr_builder.instr_at(*index, walrus::ir::LocalGet {
                    local: addr.clone()
                });
                *index += 1;
            } else {
                error!("Could not inject alternate call to function, something went wrong...");
                exit(1);
            }
        }
    }

    /// Returns the InstrSeqId of the `then` block
    fn emit_alt_body(&mut self, probe: &mut Probe, probe_loc: &mut ProbeLoc) -> (InstrSeqId, usize, InstrSeqId, usize) {
        let mut is_success = true;

        let orig_instr = self.remove_orig_bytecode(probe, probe_loc);

        // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
        let func = self.app_wasm.funcs.get_mut(probe_loc.wasm_func_id).kind.unwrap_local_mut();
        let func_builder = func.builder_mut();
        let mut instr_builder = func_builder.instr_seq(probe_loc.instr_seq_id);

        // We've injected a predicate prior to this point, need to create if/else
        // block to conditionally execute the body.
        let mut then_seq_id = None;
        let mut then_idx = None;
        let mut else_seq_id = None;
        let mut else_idx = None;
        instr_builder.if_else_at(
            probe_loc.index,
            None,
            | then | {
                then_seq_id = Some(then.id());
                // create new `index` var to store current index into the of the `then` instr sequence
                let mut idx = 0 as usize;
                // 1. emit alt body
                is_success &= emit_body(&mut self.table, &mut self.app_wasm.data,
                                        probe.body.as_mut().unwrap(), then,
                                        &mut self.metadata.as_mut().unwrap(), &mut idx);
                then_idx = Some(idx);
                // Will not emit the original instruction since this is an alternate probe
            },
            |else_| {
                else_seq_id = Some(else_.id());
                else_idx = Some(0 as usize); // leave at 0 to allow injecting parameters before the original bytecode
                if let Some((instr, _instr_loc_id)) = orig_instr {
                    else_.instr(instr.clone());
                }
            },
        );

        (then_seq_id.unwrap(), then_idx.unwrap(), else_seq_id.unwrap(), else_idx.unwrap())
    }

    fn emit_alt_call(&mut self, probe_loc: &ProbeLoc, instr_seq_id: &InstrSeqId, index: &mut usize) -> bool {
        let mut is_success = true;
        // check if we should inject an alternate call!
        // At this point the body has been visited, so "new_target_fn_name" would be defined
        let rec_id = match self.table.lookup(&"new_target_fn_name".to_string()) {
            Some(rec_id) => Some(rec_id.clone()),
            None => None
        };

        if rec_id.is_none() {
            info!("`new_target_fn_name` not configured for this probe.");
        } else {
            let (name, func_call_id) = match rec_id {
                Some(r_id) => {
                    let rec = self.table.get_record_mut(&r_id);
                    if let Some(Record::Var { value: Some(Value::Str {val, ..}), .. }) = rec {
                        (val.clone(), self.app_wasm.funcs.by_name(val))
                    } else {
                        ("".to_string(), None)
                    }
                }
                None => {
                    ("".to_string(), None)
                },
            };

            if let Some(f_call_id) = func_call_id {
                // we need to inject an alternate call to the specified fn name!
                // replace the arguments
                self.emit_params(probe_loc, &instr_seq_id, index);

                // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
                let func = self.app_wasm.funcs.get_mut(probe_loc.wasm_func_id).kind.unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_seq = func_builder.instr_seq(*instr_seq_id);

                // inject call
                instr_seq.instr_at(*index, walrus::ir::Call {
                    func: f_call_id.clone()
                });
                *index += 1;

                is_success &= true;
            } else if name != "".to_string() {
                info!("Could not find function in app Wasm specified by `new_target_fn_name`: {}", name);
                exit(1);
            } else {
                error!("Could not inject alternate call to function, something went wrong...");
                exit(1);
            }
        }
        is_success
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

        // get memory id
        let memory_id = self.app_wasm.memories
            .iter()
            .next()
            .expect("only single memory is supported")
            .id();

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
                                    memory_id,
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
                                    memory_id,
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
    fn enter_named_scope(&mut self, scope_name: &String) {
        self.table.enter_named_scope(scope_name);
    }
    fn exit_scope(&mut self) {
        self.table.exit_scope();
    }
    fn reset_children(&mut self) {
        self.table.reset_children();
    }
    
    fn emit_whamm(&mut self, _whamm: &Whamm) -> bool {
        // nothing to do here
        true
    }
    fn emit_whammy(&mut self, _whammy: &Whammy) -> bool {
        // nothing to do here
        true
    }
    fn emit_provider(&mut self, _context: &str, _provider: &mut Provider) -> bool {
        // nothing to do here
        true
    }
    fn emit_package(&mut self, context: &str, package: &mut Package) -> bool {
        let regex = Regex::new(r"whamm:whammy([0-9]+):wasm:bytecode").unwrap();
        return if let Some(_caps) = regex.captures(context) {
            let res = self.setup_wasm_bytecode_package(context, package);
            res
        } else {
            error!("Provided package, but could not find a context to provide the definition, context: {}", context);
            false
        };
    }
    fn emit_event(&mut self, _context: &str, _event: &mut Event) -> bool {
        // nothing to do here
        true
    }

    fn emit_probe(&mut self, _context: &str, probe: &mut Probe) -> bool {
        let mut is_success = true;
        if let Some(probe_locs) = self.probe_locs.get_mut(self.metadata.as_ref().unwrap().curr_event.as_str()) {
            for probe_loc in probe_locs.iter_mut() {
                let mut is_success = true;
                // TODO -- load symbols for this instruction

                if probe_loc.instr_name == "call" {
                    // save off call params if instrumenting a fn call
                    self.save_params(probe_loc);
                }

                // determine if I should inject a predicate.
                if let Some(predicate) = &mut probe.predicate {
                    // Fold predicate via constant propagation
                    self.fold_expr(predicate);
                }
                if let Some(pred_as_bool) = ExprFolder::get_single_bool(&probe.predicate.as_ref().unwrap()) {
                    // predicate has been reduced to a boolean value, check true/false
                    // otherwise: predicate has not been reduced to a boolean value, will need to inject the folded predicate
                    if !pred_as_bool {
                        // predicate is FALSE, DON'T INJECT PROBE IN GENERAL, so just return from this visit!
                        info!("Predicate is false, no need to inject probe.");
                        return true;
                    }
                    // predicate is TRUE, unconditionally inject body stmts
                    probe.predicate = None;
                }

                if let Some(predicate) = &mut probe.predicate {
                    if probe.name == "alt" {
                        self.emit_predicate(predicate, probe_loc);

                        // an alternate probe will need to emit an if/else
                        // if pred { <alt_body>; Optional(<alt_call>;) } else { <original_instr> }
                        let (
                            if_then_block_id,
                            mut if_then_idx,
                            else_block_id,
                            mut else_idx
                        ) = self.emit_alt_body(probe, probe_loc);

                        // 2. possibly emit alt call (if configured to do so)
                        if probe_loc.instr_name == "call" {
                            is_success &= self.emit_alt_call(probe_loc, &if_then_block_id, &mut if_then_idx);

                            // This is a call instruction, emit original parameters for the original call in the `else` block
                            self.emit_params(probe_loc, &else_block_id, &mut else_idx);
                        }
                    } else {
                        // other probe types will just need to have an if block conditional on the predicate
                        // if pred { <probe_body>; }
                        self.emit_predicated_body(probe, predicate, probe_loc.wasm_func_id, &probe_loc.instr_seq_id, &mut probe_loc.index);
                    }
                } else {
                    // No predicate, just emit the un-predicated probe body
                    // <probe_body>;
                    is_success &= self.emit_body(probe.body.as_mut().unwrap(), probe_loc.wasm_func_id, &probe_loc.instr_seq_id, &mut probe_loc.index);

                    if probe_loc.instr_name == "call" && probe.name == "alt" {
                        self.remove_orig_bytecode(probe, probe_loc);

                        // 2. possibly emit alt call (if configured to do so)
                        self.emit_alt_call(probe_loc, &probe_loc.instr_seq_id, &mut probe_loc.index);
                    }
                }
            }
        }
        is_success
    }

    fn emit_params(&mut self) -> bool {
        todo!()
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

    fn emit_stmt(&mut self, _stmt: &Statement) -> bool {
        unimplemented!()
    }

    fn fold_expr(&mut self, expr: &mut Expr) -> bool {
        *expr = ExprFolder::fold_expr(expr, &self.table);
        true
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

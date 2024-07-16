pub mod rules;

use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::rules::{LocInfo, Provider, WhammProvider};
use crate::emitter::Emitter;
use crate::generator::types::ExprFolder;
use crate::parser::types::{BinOp, DataType, Expr, Fn, ProbeSpec, Statement, UnOp, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use log::{debug, info};
use walrus::ir::{BinaryOp, ExtendedLoad, Instr, InstrSeqId, LoadKind, MemArg};
use walrus::{
    ActiveData, ActiveDataLocation, DataKind, FunctionBuilder, FunctionId, FunctionKind, InitExpr,
    InstrSeqBuilder, LocalFunction, MemoryId, ModuleData, ValType,
};
// =================================================================================
// ================ WasmRewritingEmitter - HELPER FUNCTIONS ========================
// Necessary to extract common logic between Emitter and InstrumentationVisitor.
// Can't pass an Emitter instance to InstrumentationVisitor due to Rust not
// allowing nested references to a common mutable object. So I can't pass the
// Emitter to the InstrumentationVisitor since I must iterate over Emitter.app_wasm
// with a construction of InstrumentationVisitor inside that loop.
// =================================================================================
// =================================================================================

const UNEXPECTED_ERR_MSG: &str =
    "WasmRewritingEmitter: Looks like you've found a bug...please report this behavior!";

fn data_type_to_val_type(ty: &DataType) -> (ValType, InitExpr) {
    match ty {
        DataType::U32 => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        DataType::I32 => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        DataType::Boolean => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        DataType::Null => unimplemented!(),
        DataType::Str => unimplemented!(),
        DataType::Tuple { .. } => unimplemented!(),
        // the ID used to track this var in the lib
        DataType::Map { .. } => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        &DataType::AssumeGood => unimplemented!(),
    }
}

fn emit_set(
    table: &mut SymbolTable,
    var_id: &mut Expr,
    instr_builder: &mut InstrSeqBuilder,
    index: &mut usize,
) -> Result<bool, Box<WhammError>> {
    if let Expr::VarId { name, .. } = var_id {
        let var_rec_id = match table.lookup(name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                                                VarId '{name}' does not exist in this scope!"
                    )),
                    None,
                )));
            }
        };
        match table.get_record_mut(&var_rec_id) {
            Some(Record::Var { addr, loc, .. }) => {
                // this will be different based on if this is a global or local var
                match addr {
                    Some(VarAddr::Global { addr }) => {
                        instr_builder.instr_at(*index, walrus::ir::GlobalSet {
                            global: *addr
                        });
                        // update index to point to what follows our insertions
                        *index += 1;
                    }
                    Some(VarAddr::Local { addr }) => {
                        instr_builder.instr_at(*index, walrus::ir::LocalSet {
                            local: *addr
                        });
                        // update index to point to what follows our insertions
                        *index += 1;
                    },
                    None => {
                        return Err(Box::new(ErrorGen::get_type_check_error_from_loc(false,
                                                                                    format!("Variable assigned before declared: {}", name), loc)));
                    }
                }
                Ok(true)
            },
            Some(ty) => {
                Err(Box::new(ErrorGen::get_unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} \
                                                Incorrect variable record, expected Record::Var, found: {:?}", ty)), None)))
            },
            None => {
                Err(Box::new(ErrorGen::get_unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} \
                                                Variable symbol does not exist!")), None)))
            }
        }
    } else {
        Err(Box::new(ErrorGen::get_unexpected_error(
            true,
            Some(format!(
                "{UNEXPECTED_ERR_MSG} \
                                        Expected VarId."
            )),
            None,
        )))
    }
}

fn emit_expr(
    table: &mut SymbolTable,
    module_data: &mut ModuleData,
    expr: &mut Expr,
    instr_builder: &mut InstrSeqBuilder,
    metadata: &mut InsertionMetadata,
    index: &mut usize,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match expr {
        Expr::UnOp { op, expr, .. } => {
            is_success &= emit_expr(table, module_data, expr, instr_builder, metadata, index)?;
            is_success &= emit_unop(op, instr_builder, index);
        }
        Expr::BinOp { lhs, op, rhs, .. } => {
            is_success &= emit_expr(table, module_data, lhs, instr_builder, metadata, index)?;
            is_success &= emit_expr(table, module_data, rhs, instr_builder, metadata, index)?;
            is_success &= emit_binop(op, instr_builder, index);
        }
        Expr::Ternary {
            cond: _cond,
            conseq: _conseq,
            alt: _alt,
            ..
        } => {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                            Ternary expressions should be handled before this point!"
                )),
                None,
            )));
        }
        Expr::Call {
            fn_target, args, ..
        } => {
            let fn_name = match &**fn_target {
                Expr::VarId { name, .. } => name.clone(),
                _ => return Ok(false),
            };

            // emit the arguments
            if let Some(args) = args {
                for boxed_arg in args.iter_mut() {
                    let arg = &mut **boxed_arg; // unbox
                    is_success &=
                        emit_expr(table, module_data, arg, instr_builder, metadata, index)?;
                }
            }

            let fn_rec_id = table.lookup(&fn_name).copied();

            match fn_rec_id {
                Some(rec_id) => {
                    let fn_rec = table.get_record_mut(&rec_id);
                    match fn_rec {
                        Some(Record::Fn { addr, .. }) => {
                            if let Some(f_id) = addr {
                                instr_builder.instr_at(*index, walrus::ir::Call { func: *f_id });
                                // update index to point to what follows our insertions
                                *index += 1;
                            } else {
                                return Err(Box::new(ErrorGen::get_unexpected_error(
                                    true,
                                    Some(format!(
                                        "{UNEXPECTED_ERR_MSG} \
                                fn_target address not in symbol table, not emitted yet..."
                                    )),
                                    None,
                                )));
                            }
                        }
                        _ => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                            fn_target not defined in symbol table!"
                                )),
                                None,
                            )));
                        }
                    }
                }
                None => {
                    // Must be defined in the Wasm
                    unimplemented!()
                }
            }
        }
        Expr::VarId { name, .. } => {
            // TODO -- support string vars (unimplemented)
            let var_rec_id = match table.lookup(name) {
                Some(rec_id) => *rec_id,
                _ => {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                    VarId '{}' does not exist in this scope!",
                            name
                        )),
                        None,
                    )));
                }
            };
            return match table.get_record_mut(&var_rec_id) {
                Some(Record::Var { addr, .. }) => {
                    // this will be different based on if this is a global or local var
                    match addr {
                        Some(VarAddr::Global { addr }) => {
                            instr_builder.instr_at(*index, walrus::ir::GlobalGet { global: *addr });
                            // update index to point to what follows our insertions
                            *index += 1;
                        }
                        Some(VarAddr::Local { addr }) => {
                            instr_builder.instr_at(*index, walrus::ir::LocalGet { local: *addr });
                            // update index to point to what follows our insertions
                            *index += 1;
                        }
                        None => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                            Variable does not exist in scope: {}",
                                    name
                                )),
                                None,
                            )));
                        }
                    }
                    Ok(true)
                }
                Some(ty) => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                    Incorrect variable record, expected Record::Var, found: {:?}",
                        ty
                    )),
                    None,
                ))),
                None => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                    Variable symbol does not exist!"
                    )),
                    None,
                ))),
            };
        }
        Expr::Primitive { val, .. } => {
            is_success &= emit_value(table, module_data, val, instr_builder, metadata, index)?;
        }
    }
    Ok(is_success)
}

fn emit_binop(op: &BinOp, instr_builder: &mut InstrSeqBuilder, index: &mut usize) -> bool {
    match op {
        BinOp::And => {
            // we only support i32's at the moment
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32And,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::Or => {
            // we only support i32's at the moment
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32Or,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::EQ => {
            // we only support i32's at the moment
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32Eq,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::NE => {
            // we only support i32's at the moment
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32Ne,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::GE => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32GeS,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::GT => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32GtS,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::LE => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32LeS,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::LT => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32LtS,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::Add => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32Add,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::Subtract => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32Sub,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::Multiply => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32Mul,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::Divide => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32DivS,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
        BinOp::Modulo => {
            // we only support i32's at the moment (assumes signed)
            instr_builder.instr_at(
                *index,
                walrus::ir::Binop {
                    op: BinaryOp::I32RemS,
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
    }
}

fn emit_unop(op: &UnOp, instr_builder: &mut InstrSeqBuilder, index: &mut usize) -> bool {
    match op {
        UnOp::Not => {
            instr_builder.instr_at(
                *index,
                walrus::ir::Unop {
                    op: walrus::ir::UnaryOp::I32Eqz, // return 1 if 0, return 0 otherwise
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            true
        }
    }
}

fn emit_value(
    table: &mut SymbolTable,
    module_data: &mut ModuleData,
    val: &mut Value,
    instr_builder: &mut InstrSeqBuilder,
    metadata: &mut InsertionMetadata,
    index: &mut usize,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match val {
        Value::Integer { val, .. } => {
            instr_builder.instr_at(
                *index,
                walrus::ir::Const {
                    value: walrus::ir::Value::I32(*val),
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            is_success &= true;
        }
        Value::Str { val, addr, ty: _ty } => {
            let data_id = module_data.add(
                DataKind::Active(ActiveData {
                    memory: metadata.mem_id,
                    location: ActiveDataLocation::Absolute(metadata.curr_mem_offset),
                }),
                Vec::from(val.as_bytes()),
            );

            // save the memory addresses/lens, so they can be used as appropriate
            *addr = Some((data_id, metadata.curr_mem_offset, val.len()));

            // emit Wasm instructions for the memory address and string length
            instr_builder.instr_at(
                *index,
                walrus::ir::Const {
                    value: walrus::ir::Value::I32(metadata.curr_mem_offset as i32),
                },
            );
            // update index to point to what follows our insertions
            *index += 1;
            instr_builder.instr_at(
                *index,
                walrus::ir::Const {
                    value: walrus::ir::Value::I32(val.len() as i32),
                },
            );
            // update index to point to what follows our insertions
            *index += 1;

            // update curr_mem_offset to account for new data
            metadata.curr_mem_offset += val.len() as u32;
            is_success &= true;
        }
        Value::Tuple { vals, .. } => {
            for val in vals.iter_mut() {
                is_success &= emit_expr(table, module_data, val, instr_builder, metadata, index)?;
            }
        }
        Value::Boolean { val, .. } => {
            // "In a boolean context, such as a br_if condition, any non-zero value is interpreted as true
            // and 0 is interpreted as false."
            // https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#booleans
            if *val {
                // insert true (non-zero)
                instr_builder.instr_at(
                    *index,
                    walrus::ir::Const {
                        value: walrus::ir::Value::I32(1),
                    },
                );
            } else {
                // insert false (zero)
                instr_builder.instr_at(
                    *index,
                    walrus::ir::Const {
                        value: walrus::ir::Value::I32(0),
                    },
                );
            }
            // update index to point to what follows our insertions
            *index += 1;
            is_success &= true;
        }
    }
    Ok(is_success)
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
    curr_loc: usize,
}
impl InstrIter {
    /// Build out a list of all local functions and their blocks/instruction indexes
    /// to visit while doing instrumentation.
    fn new() -> Self {
        Self {
            instr_locs: vec![],
            curr_loc: 0,
        }
    }
    fn init(&mut self, app_wasm: &walrus::Module) {
        // Figure out which functions to visit
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
                self.init_instr_locs(
                    app_wasm,
                    local_func,
                    &func_id,
                    func.name.clone(),
                    local_func.entry_block(),
                );
            }
        }
        debug!("Finished creating list of instructions to visit");
    }
    fn init_instr_locs(
        &mut self,
        _app_wasm: &walrus::Module,
        func: &LocalFunction,
        func_id: &FunctionId,
        func_name: Option<String>,
        instr_seq_id: InstrSeqId,
    ) {
        func.block(instr_seq_id)
            .iter()
            .enumerate()
            .for_each(|(index, (instr, _))| {
                let instr_as_str = &format!("{:?}", instr);
                let instr_name = instr_as_str.split('(').next().unwrap().to_lowercase();

                // as a hack, just save ALL INSTRS, to be visited later to possibly
                //     instrument them

                // add current instr
                self.instr_locs.push(ProbeLoc {
                    // wasm_func_name: func_name.clone(),
                    wasm_func_id: *func_id,
                    instr_seq_id,
                    index,
                    instr_name: instr_name.clone(),
                    instr: instr.clone(),
                    instr_created_args: vec![],
                    instr_alt_call: None,
                });

                // visit nested blocks
                match instr {
                    Instr::Block(block) => {
                        self.init_instr_locs(
                            _app_wasm,
                            func,
                            func_id,
                            func_name.clone(),
                            block.seq,
                        );
                    }
                    Instr::Loop(_loop) => {
                        self.init_instr_locs(
                            _app_wasm,
                            func,
                            func_id,
                            func_name.clone(),
                            _loop.seq,
                        );
                    }
                    Instr::IfElse(if_else, ..) => {
                        println!("IfElse: {:#?}", if_else);
                        self.init_instr_locs(
                            _app_wasm,
                            func,
                            func_id,
                            func_name.clone(),
                            if_else.consequent,
                        );
                        self.init_instr_locs(
                            _app_wasm,
                            func,
                            func_id,
                            func_name.clone(),
                            if_else.alternative,
                        );
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
    instr_created_args: Vec<(String, usize)>,
    instr_alt_call: Option<FunctionId>,
}
struct EmittingInstrTracker {
    // To keep track of the location of the original instruction while we're instrumenting!
    orig_instr_idx: usize,

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
    else_idx: Option<usize>,
}

pub struct WasmRewritingEmitter {
    pub app_wasm: walrus::Module,
    pub table: SymbolTable,

    // whamm! AST traversal bookkeeping
    metadata: InsertionMetadata,
    instr_iter: InstrIter,
    emitting_instr: Option<EmittingInstrTracker>,

    fn_providing_contexts: Vec<String>,
}
impl WasmRewritingEmitter {
    pub fn new(app_wasm: walrus::Module, table: SymbolTable) -> Self {
        let mem_id = app_wasm
            .memories
            .iter()
            .next()
            .expect("only single memory is supported")
            .id();

        Self {
            app_wasm,
            table,
            metadata: InsertionMetadata {
                mem_id,
                curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
            },
            instr_iter: InstrIter::new(),
            emitting_instr: None,
            fn_providing_contexts: vec!["whamm".to_string()],
        }
    }

    fn override_var_val(&mut self, rec_id: &usize, val: Option<Value>) {
        let mut rec = self.table.get_record_mut(rec_id);
        if let Some(Record::Var { value, .. }) = &mut rec {
            *value = val;
        }
    }

    fn emit_provided_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
        if context == "whamm" && f.name.name == "strcmp" {
            self.emit_whamm_strcmp_fn(f)
        } else {
            Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
            Provided function, but could not find a context to provide the definition, context: {}",
                    context
                )),
                None,
            )))
        }
    }

    fn emit_whamm_strcmp_fn(&mut self, f: &Fn) -> Result<bool, Box<WhammError>> {
        let strcmp_params = vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32];
        let strcmp_result = vec![ValType::I32];

        let mut strcmp =
            FunctionBuilder::new(&mut self.app_wasm.types, &strcmp_params, &strcmp_result);

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

                neq_block
                    .block(None, |eq_block| {
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

        let strcmp_id = strcmp.finish(
            vec![str0_offset, str0_size, str1_offset, str1_size],
            &mut self.app_wasm.funcs,
        );
        let rec_id = match self.table.lookup(&f.name.name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                `strcmp` fn symbol does not exist in this scope!"
                    )),
                    None,
                )));
            }
        };

        return if let Some(rec) = self.table.get_record_mut(&rec_id) {
            if let Record::Fn { addr, .. } = rec {
                *addr = Some(strcmp_id);
                Ok(true)
            } else {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Incorrect global variable record, expected Record::Var, found: {:?}",
                        rec
                    )),
                    None,
                )));
            }
        } else {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
            Global variable symbol does not exist!"
                )),
                None,
            )));
        };
    }

    fn emit_decl_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        match stmt {
            Statement::Decl { ty, var_id, .. } => {
                // look up in symbol table
                let mut addr = if let Expr::VarId { name, .. } = var_id {
                    let var_rec_id = match self.table.lookup(name) {
                        Some(rec_id) => *rec_id,
                        None => {
                            // TODO -- add variables from body into symbol table
                            //         (at this point, the verifier should have run to catch variable initialization without declaration)
                            self.table.put(
                                name.clone(),
                                Record::Var {
                                    ty: ty.clone(),
                                    name: name.clone(),
                                    value: None,
                                    is_comp_provided: false,
                                    addr: None,
                                    loc: None,
                                },
                            )
                        }
                    };
                    match self.table.get_record_mut(&var_rec_id) {
                        Some(Record::Var { addr, .. }) => addr,
                        Some(ty) => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                            Incorrect variable record, expected Record::Var, found: {:?}",
                                    ty
                                )),
                                None,
                            )));
                        }
                        None => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                            Variable symbol does not exist!"
                                )),
                                None,
                            )));
                        }
                    }
                } else {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                    Expected VarId."
                        )),
                        None,
                    )));
                };

                match &mut addr {
                    Some(VarAddr::Global { addr: _addr }) => {
                        // The global should already exist, do any initial setup here!
                        match ty {
                            DataType::Map {
                                key_ty: _key_ty,
                                val_ty: _val_ty,
                            } => {
                                // initialize map global variable
                                // also update value at GID (probably need to set ID of map there)
                                unimplemented!()
                            }
                            _ => Ok(true),
                        }
                    }
                    Some(VarAddr::Local { .. }) | None => {
                        // If the local already exists, it would be because the probe has been
                        // emitted at another opcode location. Simply overwrite the previously saved
                        // address.
                        let (walrus_ty, ..) = data_type_to_val_type(ty);
                        let id = self.app_wasm.locals.add(walrus_ty);
                        *addr = Some(VarAddr::Local { addr: id });
                        Ok(true)
                    }
                }
            }
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                false,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                    Wrong statement type, should be `assign`"
                )),
                None,
            ))),
        }
    }

    fn emit_assign_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        return match stmt {
            Statement::Assign { var_id, expr, .. } => {
                let mut folded_expr = ExprFolder::fold_expr(expr, &self.table);

                // Save off primitives to symbol table
                // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!
                if let (Expr::VarId { name, .. }, Expr::Primitive { val, .. }) =
                    (&var_id, &folded_expr)
                {
                    let var_rec_id = match self.table.lookup(name) {
                        Some(rec_id) => *rec_id,
                        _ => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                    Attempting to emit an assign, but VarId '{name}' does not exist in this scope!"
                                )),
                                None,
                            )));
                        }
                    };
                    match self.table.get_record_mut(&var_rec_id) {
                        Some(Record::Var {
                            value,
                            is_comp_provided,
                            ..
                        }) => {
                            *value = Some(val.clone());

                            if *is_comp_provided {
                                return Ok(true);
                            }
                        }
                        Some(ty) => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                    Incorrect variable record, expected Record::Var, found: {:?}",
                                    ty
                                )),
                                None,
                            )));
                        }
                        None => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                    Variable symbol does not exist!"
                                )),
                                None,
                            )));
                        }
                    }
                }

                match self.emit_expr(&mut folded_expr) {
                    Err(e) => Err(e),
                    Ok(_) => {
                        if let Some(curr_loc) = self.instr_iter.curr_mut() {
                            if let Some(tracker) = &mut self.emitting_instr {
                                let func = self
                                    .app_wasm
                                    .funcs
                                    .get_mut(curr_loc.wasm_func_id)
                                    .kind
                                    .unwrap_local_mut();
                                let func_builder = func.builder_mut();
                                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                                // Emit the instruction that sets the variable's value to the emitted expression
                                emit_set(
                                    &mut self.table,
                                    var_id,
                                    &mut instr_builder,
                                    &mut tracker.curr_idx,
                                )
                            } else {
                                return Err(Box::new(ErrorGen::get_unexpected_error(
                                    true,
                                    Some(format!(
                                        "{UNEXPECTED_ERR_MSG} \
                                            Something went wrong while emitting an instruction."
                                    )),
                                    None,
                                )));
                            }
                        } else {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                        Something went wrong while emitting an instruction."
                                )),
                                None,
                            )));
                        }
                    }
                }
            }
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    false,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                    Wrong statement type, should be `assign`"
                    )),
                    None,
                )));
            }
        };
    }
}

impl Emitter for WasmRewritingEmitter {
    fn enter_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.enter_scope()
    }
    fn enter_scope_via_spec(&mut self, script_id: &str, probe_spec: &ProbeSpec) -> bool {
        self.table.enter_scope_via_spec(script_id, probe_spec)
    }
    fn exit_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.exit_scope()
    }
    fn reset_children(&mut self) {
        self.table.reset_children();
    }

    fn init_instr_iter(&mut self) -> Result<(), Box<WhammError>> {
        self.instr_iter.init(&self.app_wasm);
        Ok(())
    }

    /// bool -> whether there is a next instruction to process
    fn has_next_instr(&self) -> bool {
        self.instr_iter.has_next()
    }

    fn init_first_instr(&mut self) -> bool {
        if let Some(first) = self.instr_iter.curr() {
            self.emitting_instr = Some(EmittingInstrTracker {
                orig_instr_idx: first.index,
                curr_seq_id: first.instr_seq_id,
                curr_idx: first.index,
                main_seq_id: first.instr_seq_id,
                main_idx: first.index,
                outer_seq_id: None,
                outer_idx: None,
                then_seq_id: None,
                then_idx: None,
                else_seq_id: None,
                else_idx: None,
            });
            return true;
        }
        false
    }

    /// bool -> whether it found a next instruction
    fn next_instr(&mut self) -> bool {
        if self.instr_iter.has_next() {
            if let Some(next) = self.instr_iter.next() {
                self.emitting_instr = Some(EmittingInstrTracker {
                    orig_instr_idx: next.index,
                    curr_seq_id: next.instr_seq_id,
                    curr_idx: next.index,
                    main_seq_id: next.instr_seq_id,
                    main_idx: next.index,
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

    fn curr_instr(&self) -> &Instr {
        let curr_instr = self.instr_iter.curr().unwrap();
        &curr_instr.instr
    }

    fn curr_instr_name(&self) -> &str {
        let curr_instr = self.instr_iter.curr().unwrap();
        curr_instr.instr_name.as_str()
    }

    fn incr_loc_pointer(&mut self) {
        if let Some(tracker) = &mut self.emitting_instr {
            tracker.curr_idx += 1;
            tracker.main_idx += 1;
        }
    }

    fn get_loc_info<'a>(&self, rule: &'a WhammProvider) -> Option<LocInfo<'a>> {
        let curr_instr = self.curr_instr();
        rule.get_loc_info(&self.app_wasm, curr_instr)
    }

    fn save_args(&mut self, args: &[ValType]) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                let func = self
                    .app_wasm
                    .funcs
                    .get_mut(curr_loc.wasm_func_id)
                    .kind
                    .unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                // No opcodes should have been emitted in the module yet!
                // So, we can just save off the first * items in the stack as the args
                // to the call.
                let mut arg_recs = vec![]; // vec to retain order!
                args.iter().enumerate().for_each(|(num, param_ty)| {
                    // create local for the param in the module
                    let arg_local_id = self.app_wasm.locals.add(*param_ty);

                    // emit a opcode in the event to assign the ToS to this new local
                    instr_builder.instr_at(
                        tracker.curr_idx,
                        walrus::ir::LocalSet {
                            local: arg_local_id,
                        },
                    );

                    // update index of tracker to point to what follows our insertions
                    tracker.curr_idx += 1;

                    // also update index to point to new location of instrumented instruction!
                    // (saved args go before the original instruction)
                    tracker.orig_instr_idx += 1;

                    // place in symbol table with var addr for future reference
                    let arg_name = format!("arg{}", num);
                    let id = self.table.put(
                        arg_name.clone(),
                        Record::Var {
                            ty: DataType::I32, // we only support integers right now.
                            name: arg_name.clone(),
                            value: None,
                            is_comp_provided: false,
                            addr: Some(VarAddr::Local { addr: arg_local_id }),
                            loc: None,
                        },
                    );
                    arg_recs.push((arg_name, id));
                });
                curr_loc.instr_created_args = arg_recs;
                return true;
            }
        }
        false
    }

    fn emit_args(&mut self) -> Result<bool, Box<WhammError>> {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                let func = self
                    .app_wasm
                    .funcs
                    .get_mut(curr_loc.wasm_func_id)
                    .kind
                    .unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                for (_param_name, param_rec_id) in curr_loc.instr_created_args.iter() {
                    let param_rec = self.table.get_record_mut(param_rec_id);
                    if let Some(Record::Var {
                        addr: Some(VarAddr::Local { addr }),
                        ..
                    }) = param_rec
                    {
                        // Inject at tracker.orig_instr_idx to make sure that this actually emits the args
                        // for the instrumented instruction right before that instruction is called!
                        instr_builder.instr_at(
                            tracker.orig_instr_idx,
                            walrus::ir::LocalGet { local: *addr },
                        );

                        // update index to point to new location of instrumented instruction!
                        // (re-emitted args go before the original instruction)
                        tracker.orig_instr_idx += 1;
                    } else {
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!(
                                "{UNEXPECTED_ERR_MSG} \
                        Could not emit parameters, something went wrong..."
                            )),
                            None,
                        )));
                    }
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn define(&mut self, var_name: &str, var_val: &Option<Value>) -> Result<bool, Box<WhammError>> {
        let rec_id = match self.table.lookup(var_name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                        `{var_name}` symbol does not exist in this scope!"
                    )),
                    None,
                )));
            }
        };
        self.override_var_val(&rec_id, var_val.clone());

        Ok(true)
    }

    fn reset_table_data(&mut self, loc_info: &LocInfo) {
        // reset static_data
        loc_info.static_data.iter().for_each(|(symbol_name, ..)| {
            self.table.remove_record(symbol_name);
        });

        // reset dynamic_data
        for i in 0..loc_info.args.len() {
            let arg_name = format!("arg{}", i);
            self.table.remove_record(&arg_name);
        }
    }

    fn fold_expr(&mut self, expr: &mut Expr) -> bool {
        *expr = ExprFolder::fold_expr(expr, &self.table);
        true
    }
    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        match expr {
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                is_success &= self.emit_if_else();
                is_success &= self.emit_condition();
                is_success &= self.emit_expr(cond)?;
                is_success &= self.emit_consequent();
                is_success &= self.emit_expr(conseq)?;
                is_success &= self.emit_alternate();
                is_success &= self.emit_expr(alt)?;
                is_success &= self.finish_branch();
            }
            Expr::VarId { .. }
            | Expr::UnOp { .. }
            | Expr::BinOp { .. }
            | Expr::Primitive { .. }
            | Expr::Call { .. } => {
                // Anything else can be emitted as normal
                if let Some(curr_loc) = self.instr_iter.curr_mut() {
                    if let Some(tracker) = &mut self.emitting_instr {
                        let func = self
                            .app_wasm
                            .funcs
                            .get_mut(curr_loc.wasm_func_id)
                            .kind
                            .unwrap_local_mut();
                        let func_builder = func.builder_mut();
                        let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                        is_success &= emit_expr(
                            &mut self.table,
                            &mut self.app_wasm.data,
                            expr,
                            &mut instr_builder,
                            &mut self.metadata,
                            &mut tracker.curr_idx,
                        )?;
                    } else {
                        // errors here when instrumenting 3 calls
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!(
                                "{UNEXPECTED_ERR_MSG} \
                            Something went wrong while emitting an instruction."
                            )),
                            None,
                        )));
                    }
                } else {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                        Something went wrong while emitting an instruction."
                        )),
                        None,
                    )));
                }
            }
        }
        Ok(is_success)
    }
    fn emit_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
        // figure out if this is a provided fn.
        if f.is_comp_provided {
            return if self.fn_providing_contexts.contains(&context.to_string()) {
                self.emit_provided_fn(context, f)
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Provided fn, but could not find a context to provide the definition, context: {}",
                        context
                    )),
                    None,
                )))
            };
        }

        // emit non-provided fn
        // only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    fn emit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        // only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        _val: &Option<Value>,
    ) -> Result<bool, Box<WhammError>> {
        let rec_id = match self.table.lookup(&name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Global variable symbol does not exist in this scope!"
                    )),
                    None,
                )));
            } // Ignore, continue to emit
        };

        let rec = self.table.get_record_mut(&rec_id);
        match rec {
            Some(Record::Var { ref mut addr, .. }) => {
                // emit global variable and set addr in symbol table
                // this is used for user-defined global vars in the script...
                let (walrus_ty, init_expr) = data_type_to_val_type(&ty);
                let id = self.app_wasm.globals.add_local(walrus_ty, true, init_expr);
                *addr = Some(VarAddr::Global { addr: id });

                Ok(true)
            }
            Some(&mut ref ty) => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                Incorrect global variable record, expected Record::Var, found: {:?}",
                    ty
                )),
                None,
            ))),
            None => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                Global variable symbol does not exist!"
                )),
                None,
            ))),
        }
    }

    fn remove_orig(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &self.emitting_instr {
                let func = self
                    .app_wasm
                    .funcs
                    .get_mut(curr_loc.wasm_func_id)
                    .kind
                    .unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                instr_builder.instrs_mut().remove(tracker.curr_idx);
                return true;
            }
        }
        false
    }

    fn emit_orig(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                let func = self
                    .app_wasm
                    .funcs
                    .get_mut(curr_loc.wasm_func_id)
                    .kind
                    .unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                // reset where the "orig instruction" is located in the opcode
                tracker.orig_instr_idx = tracker.curr_idx;
                instr_builder.instr_at(tracker.curr_idx, curr_loc.instr.clone());
                return true;
            }
        }
        false
    }

    fn emit_if(&mut self) -> bool {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                // This MUST be `self.app_wasm` so we're mutating what will be the instrumented application.
                let func = self
                    .app_wasm
                    .funcs
                    .get_mut(curr_loc.wasm_func_id)
                    .kind
                    .unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                let mut outer_seq_id = None;
                let mut outer_idx = None;
                let mut then_seq_id = None;
                let mut then_idx = None;

                instr_builder.block_at(tracker.curr_idx, None, |outer_block| {
                    let outer_id = outer_block.id();
                    outer_seq_id = Some(outer_id);
                    outer_idx = Some(0usize);

                    // CONDITION SHOULD BE EMITTED HERE

                    // If the block evaluates to true (any nonzero value), execute the body.
                    // If result of predicate equals 0, break out of the probe block
                    // to continue with the application code.
                    outer_block
                        .i32_const(0)
                        .binop(BinaryOp::I32Eq)
                        .br_if(outer_id);

                    outer_block.block(None, |then| {
                        then_seq_id = Some(then.id());
                        then_idx = Some(0usize);

                        // CONSEQUENT SHOULD BE EMITTED HERE
                    });
                });

                // Save the block information for future reference
                // leave outer_block_idx as 0 to enable injection of condition!
                tracker.outer_seq_id = outer_seq_id;
                tracker.outer_idx = outer_idx;
                tracker.then_seq_id = then_seq_id;
                tracker.then_idx = then_idx;
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
                let func = self
                    .app_wasm
                    .funcs
                    .get_mut(curr_loc.wasm_func_id)
                    .kind
                    .unwrap_local_mut();
                let func_builder = func.builder_mut();
                let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                let mut outer_seq_id = None;
                let mut outer_idx = None;
                let mut then_seq_id = None;
                let mut then_idx = None;
                let mut else_seq_id = None;
                let mut else_idx = None;

                instr_builder.block_at(tracker.curr_idx, None, |outer_block| {
                    outer_seq_id = Some(outer_block.id());
                    outer_idx = Some(0usize);
                    outer_block.if_else(
                        None,
                        |then| {
                            then_seq_id = Some(then.id());
                            then_idx = Some(0usize);
                        },
                        |else_| {
                            else_seq_id = Some(else_.id());
                            else_idx = Some(0usize);
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
                    tracker.curr_seq_id = *outer_seq_id;
                    tracker.curr_idx = *outer_idx;
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
                    tracker.curr_seq_id = *then_seq_id;
                    tracker.curr_idx = *then_idx;
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
                    tracker.curr_seq_id = *else_seq_id;
                    tracker.curr_idx = *else_idx;
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
    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        // NOTE: This should be done in the Module entrypoint
        //       https://docs.rs/walrus/latest/walrus/struct.Module.html

        if let Some(start_fid) = self.app_wasm.start {
            if let FunctionKind::Local(local_func) = &self.app_wasm.funcs.get(start_fid).kind {
                self.emitting_instr = Some(EmittingInstrTracker {
                    orig_instr_idx: 0usize,
                    curr_seq_id: local_func.entry_block(),
                    curr_idx: 0usize,
                    main_seq_id: local_func.entry_block(),
                    main_idx: 0usize,
                    outer_seq_id: None,
                    outer_idx: None,
                    then_seq_id: None,
                    then_idx: None,
                    else_seq_id: None,
                    else_idx: None,
                })
            }
        } else {
            for stmt in stmts.iter_mut() {
                match stmt {
                    Statement::Decl { .. } => {
                        // This is fine
                    }
                    _ => {
                        // This is NOT fine...error!
                        // Cannot emit this at the moment since there's no entrypoint for our module to emit initialization instructions into
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(
                                "This module has no configured entrypoint, \
                    unable to emit a `script` with global state"
                                    .to_string(),
                            ),
                            None,
                        )));
                    }
                }
            }
            return Ok(true);
        }

        for stmt in stmts.iter_mut() {
            // iterate over statements and emit them
            self.emit_stmt(stmt)?;
        }
        Ok(true)
    }

    fn emit_body(&mut self, body: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        for stmt in body.iter_mut() {
            self.emit_stmt(stmt)?;
        }
        Ok(true)
    }

    fn has_alt_call(&mut self) -> bool {
        // check if we should inject an alternate call!
        // At this point the body has been visited, so "new_target_fn_name" would be defined
        let rec_id = self.table.lookup("new_target_fn_name").copied();

        if rec_id.is_none() {
            info!("`new_target_fn_name` not configured for this probe.");
            return false;
        } else {
            let (name, func_call_id) = match rec_id {
                Some(r_id) => {
                    let rec = self.table.get_record_mut(&r_id);
                    if let Some(Record::Var {
                        value: Some(Value::Str { val, .. }),
                        ..
                    }) = rec
                    {
                        (val.clone(), self.app_wasm.funcs.by_name(val))
                    } else {
                        ("".to_string(), None)
                    }
                }
                None => ("".to_string(), None),
            };
            if func_call_id.is_none() {
                info!(
                    "Could not find function in app Wasm specified by `new_target_fn_name`: {}",
                    name
                );
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

    fn emit_alt_call(&mut self) -> Result<bool, Box<WhammError>> {
        if let Some(curr_loc) = self.instr_iter.curr_mut() {
            if let Some(tracker) = &mut self.emitting_instr {
                if let Some(alt_fn_id) = curr_loc.instr_alt_call {
                    // we need to inject an alternate call to the specified fn name!
                    let func = self
                        .app_wasm
                        .funcs
                        .get_mut(curr_loc.wasm_func_id)
                        .kind
                        .unwrap_local_mut();
                    let func_builder = func.builder_mut();
                    let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                    // Hack to have emit_args target this new call site!
                    tracker.orig_instr_idx = tracker.curr_idx;

                    // inject call
                    instr_builder.instr_at(tracker.curr_idx, walrus::ir::Call { func: alt_fn_id });
                    tracker.curr_idx += 1;
                } else {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                    Could not inject alternate call to function, something went wrong..."
                        )),
                        None,
                    )));
                }
            }
        }
        Ok(true)
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        match stmt {
            Statement::Decl { .. } => self.emit_decl_stmt(stmt),
            Statement::Assign { .. } => self.emit_assign_stmt(stmt),
            Statement::Expr { expr, .. } => self.emit_expr(expr),
            Statement::Return { .. } => unimplemented!(),
            Statement::If { .. } => unimplemented!(),
        }
    }

    fn dump_to_file(&mut self, output_wasm_path: String) -> Result<bool, Box<WhammError>> {
        match self.app_wasm.emit_wasm_file(&output_wasm_path) {
            Ok(..) => Ok(true),
            Err(err) => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                Failed to dump instrumented wasm to {} from error: {}",
                    &output_wasm_path, err
                )),
                None,
            ))),
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

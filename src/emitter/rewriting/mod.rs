pub mod rules;

use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::rules::LocInfo;
use crate::generator::types::ExprFolder;
use crate::parser::types::{BinOp, DataType, Expr, Fn, Statement, UnOp, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use log::info;

use orca::ir::types::{Global, InitExpr, Value as OrcaValue, DataType as OrcaType, DataSegment, DataSegmentKind};
use wasmparser::{ValType, BlockType, ConstExpr, BinaryReader, WasmFeatures};

use orca::ir::module::Module;
use orca::ir::component::Component;
use orca::ir::function::FunctionBuilder;
use orca::opcode::Opcode;

fn module_to_component(module: Module) -> Component {
    let mut component = Component::new();
    component.add_module(module);
    component
}

// transform a whamm type to default wasm type, used for creating new global
// TODO: Might be more generic to also include Local
// TODO: Do we really want to depend on wasmpaser::ValType, or create a wrapper?
fn whamm_type_to_wasm(ty: &DataType) -> Global {
    match ty {
        DataType::I32 | DataType::U32 | DataType::Boolean => Global {
            ty: wasmparser::GlobalType {
                content_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            init_expr: InitExpr::Value(OrcaValue::I32(0)),
        },
        // the ID used to track this var in the lib
        DataType::Map { .. } => Global {
            ty: wasmparser::GlobalType {
                content_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            init_expr: InitExpr::Value(OrcaValue::I32(0)),
        },
        DataType::Null => unimplemented!(),
        DataType::Str => unimplemented!(),
        DataType::Tuple { .. } => unimplemented!(),
        DataType::AssumeGood => unimplemented!(),
    }
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

const UNEXPECTED_ERR_MSG: &str =
    "WasmRewritingEmitter: Looks like you've found a bug...please report this behavior!";

fn emit_set(
    table: &mut SymbolTable,
    var_id: &mut Expr,
    func_builder: &mut FunctionBuilder
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
                    Some(VarAddr::Global { addr: _addr }) => {
                        // todo
                        // func_builder.global_set(*addr);
                    }
                    Some(VarAddr::Local { addr }) => {
                        func_builder.local_set(*addr);
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
                "{UNEXPECTED_ERR_MSG} Expected VarId."
            )),
            None,
        )))
    }
}

fn emit_expr(
    table: &mut SymbolTable,
    module_data: &mut Vec<DataSegment>,
    expr: &mut Expr,
    func_builder: &mut FunctionBuilder,
    metadata: &mut InsertionMetadata
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match expr {
        Expr::UnOp { op, expr, .. } => {
            is_success &= emit_expr(table, module_data, expr, func_builder, metadata)?;
            is_success &= emit_unop(op, func_builder);
        }
        Expr::BinOp { lhs, op, rhs, .. } => {
            is_success &= emit_expr(table, module_data, lhs, func_builder, metadata)?;
            is_success &= emit_expr(table, module_data, rhs, func_builder, metadata)?;
            is_success &= emit_binop(op, func_builder);
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
                        emit_expr(table, module_data, arg, func_builder, metadata)?;
                }
            }

            let fn_rec_id = table.lookup(&fn_name).copied();

            match fn_rec_id {
                Some(rec_id) => {
                    let fn_rec = table.get_record_mut(&rec_id);
                    match fn_rec {
                        Some(Record::Fn { addr, .. }) => {
                            if let Some(f_id) = addr {
                                func_builder.call(*f_id);
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
                        Some(VarAddr::Global { addr: _addr }) => {
                            // todo
                            // func_builder.global_get(*addr);
                        }
                        Some(VarAddr::Local { addr }) => {
                            func_builder.local_get(*addr);
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
            is_success &= emit_value(table, module_data, val, func_builder, metadata)?;
        }
    }
    Ok(is_success)
}

fn emit_binop(op: &BinOp, func_builder: &mut FunctionBuilder) -> bool {
    match op {
        BinOp::And => {
            // we only support i32's at the moment
            func_builder.i32_and();
        }
        BinOp::Or => {
            // we only support i32's at the moment
            func_builder.i32_or();
        }
        BinOp::EQ => {
            // we only support i32's at the moment
            func_builder.i32_eq();
        }
        BinOp::NE => {
            // we only support i32's at the moment
            func_builder.i32_ne();
        }
        BinOp::GE => {
            // we only support i32's at the moment (assumes signed)
            func_builder.i32_gte_signed();
        }
        BinOp::GT => {
            // we only support i32's at the moment (assumes signed)
            func_builder.i32_gt_signed();
        }
        BinOp::LE => {
            // we only support i32's at the moment (assumes signed)
            func_builder.i32_lte_signed();
        }
        BinOp::LT => {
            // we only support i32's at the moment (assumes signed)
            func_builder.i32_lt_signed();
        }
        BinOp::Add => {
            // we only support i32's at the moment
            func_builder.i32_add();
        }
        BinOp::Subtract => {
            // we only support i32's at the moment
            func_builder.i32_sub();
        }
        BinOp::Multiply => {
            // we only support i32's at the moment (assumes signed)
            func_builder.i32_mul();
        }
        BinOp::Divide => {
            // we only support i32's at the moment (assumes signed)
            func_builder.i32_div_signed();
        }
        BinOp::Modulo => {
            // we only support i32's at the moment (assumes signed)
            func_builder.i32_rem_signed();
        }
    }
    true
}

fn emit_unop(op: &UnOp, func_builder: &mut FunctionBuilder) -> bool {
    match op {
        UnOp::Not => {
            // return 1 if 0, return 0 otherwise
            func_builder.i32_eqz();
        }
    }
    true
}

fn emit_value(
    table: &mut SymbolTable,
    module_data: &mut Vec<DataSegment>,
    val: &mut Value,
    func_builder: &mut FunctionBuilder,
    metadata: &mut InsertionMetadata
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match val {
        Value::Integer { val, .. } => {
            func_builder.i32(*val);
            is_success &= true;
        }
        Value::Str { val, addr, ty: _ty } => {
            // TODO -- assuming that the data ID is the index of the object in the Vec
            // TODO -- need an API that allows the addition of data segments.
            //     there is currently an ownership issue since I can't insert
            //     an owned byte array with same lifetime as the Module data segments.
            //     For more info, uncomment the below and read error.
            // let data_id = module_data.len();
            // let val_bytes = val.as_bytes().to_owned();
            // let data_segment = DataSegment {
            //     data: val_bytes.as_slice(),
            //     kind: DataSegmentKind::Active {
            //         memory_index: metadata.mem_id,
            //         offset_expr: ConstExpr::new(BinaryReader::new(
            //             val_bytes.as_slice(),
            //             metadata.curr_mem_offset,
            //             WasmFeatures::empty()
            //         ))
            //     }
            // };
            // module_data.push(
            //     data_segment
            // );
            // 
            // // save the memory addresses/lens, so they can be used as appropriate
            // *addr = Some((data_id as u32, metadata.curr_mem_offset, val.len()));
            // 
            // // emit Wasm instructions for the memory address and string length
            // func_builder.i32(metadata.curr_mem_offset as i32);
            // func_builder.i32(val.len() as i32);
            // 
            // // update curr_mem_offset to account for new data
            // metadata.curr_mem_offset += val.len();
            is_success &= true;
        }
        Value::Tuple { vals, .. } => {
            for val in vals.iter_mut() {
                is_success &= emit_expr(table, module_data, val, func_builder, metadata)?;
            }
        }
        Value::Boolean { val, .. } => {
            // "In a boolean context, such as a br_if condition, any non-zero value is interpreted as true
            // and 0 is interpreted as false."
            // https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#booleans
            if *val {
                // insert true (non-zero)
                func_builder.i32(1);
            } else {
                // insert false (zero)
                func_builder.i32(0);
            }
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
    mem_id: u32,
    curr_mem_offset: usize,
}

// 'b is the longest living
pub struct ModuleEmitter<'a, 'b>
{
    // by reference is 'a, the module is 'b
    pub app_wasm: &'a mut Module<'b>,
    pub emitting_func: Option<FunctionBuilder<'b>>,
    pub table: SymbolTable,
    instr_alt_call: Option<i32>,
    
    metadata: InsertionMetadata,
    // instr_iter: InstrIter,
    // emitting_instr: Option<EmittingInstrTracker>,

    // TODO change instr_iter and emitting_instr with orca
    // TODO: figure out what metadata is doing
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'b> ModuleEmitter<'a, 'b>
{
    // note: only used in integration test
    pub fn new (app_wasm: &'a mut Module<'b>, table: SymbolTable) -> Self {
        if app_wasm.memories.len() > 1 {
            // TODO -- make this work with multi-memory
            panic!("only single memory is supported")
        };
        // Assuming the ID of the first memory is 0!
        let mem_id = 0;

        let a = Self {
            app_wasm,
            emitting_func: None,
            metadata: InsertionMetadata {
                mem_id,
                curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
            },
            table,
            instr_alt_call: None,
            fn_providing_contexts: vec!["whamm".to_string()],
        };

        a
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
        let strcmp_params = vec![OrcaType::I32, OrcaType::I32, OrcaType::I32, OrcaType::I32];
        let strcmp_result = vec![OrcaType::I32];

        let mut strcmp = FunctionBuilder::new(&strcmp_params, &strcmp_result);

        // create params
        let str0_offset = strcmp.add_local(OrcaType::I32);
        let str0_size = strcmp.add_local(OrcaType::I32);
        let str1_offset = strcmp.add_local(OrcaType::I32);
        let str1_size = strcmp.add_local(OrcaType::I32);

        // create locals
        let i = strcmp.add_local(OrcaType::I32);
        let str0_char = strcmp.add_local(OrcaType::I32);
        let str1_char = strcmp.add_local(OrcaType::I32);

        #[rustfmt::skip]
        strcmp
            .block(BlockType::Empty) // label = @1
                .block(BlockType::Empty) // label = @2
                    // 1. Check if sizes are equal, if not return 0
                    .local_get(str0_size)
                    .local_get(str1_size)
                    .i32_ne()
                    .br_if(1) // (;@1;)

                    // 2. Check if mem offset is equal, if yes return non-zero (we are comparing the same data)
                    .local_get(str0_offset)
                    .local_get(str1_offset)
                    .i32_eq()
                    .br_if(0) // (;@2;)

                    // 3. iterate over each string and check equivalence of chars, if any not equal, return 0
                    .i32(0)
                    .local_set(i)
                    .loop_stmt(BlockType::Empty)
                        // Check if we've reached the end of the string
                        .local_get(i)
                        .local_get(str0_size)  // (can compare with either str size, equal at this point)
                        .i32_lte_unsigned()
                        .i32(0)
                        .i32_eq()
                        .br_if(1) // (;2;),  We've reached the end without failing equality checks!
            
                        // get char for str0
                        .local_get(str0_offset)
                        .local_get(i)
                        .i32_add()
                        // TODO -- support loading a byte from memory
                        // .load(
                        //     self.metadata.mem_id,
                        //     LoadKind::I32_8 {
                        //         kind: ExtendedLoad::ZeroExtend,
                        //     },
                        //     MemArg {
                        //         offset: 0,
                        //         align: 1,
                        //     },
                        // )
                        .local_set(str0_char)
                        
                        // get char for str1
                        .local_get(str1_offset)
                        .local_get(i)
                        .i32_add()
                        // TODO -- support loading a byte from memory
                        // .load(
                        //     self.metadata.mem_id,
                        //     LoadKind::I32_8 {
                        //         kind: ExtendedLoad::ZeroExtend,
                        //     },
                        //     MemArg {
                        //         offset: 0,
                        //         align: 1,
                        //     },
                        // )
                        .local_set(str1_char)
                        
                        // compare the two chars
                        .local_get(str0_char)
                        .local_get(str1_char)
                        .i32_ne()
                        .br_if(2) // (;@1;), If they are not equal, exit and return '0'
                        
                        // Increment i and continue loop
                        .local_get(i)
                        .i32(1)
                        .i32_add()
                        .local_set(i)
                        .br(0) // (;3;)
                    .end()
            
                    // 4. Reached the end of each string without returning, return nonzero
                    .br(0) // (;2;)
                .end()
            
                // they are equal, return '1'
                .i32(1)
                .return_stmt()
            .end()
            // they are not equal, return '0'
            .i32(0)
            .return_stmt();

        let strcmp_id = strcmp.finish(&mut self.app_wasm);

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

    fn instrument_before(&mut self) {
        // todo -- for visitor (not available on funcbuilder for module)
    }
    
    fn instrument_after(&mut self) {
        // todo -- for visitor (not available on funcbuilder for module)
    }
    
    fn instrument_as_alternate(&mut self) {
        // todo -- for visitor (not available on funcbuilder for module)
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
                        let wasm_ty = whamm_type_to_wasm(ty).ty.content_type;
                        if let Some(func) = &mut self.emitting_func {
                            let id = func.add_local(OrcaType::from(wasm_ty));
                            *addr = Some(VarAddr::Local { addr: id });
                        }
                        Ok(true)
                    }
                }
            }
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                false,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Wrong statement type, should be `assign`"
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
                        if let Some(emitting_func) = &mut self.emitting_func {
                            // Emit the instruction that sets the variable's value to the emitted expression
                            emit_set(
                                &mut self.table,
                                var_id,
                                emitting_func
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
    
    pub(crate) fn enter_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.enter_scope()
    }

    pub(crate) fn exit_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.exit_scope()
    }
    pub(crate) fn reset_children(&mut self) {
        self.table.reset_children();
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

    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        match expr {
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                // change conseq and alt types to stmt for easier API call
                is_success &= self.emit_if_else(cond, &mut vec![Statement::Expr {
                    expr: (**conseq).clone(),
                    loc: None
                }], &mut vec![Statement::Expr {
                    expr: (**alt).clone(),
                    loc: None
                }])?;
            }
            Expr::VarId { .. }
            | Expr::UnOp { .. }
            | Expr::BinOp { .. }
            | Expr::Primitive { .. }
            | Expr::Call { .. } => {
                // Anything else can be emitted as normal
                if let Some(emitting_func) = &mut self.emitting_func {
                    // Emit the instruction that sets the variable's value to the emitted expression
                    is_success &= emit_expr(
                        &mut self.table,
                        &mut self.app_wasm.data,
                        expr,
                        emitting_func,
                        &mut self.metadata
                    )?;
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

    pub(crate) fn emit_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
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
        // TODO: only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    fn emit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        // TODO: only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    pub(crate) fn emit_global(
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
                let default_global = whamm_type_to_wasm(&ty);
                let global_id = self.app_wasm.add_global(default_global);
                *addr = Some(VarAddr::Global { addr: global_id });
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

    fn emit_if(&mut self, condition: &mut Expr, conseq: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        // NOTE: The structure of this code is wonky, but it's because of
        // overlapping references/calls to self.
        // To avoid that, we place all calls to self.emitting_func in a block.
        
        let mut is_success = true;

        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the beginning of the if block
            emitting_func.if_stmt(BlockType::Empty);
        }
        
        // emit the consequent body
        is_success &= self.emit_body(conseq)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the end of the if block
            emitting_func.end();
        }

        Ok(is_success)
    }

    fn emit_if_else(&mut self, condition: &mut Expr, conseq: &mut Vec<Statement>, alternate: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        // NOTE: The structure of this code is wonky, but it's because of
        // overlapping references/calls to self.
        // To avoid that, we place all calls to self.emitting_func in a block.

        let mut is_success = true;

        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition)?;
        
        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the beginning of the if block
            emitting_func.if_stmt(BlockType::Empty);
        }

        // emit the consequent body
        is_success &= self.emit_body(conseq)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the beginning of the else
            emitting_func.else_stmt();
        }

        // emit the alternate body
        is_success &= self.emit_body(alternate)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the end of the if/else block
            emitting_func.end();
        }
        
        Ok(is_success)
    }

    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        // NOTE: This should be done in the Module entrypoint
        //       https://docs.rs/walrus/latest/walrus/struct.Module.html

        if let Some(_start_fid) = self.app_wasm.start {
            // 1. create the emitting_func var, assign in self
            // 2. iterate over stmts and emit them! (will be diff for Decl stmts)
            todo!()
        } else {
            // TODO -- try to create our own start fn (for dfinity case)
            for stmt in stmts.iter_mut() {
                match stmt {
                    Statement::Decl { .. } => {
                        // This is fine
                        todo!()
                    }
                    _ => {
                        // This is NOT fine...error!
                        // Cannot emit this at the moment since there's no entrypoint for our module to emit initialization instructions into
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(
                                "This module has no configured entrypoint, \
            unable to emit a `script` with initialized global state"
                                    .to_string(),
                            ),
                            None,
                        )));
                    }
                }
            }
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
                        // TODO -- how to pull func names from module?
                        // (val.clone(), self.app_wasm.funcs.by_name(val))
                        (val.clone(), Some(1056)) // hardcoded for now to ID for `redirect_to_fault_injector` for users.wasm file
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
            self.instr_alt_call = func_call_id;
        }
        true
    }

    fn emit_alt_call(&mut self) -> Result<bool, Box<WhammError>> {
        if let (Some(emitting_func), Some(alt_fn_id)) = (&mut self.emitting_func, self.instr_alt_call) {
            emitting_func.call(alt_fn_id as u32);
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
        Ok(true)
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        match stmt {
            Statement::Decl { .. } => self.emit_decl_stmt(stmt),
            Statement::Assign { .. } => self.emit_assign_stmt(stmt),
            Statement::Expr { expr, .. } => self.emit_expr(expr),
            Statement::Return { .. } => unimplemented!(),
            Statement::If {
                // cond, conseq, alt, .. -- for eventual implementation
                ..
            } => {
                unimplemented!()
            }
        }
    }

    pub fn dump_to_file(&mut self, output_wasm_path: String) -> Result<bool, Box<WhammError>> {
        self.app_wasm.emit_wasm(&output_wasm_path)?;
        Ok(true)
    }
}

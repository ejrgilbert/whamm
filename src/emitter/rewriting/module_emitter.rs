use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::{Block, DataType, Definition, Expr, Fn, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use orca::{DataSegment, DataSegmentKind, InitExpr};
use std::collections::HashMap;

use orca::ir::types::{BlockType, DataType as OrcaType, Value as OrcaValue};
use wasmparser::GlobalType;

use crate::emitter::rewriting::{emit_body, emit_expr, emit_stmt, whamm_type_to_wasm, Emitter};
use orca::ir::function::FunctionBuilder;
use orca::ir::module::Module;
use orca::opcode::Opcode;

const UNEXPECTED_ERR_MSG: &str =
    "ModuleEmitter: Looks like you've found a bug...please report this behavior!";

pub struct MemoryTracker {
    pub mem_id: u32,
    pub curr_mem_offset: usize,
    pub emitted_strings: HashMap<String, StringAddr>,
}

pub struct StringAddr {
    pub data_id: u32,
    pub mem_offset: usize,
    pub len: usize,
}

pub struct ModuleEmitter<'a, 'b, 'c, 'd> {
    pub app_wasm: &'a mut Module<'b>,
    pub emitting_func: Option<FunctionBuilder<'b>>,
    pub table: &'c mut SymbolTable,

    mem_tracker: &'d mut MemoryTracker,
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'b, 'c, 'd> ModuleEmitter<'a, 'b, 'c, 'd> {
    // note: only used in integration test
    pub fn new(
        app_wasm: &'a mut Module<'b>,
        table: &'c mut SymbolTable,
        mem_tracker: &'d mut MemoryTracker,
    ) -> Self {
        Self {
            app_wasm,
            emitting_func: None,
            mem_tracker,
            table,
            fn_providing_contexts: vec!["whamm".to_string()],
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
        let strcmp_params = vec![OrcaType::I32, OrcaType::I32, OrcaType::I32, OrcaType::I32];
        let strcmp_result = vec![OrcaType::I32];

        let mut strcmp = FunctionBuilder::new(&strcmp_params, &strcmp_result);

        // specify params
        let str0_offset = 0u32;
        let str0_size = 1u32;
        let str1_offset = 2u32;
        let str1_size = 3u32;

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
            .i32_const(0)
            .local_set(i)
            .loop_stmt(BlockType::Empty)
            // Check if we've reached the end of the string
            .local_get(i)
            .local_get(str0_size)  // (can compare with either str size, equal at this point)
            .i32_lt_unsigned()
            .i32_eqz()
            .br_if(1) // (;2;),  We've reached the end without failing equality checks!

            // get char for str0
            .local_get(str0_offset)
            .local_get(i)
            .i32_add()
            // load a byte from memory
            .i32_load8_u(
                wasmparser::MemArg {
                    align: 0,
                    max_align: 0,
                    offset: 0,
                    memory: self.mem_tracker.mem_id
                }
            )
            .local_set(str0_char)

            // get char for str1
            .local_get(str1_offset)
            .local_get(i)
            .i32_add()
            // load a byte from memory
            .i32_load8_u(
                wasmparser::MemArg {
                    align: 0,
                    max_align: 0,
                    offset: 0,
                    memory: self.mem_tracker.mem_id
                }
            )
            .local_set(str1_char)

            // compare the two chars
            .local_get(str0_char)
            .local_get(str1_char)
            .i32_ne()
            .br_if(2) // (;@1;), If they are not equal, exit and return '0'

            // Increment i and continue loop
            .local_get(i)
            .i32_const(1)
            .i32_add()
            .local_set(i)
            .br(0) // (;3;)
            .end()

            // 4. Reached the end of each string without returning, return nonzero
            .br(0) // (;2;)
            .end()

            // they are equal, return '1'
            .i32_const(1)
            .return_stmt()
            .end()
            // they are not equal, return '0'
            .i32_const(0)
            .return_stmt();

        let strcmp_id = strcmp.finish(self.app_wasm);
        self.app_wasm
            .set_fn_name(strcmp_id - self.app_wasm.num_import_func(), "strcmp");

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

    pub(crate) fn enter_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.enter_scope()
    }

    pub(crate) fn exit_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.exit_scope()
    }
    pub(crate) fn reset_children(&mut self) {
        self.table.reset_children();
    }

    pub(crate) fn emit_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
        // figure out if this is a provided fn.
        if f.def == Definition::CompilerDynamic {
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

    pub fn emit_string(&mut self, value: &mut Value) -> Result<bool, Box<WhammError>> {
        match value {
            Value::Str { val, .. } => {
                // assuming that the data ID is the index of the object in the Vec
                let data_id = self.app_wasm.data.len();
                let val_bytes = val.as_bytes().to_owned();
                let data_segment = DataSegment {
                    data: val_bytes,
                    kind: DataSegmentKind::Active {
                        memory_index: self.mem_tracker.mem_id,
                        offset_expr: InitExpr::Value(OrcaValue::I32(
                            self.mem_tracker.curr_mem_offset as i32,
                        )),
                    },
                };
                self.app_wasm.data.push(data_segment);

                // save the memory addresses/lens, so they can be used as appropriate
                self.mem_tracker.emitted_strings.insert(
                    val.clone(),
                    StringAddr {
                        data_id: data_id as u32,
                        mem_offset: self.mem_tracker.curr_mem_offset,
                        len: val.len(),
                    },
                );

                // update curr_mem_offset to account for new data
                self.mem_tracker.curr_mem_offset += val.len();
                Ok(true)
            }
            Value::Integer { .. } | Value::Tuple { .. } | Value::Boolean { .. } => {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Called 'emit_string', but this is not a string type: {:?}",
                        value
                    )),
                    None,
                )))
            }
        }
    }

    pub(crate) fn emit_global_getter(
        &mut self,
        global_id: &u32,
        name: String,
        ty: &GlobalType,
    ) -> Result<bool, Box<WhammError>> {
        let getter_params = vec![];
        let getter_res = vec![OrcaType::from(ty.content_type)];

        let mut getter = FunctionBuilder::new(&getter_params, &getter_res);
        getter.global_get(*global_id);

        let getter_id = getter.finish(self.app_wasm);

        let fn_name = format!("get_{name}");
        self.app_wasm.add_export_func(fn_name.leak(), getter_id);

        Ok(true)
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
        let (global_id, ty) = match rec {
            Some(Record::Var { ref mut addr, .. }) => {
                // emit global variable and set addr in symbol table
                // this is used for user-defined global vars in the script...
                let default_global = whamm_type_to_wasm(&ty);
                let global_id = self.app_wasm.add_global(default_global.clone());
                *addr = Some(VarAddr::Global { addr: global_id });
                (global_id, default_global.ty)
            }
            Some(&mut ref ty) => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Incorrect global variable record, expected Record::Var, found: {:?}",
                        ty
                    )),
                    None,
                )))
            }
            None => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Global variable symbol does not exist!"
                    )),
                    None,
                )))
            }
        };

        self.emit_global_getter(&global_id, name, &ty)
    }

    pub fn emit_global_stmts(&mut self, stmts: &mut [Statement]) -> Result<bool, Box<WhammError>> {
        // NOTE: This should be done in the Module entrypoint
        //       https://docs.rs/walrus/latest/walrus/struct.Module.html

        if let Some(_start_fid) = self.app_wasm.start {
            // 1. create the emitting_func var, assign in self
            // 2. iterate over stmts and emit them! (will be different for Decl stmts)
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

    fn get_unexpected_err(&mut self) -> Box<WhammError> {
        Box::new(ErrorGen::get_unexpected_error(
            true,
            Some(format!(
                "{UNEXPECTED_ERR_MSG} Something went wrong while emitting an instruction."
            )),
            None,
        ))
    }
}
impl Emitter for ModuleEmitter<'_, '_, '_, '_> {
    fn emit_body(&mut self, body: &mut Block) -> Result<bool, Box<WhammError>> {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_body(
                body,
                emitting_func,
                self.table,
                self.mem_tracker,
                UNEXPECTED_ERR_MSG,
            )
        } else {
            Err(self.get_unexpected_err())
        }
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_stmt(
                stmt,
                emitting_func,
                self.table,
                self.mem_tracker,
                UNEXPECTED_ERR_MSG,
            )
        } else {
            Err(self.get_unexpected_err())
        }
    }

    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>> {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_expr(
                expr,
                emitting_func,
                self.table,
                self.mem_tracker,
                UNEXPECTED_ERR_MSG,
            )
        } else {
            Err(self.get_unexpected_err())
        }
    }
}

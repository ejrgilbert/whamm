use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::report_var_metadata::ReportVarMetadata;
use crate::parser::types::{Block, DataType, Definition, Expr, Fn, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use orca_wasm::{DataSegment, DataSegmentKind, InitExpr, Location};
use std::collections::HashMap;

use crate::emitter::rewriting::rules::Arg;
use crate::emitter::utils::{emit_body, emit_expr, emit_stmt, whamm_type_to_wasm_global};
use crate::emitter::Emitter;
use crate::libraries::core::maps::map_adapter::MapLibAdapter;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::{BlockType as OrcaBlockType, DataType as OrcaType, Value as OrcaValue};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::{Instrumenter, Opcode};

const UNEXPECTED_ERR_MSG: &str =
    "ModuleEmitter: Looks like you've found a bug...please report this behavior!";

pub struct MemoryTracker {
    pub mem_id: u32,
    pub curr_mem_offset: usize,
    pub required_initial_mem_size: u64,
    pub emitted_strings: HashMap<String, StringAddr>,
}

pub struct StringAddr {
    pub data_id: u32,
    pub mem_offset: usize,
    pub len: usize,
}

pub struct ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f> {
    pub app_wasm: &'a mut Module<'b>,
    pub emitting_func: Option<FunctionBuilder<'b>>,
    pub table: &'c mut SymbolTable,
    mem_tracker: &'d mut MemoryTracker,
    pub map_lib_adapter: &'e mut MapLibAdapter,
    pub report_var_metadata: &'f mut ReportVarMetadata,
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f> ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f> {
    // note: only used in integration test
    pub fn new(
        app_wasm: &'a mut Module<'b>,
        table: &'c mut SymbolTable,
        mem_tracker: &'d mut MemoryTracker,
        map_lib_adapter: &'e mut MapLibAdapter,
        report_var_metadata: &'f mut ReportVarMetadata,
    ) -> Self {
        Self {
            app_wasm,
            emitting_func: None,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            table,
            fn_providing_contexts: vec!["whamm".to_string()],
        }
    }

    fn emit_provided_fn(
        &mut self,
        context: &str,
        f: &Fn,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        if context == "whamm" && f.name.name == "strcmp" {
            self.emit_whamm_strcmp_fn(f, err)
        } else {
            err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
            Provided function, but could not find a context to provide the definition, context: {}",
                    context
                )),
                None,
            ));
            None
        }
    }

    fn emit_whamm_strcmp_fn(&mut self, f: &Fn, err: &mut ErrorGen) -> Option<FunctionID> {
        let strcmp_params = vec![OrcaType::I32, OrcaType::I32, OrcaType::I32, OrcaType::I32];
        let strcmp_result = vec![OrcaType::I32];

        let mut strcmp = FunctionBuilder::new(&strcmp_params, &strcmp_result);

        // specify params
        let str0_offset = LocalID(0);
        let str0_size = LocalID(1);
        let str1_offset = LocalID(2);
        let str1_size = LocalID(3);

        // create locals
        let i = strcmp.add_local(OrcaType::I32);
        let str0_char = strcmp.add_local(OrcaType::I32);
        let str1_char = strcmp.add_local(OrcaType::I32);

        #[rustfmt::skip]
        strcmp
            .block(OrcaBlockType::Empty) // label = @1
            .block(OrcaBlockType::Empty) // label = @2
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
            .loop_stmt(OrcaBlockType::Empty)
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
                    memory: 0 // app memory!
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
                    memory: self.mem_tracker.mem_id // instr memory!
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

        let strcmp_id = strcmp.finish_module(self.app_wasm);
        self.app_wasm.set_fn_name(strcmp_id, "strcmp".to_string());

        let Record::Fn { addr, .. } = self.table.lookup_fn_mut(&f.name.name, err)? else {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
            return None;
        };
        *addr = Some(*strcmp_id);
        Some(strcmp_id)
    }

    pub(crate) fn enter_scope(&mut self, err: &mut ErrorGen) {
        self.table.enter_scope(err)
    }

    pub(crate) fn exit_scope(&mut self, err: &mut ErrorGen) {
        self.table.exit_scope(err)
    }
    pub(crate) fn reset_table(&mut self) {
        self.table.reset();
    }

    pub(crate) fn emit_fn(
        &mut self,
        context: &str,
        f: &Fn,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        // figure out if this is a provided fn.
        if f.def == Definition::CompilerDynamic {
            return if self.fn_providing_contexts.contains(&context.to_string()) {
                self.emit_provided_fn(context, f, err)
            } else {
                err.add_error(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Provided fn, but could not find a context to provide the definition, context: {}",
                        context
                    )),
                    None,
                ));
                None
            };
        }

        // emit non-provided fn
        // TODO: only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    pub fn emit_special_fn(
        &mut self,
        name: Option<String>,
        params: &[OrcaType],
        results: &[OrcaType],
        block: &mut Block,
        err: &mut ErrorGen,
    ) -> Option<u32> {
        let func = FunctionBuilder::new(params, results);
        self.emitting_func = Some(func);

        // emit the predicate function body
        self.emit_body(&[], block, err);

        // emit the function
        self.finish_emitting_fn(name)
    }

    // pub fn emit_body_as_fn(
    //     &mut self,
    //     name: Option<String>,
    //     params: &[OrcaType],
    //     results: &[OrcaType],
    //     body: &mut Block,
    //     err: &mut ErrorGen,
    // ) -> Option<u32> {
    //     let pred_func = FunctionBuilder::new(params, results);
    //     self.emitting_func = Some(pred_func);
    //
    //     // emit the predicate function body
    //     self.emit_body(&[], body, err);
    //
    //     // emit the function
    //     self.finish_emitting_fn(name)
    // }

    pub fn finish_emitting_fn(&mut self, name: Option<String>) -> Option<u32> {
        if let Some(func) = self.emitting_func.take() {
            let fid = func.finish_module(self.app_wasm);
            if let Some(name) = name {
                self.app_wasm.set_fn_name(fid, name);
            }
            Some(*fid)
        } else {
            None
        }
    }

    pub fn emit_string(&mut self, value: &mut Value, err: &mut ErrorGen) -> bool {
        match value {
            Value::Str { val, .. } => {
                if self.mem_tracker.emitted_strings.contains_key(val) {
                    // the string has already been emitted into the module, don't emit again
                    return true;
                }
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
                true
            }
            _ => {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Called 'emit_string', but this is not a string type: {:?}",
                        value
                    )),
                    None,
                );
                false
            }
        }
    }

    pub(crate) fn memory_grow(&mut self) {
        // If we've emitted any strings, bump the app's memory up to account for that
        if !self.mem_tracker.emitted_strings.is_empty() {
            if let Some(mem) = self.app_wasm.memories.get_mut(0) {
                if mem.initial < self.mem_tracker.required_initial_mem_size {
                    mem.initial = self.mem_tracker.required_initial_mem_size;
                }
            }
        }
    }

    pub(crate) fn emit_global_getter(
        &mut self,
        global_id: &u32,
        name: String,
        ty: OrcaType,
    ) -> FunctionID {
        // todo -- make this conditional on 'testing' mode
        let getter_params = vec![];
        let getter_res = vec![ty];

        let mut getter = FunctionBuilder::new(&getter_params, &getter_res);
        getter.global_get(GlobalID(*global_id));

        let getter_id = getter.finish_module(self.app_wasm);
        let fn_name = format!("get_{name}");
        self.app_wasm.set_fn_name(getter_id, fn_name.clone());
        self.app_wasm.exports.add_export_func(fn_name, *getter_id);

        getter_id
    }

    pub(crate) fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        self.emit_global_inner(name, ty, val, false, err)
    }
    pub fn emit_report_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        self.emit_global_inner(name, ty, val, true, err)
    }
    pub fn emit_global_inner(
        &mut self,
        name: String,
        _ty: DataType,
        _val: &Option<Value>,
        report_mode: bool,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        let Record::Var { addr, ty, .. } = self.table.lookup_var_mut(&name, &None, err)? else {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
            return None;
        };

        // emit global variable and set addr in symbol table
        // this is used for user-defined global vars in the script...
        match ty {
            DataType::Map { .. } => {
                //time to instrument the start fn
                let Some(init_id) = self
                    .app_wasm
                    .functions
                    .get_local_fid_by_name("global_map_init")
                else {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                                No global_map_init found in the module!"
                        )),
                        None,
                    );
                    return None;
                };

                let Some(mut init_fn) = self.app_wasm.functions.get_fn_modifier(init_id) else {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                                No global_map_init found in the module!"
                        )),
                        None,
                    );
                    return None;
                };
                init_fn.before_at(Location::Module {
                    func_idx: init_id, // not used
                    instr_idx: 0,
                });
                let map_id = if report_mode {
                    self.map_lib_adapter.map_create_report(
                        name,
                        ty.clone(),
                        &mut init_fn,
                        self.report_var_metadata,
                        false,
                        err,
                    )
                } else {
                    self.map_lib_adapter
                        .map_create(ty.clone(), &mut init_fn, err)
                };

                *addr = Some(VarAddr::MapId { addr: map_id });
                None
            }
            _ => {
                let (global_id, global_ty) = whamm_type_to_wasm_global(self.app_wasm, ty);
                *addr = Some(VarAddr::Global { addr: *global_id });
                //now save off the global variable metadata
                if report_mode {
                    self.report_var_metadata
                        .put_global_metadata(*global_id, name.clone(), err);
                }
                Some(self.emit_global_getter(&global_id, name, global_ty))
            }
        }
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
}
impl Emitter for ModuleEmitter<'_, '_, '_, '_, '_, '_> {
    fn emit_body(
        &mut self,
        _curr_instr_args: &[Arg],
        body: &mut Block,
        err: &mut ErrorGen,
    ) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_body(
                body,
                emitting_func,
                self.table,
                self.mem_tracker,
                self.map_lib_adapter,
                self.report_var_metadata,
                UNEXPECTED_ERR_MSG,
                err,
            )
        } else {
            false
        }
    }

    fn emit_stmt(
        &mut self,
        _curr_instr_args: &[Arg],
        stmt: &mut Statement,
        err: &mut ErrorGen,
    ) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_stmt(
                stmt,
                emitting_func,
                self.table,
                self.mem_tracker,
                self.map_lib_adapter,
                self.report_var_metadata,
                UNEXPECTED_ERR_MSG,
                err,
            )
        } else {
            false
        }
    }

    fn emit_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_expr(
                expr,
                emitting_func,
                self.table,
                self.mem_tracker,
                self.map_lib_adapter,
                self.report_var_metadata,
                UNEXPECTED_ERR_MSG,
                err,
            )
        } else {
            false
        }
    }
}

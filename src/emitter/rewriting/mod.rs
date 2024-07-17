pub mod rules;

use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::rules::{LocInfo, Provider, WhammProvider};
use crate::emitter::Emitter;
use crate::generator::types::ExprFolder;
use crate::parser::types::{BinOp, DataType, Expr, Fn, ProbeSpec, Statement, UnOp, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use log::{debug, info};
use orca::iterator::module_iterator::ModuleIterator;
use walrus::ir::{BinaryOp, ExtendedLoad, Instr, InstrSeqId, LoadKind, MemArg};
use walrus::{
    ActiveData, ActiveDataLocation, DataKind, FunctionBuilder, FunctionId, FunctionKind,
    InstrSeqBuilder, LocalFunction, MemoryId, ModuleData,
};

use orca::ir::types::{Global, InitExpr, Value as OrcaValue, DataType as OrcaType};
use wasmparser::{ValType, Operator};

use orca::ir::module::Module;
use orca::ir::component::Component;

fn module_to_component(module: Module) -> Component {
    let mut component = Component::new();
    component.add_module(module);
    component
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

// transform a whamm type to default wasm type, used for creating new global
// TODO: Might be more generic to also inlcude Local
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

// TODO: the following helper function is an unfortunate workaround for some problems
// with interacting with Self
// emit_set, emit_expr, emit_binop, emit_value

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

// 'b is the longest living
pub struct WasmRewritingEmitter<'a, 'b> 
{
    // by refernce is 'a, the module is 'b
    pub app_wasm: &'a mut orca::ir::module::Module<'b>,
    pub table: SymbolTable,

    // TODO: remove the Option here, (the new function)
    pub(crate) iterator: Option<ModuleIterator<'a, 'b>>,

    // whamm! AST traversal bookkeeping
    // metadata: InsertionMetadata,
    // instr_iter: InstrIter,
    // emitting_instr: Option<EmittingInstrTracker>,

    // TODO change instr_iter and emitting_instr with orca
    // TODO: figure out what metadata is doing
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'b> WasmRewritingEmitter<'a, 'b>
{
    // note: only used in integration test
    pub fn new (app_wasm: &'a mut orca::ir::module::Module<'b>, table: SymbolTable) -> Self {
        let a = Self {
            app_wasm,
            table,
            iterator: None,
            fn_providing_contexts: vec!["whamm".to_string()],
        };

        // a.iterator = Some(ModuleIterator::new(self.app_wasm));
        a
    }

    // pub fn add_iterator<'c>(&'c mut self)
    //     where 'c: 'b
    // {
    //     self.iterator = Some(ModuleIterator::<'a, 'b>::new(&mut self.app_wasm));
    // }

    fn emit_provided_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
        if context == "whamm" && f.name.name == "strcmp" {
            // TODO: emit strcmp function
            // self.emit_whamm_strcmp_fn(f)
            Ok(true)
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

    fn emit_decl_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        todo!();
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
                        todo!();
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

impl <'a, 'b> Emitter for WasmRewritingEmitter<'a, 'b> 
{
    fn enter_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.enter_scope()
    }

    fn enter_scope_via_spec(&mut self, script_id: &str, probe_spec: &ProbeSpec) -> bool {
        self.table.enter_scope_via_spec(script_id, probe_spec)
    }

    // fn init_iterator(&mut self) -> WasmRewritingEmitter<'a, 'b>  {
    //     self.iterator = Some(ModuleIterator::<'a, 'b>::new(&mut self.app_wasm));
    //     Ok(())
    // }

    fn exit_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.exit_scope()
    }
    fn reset_children(&mut self) {
        self.table.reset_children();
    }

    // self must outlive ModuleIterator
    // app_wasm should outlive iterator
    fn init_instr_iter(&mut self) -> Result<(), Box<WhammError>> 
    {
        // LOOK AT ME
        self.iterator = Some(ModuleIterator::new(self.app_wasm));
        // self.instr_iter.init(&self.app_wasm);
        Ok(())
    }

    /// bool -> whether there is a next instruction to process
    fn has_next_instr(&self) -> bool {
        // TODO
        false
    }

    fn init_first_instr(&mut self) -> bool {
        false
    }

    /// bool -> whether it found a next instruction
    fn next_instr(&mut self) -> bool {
        // TODO
        false
    }

    fn curr_instr(&self) -> &Operator {
        // let curr_instr = self.instr_iter.curr().unwrap();
        // &curr_instr.instr
        unimplemented!()
    }
    
    fn curr_instr_name(&self) -> &str {
        // let curr_instr = self.instr_iter.curr().unwrap();
        // curr_instr.instr_name.as_str()
        unimplemented!()
    }

    fn incr_loc_pointer(&mut self) {
        // TODO
    }

    fn get_loc_info<'d>(&self, rule: &'d WhammProvider) -> Option<LocInfo<'d>> {
        // let curr_instr = self.curr_instr();
        // rule.get_loc_info(&self.app_wasm, curr_instr)
        None
    }

    fn save_args(&mut self, args: &[OrcaType]) -> bool {
        // if let Some(curr_loc) = self.instr_iter.curr_mut() {
        //     if let Some(tracker) = &mut self.emitting_instr {
        //         let func = self
        //             .app_wasm
        //             .funcs
        //             .get_mut(curr_loc.wasm_func_id)
        //             .kind
        //             .unwrap_local_mut();
        //         let func_builder = func.builder_mut();
        //         let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

        //         // No opcodes should have been emitted in the module yet!
        //         // So, we can just save off the first * items in the stack as the args
        //         // to the call.
        //         let mut arg_recs = vec![]; // vec to retain order!
        //         args.iter().enumerate().for_each(|(num, param_ty)| {
        //             // create local for the param in the module
        //             let arg_local_id = self.app_wasm.locals.add(*param_ty);

        //             // emit a opcode in the event to assign the ToS to this new local
        //             instr_builder.instr_at(
        //                 tracker.curr_idx,
        //                 walrus::ir::LocalSet {
        //                     local: arg_local_id,
        //                 },
        //             );

        //             // update index of tracker to point to what follows our insertions
        //             tracker.curr_idx += 1;

        //             // also update index to point to new location of instrumented instruction!
        //             // (saved args go before the original instruction)
        //             tracker.orig_instr_idx += 1;

        //             // place in symbol table with var addr for future reference
        //             let arg_name = format!("arg{}", num);
        //             let id = self.table.put(
        //                 arg_name.clone(),
        //                 Record::Var {
        //                     ty: DataType::I32, // we only support integers right now.
        //                     name: arg_name.clone(),
        //                     value: None,
        //                     is_comp_provided: false,
        //                     addr: Some(VarAddr::Local { addr: arg_local_id }),
        //                     loc: None,
        //                 },
        //             );
        //             arg_recs.push((arg_name, id));
        //         });
        //         curr_loc.instr_created_args = arg_recs;
        //         return true;
        //     }
        // }
        false
    }

    fn emit_args(&mut self) -> Result<bool, Box<WhammError>> {
        // if let Some(curr_loc) = self.instr_iter.curr_mut() {
        //     if let Some(tracker) = &mut self.emitting_instr {
        //         let func = self
        //             .app_wasm
        //             .funcs
        //             .get_mut(curr_loc.wasm_func_id)
        //             .kind
        //             .unwrap_local_mut();
        //         let func_builder = func.builder_mut();
        //         let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

        //         for (_param_name, param_rec_id) in curr_loc.instr_created_args.iter() {
        //             let param_rec = self.table.get_record_mut(param_rec_id);
        //             if let Some(Record::Var {
        //                 addr: Some(VarAddr::Local { addr }),
        //                 ..
        //             }) = param_rec
        //             {
        //                 // Inject at tracker.orig_instr_idx to make sure that this actually emits the args
        //                 // for the instrumented instruction right before that instruction is called!
        //                 instr_builder.instr_at(
        //                     tracker.orig_instr_idx,
        //                     walrus::ir::LocalGet { local: *addr },
        //                 );

        //                 // update index to point to new location of instrumented instruction!
        //                 // (re-emitted args go before the original instruction)
        //                 tracker.orig_instr_idx += 1;
        //             } else {
        //                 return Err(Box::new(ErrorGen::get_unexpected_error(
        //                     true,
        //                     Some(format!(
        //                         "{UNEXPECTED_ERR_MSG} \
        //                 Could not emit parameters, something went wrong..."
        //                     )),
        //                     None,
        //                 )));
        //             }
        //         }
        //         return Ok(true);
        //     }
        // }
        Ok(false)
    }

    fn define(&mut self, var_name: &str, var_val: &Option<Value>) -> Result<bool, Box<WhammError>> {
        // let rec_id = match self.table.lookup(var_name) {
        //     Some(rec_id) => *rec_id,
        //     _ => {
        //         return Err(Box::new(ErrorGen::get_unexpected_error(
        //             true,
        //             Some(format!(
        //                 "{UNEXPECTED_ERR_MSG} \
        //                 `{var_name}` symbol does not exist in this scope!"
        //             )),
        //             None,
        //         )));
        //     }
        // };
        // self.override_var_val(&rec_id, var_val.clone());

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
                // // Anything else can be emitted as normal
                // if let Some(curr_loc) = self.instr_iter.curr_mut() {
                //     if let Some(tracker) = &mut self.emitting_instr {
                //         let func = self
                //             .app_wasm
                //             .funcs
                //             .get_mut(curr_loc.wasm_func_id)
                //             .kind
                //             .unwrap_local_mut();
                //         let func_builder = func.builder_mut();
                //         let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                //         is_success &= emit_expr(
                //             &mut self.table,
                //             &mut self.app_wasm.data,
                //             expr,
                //             &mut instr_builder,
                //             &mut self.metadata,
                //             &mut tracker.curr_idx,
                //         )?;
                //     } else {
                //         return Err(Box::new(ErrorGen::get_unexpected_error(
                //             true,
                //             Some(format!(
                //                 "{UNEXPECTED_ERR_MSG} \
                //             Something went wrong while emitting an instruction."
                //             )),
                //             None,
                //         )));
                //     }
                // } else {
                //     return Err(Box::new(ErrorGen::get_unexpected_error(
                //         true,
                //         Some(format!(
                //             "{UNEXPECTED_ERR_MSG} \
                //         Something went wrong while emitting an instruction."
                //         )),
                //         None,
                //     )));
                // }
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
        // TODO: only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    fn emit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        // TODO: only when we're supporting user-defined fns in script...
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

    fn remove_orig(&mut self) -> bool {
        // todo!();
        false
    }

    fn emit_orig(&mut self) -> bool {
        // todo!();
        false
    }

    fn emit_if(&mut self) -> bool {
        // todo!()
        false
    }

    fn emit_if_else(&mut self) -> bool {
        // todo!()
        false
    }

    /// Will configure the emitter to emit subsequent expression as the condition of an if or if/else stmt
    /// Then emits the passed condition at that location.
    fn emit_condition(&mut self) -> bool {
        // todo!();
        false
    }

    /// Will configure the emitter to emit subsequent statements into the consequent body of an if or if/else stmt
    fn emit_consequent(&mut self) -> bool {
        // todo!();
        false
    }

    /// Will configure the emitter to emit subsequent statements into the alternate body of an if/else stmt
    fn emit_alternate(&mut self) -> bool {
        // todo!()
        false
    }

    /// Will configure the emitter to emit subsequent statements in the outer block of some branching logic
    fn finish_branch(&mut self) -> bool {
        // todo!()
        false
    }

    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        // NOTE: This should be done in the Module entrypoint
        //       https://docs.rs/walrus/latest/walrus/struct.Module.html

        // TODO: need to reason with start funciton (dfinity case)

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
        Ok(true)
    }

    fn emit_body(&mut self, body: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        for stmt in body.iter_mut() {
            self.emit_stmt(stmt)?;
        }
        Ok(true)
    }

    fn has_alt_call(&mut self) -> bool {
        todo!()
    }

    fn emit_alt_call(&mut self) -> Result<bool, Box<WhammError>> {
        todo!()
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        match stmt {
            Statement::Decl { .. } => self.emit_decl_stmt(stmt),
            Statement::Assign { .. } => self.emit_assign_stmt(stmt),
            Statement::Expr { expr, .. } => self.emit_expr(expr),
            Statement::Return { .. } => unimplemented!(),
            Statement::If {
                // cond, conseq, alt, .. -- for eventual implimentation
                ..
            } => {
                unimplemented!()
            }
        }
    }

    fn dump_to_file(&mut self, output_wasm_path: String) -> Result<bool, Box<WhammError>> {
        self.app_wasm.emit_wasm(&output_wasm_path)?;
        Ok(true)
    }
}

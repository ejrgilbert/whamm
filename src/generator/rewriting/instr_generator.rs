use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::StringAddr;
use crate::emitter::report_var_metadata::{BytecodeLoc, LocationData, Metadata};
use crate::emitter::rewriting::rules::{provider_factory, Arg, LocInfo, ProbeSpec, WhammProvider};
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::Emitter;
use crate::generator::folding::ExprFolder;
use crate::generator::rewriting::simple_ast::{SimpleAST, SimpleProbe};
use crate::parser::rules::core::WhammModeKind;
use crate::parser::types::{Block, Expr, Value};
use crate::verifier::types::Record;
use orca_wasm::ir::id::{FunctionID, GlobalID};
use orca_wasm::ir::types::{BlockType as OrcaBlockType, Value as OrcaValue};
use orca_wasm::iterator::iterator_trait::Iterator;
use orca_wasm::opcode::Instrumenter;
use orca_wasm::{DataSegment, DataSegmentKind, InitExpr, Opcode};
use orca_wasm::{Location as OrcaLocation, Location};
use std::collections::HashMap;
use std::iter::Iterator as StdIter;

const UNEXPECTED_ERR_MSG: &str =
    "InstrGenerator: Looks like you've found a bug...please report this behavior!";

fn get_loc_info<'a>(rule: &'a WhammProvider, emitter: &VisitingEmitter) -> Option<LocInfo<'a>> {
    // Pull the curr instr each time this is called to keep from having
    // long-lasting refs into self.emitter.
    emitter.get_loc_info(rule)
}

fn emit_dynamic_compiler_data(
    data: &HashMap<String, Option<Value>>,
    emitter: &mut VisitingEmitter,
    err: &mut ErrorGen,
) {
    emitter.emit_dynamic_compiler_data(data, err);
}

fn add_to_table(
    data: &HashMap<String, Option<Value>>,
    emitter: &mut VisitingEmitter,
    err: &mut ErrorGen,
) {
    data.iter().for_each(|(dyn_var_name, dyn_var_val)| {
        emitter.define(dyn_var_name, dyn_var_val, err);
    });
}

/// The second phase of instrumenting a Wasm module by actually emitting the
/// instrumentation code.
///
/// To do this, the generator traverses the BehaviorTree AST and calls the
/// passed emitter to emit instrumentation code.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the passed Emitter type.
pub struct InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    pub emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
    pub ast: SimpleAST,
    pub err: &'h mut ErrorGen,
    curr_instr_args: Vec<Arg>,
    curr_probe_mode: WhammModeKind,
    /// The current probe's body and predicate
    curr_probe: Option<(Option<Block>, Option<Expr>)>,
}
impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    pub fn new(
        emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
        ast: SimpleAST,
        err: &'h mut ErrorGen,
    ) -> Self {
        Self {
            emitter,
            ast,
            err,
            curr_instr_args: vec![],
            curr_probe_mode: WhammModeKind::Begin,
            curr_probe: None,
        }
    }

    pub fn configure_probe_mode(&mut self) -> bool {
        // function entry/exit should be handled before this point!
        match self.curr_probe_mode {
            WhammModeKind::Before => self.emitter.before(),
            WhammModeKind::After => self.emitter.after(),
            WhammModeKind::Alt => self.emitter.alternate(),
            WhammModeKind::SemanticAfter => self.emitter.semantic_after(),
            WhammModeKind::Entry => self.emitter.block_entry(),
            WhammModeKind::Exit => self.emitter.block_exit(),
            WhammModeKind::BlockAlt => self.emitter.block_alt(),
            _ => return false,
        }
        true
    }

    pub fn run(&mut self) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();

        // Here we do the following logic:
        // 1. initialize the emitter rules
        // 2. for each instruction (iterate over app_wasm)
        //    1. rules.process_instr() -> Vec<MatchedRules>
        //    2. for each matched probe
        //       1. enter the scope using (script_id, probe_spec)
        //       2. initialize the symbol table with the metadata at this program point
        //       3. create a new clone of the probe, fold the predicate
        //       4. traverse the behavior tree to emit code! (if predicate is not false)

        // Initialize the emitter rules
        let rules = provider_factory::<WhammProvider>(&self.ast.probes);

        // Iterate over each instruction in the application Wasm bytecode
        let mut is_success = true;
        let mut first_instr = true;
        while first_instr || self.emitter.next_instr() {
            first_instr = false;
            rules.iter().for_each(|rule| {
                // Check if any of the configured rules match this instruction in the application.
                if let Some(loc_info) = get_loc_info(rule, &self.emitter) {
                    if loc_info.num_alt_probes > 1 {
                        self.err
                            .multiple_alt_matches(self.emitter.curr_instr_name().as_str());
                    }
                    // This location has matched some rules, inject each matched probe!
                    loc_info.probes.iter().for_each(|(probe_spec, probe)| {
                        // Enter the scope for this matched probe
                        self.set_curr_loc(probe_spec, probe);
                        is_success = self
                            .emitter
                            .enter_scope_via_spec(&probe.script_id, probe_spec);

                        // Initialize the symbol table with the metadata at this program point
                        add_to_table(&loc_info.static_data, &mut self.emitter, self.err);

                        // Create a new clone of the probe, fold the predicate.
                        // NOTE: We make a clone so that the probe is reset for each instruction!
                        let (body_clone, mut pred_clone) =
                            (probe.body.clone(), probe.predicate.clone());
                        if let Some(pred) = &mut pred_clone {
                            // Fold predicate
                            is_success = self.emitter.fold_expr(pred, self.err);

                            // If the predicate evaluates to false, short-circuit!
                            if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
                                if !pred_as_bool {
                                    // predicate is reduced to false, short-circuit!
                                    return;
                                }
                            }
                        }

                        self.curr_instr_args = loc_info.args.clone(); // must clone so that this lives long enough
                        self.curr_probe_mode = probe_spec.mode.as_ref().unwrap().clone();
                        self.curr_probe = Some((body_clone, pred_clone));

                        // emit the probe (since the predicate is not false)
                        is_success &= self.emit_probe(&loc_info.dynamic_data);

                        // Now that we've emitted this probe, reset the symbol table's static/dynamic
                        // data defined for this instr
                        self.emitter.reset_table_data(&loc_info);
                    });
                }
            });
        }
        is_success &= self.after_run();
        is_success
    }
    fn set_curr_loc(&mut self, probe_spec: &ProbeSpec, probe: &SimpleProbe) {
        let curr_script_id = probe.script_id.clone();
        self.emitter.curr_num_reports = probe.num_reports;
        let curr_provider = match &probe_spec.provider {
            Some(provider) => provider.name.clone(),
            None => "".to_string(),
        };
        let curr_package = match &probe_spec.package {
            Some(package) => package.name.clone(),
            None => "".to_string(),
        };
        let curr_event = match &probe_spec.event {
            Some(event) => event.name.clone(),
            None => "".to_string(),
        };
        let curr_mode = match &probe_spec.mode {
            Some(mode) => mode.name().clone(),
            None => "".to_string(),
        };
        let curr_probe_id = format!(
            "{}_{}:{}:{}:{}",
            probe.probe_number, curr_provider, curr_package, curr_event, curr_mode
        );
        let loc = match self.emitter.app_iter.curr_loc().0 {
            OrcaLocation::Module {
                func_idx,
                instr_idx,
                ..
            }
            | OrcaLocation::Component {
                func_idx,
                instr_idx,
                ..
            } => BytecodeLoc::new(*func_idx, instr_idx as u32),
        };
        //set the current location in bytecode and load some new globals for potential report vars
        self.emitter.report_var_metadata.curr_location = LocationData::Local {
            script_id: curr_script_id,
            bytecode_loc: loc,
            probe_id: curr_probe_id,
            num_reports: self.emitter.curr_num_reports, //this is still used in the emitter to determine how many new globals to emit
        };
    }
}
impl<'b> InstrGenerator<'_, 'b, '_, '_, '_, '_, '_, '_> {
    fn emit_probe(&mut self, dynamic_data: &HashMap<String, Option<Value>>) -> bool {
        let mut is_success = true;

        is_success &= self.save_args();
        //after saving args, we run the check if we need to initialize global maps
        // only inject this IF NEEDED (not all scripts need global init)
        if self.emitter.map_lib_adapter.is_used {
            self.emit_global_map_init();
        }
        self.configure_probe_mode();

        // Now we know we're going to insert the probe, let's define
        // the dynamic information
        emit_dynamic_compiler_data(dynamic_data, &mut self.emitter, self.err);
        if self.pred_is_true() {
            // The predicate has been reduced to a 'true', emit un-predicated body
            self.emit_body();
            if !matches!(self.curr_probe_mode, WhammModeKind::Alt) {
                self.replace_args();
            }
        } else {
            // The predicate still has some conditionals (remember we already checked for
            // it being false in run() above)
            match self.curr_probe_mode {
                WhammModeKind::Before
                | WhammModeKind::After
                | WhammModeKind::SemanticAfter
                | WhammModeKind::Entry
                | WhammModeKind::Exit => {
                    is_success &= self.emit_probe_as_if();
                    self.replace_args();
                }
                WhammModeKind::Alt => {
                    is_success &= self.emit_probe_as_if_else();
                }
                _ => {
                    self.err.unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} Unexpected probe mode '{}'",
                            self.curr_probe_mode.name()
                        )),
                        None,
                    );
                    is_success &= false;
                }
            }
        }

        is_success
    }

    fn save_args(&mut self) -> bool {
        if !self.curr_instr_args.is_empty() {
            // The current instruction has args, save them (before)
            self.emitter.before();
            self.emitter.save_args(&self.curr_instr_args)
        } else {
            // If no args, just return true
            true
        }
    }
    fn replace_args(&mut self) -> bool {
        // Place the original arguments back on the stack.
        self.emitter.before();
        self.emitter.emit_args(self.err)
    }

    fn pred_is_true(&mut self) -> bool {
        if let Some((.., pred)) = &self.curr_probe {
            if let Some(pred) = pred {
                if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
                    // predicate has been reduced to a boolean value
                    return pred_as_bool;
                }
            } else {
                // the predicate is not defined, it is automatically true
                // first, set the before/after/alt mode of the probe
                return true;
            }
        }
        false
    }

    fn emit_body(&mut self) -> bool {
        if let Some((body, ..)) = &mut self.curr_probe {
            if let Some(ref mut body) = body {
                self.emitter
                    .emit_body(&self.curr_instr_args, body, self.err)
            } else if body.is_none() {
                if self.curr_probe_mode == WhammModeKind::Alt {
                    match self.emitter.emit_empty_alternate() {
                        Err(e) => {
                            self.err.add_error(*e);
                            false
                        }
                        Ok(res) => res,
                    }
                } else if self.curr_probe_mode == WhammModeKind::BlockAlt {
                    match self.emitter.emit_empty_block_alt() {
                        Err(e) => {
                            self.err.add_error(*e);
                            false
                        }
                        Ok(res) => res,
                    }
                } else {
                    // no body to emit!
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn emit_probe_as_if(&mut self) -> bool {
        if let Some((Some(ref mut body), Some(ref mut pred))) = self.curr_probe {
            match self
                .emitter
                .emit_if(&self.curr_instr_args, pred, body, self.err)
            {
                Err(e) => {
                    self.err.add_error(*e);
                    false
                }
                Ok(res) => res,
            }
        } else {
            false
        }
    }

    fn emit_probe_as_if_else(&mut self) -> bool {
        if let Some((Some(ref mut body), Some(ref mut pred))) = self.curr_probe {
            match self.emitter.emit_if_with_orig_as_else(
                &self.curr_instr_args,
                pred,
                body,
                self.err,
            ) {
                Err(e) => {
                    self.err.add_error(*e);
                    false
                }
                Ok(res) => res,
            }
        } else {
            false
        }
    }
    fn emit_global_map_init(&mut self) {
        //1 means it isn't initialized, 0 means it is
        let to_call = match self
            .emitter
            .app_iter
            .module
            .functions
            .get_local_fid_by_name("global_map_init")
        {
            Some(to_call) => to_call,
            None => {
                self.err.add_error(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                    No global_map_init function found in the module!"
                    )),
                    None,
                ));
                return;
            }
        };
        self.emitter.before();
        let app_iter = &mut self.emitter.app_iter;
        app_iter.global_get(GlobalID(self.emitter.map_lib_adapter.init_bool_location));
        app_iter.if_stmt(OrcaBlockType::Empty);
        app_iter.i32_const(0);
        app_iter.global_set(GlobalID(self.emitter.map_lib_adapter.init_bool_location));
        app_iter.call(to_call);
        app_iter.end();
    }

    fn after_run(&mut self) -> bool {
        if self
            .emitter
            .report_var_metadata
            .variable_metadata
            .is_empty()
            && self.emitter.report_var_metadata.map_metadata.is_empty()
        {
            return true;
        }
        //after running, emit the metadata from the report_var_metadata into maps 0 and 1 in app_wasm - if meta exists
        let report_var_metadata = &self.emitter.report_var_metadata;
        let var_meta = &report_var_metadata.variable_metadata;
        let map_meta = &report_var_metadata.map_metadata;
        let mut var_meta_str: HashMap<u32, String> = HashMap::new();
        let mut map_meta_str: HashMap<u32, String> = HashMap::new();
        //convert the metadata into strings, add those to the data section, then use those to populate the maps
        for (key, value) in var_meta.iter() {
            //first, emit the string to data section
            let val = value.to_csv();
            let data_id = self.emitter.app_iter.module.data.len();
            let val_bytes = val.as_bytes().to_owned();
            let data_segment = DataSegment {
                data: val_bytes,
                kind: DataSegmentKind::Active {
                    memory_index: self.emitter.mem_tracker.mem_id,
                    offset_expr: InitExpr::Value(OrcaValue::I32(
                        self.emitter.mem_tracker.curr_mem_offset as i32,
                    )),
                },
            };
            self.emitter.app_iter.module.data.push(data_segment);
            // save the memory addresses/lens, so they can be used as appropriate
            self.emitter.mem_tracker.emitted_strings.insert(
                val.clone(),
                StringAddr {
                    data_id: data_id as u32,
                    mem_offset: self.emitter.mem_tracker.curr_mem_offset,
                    len: val.len(),
                },
            );
            // update curr_mem_offset to account for new data
            self.emitter.mem_tracker.curr_mem_offset += val.len();
            //now set the new key value for the new maps
            var_meta_str.insert(*key, val);
        }
        for (key, value) in map_meta.iter() {
            //first, emit the string to data section
            let val = value.to_csv();
            let data_id = self.emitter.app_iter.module.data.len();
            let val_bytes = val.as_bytes().to_owned();
            let data_segment = DataSegment {
                data: val_bytes,
                kind: DataSegmentKind::Active {
                    memory_index: self.emitter.mem_tracker.mem_id,
                    offset_expr: InitExpr::Value(OrcaValue::I32(
                        self.emitter.mem_tracker.curr_mem_offset as i32,
                    )),
                },
            };
            self.emitter.app_iter.module.data.push(data_segment);
            // save the memory addresses/lens, so they can be used as appropriate
            self.emitter.mem_tracker.emitted_strings.insert(
                val.clone(),
                StringAddr {
                    data_id: data_id as u32,
                    mem_offset: self.emitter.mem_tracker.curr_mem_offset,
                    len: val.len(),
                },
            );
            // update curr_mem_offset to account for new data
            self.emitter.mem_tracker.curr_mem_offset += val.len();
            //now set the new key value for the new maps
            map_meta_str.insert(*key, val);
        }
        let mut is_success = self.setup_print_global_meta(&var_meta_str);
        is_success &= self.setup_print_map_meta(&map_meta_str);
        is_success
    }

    /// set up the print_global_meta function for insertions
    fn setup_print_global_meta(&mut self, var_meta_str: &HashMap<u32, String>) -> bool {
        // get the function
        // todo(maps) -- look up the func name instead!
        let print_global_meta_id = if let Some(Record::Fn { addr: Some(id), .. }) =
            self.emitter.table.lookup_fn("print_global_meta", self.err)
        {
            *id
        } else {
            return false;
        };
        let print_global_meta_id = FunctionID(print_global_meta_id);

        let mut print_global_meta = match self
            .emitter
            .app_iter
            .module
            .functions
            .get_fn_modifier(print_global_meta_id)
        {
            Some(func) => func,
            None => {
                self.err.add_error(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                    No 'print_global_meta' function found in the module!"
                    )),
                    None,
                ));
                return false;
            }
        };
        //now set up the actual module editing
        print_global_meta.before_at(Location::Module {
            func_idx: print_global_meta_id, // not used
            instr_idx: 0,
        });

        // output the header data segment
        let header = Metadata::get_csv_header();
        self.emitter
            .io_adapter
            .putsln(header.clone(), &mut print_global_meta, self.err);

        // for each of the report globals, emit the printing logic
        for (key, val) in var_meta_str.iter() {
            self.emitter.io_adapter.puts(
                format!("i32,{key},{val},"),
                &mut print_global_meta,
                self.err,
            );

            // get the value of this report global
            print_global_meta.global_get(GlobalID(*key));
            self.emitter
                .io_adapter
                .call_puti(&mut print_global_meta, self.err);
            self.emitter
                .io_adapter
                .putln(&mut print_global_meta, self.err);
        }
        true
    }
    fn setup_print_map_meta(&mut self, map_meta_str: &HashMap<u32, String>) -> bool {
        // get the function
        //first, we need to create the maps in global_map_init - where all the other maps are initialized
        // todo(maps) -- look up the func name instead!
        let print_map_meta_id = if let Some(Record::Fn { addr: Some(id), .. }) =
            self.emitter.table.lookup_fn("print_map_meta", self.err)
        {
            *id
        } else {
            return false;
        };
        let print_map_meta_id = FunctionID(print_map_meta_id);

        let mut print_map_meta = match self
            .emitter
            .app_iter
            .module
            .functions
            .get_fn_modifier(print_map_meta_id)
        {
            Some(func) => func,
            None => {
                self.err.add_error(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                    No 'print_map_meta' function found in the module!"
                    )),
                    None,
                ));
                return false;
            }
        };
        //now set uxp the actual module editing
        print_map_meta.before_at(Location::Module {
            func_idx: print_map_meta_id, // not used
            instr_idx: 0,
        });

        // for each of the report maps, emit the printing logic
        for (key, val) in map_meta_str.iter() {
            self.emitter.io_adapter.puts(
                format!("map,{key},{val},"),
                &mut print_map_meta,
                self.err,
            );

            // print the value(s) of this map
            self.emitter
                .map_lib_adapter
                .print_map(*key, &mut print_map_meta, self.err);
            self.emitter.io_adapter.putln(&mut print_map_meta, self.err);
        }
        true
    }
}
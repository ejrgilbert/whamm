use std::collections::HashMap;
use std::string;

use orca::ir::types::Value as OrcaValue;
use orca::{DataSegment, DataSegmentKind, InitExpr, Opcode};

use crate::common::error::ErrorGen;
use crate::emitter::report_var_metadata::convert_meta_to_string;
use crate::emitter::rewriting::module_emitter::StringAddr;
use crate::emitter::rewriting::rules::{provider_factory, Arg, LocInfo, WhammProvider};
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::rewriting::Emitter;
use crate::generator::simple_ast::{SimpleAST, SimpleProbe};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Expr, ProbeSpec, Statement};

const UNEXPECTED_ERR_MSG: &str =
    "InstrGenerator: Looks like you've found a bug...please report this behavior!";

fn get_loc_info<'a>(rule: &'a WhammProvider, emitter: &VisitingEmitter) -> Option<LocInfo<'a>> {
    // Pull the curr instr each time this is called to keep from having
    // long-lasting refs into self.emitter.
    emitter.get_loc_info(rule)
}

/// The second phase of instrumenting a Wasm module by actually emitting the
/// instrumentation code.
///
/// To do this, the generator traverses the BehaviorTree AST and calls the
/// passed emitter to emit instrumentation code.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the passed Emitter type.
pub struct InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f>,
    pub ast: SimpleAST,
    pub err: &'g mut ErrorGen,
    curr_instr_args: Vec<Arg>,
    curr_probe_mode: String,
    /// The current probe's body and predicate
    curr_probe: Option<(Option<Vec<Statement>>, Option<Expr>)>,
}
impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub fn new(
        emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f>,
        ast: SimpleAST,
        err: &'g mut ErrorGen,
    ) -> Self {
        Self {
            emitter,
            ast,
            err,
            curr_instr_args: vec![],
            curr_probe_mode: "____0".to_string(),
            curr_probe: None,
        }
    }

    pub fn configure_probe_mode(&mut self) -> bool {
        // TODO -- make the probe mode an enum!
        match self.curr_probe_mode.as_str() {
            "before" => self.emitter.before(),
            "after" => self.emitter.after(),
            "alt" => self.emitter.alternate(),
            _ => return false,
        }
        true
    }

    pub fn run(&mut self) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_children();

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
                        self.set_curr_probe(probe_spec, probe);
                        is_success = self
                            .emitter
                            .enter_scope_via_spec(&probe.script_id, probe_spec);

                        // Initialize the symbol table with the metadata at this program point
                        loc_info.static_data.iter().for_each(
                            |(static_var_name, static_var_val)| {
                                if let Err(e) = self.emitter.define(static_var_name, static_var_val)
                                {
                                    self.err.add_error(*e);
                                }
                            },
                        );

                        // Create a new clone of the probe, fold the predicate.
                        // NOTE: We make a clone so that the probe is reset for each instruction!
                        let (body_clone, mut pred_clone) =
                            (probe.body.clone(), probe.predicate.clone());
                        if let Some(pred) = &mut pred_clone {
                            // Fold predicate
                            is_success = self.emitter.fold_expr(pred);

                            // If the predicate evaluates to false, short-circuit!
                            if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
                                if !pred_as_bool {
                                    // predicate is reduced to false, short-circuit!
                                    return;
                                }
                            }
                        }

                        self.curr_instr_args = loc_info.args.clone(); // must clone so that this lives long enough
                        self.curr_probe_mode = probe_spec.mode.as_ref().unwrap().name.clone();
                        self.curr_probe = Some((body_clone, pred_clone));

                        // emit the probe (since the predicate is not false)
                        is_success &= self.emit_probe();

                        // Now that we've emitted this probe, reset the symbol table's static/dynamic
                        // data defined for this instr
                        self.emitter.reset_table_data(&loc_info);
                    });
                }
            });
        }
        // //after running, emit the metadata from the report_var_metadata into maps 0 and 1 in app_wasm
        // let ref report_var_metadata = self.emitter.report_var_metadata;
        // let ref mut module = self.emitter.app_iter.module;
        // let ref var_meta = report_var_metadata.variable_metadata;
        // let ref map_meta = report_var_metadata.map_metadata;
        // let mut var_meta_str: HashMap<i32, String> = HashMap::new();
        // let mut map_meta_str: HashMap<i32, String> = HashMap::new();
        // //convert the metadata into strings, add those to the data section, then use those to populate the maps
        // for (key, value) in var_meta.iter() {
        //     //first, emit the string to data section
        //     let val = convert_meta_to_string(value);
        //     let data_id = module.data.len();
        //         let val_bytes = val.as_bytes().to_owned();
        //         let data_segment = DataSegment {
        //             data: val_bytes,
        //             kind: DataSegmentKind::Active {
        //                 memory_index: self.emitter.mem_tracker.mem_id,
        //                 offset_expr: InitExpr::Value(OrcaValue::I32(
        //                     self.emitter.mem_tracker.curr_mem_offset as i32,
        //                 )),
        //             },
        //         };
        //         module.data.push(data_segment);
        //         // save the memory addresses/lens, so they can be used as appropriate
        //         self.emitter.mem_tracker.emitted_strings.insert(
        //             val.clone(),
        //             StringAddr {
        //                 data_id: data_id as u32,
        //                 mem_offset: self.emitter.mem_tracker.curr_mem_offset,
        //                 len: val.len(),
        //             },
        //         );
        //         // update curr_mem_offset to account for new data
        //     self.emitter.mem_tracker.curr_mem_offset += val.len();
        //     //now set the new key value for the new maps
        //     var_meta_str.insert(*key as i32, val);
        // }
        // for (key, value) in map_meta.iter() {
        //     //first, emit the string to data section
        //     let val = convert_meta_to_string(value);
        //     let data_id = module.data.len();
        //         let val_bytes = val.as_bytes().to_owned();
        //         let data_segment = DataSegment {
        //             data: val_bytes,
        //             kind: DataSegmentKind::Active {
        //                 memory_index: self.emitter.mem_tracker.mem_id,
        //                 offset_expr: InitExpr::Value(OrcaValue::I32(
        //                     self.emitter.mem_tracker.curr_mem_offset as i32,
        //                 )),
        //             },
        //         };
        //         module.data.push(data_segment);
        //         // save the memory addresses/lens, so they can be used as appropriate
        //         self.emitter.mem_tracker.emitted_strings.insert(
        //             val.clone(),
        //             StringAddr {
        //                 data_id: data_id as u32,
        //                 mem_offset: self.emitter.mem_tracker.curr_mem_offset,
        //                 len: val.len(),
        //             },
        //         );
        //         // update curr_mem_offset to account for new data
        //     self.emitter.mem_tracker.curr_mem_offset += val.len();
        //     //now set the new key value for the new maps
        //     map_meta_str.insert(*key as i32, val);
        // }
        // //first, we need to create the maps in _start
        // let start_id = match module.get_fid_by_name("__wasm_call_ctors") {
        //     Some(start_id) => start_id,
        //     None => {
        //         self.err.add_error(ErrorGen::get_unexpected_error(
        //             true,
        //             Some(format!(
        //                 "{UNEXPECTED_ERR_MSG} \
        //             No start function found in the module!"
        //             )),
        //             None,
        //         ));
        //         return false;
        //     }
        // };
        // let mut start_fn = match module.get_fn(start_id - module.num_import_func()) {
        //     Some(start_fn) => start_fn,
        //     None => {
        //         self.err.add_error(ErrorGen::get_unexpected_error(
        //             true,
        //             Some(format!(
        //                 "{UNEXPECTED_ERR_MSG} \
        //             No start function found in the module!"
        //             )),
        //             None,
        //         ));
        //         return false;
        //     }
        // };
        // //now set up the actual module editing
        // start_fn.before_at(0);
        // let create_i32_string = match self.emitter.map_lib_adapter.create_map_insert(DataType::I32, DataType::Str) {
        //     Ok(string) => string,
        //     Err(e) => {
        //         self.err.add_error(*e);
        //         return false;
        //     }
        // };
        // let mut to_call = self.emitter.table
        //     .lookup(&create_i32_string)
        //     .expect("Map function not in symbol table")
        //     .clone(); //clone to close the borrow
        // //now create the maps
        // start_fn.i32_const(0);
        // start_fn.call(to_call as u32);
        // start_fn.i32_const(1);
        // start_fn.call(to_call as u32);
        // //set "to_call" to the insert function
        // to_call = self.emitter.table
        //     .lookup(&"insert_i32_string".to_string())
        //     .expect("Map function not in symbol table")
        //     .clone(); //clone to close the borrow

        // //now, for each of the maps, emit the correct stuff
        // for (key, val) in var_meta_str.iter() {
        //     if let Some(val_addr) = self.emitter.mem_tracker.emitted_strings.get(val) {
        //         //now, emit the map entry
        //         start_fn.i32_const(0);
        //         start_fn.i32_const(*key as i32);
        //         start_fn.i32_const(val_addr.mem_offset as i32);
        //         start_fn.i32_const(val_addr.len as i32);
        //         start_fn.call(to_call as u32);
        //     } else {
        //         self.err.add_error(ErrorGen::get_unexpected_error(
        //             true,
        //             Some(format!(
        //                 "Failed to find emitted string for metadata with key: {}",
        //                 key
        //             )),
        //             None,
        //         ));
        //     }
        // }
        // for (key, val) in map_meta_str.iter() {
        //     if let Some(val_addr) = self.emitter.mem_tracker.emitted_strings.get(val) {
        //         //now, emit the map entry
        //         start_fn.i32_const(0);
        //         start_fn.i32_const(*key as i32);
        //         start_fn.i32_const(val_addr.mem_offset as i32);
        //         start_fn.i32_const(val_addr.len as i32);
        //         start_fn.call(to_call as u32);
        //     } else {
        //         self.err.add_error(ErrorGen::get_unexpected_error(
        //             true,
        //             Some(format!(
        //                 "Failed to find emitted string for metadata with key: {}",
        //                 key
        //             )),
        //             None,
        //         ));
        //     }
        // }
        is_success
    }
    fn set_curr_probe(&mut self, probe_spec: &ProbeSpec, probe: &&SimpleProbe) {
        self.emitter.curr_script_id = probe.script_id.clone();
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
            Some(mode) => mode.name.clone(),
            None => "".to_string(),
        };
        let mut curr_probe_id = format!(
            "{}_{}_{}_{}",
            curr_provider, curr_package, curr_event, curr_mode
        );
        let mut emitter_probe_id = self.emitter.curr_probe_id.clone();
        if emitter_probe_id.is_empty() || curr_probe_id.is_empty() {
            emitter_probe_id = curr_probe_id + "0";
        } else {
            //remove the last chars while they are digits, then add 1 and put it back
            let mut running_count = 0;
            let mut last_digit = emitter_probe_id.pop().unwrap();
            while last_digit.is_digit(10) {
                running_count = running_count * 10 + last_digit.to_digit(10).unwrap();
                last_digit = emitter_probe_id.pop().unwrap();
            }
            running_count += 1;
            curr_probe_id.push_str(&running_count.to_string());
            emitter_probe_id = curr_probe_id;
        }
        self.emitter.curr_probe_id = emitter_probe_id;
    }
}
impl InstrGenerator<'_, '_, '_, '_, '_, '_, '_> {
    fn emit_probe(&mut self) -> bool {
        let mut is_success = true;

        is_success &= self.save_args();

        self.configure_probe_mode();
        if self.pred_is_true() {
            // The predicate has been reduced to a 'true', emit un-predicated body
            self.emit_body();
            // Place the original arguments back on the stack.
            if let Err(e) = self.emitter.emit_args() {
                self.err.add_error(*e);
                return false;
            }
        } else {
            // The predicate still has some conditionals (remember we already checked for
            // it being false in run() above)
            match self.curr_probe_mode.as_str() {
                "before" => {
                    is_success &= self.emit_probe_as_if();
                    // Place the original arguments back on the stack.
                    if let Err(e) = self.emitter.emit_args() {
                        self.err.add_error(*e);
                        return false;
                    }
                }
                "after" => {
                    is_success &= self.emit_probe_as_if();
                }
                "alt" => {
                    is_success &= self.emit_probe_as_if_else();
                }
                _ => {
                    self.err.unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} Unexpected probe mode '{}'",
                            self.curr_probe_mode
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
        if let Some((Some(ref mut body), ..)) = self.curr_probe {
            match self.emitter.emit_body(body) {
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

    fn emit_probe_as_if(&mut self) -> bool {
        if let Some((Some(ref mut body), Some(ref mut pred))) = self.curr_probe {
            match self.emitter.emit_if(pred, body) {
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
            match self.emitter.emit_if_with_orig_as_else(pred, body) {
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
}

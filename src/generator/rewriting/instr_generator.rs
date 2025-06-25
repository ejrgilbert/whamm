use crate::api::instrument::Config;
use crate::common::error::ErrorGen;
use crate::emitter::rewriting::rules::{Arg, LocInfo, ProbeRule};
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::emitter::Emitter;
use crate::generator::ast::Probe;
use crate::generator::folding::ExprFolder;
use crate::generator::rewriting::simple_ast::SimpleAST;
use crate::lang_features::report_vars::{BytecodeLoc, LocationData};
use crate::parser::provider_handler::ModeKind;
use crate::parser::types::{Block, Expr, Location};
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::iterator::iterator_trait::{IteratingInstrumenter, Iterator};
use orca_wasm::opcode::Instrumenter;
use orca_wasm::{Location as OrcaLocation, Opcode};
use std::collections::HashMap;
use std::iter::Iterator as StdIter;

const UNEXPECTED_ERR_MSG: &str =
    "InstrGenerator: Looks like you've found a bug...please report this behavior!";

fn emit_dynamic_compiler_data(
    data: &HashMap<String, Block>,
    emitter: &mut VisitingEmitter,
    err: &mut ErrorGen,
) {
    emitter.emit_dynamic_compiler_data(data, err);
}

fn add_to_table(info: &LocInfo, emitter: &mut VisitingEmitter) {
    // define static data
    info.static_data
        .iter()
        .for_each(|(dyn_var_name, dyn_var_val)| {
            emitter.define_data(dyn_var_name, dyn_var_val);
        });
    // define dynamic aliases
    info.dynamic_alias
        .iter()
        .for_each(|(var_name, (ty, addr))| {
            emitter.define_alias(var_name, ty, addr);
        })
}

/// The second phase of instrumenting a Wasm module by actually emitting the
/// instrumentation code.
///
/// To do this, the generator traverses the BehaviorTree AST and calls the
/// passed emitter to emit instrumentation code.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the passed Emitter type.
pub struct InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    pub emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
    pub ast: SimpleAST,
    pub err: &'h mut ErrorGen,
    curr_instr_args: Vec<Arg>,
    curr_probe_mode: ModeKind,
    curr_probe_loc: Option<Location>,
    /// The current probe's body and predicate
    curr_probe: Option<(Option<Block>, Option<Expr>)>,

    /// Whether there are reports to flush at the end of execution
    has_reports: bool,
    on_exit_fid: Option<u32>,
    config: &'i Config,
}
impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    pub fn new(
        emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
        ast: SimpleAST,
        err: &'h mut ErrorGen,
        config: &'i Config,
        has_reports: bool,
    ) -> Self {
        Self {
            emitter,
            ast,
            err,
            curr_instr_args: vec![],
            curr_probe_mode: ModeKind::Before,
            curr_probe_loc: None,
            curr_probe: None,
            has_reports,
            on_exit_fid: None,
            config,
        }
    }

    pub fn configure_probe_mode(&mut self) -> bool {
        // function entry/exit should be handled before this point!
        match self.curr_probe_mode {
            ModeKind::Before => self.emitter.before(),
            ModeKind::After => self.emitter.after(),
            ModeKind::Alt => self.emitter.alternate(),
            ModeKind::SemanticAfter => self.emitter.semantic_after(),
            ModeKind::Entry => self.emitter.block_entry(),
            ModeKind::Exit => self.emitter.block_exit(),
            ModeKind::BlockAlt => self.emitter.block_alt(),
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
        //       1. enter the scope using (script_id, probe_rule)
        //       2. initialize the symbol table with the metadata at this program point
        //       3. create a new clone of the probe, fold the predicate
        //       4. traverse the behavior tree to emit code! (if predicate is not false)

        // Initialize the emitter rules
        // let rules = provider_factory::<WhammProvider>(&self.ast.probes);

        // Iterate over each instruction in the application Wasm bytecode
        let mut is_success = true;
        let mut first_instr = true;
        while first_instr || self.emitter.next_instr() {
            first_instr = false;
            // Check if any of the configured rules match this instruction in the application.
            if let Some(loc_info) = self.emitter.get_loc_info(&self.ast) {
                // Inject a call to the on-exit flush function
                if loc_info.is_prog_exit {
                    if self.on_exit_fid.is_none() {
                        let on_exit = FunctionBuilder::new(&[], &[]);
                        let on_exit_id = on_exit.finish_module_with_tag(
                            self.emitter.app_iter.module,
                            get_tag_for(&None),
                        );
                        self.emitter
                            .app_iter
                            .module
                            .set_fn_name(on_exit_id, "on_exit".to_string());

                        self.on_exit_fid = Some(*on_exit_id);
                    }

                    if let Some(fid) = self.on_exit_fid {
                        self.emitter.before();
                        self.emitter.app_iter.call(FunctionID(fid));
                    } else {
                        panic!("something went horribly wrong")
                    }
                }

                if loc_info.num_alt_probes > 1 {
                    self.err
                        .multiple_alt_matches(self.emitter.curr_instr_name().as_str());
                }
                // This location has matched some rules, inject each matched probe!
                for (probe_rule, probe) in loc_info.probes.iter() {
                    // Enter the scope for this matched probe
                    self.set_curr_loc(probe_rule, probe);
                    assert!(
                        self.emitter
                            .enter_scope_via_rule(&probe.script_id.to_string(), probe_rule),
                        "Failed to enter scope: {}",
                        probe_rule
                    );

                    // Initialize the symbol table with the metadata at this program point
                    add_to_table(&loc_info, &mut self.emitter);

                    // Create a new clone of the probe, fold the predicate.
                    // NOTE: We make a clone so that the probe is reset for each instruction!
                    let (body_clone, mut pred_clone, loc_clone) = (
                        probe.body.clone(),
                        probe.predicate.clone(),
                        probe.loc.clone(),
                    );
                    if let Some(pred) = &mut pred_clone {
                        // Fold predicate
                        is_success = self.emitter.fold_expr(pred, self.err);

                        // If the predicate evaluates to false, short-circuit!
                        if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
                            if !pred_as_bool {
                                // predicate is reduced to false, short-circuit!
                                continue;
                            }
                        }
                    }

                    self.curr_instr_args = loc_info.args.clone(); // must clone so that this lives long enough
                    self.curr_probe_mode = probe_rule.mode.as_ref().unwrap().clone();
                    self.curr_probe_loc = loc_clone;
                    self.curr_probe = Some((body_clone, pred_clone));

                    if !self.config.no_bundle {
                        // since we're only supporting 'no_bundle' when 'no_body' and 'no_pred' are also true
                        // we can simplify the check to just not emitting the probe altogether

                        // emit the probe (since the predicate is not false)
                        is_success &= self.emit_probe(&loc_info.dynamic_data);
                    }

                    // Now that we've emitted this probe, reset the symbol table's static/dynamic
                    // data defined for this instr
                    self.emitter.reset_table_data(&loc_info);
                }
            };
        }
        is_success &= self.after_run();
        is_success
    }
    fn set_curr_loc(&mut self, probe_rule: &ProbeRule, probe: &Probe) {
        let curr_script_id = probe.script_id;
        // todo -- this clone is bad
        self.emitter.curr_unshared = probe.unshared_to_alloc.clone();
        let probe_rule_str = probe_rule.to_string();
        let curr_probe_id = format!("{}_{}", probe.probe_number, probe_rule_str);
        let (loc, new_fid) = match self.emitter.app_iter.curr_loc().0 {
            OrcaLocation::Module {
                func_idx,
                instr_idx,
                ..
            }
            | OrcaLocation::Component {
                func_idx,
                instr_idx,
                ..
            } => (BytecodeLoc::new(*func_idx, instr_idx as u32), *func_idx),
        };

        if let LocationData::Local {
            bytecode_loc: BytecodeLoc { fid: prev_fid, .. },
            ..
        } = self.emitter.report_vars.curr_location
        {
            if prev_fid != new_fid {
                // we're now visiting a new function! reset the locals!
                self.emitter.reset_locals_for_function();
            }
        };

        //set the current location in bytecode and load some new globals for potential report vars
        self.emitter.report_vars.curr_location = LocationData::Local {
            script_id: curr_script_id,
            bytecode_loc: loc,
            probe_id: curr_probe_id,
        };
    }
}
impl InstrGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    fn emit_probe(&mut self, dynamic_data: &HashMap<String, Block>) -> bool {
        let mut is_success = true;

        is_success &= self.save_args();
        //after saving args, we run the check if we need to initialize global maps
        self.emitter.inject_map_init();
        self.configure_probe_mode();

        // Now we know we're going to insert the probe, let's define
        // the dynamic information
        emit_dynamic_compiler_data(dynamic_data, &mut self.emitter, self.err);
        if self.pred_is_true() {
            // The predicate has been reduced to a 'true', emit un-predicated body
            if !self.config.no_body {
                // Only emit the body if we're configured to do so
                self.emit_body();
            }
            if !matches!(self.curr_probe_mode, ModeKind::Alt) {
                self.replace_args();
            }
        } else {
            // The predicate still has some conditionals (remember we already checked for
            // it being false in run() above)
            match self.curr_probe_mode {
                ModeKind::Before
                | ModeKind::After
                | ModeKind::SemanticAfter
                | ModeKind::Entry
                | ModeKind::Exit => {
                    is_success &= self.emit_probe_as_if();
                    self.replace_args();
                }
                ModeKind::Alt => {
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
        self.emitter.reset_locals_for_probe();

        let op_idx = self.emitter.app_iter.curr_instr_len() as u32;
        self.emitter
            .app_iter
            .append_to_tag(get_probe_tag_data(&self.curr_probe_loc, op_idx));

        is_success
    }

    fn save_args(&mut self) -> bool {
        if !self.curr_instr_args.is_empty() {
            // The current instruction has args, save them (before)
            self.emitter.before();
            self.emitter.save_args(&self.curr_instr_args, self.err)
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
                if self.curr_probe_mode == ModeKind::Alt {
                    match self.emitter.emit_empty_alternate() {
                        Err(e) => {
                            self.err.add_error(*e);
                            false
                        }
                        Ok(res) => res,
                    }
                } else if self.curr_probe_mode == ModeKind::BlockAlt {
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
            match (self.config.no_body, self.config.no_pred) {
                // emit as normal
                (false, false) => {
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
                }
                // emit an unpredicated body
                (false, true) => self.emit_body(),
                // emit empty if block
                (true, false) => match self.emitter.emit_if(
                    &self.curr_instr_args,
                    pred,
                    &mut Block::default(),
                    self.err,
                ) {
                    Err(e) => {
                        self.err.add_error(*e);
                        false
                    }
                    Ok(res) => res,
                },
                // emit nothing
                (true, true) => true,
            }
        } else {
            false
        }
    }

    fn emit_probe_as_if_else(&mut self) -> bool {
        if let Some((Some(ref mut body), Some(ref mut pred))) = self.curr_probe {
            match (self.config.no_body, self.config.no_pred) {
                // normal
                (false, false) => match self.emitter.emit_if_with_orig_as_else(
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
                },
                // unpredicated body
                (false, true) => self.emit_body(),
                // empty if stmt
                (true, false) => match self.emitter.emit_if_with_orig_as_else(
                    &self.curr_instr_args,
                    pred,
                    &mut Block::default(),
                    self.err,
                ) {
                    Err(e) => {
                        self.err.add_error(*e);
                        false
                    }
                    Ok(res) => res,
                },
                // emit nothing
                (true, true) => true,
            }
        } else {
            false
        }
    }
    fn after_run(&mut self) -> bool {
        if !self.config.no_report {
            self.emitter
                .configure_flush_routines(self.has_reports, self.err);
        }
        true
    }
}

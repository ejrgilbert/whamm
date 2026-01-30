use crate::api::instrument::Config;
use crate::common::error::ErrorGen;
use crate::emitter::Emitter;
use crate::emitter::rewriting::rules::{LocInfo, MatchState, ProbeRule, StackVal};
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::generator::ast::Probe;
use crate::generator::folding::expr::ExprFolder;
use crate::generator::rewriting::simple_ast::SimpleAST;
use crate::lang_features::report_vars::{BytecodeLoc, LocationData};
use crate::parser::provider_handler::ModeKind;
use crate::parser::types::{Block, Expr, Location, Statement};
use std::collections::HashMap;
use std::iter::Iterator as StdIter;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::FunctionID;
use wirm::ir::types::InstrumentationMode;
use wirm::iterator::iterator_trait::{IteratingInstrumenter, Iterator};
use wirm::opcode::Instrumenter;
use wirm::{Location as WirmLocation, Opcode};

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
pub struct InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l> {
    pub emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>,
    pub ast: SimpleAST,
    pub err: &'k mut ErrorGen,
    curr_instr_args: Vec<StackVal>,
    curr_instr_results: Vec<StackVal>,
    curr_probe_rule: ProbeRule,
    is_prog_exit: bool,
    curr_probe_loc: Option<Location>,
    /// The current probe's body and predicate
    curr_probe: Option<(Vec<Statement>, Option<Block>, Option<Expr>)>,

    /// Whether there are reports to flush at the end of execution
    has_reports: bool,
    on_exit_fid: Option<u32>,
    config: &'l Config,
}
impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l>
    InstrGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l>
{
    pub fn new(
        emitter: VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>,
        ast: SimpleAST,
        err: &'k mut ErrorGen,
        config: &'l Config,
        has_reports: bool,
    ) -> Self {
        Self {
            emitter,
            ast,
            err,
            curr_instr_args: vec![],
            curr_instr_results: vec![],
            curr_probe_rule: ProbeRule::default(),
            is_prog_exit: false,
            curr_probe_loc: None,
            curr_probe: None,
            has_reports,
            on_exit_fid: None,
            config,
        }
    }

    pub fn configure_probe_mode(&mut self, mode_override: &Option<InstrumentationMode>) -> bool {
        self.probe_mode_for_provider(mode_override)
    }
    fn probe_mode_for_provider(&mut self, mode_override: &Option<InstrumentationMode>) -> bool {
        match &self.curr_probe_rule.provider {
            Some(prov) => match prov.name.as_str() {
                "wasm" => self.probe_mode_for_package(mode_override),
                _ => unreachable!("Invalid probe provider: {}", prov.name),
            },
            _ => unreachable!("Probe does not have a provider."),
        }
    }
    fn probe_mode_for_package(&mut self, mode_override: &Option<InstrumentationMode>) -> bool {
        match &self.curr_probe_rule.package {
            Some(prov) => match prov.name.as_str() {
                "opcode" => self
                    .probe_mode_by_whamm_mode(&self.curr_probe_rule.mode.as_ref().unwrap().clone()),
                "func" => self.probe_mode_for_func_modes(&ModeKind::from(
                    self.curr_probe_rule.event.as_ref().unwrap().name.clone(),
                )),
                "block" => {
                    if let Some(mode) = mode_override {
                        self.probe_mode_by_wirm_opcode_mode(mode);
                    } else {
                        panic!("should have had a wirm instrumentation mode for wasm:block:*");
                    }
                    true
                }
                _ => unreachable!("Invalid probe provider: {}", prov.name),
            },
            _ => unreachable!("Probe does not have a provider."),
        }
    }
    fn probe_mode_by_whamm_mode(&mut self, mode: &ModeKind) -> bool {
        match mode {
            ModeKind::Before => self.emitter.before(),
            ModeKind::After => self.emitter.after(),
            ModeKind::Alt => self.emitter.alternate(),
            ModeKind::SemanticAfter => self.emitter.semantic_after(),
            ModeKind::Entry => self.emitter.block_entry(),
            ModeKind::Exit => self.emitter.block_exit(),
            ModeKind::BlockAlt => self.emitter.block_alt(),
            _ => unreachable!("invalid probe mode: {}", mode),
        }
        true
    }
    fn probe_mode_by_wirm_opcode_mode(&mut self, mode: &InstrumentationMode) -> bool {
        match mode {
            InstrumentationMode::Before => self.emitter.before(),
            InstrumentationMode::After => self.emitter.after(),
            InstrumentationMode::Alternate => self.emitter.alternate(),
            InstrumentationMode::SemanticAfter => self.emitter.semantic_after(),
            InstrumentationMode::BlockEntry => self.emitter.block_entry(),
            InstrumentationMode::BlockExit => self.emitter.block_exit(),
            InstrumentationMode::BlockAlt => self.emitter.block_alt(),
        }
        true
    }
    fn probe_mode_for_func_modes(&mut self, mode: &ModeKind) -> bool {
        match mode {
            ModeKind::Entry => self.emitter.func_entry(),
            ModeKind::Exit => {
                if self.is_prog_exit {
                    // if we're at the program exit (e.g. a wasi:exiting call),
                    // we want to do something slightly different. Inject a before
                    // at this location
                    self.emitter.before()
                } else {
                    self.emitter.func_exit()
                }
            }
            _ => unreachable!("invalid func mode: {}", mode),
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
        let mut match_state = MatchState::default();
        while first_instr || self.emitter.next_instr() {
            first_instr = false;
            // Check if any of the configured rules match this instruction in the application.
            if let Some(loc_info) = self.emitter.get_loc_info(&mut match_state, &mut self.ast) {
                // Inject a call to the on-exit flush function
                self.is_prog_exit = loc_info.is_prog_exit;
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
                        let op_idx = self.emitter.app_iter.curr_instr_len() as u32;
                        // this is for Whamm reporting, not tied to this probe specifically
                        self.emitter
                            .app_iter
                            .append_to_tag(get_probe_tag_data(&None, op_idx));
                    } else {
                        panic!("something went horribly wrong")
                    }
                }

                if loc_info.num_alt_probes > 1 {
                    self.err
                        .multiple_alt_matches(self.emitter.curr_instr_name().as_str());
                }
                // This location has matched some rules, inject each matched probe!
                for (probe_rule, probe, mode) in loc_info.probes.iter() {
                    // Enter the scope for this matched probe
                    self.set_curr_loc(probe_rule, probe);

                    // enter mode scope
                    assert!(
                        self.emitter.enter_scope_via_rule(
                            &probe.script_id.to_string(),
                            probe_rule,
                            probe.scope_id
                        ),
                        "Failed to enter scope: {}",
                        probe_rule
                    );
                    self.emitter
                        .table
                        .enter_named_scope(&probe.probe_number.to_string()); // enter probe scope

                    // Initialize the symbol table with the metadata at this program point
                    add_to_table(&loc_info, &mut self.emitter);

                    // Create a new clone of the probe, fold the predicate.
                    // NOTE: We make a clone so that the probe is reset for each instruction!
                    let (state_init_clone, body_clone, mut pred_clone, loc_clone) = (
                        probe.init_logic.clone(),
                        probe.body.clone(),
                        probe.predicate.clone(),
                        probe.loc.clone(),
                    );
                    if let Some(pred) = &mut pred_clone {
                        // Fold predicate
                        is_success = self.emitter.fold_expr(pred, self.err);

                        // If the predicate evaluates to false, short-circuit!
                        if let Some(pred_as_bool) =
                            ExprFolder::get_single_bool(pred, self.emitter.registry, false)
                        {
                            if !pred_as_bool {
                                // predicate is reduced to false, short-circuit!
                                continue;
                            }
                        }
                    }

                    self.curr_instr_args = loc_info.args.clone(); // must clone so that this lives long enough
                    self.curr_instr_results = loc_info.results.clone(); // must clone so that this lives long enough
                    self.curr_probe_rule = probe_rule.clone();
                    self.curr_probe_loc = loc_clone;
                    self.curr_probe = Some((state_init_clone, body_clone, pred_clone));

                    if !self.config.no_bundle {
                        // since we're only supporting 'no_bundle' when 'no_body' and 'no_pred' are also true
                        // we can simplify the check to just not emitting the probe altogether

                        // emit the probe (since the predicate is not false)
                        is_success &= self.emit_probe(&loc_info.dynamic_data, mode);
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
        let loc = self.emitter.app_iter.curr_loc().0;
        let (loc, new_fid) = match loc {
            WirmLocation::Module { func_idx, .. } | WirmLocation::Component { func_idx, .. } => (
                BytecodeLoc::new(
                    *func_idx,
                    VisitingEmitter::lookup_pc_offset_for(self.emitter.app_iter.module, &loc),
                ),
                *func_idx,
            ),
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
impl InstrGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    fn emit_probe(
        &mut self,
        dynamic_data: &HashMap<String, Block>,
        mode_override: &Option<InstrumentationMode>,
    ) -> bool {
        let mut is_success = true;

        is_success &= self.save_args();
        if matches!(self.curr_probe_rule.mode.as_ref().unwrap(), ModeKind::After) {
            self.emitter.after();
            is_success &= self.save_results();
        }
        self.configure_probe_mode(mode_override);

        // Now we know we're going to insert the probe, let's define
        // the dynamic information
        emit_dynamic_compiler_data(dynamic_data, &mut self.emitter, self.err);
        if self.pred_is_true() {
            // The predicate has been reduced to a 'true', emit un-predicated body
            if matches!(self.curr_probe_rule.mode.as_ref().unwrap(), ModeKind::After) {
                self.replace_args();
                self.emitter.after();
            }
            if !self.config.no_body {
                // Only emit the body if we're configured to do so
                self.emit_body();
            }
            if !matches!(
                self.curr_probe_rule.mode.as_ref().unwrap(),
                ModeKind::Alt | ModeKind::After
            ) {
                self.replace_args();
            }
            if matches!(self.curr_probe_rule.mode.as_ref().unwrap(), ModeKind::After) {
                is_success &= self.replace_results();
            }
        } else {
            let curr_probe_mode = self.curr_probe_rule.mode.as_ref().unwrap();
            // The predicate still has some conditionals (remember we already checked for
            // it being false in run() above)
            match curr_probe_mode {
                ModeKind::Before
                | ModeKind::SemanticAfter
                | ModeKind::Entry
                | ModeKind::Exit
                | ModeKind::Null => {
                    is_success &= self.emit_probe_as_if();
                    self.replace_args();
                }
                ModeKind::After => {
                    self.emitter.before();
                    self.replace_args();
                    self.emitter.after();
                    is_success &= self.emit_probe_as_if();
                    self.replace_results();
                }
                ModeKind::Alt => {
                    is_success &= self.emit_probe_as_if_else();
                }
                _ => {
                    unreachable!(
                        "{} Unexpected probe mode '{}'",
                        UNEXPECTED_ERR_MSG,
                        curr_probe_mode.name()
                    );
                }
            }
        }
        self.emitter.reset_locals_for_probe();

        let op_idx = self.emitter.app_iter.curr_instr_len() as u32;
        self.emitter
            .app_iter
            .append_to_tag(get_probe_tag_data(&self.curr_probe_loc, op_idx));
        self.emitter.finish_instr();

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

    fn save_results(&mut self) -> bool {
        if !self.curr_instr_results.is_empty() {
            // The current instruction has results, save them (after)
            self.emitter.after();
            self.emitter.save_results(&self.curr_instr_results)
        } else {
            // If no results, just return true
            true
        }
    }
    fn replace_results(&mut self) -> bool {
        // Place the original arguments back on the stack.
        self.emitter.after();
        self.emitter.emit_results(self.err)
    }

    fn pred_is_true(&mut self) -> bool {
        if let Some((.., pred)) = &self.curr_probe {
            if let Some(pred) = pred {
                if let Some(pred_as_bool) =
                    ExprFolder::get_single_bool(pred, self.emitter.registry, false)
                {
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
        if let Some((state_init, body, ..)) = &mut self.curr_probe {
            if let Some(body) = body {
                self.emitter.init_probe_state(state_init, self.err);
                self.emitter.emit_body(body, self.err)
            } else {
                let curr_probe_mode = self.curr_probe_rule.mode.as_ref().unwrap();
                if matches!(curr_probe_mode, ModeKind::Alt) {
                    match self.emitter.emit_empty_alternate() {
                        Err(e) => {
                            self.err.add_error(*e);
                            false
                        }
                        Ok(res) => res,
                    }
                } else if matches!(curr_probe_mode, ModeKind::BlockAlt) {
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
            }
        } else {
            false
        }
    }

    fn emit_probe_as_if(&mut self) -> bool {
        if let Some((state_init, Some(body), Some(pred))) = &mut self.curr_probe {
            match (self.config.no_body, self.config.no_pred) {
                // emit as normal
                (false, false) => {
                    self.emitter.init_probe_state(state_init, self.err);
                    match self.emitter.emit_if(pred, body, self.err) {
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
                (true, false) => {
                    match self.emitter.emit_if(pred, &mut Block::default(), self.err) {
                        Err(e) => {
                            self.err.add_error(*e);
                            false
                        }
                        Ok(res) => res,
                    }
                }
                // emit nothing
                (true, true) => true,
            }
        } else {
            false
        }
    }

    fn emit_probe_as_if_else(&mut self) -> bool {
        if let Some((state_init, Some(body), Some(pred))) = &mut self.curr_probe {
            match (self.config.no_body, self.config.no_pred) {
                // normal
                (false, false) => {
                    self.emitter.init_probe_state(state_init, self.err);
                    match self.emitter.emit_if_with_orig_as_else(pred, body, self.err) {
                        Err(e) => {
                            self.err.add_error(*e);
                            false
                        }
                        Ok(res) => res,
                    }
                }
                // unpredicated body
                (false, true) => self.emit_body(),
                // empty if stmt
                (true, false) => match self.emitter.emit_if_with_orig_as_else(
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
                .configure_flush_routines(self.has_reports, self.err, &mut self.ast);
        }
        true
    }
}

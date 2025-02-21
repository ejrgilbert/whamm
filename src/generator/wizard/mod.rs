pub mod ast;
pub mod metadata_collector;

use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::emitter::memory_allocator::VAR_BLOCK_BASE_VAR;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::generator::wizard::ast::{UnsharedVar, WizardProbe, WizardScript};
use crate::generator::GeneratingVisitor;
use crate::lang_features::alloc_vars::wizard::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::report_vars::{BytecodeLoc, LocationData};
use crate::parser::types::{
    Block, DataType, Definition, Expr, FnId, Statement, Value, WhammVisitorMut,
};
use crate::verifier::types::{Record, VarAddr};
use log::trace;
use orca_wasm::ir::id::{FunctionID, LocalID};
use orca_wasm::ir::types::DataType as OrcaType;
use std::collections::{HashMap, HashSet};

pub struct WizardGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
    pub io_adapter: &'h mut IOAdapter,
    pub context_name: String,
    pub err: &'i mut ErrorGen,
    pub injected_funcs: &'j mut Vec<FunctionID>,
    pub config: &'k Config,

    // tracking
    pub curr_script_id: u8,
    pub unshared_var_handler: &'l mut UnsharedVarHandler,
}

impl WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(
        &mut self,
        ast: Vec<WizardScript>,
        used_provided_funcs: HashSet<(String, String)>,
        used_report_dts: HashSet<DataType>,
        strings_to_emit: Vec<String>,
    ) {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.emitter.setup_module(self.err);
        self.emit_needed_funcs(used_provided_funcs);
        self.emit_strings(strings_to_emit);
        self.visit_ast(ast);

        self.emit_end_func(used_report_dts);
    }

    fn emit_strings(&mut self, strings_to_emit: Vec<String>) {
        for string in strings_to_emit.iter() {
            self.emitter.emit_string(
                &mut Value::Str {
                    val: string.clone(),
                },
                self.err,
            );
        }
    }

    fn emit_needed_funcs(&mut self, funcs: HashSet<(String, String)>) {
        for (context, fname) in funcs.iter() {
            if let Some(fid) = self.emitter.emit_provided_fn(
                context,
                &crate::parser::types::Fn {
                    def: Definition::CompilerDynamic,
                    name: FnId {
                        name: fname.clone(),
                        loc: None,
                    },
                    params: vec![],
                    return_ty: DataType::Boolean,
                    body: Default::default(),
                },
                self.err,
            ) {
                self.add_injected_func(fid);
            };
        }
    }

    fn emit_end_func(&mut self, used_report_dts: HashSet<DataType>) {
        self.emitter
            .emit_end_fn(used_report_dts, self.io_adapter, self.err);
    }

    // Visit the AST
    fn visit_ast(&mut self, mut ast: Vec<WizardScript>) {
        for script in ast.iter_mut() {
            self.visit_wiz_script(script);
        }
    }

    fn visit_wiz_script(&mut self, script: &mut WizardScript) {
        trace!("Entering: CodeGenerator::visit_script");
        self.enter_named_scope(&script.id.to_string());
        self.set_context_name(script.id.to_string());
        self.curr_script_id = script.id;

        self.set_curr_loc(LocationData::Global {
            script_id: script.id,
        });

        // visit fns
        script.fns.iter_mut().for_each(|f| {
            self.visit_fn(f);
        });
        // inject globals
        self.visit_globals(&script.globals);
        // visit probes
        script.probes.iter_mut().for_each(|probe| {
            self.visit_wiz_probe(probe);
        });

        trace!("Exiting: CodeGenerator::visit_script");
        self.exit_scope();
    }

    fn emit_special_func(
        &mut self,
        // the base memory offset for this function's var block
        alloc_base: Option<LocalID>,
        param_reqs: &HashSet<(String, DataType)>,
        dynamic_pred: Option<&mut Expr>,
        results: &[OrcaType],
        body: &mut Block,
        export: bool,
    ) -> (Option<u32>, String) {
        // create the function
        let mut params = vec![];
        let mut param_str = "".to_string();

        // handle $alloc param (if there are unshared vars)
        if let Some(alloc) = alloc_base {
            params.push(OrcaType::I32);
            // add param definition to the symbol table
            self.emitter.table.put(
                VAR_BLOCK_BASE_VAR.to_string(),
                Record::Var {
                    ty: DataType::I32,
                    name: VAR_BLOCK_BASE_VAR.to_string(),
                    value: None,
                    def: Definition::CompilerStatic,
                    is_report_var: false,
                    addr: Some(VarAddr::Local { addr: *alloc }),
                    loc: None,
                },
            );
        }

        // handle the parameters
        for (param_name, param_ty) in param_reqs.iter() {
            let local_id = params.len() as u32;
            // handle param list
            params.extend(param_ty.to_wasm_type());

            // handle the param string
            if !param_str.is_empty() {
                param_str += ", "
            }
            param_str += param_name;

            // add param definition to the symbol table
            self.emitter.table.put(
                param_name.clone(),
                Record::Var {
                    ty: param_ty.clone(),
                    name: param_name.clone(),
                    value: None,
                    def: Definition::CompilerStatic,
                    is_report_var: false,
                    addr: Some(VarAddr::Local { addr: local_id }),
                    loc: None,
                },
            );
        }

        let fid = self.emitter.emit_special_fn(
            None,
            &params,
            dynamic_pred,
            results,
            body,
            export,
            self.err,
        );

        (fid, param_str.to_string())
    }

    fn create_curr_loc(&self, probe: &WizardProbe) -> LocationData {
        let probe_id = format!("{}_{}", probe.probe_number, probe.rule);

        // translate wizard unshared vars to the correct format
        let mut vars = HashMap::default();
        for UnsharedVar { ty, .. } in probe.unshared_to_alloc.iter() {
            vars.entry(ty.clone())
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        }

        //set the current location in bytecode and load some new globals for potential report vars
        LocationData::Local {
            script_id: self.curr_script_id,
            bytecode_loc: BytecodeLoc::new(0, 0), // TODO -- request this from wizard
            probe_id,
            unshared: vars, //this is still used in the emitter to determine the new globals to emit
        }
    }

    fn visit_wiz_probe(&mut self, probe: &mut WizardProbe) {
        self.set_curr_loc(self.create_curr_loc(probe));

        let (pred_fid, pred_param_str, dynamic_pred) = if let Some(pred) = &mut probe.predicate {
            if probe.metadata.pred_is_dynamic {
                // dynamic analysis of the predicate will go here!
                // See: https://github.com/ejrgilbert/whamm/issues/163
                (None, "".to_string(), Some(pred))
            } else {
                let mut block = Block {
                    stmts: vec![Statement::Expr {
                        expr: pred.clone(),
                        loc: None,
                    }],
                    return_ty: None,
                    loc: None,
                };
                let (fid, str) = self.emit_special_func(
                    None,
                    &probe.metadata.pred_args,
                    None,
                    &[OrcaType::I32],
                    &mut block,
                    true,
                );
                (fid, str, None)
            }
        } else {
            (None, "".to_string(), None)
        };

        // create the probe's $alloc method
        let (alloc_fid, alloc_param_str) = self.unshared_var_handler.emit_alloc_func(
            &mut probe.unshared_to_alloc,
            &mut self.emitter,
            self.err,
        );

        // create the probe body function
        let (body_fid, body_param_str) = if let Some(body) = &mut probe.body {
            let alloc_local = if alloc_fid.is_some() {
                Some(LocalID(0))
            } else {
                None
            };
            self.emit_special_func(
                alloc_local,
                &probe.metadata.body_args,
                dynamic_pred,
                &[],
                body,
                false,
            )
        } else {
            (None, "".to_string())
        };

        // emit the export with the appropriate name
        let match_rule = self.create_wizard_match_rule(
            &probe.rule,
            pred_fid,
            &pred_param_str,
            alloc_fid,
            &alloc_param_str,
            &body_param_str,
        );
        if let Some(fid) = body_fid {
            self.emitter
                .app_wasm
                .exports
                .add_export_func(match_rule, fid);
        } else {
            unreachable!()
        }
    }

    fn create_wizard_match_rule(
        &self,
        probe_name: &str,
        pred_fid: Option<u32>,
        pred_params: &str,
        alloc_fid: Option<u32>,
        alloc_params: &str,
        body_params: &str,
    ) -> String {
        let pred_part = if let Some(pred_fid) = pred_fid {
            format!("/ ${pred_fid}({pred_params}) /")
        } else {
            "".to_string()
        };

        let body_part = if let Some(alloc_fid) = alloc_fid {
            &format!("${alloc_fid}({alloc_params}), {body_params}")
        } else {
            body_params
        };

        format!("{probe_name} {pred_part} ({body_part})")
    }
}

impl GeneratingVisitor for WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    // TODO -- these are all duplicates, try to factor out
    fn emit_string(&mut self, val: &mut Value) -> bool {
        self.emitter.emit_string(val, self.err)
    }

    fn emit_fn(&mut self, context: &str, f: &crate::parser::types::Fn) -> Option<FunctionID> {
        self.emitter.emit_fn(context, f, self.err)
    }

    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID> {
        self.emitter.emit_global(name, ty, value, self.err)
    }

    fn emit_report_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID> {
        self.emitter.emit_report_global(name, ty, value, self.err)
    }

    fn add_injected_func(&mut self, fid: FunctionID) {
        self.injected_funcs.push(fid);
    }

    fn get_context_name_mut(&mut self) -> &mut String {
        &mut self.context_name
    }

    fn get_context_name(&self) -> &String {
        &self.context_name
    }

    fn set_curr_loc(&mut self, loc: LocationData) {
        self.emitter.report_vars.curr_location = loc;
    }

    fn enter_named_scope(&mut self, name: &str) {
        self.emitter.table.enter_named_scope(name);
    }

    fn enter_scope(&mut self) {
        self.emitter.enter_scope(self.err);
    }

    fn exit_scope(&mut self) {
        self.emitter.exit_scope(self.err);
    }
    fn lookup_var_mut(&mut self, name: &str) -> Option<&mut Record> {
        self.emitter.table.lookup_var_mut(name, &None, self.err)
    }
}

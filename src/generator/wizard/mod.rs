pub mod ast;
pub mod metadata_collector;

use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::report_var_metadata::LocationData;
use crate::emitter::utils::whamm_type_to_wasm_type;
use crate::generator::wizard::ast::{WizardProbe, WizardScript};
use crate::generator::GeneratingVisitor;
use crate::libraries::core::io::io_adapter::IOAdapter;
use crate::parser::types::{Block, DataType, Definition, Statement, Value, WhammVisitorMut};
use crate::verifier::types::{Record, VarAddr};
use log::trace;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::ir::types::DataType as OrcaType;

pub struct WizardGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    pub emitter: ModuleEmitter<'b, 'c, 'd, 'e, 'f, 'g>,
    pub io_adapter: &'h mut IOAdapter,
    pub context_name: String,
    pub err: &'a mut ErrorGen,
    pub injected_funcs: &'h mut Vec<FunctionID>,
    pub config: &'i Config,
}

impl WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(&mut self, ast: Vec<WizardScript>) {
        self.emitter.reset_table();
        self.visit_ast(ast);
    }

    fn visit_ast(&mut self, mut ast: Vec<WizardScript>) {
        for script in ast.iter_mut() {
            self.visit_wiz_script(script);
        }
    }

    fn visit_wiz_script(&mut self, script: &mut WizardScript) {
        trace!("Entering: CodeGenerator::visit_script");
        self.enter_named_scope(&script.name);
        self.set_context_name(script.name.clone());

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
        param_reqs: &[(String, DataType)],
        results: &[OrcaType],
        body: &mut Block,
    ) -> (Option<u32>, String) {
        // create the predicate function
        let mut params = vec![];
        let mut param_str = "".to_string();

        for (local_id, (param_name, param_ty)) in param_reqs.iter().enumerate() {
            // handle param list
            params.push(whamm_type_to_wasm_type(param_ty));

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
                    addr: Some(VarAddr::Local {
                        addr: local_id as u32,
                    }),
                    loc: None,
                },
            );
        }

        let fid = self
            .emitter
            .emit_special_fn(None, &params, results, body, self.err);

        (fid, param_str.to_string())
    }

    fn visit_wiz_probe(&mut self, probe: &mut WizardProbe) {
        // TODO -- handle provided functions (provider, package, event, probe)
        let (pred_fid, pred_param_str) = if let Some(pred) = &mut probe.predicate {
            let mut block = Block {
                stmts: vec![Statement::Expr {
                    expr: pred.clone(),
                    loc: None,
                }],
                return_ty: None,
                loc: None,
            };
            self.emit_special_func(&probe.metadata.pred_args, &[OrcaType::I32], &mut block)
        } else {
            (None, "".to_string())
        };

        // create the probe body function
        let (body_fid, body_param_str) = if let Some(body) = &mut probe.body {
            self.emit_special_func(&probe.metadata.body_args, &[], body)
        } else {
            (None, "".to_string())
        };

        // emit the export with the appropriate name
        let match_rule =
            self.create_wizard_match_rule(&probe.rule, pred_fid, &pred_param_str, &body_param_str);
        if let Some(fid) = body_fid {
            self.emitter
                .app_wasm
                .exports
                .add_export_func(match_rule, fid);
        } else {
            // ignore
        }
    }

    fn create_wizard_match_rule(
        &self,
        probe_name: &str,
        pred_fid: Option<u32>,
        pred_params: &str,
        body_params: &str,
    ) -> String {
        let pred_part = if let Some(pred_fid) = pred_fid {
            format!("/ ${pred_fid}({pred_params}) /")
        } else {
            "".to_string()
        };

        format!("{probe_name} {pred_part} ({body_params})")
    }
}

impl GeneratingVisitor for WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
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
        self.emitter.report_var_metadata.curr_location = loc;
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
}

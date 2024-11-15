pub mod ast;
pub mod metadata_collector;

use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::utils::{wasm_type_to_whamm_type, whamm_type_to_wasm_type};
use crate::generator::wizard::ast::{UnsharedVar, WizardProbe, WizardScript};
use crate::generator::GeneratingVisitor;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::report_vars::{BytecodeLoc, LocationData};
use crate::parser::types::{Block, DataType, Definition, FnId, Statement, Value, WhammVisitorMut};
use crate::verifier::types::{Record, VarAddr};
use log::trace;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, LocalID};
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::Opcode;
use crate::emitter::memory_allocator::VAR_BLOCK_BASE_VAR;

pub struct WizardGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    pub emitter: ModuleEmitter<'b, 'c, 'd, 'e, 'f, 'g, 'h>,
    pub io_adapter: &'i mut IOAdapter,
    pub context_name: String,
    pub err: &'a mut ErrorGen,
    pub injected_funcs: &'i mut Vec<FunctionID>,
    pub config: &'j Config,

    // tracking
    pub curr_script_id: String,
}

impl WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(
        &mut self,
        ast: Vec<WizardScript>,
        used_provided_funcs: Vec<(String, String)>,
        strings_to_emit: Vec<String>,
    ) {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.emitter.setup_module(true, self.err);
        self.emit_needed_funcs(used_provided_funcs);
        self.emit_strings(strings_to_emit);
        self.visit_ast(ast);
        self.emitter.memory_grow(); // account for emitted strings in memory

        // set the value of curr_mem_offset Wasm global to mem_allocator.curr_mem_offset
        self.emitter.configure_mem_tracker_global(self.err);
        self.emitter
            .configure_flush_routines(self.io_adapter, self.err);
    }

    fn emit_strings(&mut self, strings_to_emit: Vec<String>) {
        for string in strings_to_emit.iter() {
            self.emitter.emit_string(
                &mut Value::Str {
                    ty: DataType::Str,
                    val: string.clone(),
                },
                self.err,
            );
        }
    }

    fn emit_needed_funcs(&mut self, funcs: Vec<(String, String)>) {
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

    // Visit the AST
    fn visit_ast(&mut self, mut ast: Vec<WizardScript>) {
        for script in ast.iter_mut() {
            self.visit_wiz_script(script);
        }
    }

    fn visit_wiz_script(&mut self, script: &mut WizardScript) {
        trace!("Entering: CodeGenerator::visit_script");
        self.enter_named_scope(&script.name);
        self.set_context_name(script.name.clone());
        self.curr_script_id = script.name.clone();

        self.set_curr_loc(LocationData::Global {
            script_id: script.name.clone(),
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
        param_reqs: &[(String, DataType)],
        results: &[OrcaType],
        body: &mut Block,
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
                    addr: Some(VarAddr::Local {
                        addr: *alloc,
                    }),
                    loc: None,
                },
            );
        }

        // handle the parameters
        for (param_name, param_ty) in param_reqs.iter() {
            let local_id = params.len() as u32;
            // handle param list
            params.extend(whamm_type_to_wasm_type(param_ty));

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
                        addr: local_id,
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

    fn emit_alloc_func(&mut self, probe: &mut WizardProbe) -> (Option<u32>, String) {
        struct Local {
            id: LocalID,
            ty: OrcaType
        }

        if probe.unshared_to_alloc.is_empty() {
            (None, "".to_string())
        } else {
            // specify params
            let fid = Local {
                id: LocalID(0),
                ty: OrcaType::I32
            };
            let pc = Local {
                id: LocalID(1),
                ty: OrcaType::I32
            };

            // params: (fid, pc)
            let alloc_params = vec![fid.ty, pc.ty];
            // results: mem_offset
            let alloc_results = vec![OrcaType::I32];

            let mut alloc = FunctionBuilder::new(&alloc_params, &alloc_results);
            // specify locals
            let orig_offset = Local {
                id: alloc.add_local(OrcaType::I32),
                ty: OrcaType::I32
            };

            // remember the original memory offset
            alloc.global_get(self.emitter.mem_allocator.mem_tracker_global);
            alloc.local_set(orig_offset.id);

            // track what's been allocated for this function thus far
            let mut next_var_offset = 0;

            // store fid and pc
            let (fid_addr, bytes_used) = self.emitter.mem_allocator.emit_store_from_local(next_var_offset, fid.id, &wasm_type_to_whamm_type(&fid.ty), &mut alloc);
            next_var_offset += bytes_used;
            self.emitter.table.put("fid".to_string(), Record::Var {
                ty: wasm_type_to_whamm_type(&fid.ty),
                name: "fid".to_string(),
                value: None,
                def: Definition::CompilerDynamic,
                is_report_var: false,
                addr: Some(fid_addr),
                loc: None,
            });

            let (pc_addr, bytes_used) = self.emitter.mem_allocator.emit_store_from_local(next_var_offset, pc.id, &wasm_type_to_whamm_type(&pc.ty), &mut alloc);
            next_var_offset += bytes_used;
            self.emitter.table.put("pc".to_string(), Record::Var {
                ty: wasm_type_to_whamm_type(&fid.ty),
                name: "pc".to_string(),
                value: None,
                def: Definition::CompilerDynamic,
                is_report_var: false,
                addr: Some(pc_addr),
                loc: None,
            });

            // alloc each var
            for UnsharedVar { ty, name, is_report: _ } in probe.unshared_to_alloc.iter() {
                let (var_addr, bytes_used) = self.emitter.mem_allocator.alloc_mem_space(next_var_offset, ty, &mut alloc);
                next_var_offset += bytes_used;
                self.emitter.table.put(name.clone(), Record::Var {
                    ty: wasm_type_to_whamm_type(&fid.ty),
                    name: name.clone(),
                    value: None,
                    def: Definition::CompilerDynamic,
                    is_report_var: false,
                    addr: Some(var_addr),
                    loc: None,
                });

                // TODO handle report variables!
            }

            // return the base memory offset where this function's var block starts
            alloc.local_get(orig_offset.id);

            let alloc_id = alloc.finish_module(self.emitter.app_wasm);
            (Some(*alloc_id), "fid, pc".to_string())
        }
    }

    fn create_curr_loc(&self, probe: &WizardProbe) -> LocationData {
        let probe_id = format!("{}_{}", probe.probe_number, probe.rule);

        //set the current location in bytecode and load some new globals for potential report vars
        LocationData::Local {
            script_id: self.curr_script_id.clone(),
            bytecode_loc: BytecodeLoc::new(0, 0), // TODO -- request this from wizard
            probe_id,
            num_unshared: probe.unshared_to_alloc.len() as i32, //this is still used in the emitter to determine how many new globals to emit
        }
    }

    fn visit_wiz_probe(&mut self, probe: &mut WizardProbe) {
        self.set_curr_loc(self.create_curr_loc(probe));

        let (pred_fid, pred_param_str) = if let Some(pred) = &mut probe.predicate {
            let mut block = Block {
                stmts: vec![Statement::Expr {
                    expr: pred.clone(),
                    loc: None,
                }],
                return_ty: None,
                loc: None,
            };
            self.emit_special_func(None, &probe.metadata.pred_args, &[OrcaType::I32], &mut block)
        } else {
            (None, "".to_string())
        };

        // create the probe's $alloc method
        let (alloc_fid, alloc_param_str) = self.emit_alloc_func(probe);

        // create the probe body function
        let (body_fid, body_param_str) = if let Some(body) = &mut probe.body {
            let alloc_local = if alloc_fid.is_some() {
                Some(LocalID(0))
            } else {
                None
            };
            self.emit_special_func(alloc_local, &probe.metadata.body_args, &[], body)
        } else {
            (None, "".to_string())
        };

        // emit the export with the appropriate name
        let match_rule =
            self.create_wizard_match_rule(&probe.rule, pred_fid, &pred_param_str, alloc_fid, &alloc_param_str, &body_param_str);
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

impl GeneratingVisitor for WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
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
}

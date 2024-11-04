pub mod ast;
pub mod init_generator;
pub mod metadata_collector;

use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::emitter::module_emitter::{ModuleEmitter, StringAddr};
use crate::emitter::report_var_metadata::{BytecodeLoc, LocationData, Metadata};
use crate::emitter::utils::{whamm_type_to_wasm_global, whamm_type_to_wasm_type};
use crate::generator::wizard::ast::{WizardProbe, WizardScript};
use crate::generator::GeneratingVisitor;
use crate::libraries::core::io::io_adapter::IOAdapter;
use crate::parser::types::{Block, DataType, Definition, FnId, Statement, Value, WhammVisitorMut};
use crate::verifier::types::{Record, VarAddr};
use log::{debug, trace};
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID};
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::ir::types::Value as OrcaValue;
use orca_wasm::{DataSegment, DataSegmentKind, InitExpr, Opcode};
use std::collections::HashMap;

const UNEXPECTED_ERR_MSG: &str =
    "WizardGenerator: Looks like you've found a bug...please report this behavior!";

pub struct WizardGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    pub emitter: ModuleEmitter<'b, 'c, 'd, 'e, 'f, 'g>,
    pub io_adapter: &'h mut IOAdapter,
    pub context_name: String,
    pub err: &'a mut ErrorGen,
    pub injected_funcs: &'h mut Vec<FunctionID>,
    pub config: &'i Config,

    // tracking
    pub curr_script_id: String,
}

impl WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(
        &mut self,
        ast: Vec<WizardScript>,
        used_provided_funcs: Vec<(String, String)>,
        strings_to_emit: Vec<String>,
    ) {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.setup_report_vars();
        self.emit_needed_funcs(used_provided_funcs);
        self.emit_strings(strings_to_emit);
        self.visit_ast(ast);
        self.emitter.memory_grow(); // account for emitted strings in memory

        self.on_end();
    }

    // Private helper functions
    fn setup_report_vars(&mut self) {
        if self.emitter.map_lib_adapter.is_used {
            self.create_global_map_init();
            self.create_print_map_meta();
        }
        self.create_print_global_meta();
    }

    fn create_global_map_init(&mut self) {
        //make a global bool for whether to run the global_map_init fn
        self.emitter.map_lib_adapter.init_bool_location = *self.emitter.app_wasm.add_global(
            InitExpr::Value(OrcaValue::I32(1)),
            OrcaType::I32,
            true,
            false,
        );
        match self
            .emitter
            .app_wasm
            .functions
            .get_local_fid_by_name("global_map_init")
        {
            Some(_) => {
                debug!("global_map_init function already exists");
                self.err.add_error(ErrorGen::get_unexpected_error(
                    true,
                    Some(
                        "global_map_init function already exists - needs to be created by Whamm"
                            .to_string(),
                    ),
                    None,
                ));
            }
            None => {
                //time to make a global_map_init fn
                debug!("No global_map_init function found, creating one");
                let global_map_init_fn = FunctionBuilder::new(&[], &[]);
                let global_map_init_id = global_map_init_fn.finish_module(self.emitter.app_wasm);
                self.emitter
                    .app_wasm
                    .set_fn_name(global_map_init_id, "global_map_init".to_string());
            }
        }
    }

    fn create_print_map_meta(&mut self) {
        if self
            .emitter
            .app_wasm
            .functions
            .get_local_fid_by_name("print_map_meta")
            .is_some()
        {
            debug!("print_map_meta function already exists");
            self.err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(
                    "print_map_meta function already exists - needs to be created by Whamm"
                        .to_string(),
                ),
                None,
            ));
            return;
        }

        debug!("Creating the print_map_meta function");
        let print_map_meta_fn = FunctionBuilder::new(&[], &[]);
        let print_map_meta_id = print_map_meta_fn.finish_module(self.emitter.app_wasm);
        self.emitter
            .app_wasm
            .set_fn_name(print_map_meta_id, "print_map_meta".to_string());

        self.emitter.table.put(
            "print_map_meta".to_string(),
            Record::Fn {
                name: FnId {
                    name: "print_map_meta".to_string(),
                    loc: None,
                },
                params: vec![],
                ret_ty: DataType::Tuple { ty_info: vec![] },
                def: Definition::CompilerStatic,
                addr: Some(*print_map_meta_id),
                loc: None,
            },
        );
    }

    fn create_print_global_meta(&mut self) {
        if self
            .emitter
            .app_wasm
            .functions
            .get_local_fid_by_name("print_global_meta")
            .is_some()
        {
            debug!("print_global_meta function already exists");
            self.err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(
                    "print_global_meta function already exists - needs to be created by Whamm"
                        .to_string(),
                ),
                None,
            ));
            return;
        }

        debug!("Creating the print_global_meta function");
        let print_global_meta_fn = FunctionBuilder::new(&[], &[]);
        let print_global_meta_id = print_global_meta_fn.finish_module(self.emitter.app_wasm);
        self.emitter
            .app_wasm
            .set_fn_name(print_global_meta_id, "print_global_meta".to_string());

        self.emitter.table.put(
            "print_global_meta".to_string(),
            Record::Fn {
                name: FnId {
                    name: "print_global_meta".to_string(),
                    loc: None,
                },
                params: vec![],
                ret_ty: DataType::Tuple { ty_info: vec![] },
                def: Definition::CompilerStatic,
                addr: Some(*print_global_meta_id),
                loc: None,
            },
        );
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

    // Run at the end

    fn on_end(&mut self) {
        if self
            .emitter
            .report_var_metadata
            .variable_metadata
            .is_empty()
            && self.emitter.report_var_metadata.map_metadata.is_empty()
        {
            return;
        }

        // configure the flushing routines!
        let report_var_metadata = &self.emitter.report_var_metadata;
        let var_meta = &report_var_metadata.variable_metadata;
        let map_meta = &report_var_metadata.map_metadata;
        let mut var_meta_str: HashMap<u32, String> = HashMap::new();
        let mut map_meta_str: HashMap<u32, String> = HashMap::new();
        //convert the metadata into strings, add those to the data section, then use those to populate the maps
        for (key, value) in var_meta.iter() {
            //first, emit the string to data section
            let val = value.to_csv();
            let data_id = self.emitter.app_wasm.data.len();
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
            self.emitter.app_wasm.data.push(data_segment);
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
            let data_id = self.emitter.app_wasm.data.len();
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
            self.emitter.app_wasm.data.push(data_segment);
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
        self.setup_print_global_meta(&var_meta_str);
        self.setup_print_map_meta(&map_meta_str);
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
            .app_wasm
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

        // output the header data segment
        let header = Metadata::get_csv_header();
        self.io_adapter
            .putsln(header.clone(), &mut print_global_meta, self.err);

        // for each of the report globals, emit the printing logic
        for (key, val) in var_meta_str.iter() {
            self.io_adapter.puts(
                format!("i32,{key},{val},"),
                &mut print_global_meta,
                self.err,
            );

            // get the value of this report global
            print_global_meta.global_get(GlobalID(*key));
            self.io_adapter.call_puti(&mut print_global_meta, self.err);
            self.io_adapter.putln(&mut print_global_meta, self.err);
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
            .app_wasm
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

        // for each of the report maps, emit the printing logic
        for (key, val) in map_meta_str.iter() {
            self.io_adapter
                .puts(format!("map,{key},{val},"), &mut print_map_meta, self.err);

            // print the value(s) of this map
            self.emitter
                .map_lib_adapter
                .print_map(*key, &mut print_map_meta, self.err);
            self.io_adapter.putln(&mut print_map_meta, self.err);
        }
        true
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
        param_reqs: &[(String, DataType)],
        results: &[OrcaType],
        body: &mut Block,
    ) -> (Option<u32>, String) {
        // create the function
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

    fn create_curr_loc(&self, probe: &WizardProbe) -> LocationData {
        let probe_id = format!("{}_{}", probe.probe_number, probe.rule);

        //set the current location in bytecode and load some new globals for potential report vars
        LocationData::Local {
            script_id: self.curr_script_id.clone(),
            bytecode_loc: BytecodeLoc::new(0, 0), // TODO -- request this from wizard
            probe_id,
            num_reports: probe.num_reports, //this is still used in the emitter to determine how many new globals to emit
        }
    }

    fn visit_wiz_probe(&mut self, probe: &mut WizardProbe) {
        // TODO -- handle provided functions (provider, package, event, probe)
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
            self.emit_special_func(&probe.metadata.pred_args, &[OrcaType::I32], &mut block)
        } else {
            (None, "".to_string())
        };

        // create the probe's report variable globals!
        for _ in 0..probe.num_reports {
            let (global_id, ..) = whamm_type_to_wasm_global(self.emitter.app_wasm, &DataType::I32);
            self.emitter
                .report_var_metadata
                .available_i32_gids
                .push(*global_id);
        }

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

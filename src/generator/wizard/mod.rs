pub mod metadata_collector;
pub mod ast;

use orca_wasm::ir::id::FunctionID;
use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::report_var_metadata::LocationData;
use crate::generator::GeneratingVisitor;
use crate::libraries::core::io::io_adapter::IOAdapter;
use crate::parser::rules::{Event, Probe};
use crate::parser::types::{DataType, Value, Whamm};

pub struct WizardGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    pub emitter: ModuleEmitter<'b, 'c, 'd, 'e, 'f, 'g>,
    pub io_adapter: &'h mut IOAdapter,
    pub context_name: String,
    pub err: &'a mut ErrorGen,
    pub injected_funcs: &'h mut Vec<FunctionID>,
    pub config: &'i Config
}

impl WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(&mut self, whamm: &mut Whamm) -> bool {
        // see: https://github.com/ejrgilbert/whamm/blob/0e8336956eb7d6a0ab741147576ba0f5dcdac1ca/src/emitter/wizard/init_generator.rs
        todo!()
    }
}

impl GeneratingVisitor for WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    // TODO -- these are all duplicates, try to factor out
    fn emit_string(&mut self, val: &mut Value) -> bool {
        self.emitter.emit_string(val, &mut self.err)
    }

    fn emit_fn(&mut self, context: &str, f: &crate::parser::types::Fn) -> Option<FunctionID> {
        self.emitter.emit_fn(context, f, &mut self.err)
    }

    fn emit_global(&mut self, name: String, ty: DataType, value: &Option<Value>) -> Option<FunctionID> {
        self.emitter.emit_global(name, ty, value, &mut self.err)
    }

    fn emit_report_global(&mut self, name: String, ty: DataType, value: &Option<Value>) -> Option<FunctionID> {
        self.emitter.emit_report_global(name, ty, value, &mut self.err)
    }

    fn add_injected_func(&mut self, fid: FunctionID) {
        self.injected_funcs.push(fid);
    }

    fn set_context_name(&mut self, val: String) {
        self.context_name = val;
    }

    fn get_context_name(&self) -> &String {
        &self.context_name
    }

    fn append_context_name(&mut self, val: String) {
        self.context_name += &val;
    }

    fn set_curr_loc(&mut self, loc: LocationData) {
        self.emitter.report_var_metadata.curr_location = loc;
    }

    fn enter_scope(&mut self) {
        self.emitter.enter_scope(&mut self.err);
    }

    fn exit_scope(&mut self) {
        self.emitter.exit_scope(&mut self.err);
    }

    fn visit_before_probes(&mut self, event: &mut dyn Event) -> bool {
        todo!()
    }

    fn visit_alt_probes(&mut self, event: &mut dyn Event) -> bool {
        if !self.config.enable_wizard_alt {
            // error!
            todo!()
        } else {
            todo!()
        }
    }

    fn visit_after_probes(&mut self, event: &mut dyn Event) -> bool {
        todo!()
    }

    fn on_enter_visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> bool {
        // At this point, each event will correspond to some actions function
        // skip the whamm context and script index
        // let mnemonic = self
        //     .context_name
        //     .clone()
        //     .split(':')
        //     .skip(2)
        //     .collect::<Vec<&str>>()
        //     .join(":");
        // let mut action = FunctionBuilder::new(&[], &[]);
        todo!()
    }

    fn on_exit_visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> bool {
        // export the event function
        // let action_id = action.finish_module(self.emitter.app_wasm);
        // self.emitter.export_mnemonic(mnemonic, action_id);
        // // after visiting the statements, we finished building the function
        // if let Some(func) = self.emitter.emitting_func.take() {
        //     let fid = func.finish_module(self.emitter.app_wasm);
        //     // TODO: can we set a good name for this function
        //     (is_success, Some(fid))
        // } else {
        //     panic!("No emitting function was set!");
        // }
        todo!()
    }

    // TODO -- right now, this is using init_generator's impl to visit the AST...
    //   but this is really to find when certain things happen rather than emitting them
    //   how to override to emit the body of a probe? (Can side-step the deep AST visiting, more efficient!)
}
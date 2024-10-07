use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::libraries::core::io::io_adapter::IOAdapter;
use crate::parser::types::Whamm;

pub struct WizardGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    pub emitter: ModuleEmitter<'b, 'c, 'd, 'e, 'f, 'g>,
    pub io_adapter: &'h mut IOAdapter,
    pub context_name: String,
    pub err: &'a mut ErrorGen
}

impl WizardGenerator<'_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(&mut self, whamm: &mut Whamm) -> bool {
        // see: https://github.com/ejrgilbert/whamm/blob/0e8336956eb7d6a0ab741147576ba0f5dcdac1ca/src/emitter/wizard/init_generator.rs
        todo!()
    }
}
pub mod maps;

use std::collections::HashSet;
use crate::linker::core::maps::MapLibPackage;
use crate::parser::types::WhammVisitor;

pub fn get_packages() -> Vec<Box<dyn LibPackage>> {
    vec![
        Box::new(MapLibPackage::default())
    ]
}

// A lib package needs to be able to visit the AST and determine if it's needed (should be linked)
pub trait LibPackage: WhammVisitor<()> {
    fn is_used(&self) -> bool;
    fn get_fn_names(&self) -> &HashSet<String>;
    // fn get_lib_adapter(&self) -> Box<&dyn LibAdapter>;
}
pub trait LibAdapter {
    fn get_fn_names(&self) -> &HashSet<String>;
}

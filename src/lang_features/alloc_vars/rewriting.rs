use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use std::collections::HashMap;

pub fn allocate_vars(_to_alloc: Vec<(String, DataType)>) -> HashMap<String, (VarAddr, DataType)> {
    // Called once per probe match point with `alloc` OR `report` vars.

    // result: The new global variables: name -> (addr, ty)
    //     as a hashmap to enable caller to place in SymbolTable and handle report variables
    //   Add these VarAddrs to the symbol table.
    //   Can now emit the rest of the probe body logic as normal.

    // NOTE: `decl_init` statements should be run ONCE

    // See utils.rs/`emit_report_decl_stmt`
    //    basically want to do just refactor to call out to THIS function instead (more modular)
    //    this function will also handle if the var is a report variable
    todo!()
}
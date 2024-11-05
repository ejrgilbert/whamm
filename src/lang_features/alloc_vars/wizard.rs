use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use orca_wasm::ir::id::FunctionID;
use std::collections::HashMap;

pub fn create_alloc_func(_to_alloc: Vec<DataType>) -> FunctionID {
    // Called once per probe definition with `alloc` OR `report` vars.

    // $alloc description:
    // Will generate a function that allocates the memory required.
    // The order of the data will go in the order of the `to_alloc` param.
    // The function will return the memory offset to use by the probe logic.
    // Will also need to update some global value that keeps track of the already (use memory allocator?)
    // allocated memory space. Make sure to check that memory size is big enough!

    todo!()
}

pub fn load_alloc_vars(
    _alloc_mem_offset: VarAddr,
    _alloc_vars: Vec<(String, DataType)>,
) -> HashMap<String, (VarAddr, DataType)> {
    // alloc_mem_offset: parameter that specifies the result of calling $alloc (should be wasm local var)
    // alloc_vars: Vec<(var_name, var_ty)>, in the order they should appear in memory starting at
    //     the offset `alloc_mem_offset`
    // result: The new local variables: name -> (addr, ty)
    //     as a hashmap to enable caller to place in SymbolTable and handle report variables

    // At start of probe logic, pull the current values of the 'alloc' variables from memory.
    //   Add these VarAddrs to the symbol table.
    //   Can now emit the rest of the probe body logic as normal.

    todo!()
}

pub fn save_alloc_vars(
    _alloc_mem_offset: VarAddr,
    _allocated_vars: HashMap<String, (VarAddr, DataType)>,
) {
    // At end of probe logic, save the values of 'alloc' variables back into memory.

    todo!()
}

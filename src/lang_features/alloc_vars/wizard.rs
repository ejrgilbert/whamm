use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use orca_wasm::ir::id::FunctionID;
use std::collections::HashMap;

pub fn create_alloc_func(_to_alloc: Vec<DataType>) -> FunctionID {
    // Called once per probe definition with `unshared` OR `report` vars.

    // $alloc description:
    // Will generate a function that allocates the memory required.
    // The order of the data will go in the order of the `_to_alloc` param.
    // The function will return the memory offset to use by the probe logic.
    // Will also need to update some global value that keeps track of the already (use memory allocator?)
    // allocated memory space. Make sure to check that memory size is big enough!

    // NOTE: `decl_init` statements should be run ONCE (can be in $alloc func)
    todo!()
}

pub fn load_unshared_vars(
    _alloc_mem_offset: VarAddr,
    _allocated_vars: Vec<(String, DataType)>,
) -> HashMap<String, (VarAddr, DataType)> {
    // increment by bytes used by each variable's DataType as we load them
    // will be used to load the next variable AND to save in the VarAddr!
    let _used_bytes = 0;

    // alloc_mem_offset: parameter that specifies the result of calling $alloc (should be wasm local var)
    // alloc_vars: Vec<(var_name, var_ty)>, in the order they should appear in memory starting at
    //     the offset `alloc_mem_offset`
    // result: The new local variables: name -> (addr, ty)
    //     as a hashmap to enable caller to place in SymbolTable and handle report variables
    //     the new VarAddr will be a pointer into memory with an offset (real addr should be alloc_mem_offset + len_of_prev_vars)

    // At start of probe logic, pull the current values of the allocated variables from memory.
    //   Add these VarAddrs to the symbol table.
    //   Can now emit the rest of the probe body logic as normal.

    todo!()
}

pub fn save_allocated_vars(
    _alloc_mem_offset: VarAddr,
    _allocated_vars: HashMap<String, (VarAddr, DataType)>,
) {
    // At end of probe logic, save the values of allocated variables back into memory.

    todo!()
}

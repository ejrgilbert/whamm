use crate::parser::types::{DataType, Definition};
use crate::verifier::types::{Record, VarAddr};
use orca_wasm::ir::id::{FunctionID, LocalID};
use std::collections::HashMap;
use orca_wasm::ir::function::FunctionBuilder;
use crate::generator::wizard::ast::UnsharedVar;
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::Opcode;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::utils::wasm_type_to_whamm_type;

pub fn create_alloc_func(_to_alloc: Vec<DataType>) -> FunctionID {


    // NOTE: `decl_init` statements should be run ONCE (can be in $alloc func)
    todo!()
}

pub fn emit_alloc_func(unshared_to_alloc: &mut Vec<UnsharedVar>, emitter: &mut ModuleEmitter) -> (Option<u32>, String) {
    // Called once per probe definition with `unshared` OR `report` vars.
    // $alloc description:
    // Will generate a function that allocates the memory required.
    // The order of the data will go in the order of the `unshared_to_alloc` param.
    // The function will return the memory offset to use by the probe logic.
    // Will also need to update some global value that keeps track of the already (use memory allocator?)
    // allocated memory space. Make sure to check that memory size is big enough!
    struct Local {
        id: LocalID,
        ty: OrcaType,
    }

    if unshared_to_alloc.is_empty() {
        (None, "".to_string())
    } else {
        // specify params
        let fid = Local {
            id: LocalID(0),
            ty: OrcaType::I32,
        };
        let pc = Local {
            id: LocalID(1),
            ty: OrcaType::I32,
        };

        // params: (fid, pc)
        let alloc_params = vec![fid.ty, pc.ty];
        // results: mem_offset
        let alloc_results = vec![OrcaType::I32];

        let mut alloc = FunctionBuilder::new(&alloc_params, &alloc_results);
        // specify locals
        let orig_offset = Local {
            id: alloc.add_local(OrcaType::I32),
            ty: OrcaType::I32,
        };

        // remember the original memory offset
        alloc.global_get(emitter.mem_allocator.mem_tracker_global);
        alloc.local_set(orig_offset.id);

        // track what's been allocated for this function thus far
        let mut next_var_offset = 0;

        // store fid and pc
        let (_, bytes_used) = emitter.mem_allocator.emit_store_from_local(
            next_var_offset,
            fid.id,
            &wasm_type_to_whamm_type(&fid.ty),
            &mut alloc,
        );
        next_var_offset += bytes_used;
        // TODO: I don't think I need this
        // self.emitter.table.put("fid".to_string(), Record::Var {
        //     ty: wasm_type_to_whamm_type(&fid.ty),
        //     name: "fid".to_string(),
        //     value: None,
        //     def: Definition::CompilerDynamic,
        //     is_report_var: false,
        //     addr: Some(fid_addr),
        //     loc: None,
        // });

        let (_, bytes_used) = emitter.mem_allocator.emit_store_from_local(
            next_var_offset,
            pc.id,
            &wasm_type_to_whamm_type(&pc.ty),
            &mut alloc,
        );
        next_var_offset += bytes_used;
        // TODO: I don't think I need this
        // self.emitter.table.put("pc".to_string(), Record::Var {
        //     ty: wasm_type_to_whamm_type(&fid.ty),
        //     name: "pc".to_string(),
        //     value: None,
        //     def: Definition::CompilerDynamic,
        //     is_report_var: false,
        //     addr: Some(pc_addr),
        //     loc: None,
        // });

        // alloc each var
        for UnsharedVar {
            ty,
            name,
            is_report: _,
        } in unshared_to_alloc.iter()
        {
            let (var_addr, bytes_used) =
                emitter
                    .mem_allocator
                    .alloc_mem_space(next_var_offset, ty, &mut alloc);
            next_var_offset += bytes_used;
            emitter.table.put(
                name.clone(),
                Record::Var {
                    ty: wasm_type_to_whamm_type(&fid.ty),
                    name: name.clone(),
                    value: None,
                    def: Definition::CompilerDynamic,
                    is_report_var: false,
                    addr: Some(var_addr),
                    loc: None,
                },
            );

            // TODO handle report variables!
        }

        // return the base memory offset where this function's var block starts
        alloc.local_get(orig_offset.id);

        let alloc_id = alloc.finish_module(emitter.app_wasm);
        (Some(*alloc_id), "fid, pc".to_string())
    }
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

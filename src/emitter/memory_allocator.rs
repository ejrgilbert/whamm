use std::collections::HashMap;
use orca_wasm::{DataSegment, DataSegmentKind, InitExpr, Module};
use orca_wasm::ir::types::Value as OrcaValue;

pub struct MemoryAllocator {
    pub mem_id: u32,
    pub curr_mem_offset: usize,
    pub required_initial_mem_size: u64,
    pub emitted_strings: HashMap<String, StringAddr>,
}
impl MemoryAllocator {
    pub fn emit_string(&mut self,
                       wasm: &mut Module,
                       val: &mut String) -> bool {
        if self.emitted_strings.contains_key(val) {
            // the string has already been emitted into the module, don't emit again
            return true;
        }
        // assuming that the data ID is the index of the object in the Vec
        let data_id = wasm.data.len();
        let val_bytes = val.as_bytes().to_owned();
        let data_segment = DataSegment {
            data: val_bytes,
            kind: DataSegmentKind::Active {
                memory_index: self.mem_id,
                offset_expr: InitExpr::Value(OrcaValue::I32(
                    self.curr_mem_offset as i32,
                )),
            },
        };
        wasm.data.push(data_segment);

        // save the memory addresses/lens, so they can be used as appropriate
        self.emitted_strings.insert(
            val.clone(),
            StringAddr {
                data_id: data_id as u32,
                mem_offset: self.curr_mem_offset,
                len: val.len(),
            },
        );

        // update curr_mem_offset to account for new data
        self.curr_mem_offset += val.len();
        true
    }

    pub(crate) fn memory_grow(&mut self, wasm: &mut Module) {
        // If we've allocated any memory, bump the app's memory up to account for that

        // TODO -- this doesn't actually increase the required_initial_mem_size at any point
        if !self.emitted_strings.is_empty() {
            if let Some(mem) = wasm.memories.get_mut(0) {
                if mem.initial < self.required_initial_mem_size {
                    mem.initial = self.required_initial_mem_size;
                }
            }
        }
    }
}

pub struct StringAddr {
    pub data_id: u32,
    pub mem_offset: usize,
    pub len: usize,
}
use wirm::ir::id::MemoryID;
use wirm::ir::module::module_globals::GlobalKind;
use wirm::ir::module::module_memories::MemKind;
use wirm::ir::types::InitExpr;
use wirm::ir::types::Value;
use wirm::DataSegmentKind;
use wirm::InitInstr;
use wirm::Module;

fn eval_init_expr(app_wasm: &Module, expr: &InitExpr) -> Option<u32> {
    let mut stack: Vec<u64> = vec![];
    for instr in &expr.exprs {
        match instr {
            InitInstr::Value(v) => match v {
                Value::I32(n) => stack.push(*n as u32 as u64),
                Value::I64(n) => stack.push(*n as u64),
                _ => return None, // float/v128 can't be memory offsets
            },
            InitInstr::Global(id) => match app_wasm.globals.get_kind(*id) {
                GlobalKind::Local(local) => {
                    let val = eval_init_expr(app_wasm, &local.init_expr)?;
                    stack.push(val as u64);
                }
                GlobalKind::Import(_) => {
                    eprintln!("warning: imported global, cannot resolve statically");
                    return None;
                }
            },
            _ => return None, // ref types aren't memory offsets
        }
    }
    stack.last().map(|v| *v as u32)
}

pub fn do_on_active_segs<F>(app_wasm: &Module, mem_id: MemoryID, mut action: F)
where
    F: FnMut(Option<u32>, usize),
{
    for seg in app_wasm.data.iter() {
        match &seg.kind {
            DataSegmentKind::Active {
                offset_expr,
                memory_index,
            } => {
                if *memory_index != *mem_id {
                    continue;
                }
                action(eval_init_expr(app_wasm, offset_expr), seg.data.len());
            }
            DataSegmentKind::Passive => {}
        }
    }
}

pub fn get_active_data_start(app_wasm: &Module, mem_id: MemoryID) -> u32 {
    let mut current_min: u32 = 0;
    let mut current_min_initialized: bool = false;

    do_on_active_segs(app_wasm, mem_id, |offset, _| {
        if let Some(offset) = offset {
            if !current_min_initialized {
                current_min = offset;
                current_min_initialized = true;
            } else {
                current_min = current_min.min(offset);
            }
        }
    });
    current_min
}

pub fn get_active_data_end(app_wasm: &Module, mem_id: MemoryID) -> u32 {
    let mut current_max: u32 = 0;
    let mut current_max_initialized: bool = false;

    do_on_active_segs(app_wasm, mem_id, |offset, len| {
        if let Some(offset) = offset {
            let end = offset + len as u32;
            if !current_max_initialized {
                current_max = end;
                current_max_initialized = true;
            } else {
                current_max = current_max.max(end);
            }
        }
    });
    current_max
}

pub fn get_active_data_len(app_wasm: &Module, mem_id: MemoryID) -> u32 {
    let min = get_active_data_start(app_wasm, mem_id);
    let max = get_active_data_end(app_wasm, mem_id);

    max - min
}

// This defaults to the first local memory of the application
// if it doesn't have one, error!
pub fn get_first_local_mem_id(app_wasm: &Module) -> Result<MemoryID, String> {
    for mem in app_wasm.memories.iter() {
        if let MemKind::Local(info) = mem.kind() {
            return Ok(info.mem_id);
        }
    }
    Err("No local memory found".to_string())
}

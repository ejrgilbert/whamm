use crate::emitter::rewriting::rules::core::CorePackage;
use crate::emitter::rewriting::rules::wasm::{is_prog_exit_call, WasmPackage};
use crate::generator::ast::{Probe, ReqArgs, WhammParam};
use crate::generator::rewriting::simple_ast::SimpleAstProbes;
use crate::parser::rules::core::WhammModeKind;
use crate::parser::rules::{FromStr, WhammProviderKind};
use crate::parser::types::{Block, DataType, Definition, Expr, NumLit, RulePart, Statement, Value};
use orca_wasm::ir::id::{FunctionID, GlobalID, TypeID};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::Location;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use log::warn;
use orca_wasm::ir::module::module_functions::{FuncKind, ImportedFunction, LocalFunction};
use orca_wasm::ir::module::module_globals::{GlobalKind, ImportedGlobal, LocalGlobal};
use wasmparser::{BrTable, GlobalType, MemArg, Operator};

mod core;
pub mod wasm;

pub fn get_loc_info_for_active_probes(
        app_wasm: &Module,
        loc: Location,
        instr: &Operator,
        probes: &SimpleAstProbes
    ) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    for (provider, packages) in probes.iter() {
        if let Some(mut tmp) = handle_provider(app_wasm, loc, instr, provider, packages) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }
    res
}

fn handle_provider(
    app_wasm: &Module,
    loc: Location,
    instr: &Operator,
    provider: &String,
    packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>
) -> Option<LocInfo> {
    match provider.as_str() {
        "wasm" => {
            handle_wasm(app_wasm, loc, instr, packages)
        }
        _ => todo!()
    }
}

fn handle_wasm(
    app_wasm: &Module,
    loc: Location,
    instr: &Operator,
    packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>
) -> Option<LocInfo> {
    let mut loc_info = LocInfo::new();
    let (fid, pc, fname) = match loc {
        Location::Module {
            func_idx,
            instr_idx,
        }
        | Location::Component {
            func_idx,
            instr_idx,
            ..
        } => {
            let mut fname = String::default();
            let name = app_wasm.functions.get_name(func_idx).as_ref();
            if let Some(name) = name {
                fname = name.clone();
            }
            (func_idx, instr_idx, fname)
        }
    };
    loc_info
        .static_data
        .insert("fid".to_string(), Some(Value::gen_u32(*fid)));

    loc_info
        .static_data
        .insert("fname".to_string(), Some(Value::Str { val: fname.clone() }));

    loc_info
        .static_data
        .insert("pc".to_string(), Some(Value::gen_u32(pc as u32)));

    let mut res: Option<LocInfo> = Some(loc_info);
    for (package, events) in packages.iter() {
        if let Some(mut tmp) = handle_wasm_packages(app_wasm, &fid, instr, package, events) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }
    res
}

fn handle_wasm_packages(
    app_wasm: &Module,
    fid: &FunctionID,
    instr: &Operator,
    package: &String,
    events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>
) -> Option<LocInfo> {
    match package.as_str() {
        "opcode" => handle_opcode(app_wasm, fid, instr, events),
        "begin" => todo!(),
        "end" => todo!(),
        _ => todo!()
    }
}

fn handle_opcode(
    app_wasm: &Module,
    fid: &FunctionID,
    instr: &Operator,
    events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>
) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    for (package, events) in events.iter() {
        // See OpcodeEvent.get_loc_info
        if let Some(mut tmp) = handle_opcode_events(app_wasm, fid, instr, package, events) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }
    res
}

fn handle_opcode_events(
    app_wasm: &Module,
    fid: &FunctionID,
    instr: &Operator,
    event: &String,
    probes: &HashMap<WhammModeKind, Vec<Probe>>
) -> Option<LocInfo> {
    let mut loc_info = LocInfo::new();

    // create a combination of WhammParams for all probes here
    let mut all_params = HashSet::new();
    for (_, probes) in probes.iter() {
        for Probe { metadata, .. } in probes.iter() {
            for param in metadata.body_args.params.iter() {
                all_params.insert(param);
            }

            for param in metadata.pred_args.params.iter() {
                all_params.insert(param);
            }
        }
    }
    let mut req_args = ReqArgs::None;
    let probe_rule = ProbeRule {
        provider: Some(RulePart::new("wasm".to_string(), None)),
        package: Some(RulePart::new("opcode".to_string(), None)),
        event: Some(RulePart::new(event.clone(), None)),
        mode: None,
    };
    match event.as_str() {
        "unreachable" => if let Operator::Unreachable = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "nop" => if let Operator::Nop = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "block" => if let Operator::Block {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "loop" => if let Operator::Loop {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "if" => if let Operator::If {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "else" => if let Operator::Else {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "try_table" => if let Operator::TryTable {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "throw" => if let Operator::Throw { tag_index } = instr {
            define_imm0::<u32>(*tag_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "throw_ref" => if let Operator::ThrowRef {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "end" => if let Operator::End {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "br" => if let Operator::Br { relative_depth } = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "br_if" => if let Operator::BrIf { relative_depth } = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "br_table" => if let Operator::BrTable { targets } = instr {
            if bind_vars_br_table(targets, &mut loc_info, &all_params).is_none() {
                return None;
            }

            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "return" => if let Operator::Return {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "call" => if let Operator::Call {function_index} = instr {
            bind_vars_call(&mut loc_info, &all_params, *function_index, app_wasm);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "call_indirect" => if let Operator::CallIndirect {type_index,
            table_index,} = instr {
            define_imm0_u32_imm1_u32(*type_index, *table_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "return_call" => if let Operator::ReturnCall {function_index} = instr {
            bind_vars_call(&mut loc_info, &all_params, *function_index, app_wasm);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "return_call_indirect" => if let Operator::ReturnCallIndirect {type_index,
            table_index,} = instr {
            define_imm0_u32_imm1_u32(*type_index, *table_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "drop" => if let Operator::Drop = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "select" => if let Operator::Select = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "typed_select" => if let Operator::TypedSelect {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "local.get" => if let Operator::LocalGet {local_index} = instr {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "local.set" => if let Operator::LocalSet {local_index} = instr {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "local.tee" => if let Operator::LocalTee {local_index} = instr {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "global.get" => if let Operator::GlobalGet {global_index} = instr {
            define_imm0::<u32>(*global_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "global.set" => if let Operator::GlobalSet {global_index} = instr {
            define_imm0::<u32>(*global_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.load" => if let Operator::I32Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.load" => if let Operator::I64Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.load" => if let Operator::F32Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.load" => if let Operator::F64Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.load8_s" => if let Operator::I32Load8S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.load8_u" => if let Operator::I32Load8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.load16_s" => if let Operator::I32Load16S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.load16_u" => if let Operator::I32Load16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.load8_s" => if let Operator::I64Load8S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.load8_u" => if let Operator::I64Load8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.load16_s" => if let Operator::I64Load16S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.load16_u" => if let Operator::I64Load16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.load32_s" => if let Operator::I64Load32S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.load32_u" => if let Operator::I64Load32U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.store" => if let Operator::I32Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.store" => if let Operator::I64Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.store" => if let Operator::F32Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.store" => if let Operator::F64Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.store8" => if let Operator::I32Store8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.store16" => if let Operator::I32Store16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.store8" => if let Operator::I64Store8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.store16" => if let Operator::I64Store16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.store32" => if let Operator::I64Store32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.size" => if let Operator::MemorySize {mem} = instr {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.grow" => if let Operator::MemoryGrow {mem} = instr {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.const" => if let Operator::I32Const {value} = instr {
            define_imm0::<i32>(*value, DataType::I32, &Value::gen_i32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.const" => if let Operator::I64Const {value} = instr {
            define_imm0::<i64>(*value, DataType::I64, &Value::gen_i64, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.const" => if let Operator::F32Const {value} = instr {
            define_imm0::<f32>(f32::from(*value), DataType::F32, &Value::gen_f32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.const" => if let Operator::F64Const {value} = instr {
            define_imm0::<f64>(f64::from(*value), DataType::F64, &Value::gen_f64, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.null" => if let Operator::RefNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.is_null" => if let Operator::RefIsNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.func" => if let Operator::RefFunc {function_index} = instr {
            define_imm0::<u32>(*function_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.eq" => if let Operator::RefEq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.eqz" => if let Operator::I32Eqz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.eq" => if let Operator::I32Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.ne" => if let Operator::I32Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.lt_s" => if let Operator::I32LtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.lt_u" => if let Operator::I32LtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.gt_s" => if let Operator::I32GtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.gt_u" => if let Operator::I32GtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.le_s" => if let Operator::I32LeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.le_u" => if let Operator::I32LeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.ge_s" => if let Operator::I32GeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.ge_u" => if let Operator::I32GeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.eqz" => if let Operator::I64Eqz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.eq" => if let Operator::I64Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.ne" => if let Operator::I64Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.lt_s" => if let Operator::I64LtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.lt_u" => if let Operator::I64LtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.gt_s" => if let Operator::I64GtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.gt_u" => if let Operator::I64GtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.le_s" => if let Operator::I64LeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.le_u" => if let Operator::I64LeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.ge_s" => if let Operator::I64GeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.ge_u" => if let Operator::I64GeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.eq" => if let Operator::F32Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.ne" => if let Operator::F32Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.lt" => if let Operator::F32Lt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.gt" => if let Operator::F32Gt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.le" => if let Operator::F32Le {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.ge" => if let Operator::F32Ge {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.eq" => if let Operator::F64Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.ne" => if let Operator::F64Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.lt" => if let Operator::F64Lt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.gt" => if let Operator::F64Gt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.le" => if let Operator::F64Le {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.ge" => if let Operator::F64Ge {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.clz" => if let Operator::I32Clz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.ctz" => if let Operator::I32Ctz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.popcnt" => if let Operator::I32Popcnt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.add" => if let Operator::I32Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.sub" => if let Operator::I32Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.mul" => if let Operator::I32Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.div_s" => if let Operator::I32DivS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.div_u" => if let Operator::I32DivU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.rem_s" => if let Operator::I32RemS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.rem_u" => if let Operator::I32RemU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.and" => if let Operator::I32And {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.or" => if let Operator::I32Or {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.xor" => if let Operator::I32Xor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.shl" => if let Operator::I32Shl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.shr_s" => if let Operator::I32ShrS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.shr_u" => if let Operator::I32ShrU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.rotl" => if let Operator::I32Rotl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.rotr" => if let Operator::I32Rotr {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.clz" => if let Operator::I64Clz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.ctz" => if let Operator::I64Ctz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.popcnt" => if let Operator::I64Popcnt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.add" => if let Operator::I64Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.sub" => if let Operator::I64Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.mul" => if let Operator::I64Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.div_s" => if let Operator::I64DivS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.div_u" => if let Operator::I64DivU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.rem_s" => if let Operator::I64RemS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.rem_u" => if let Operator::I64RemU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.and" => if let Operator::I64And {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.or" => if let Operator::I64Or {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.xor" => if let Operator::I64Xor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.shl" => if let Operator::I64Shl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.shr_s" => if let Operator::I64ShrS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.shr_u" => if let Operator::I64ShrU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.rotl" => if let Operator::I64Rotl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.rotr" => if let Operator::I64Rotr {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.abs" => if let Operator::F32Abs {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.neg" => if let Operator::F32Neg {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.ceil" => if let Operator::F32Ceil {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.floor" => if let Operator::F32Floor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.trunc" => if let Operator::F32Trunc {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.nearest" => if let Operator::F32Nearest {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.sqrt" => if let Operator::F32Sqrt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.add" => if let Operator::F32Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.sub" => if let Operator::F32Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.mul" => if let Operator::F32Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.div" => if let Operator::F32Div {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.min" => if let Operator::F32Min {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.max" => if let Operator::F32Max {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.copysign" => if let Operator::F32Copysign {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.abs" => if let Operator::F64Abs {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.neg" => if let Operator::F64Neg {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.ceil" => if let Operator::F64Ceil {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.floor" => if let Operator::F64Floor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.trunc" => if let Operator::F64Trunc {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.nearest" => if let Operator::F64Nearest {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.sqrt" => if let Operator::F64Sqrt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.add" => if let Operator::F64Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.sub" => if let Operator::F64Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.mul" => if let Operator::F64Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.div" => if let Operator::F64Div {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.min" => if let Operator::F64Min {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.max" => if let Operator::F64Max {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.copysign" => if let Operator::F64Copysign {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.wrap_i64" => if let Operator::I32WrapI64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_f32_s" => if let Operator::I32TruncF32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_f32_u" => if let Operator::I32TruncF32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_f64_s" => if let Operator::I32TruncF64S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_f64_u" => if let Operator::I32TruncF64U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.extend_i32_s" => if let Operator::I64ExtendI32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.extend_i32_u" => if let Operator::I64ExtendI32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.trunc_f32_s" => if let Operator::I64TruncF32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.trunc_f32_u" => if let Operator::I64TruncF32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.convert_i32_s" => if let Operator::F32ConvertI32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.convert_i32_u" => if let Operator::F32ConvertI32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.convert_i64_s" => if let Operator::F32ConvertI64S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.convert_i64_u" => if let Operator::F32ConvertI64U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.demote_f64" => if let Operator::F32DemoteF64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.convert_i32_s" => if let Operator::F64ConvertI32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.convert_i32_u" => if let Operator::F64ConvertI32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.convert_i64_s" => if let Operator::F64ConvertI64S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.convert_i64_u" => if let Operator::F64ConvertI64U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.promote_f32" => if let Operator::F64PromoteF32 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.reinterpret_f32" => if let Operator::I32ReinterpretF32 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.reinterpret_f64" => if let Operator::I64ReinterpretF64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f32.reinterpret_i32" => if let Operator::F32ReinterpretI32 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "f64.reinterpret_i64" => if let Operator::F64ReinterpretI64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.extend8_s" => if let Operator::I32Extend8S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.extend16_s" => if let Operator::I32Extend16S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.extend8_s" => if let Operator::I64Extend8S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.extend16_s" => if let Operator::I64Extend16S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.extend32_s" => if let Operator::I64Extend32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "struct.new" => if let Operator::StructNew {struct_type_index} = instr {
            define_imm0::<u32>(*struct_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "struct.new_default" => if let Operator::StructNewDefault {struct_type_index} = instr {
            define_imm0::<u32>(*struct_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "struct.get" => if let Operator::StructGet {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "struct.get_s" => if let Operator::StructGetS {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "struct.get_u" => if let Operator::StructGetU {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "struct.set" => if let Operator::StructSet {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.new" => if let Operator::ArrayNew {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.new_default" => if let Operator::ArrayNewDefault {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.new_fixed" => if let Operator::ArrayNewFixed {array_type_index, array_size} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_size, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.new_data" => if let Operator::ArrayNewData {array_type_index, array_data_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_data_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.new_elem" => if let Operator::ArrayNewElem {array_type_index, array_elem_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_elem_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.get" => if let Operator::ArrayGet {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.get_s" => if let Operator::ArrayGetS {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.get_u" => if let Operator::ArrayGetU {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.set" => if let Operator::ArraySet {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.len" => if let Operator::ArrayLen = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.fill" => if let Operator::ArrayFill {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.copy" => if let Operator::ArrayCopy {array_type_index_dst, array_type_index_src} = instr {
            define_imm0_u32_imm1_u32(*array_type_index_dst, *array_type_index_src, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.init_data" => if let Operator::ArrayInitData {array_type_index, array_data_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_data_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "array.init_elem" => if let Operator::ArrayInitElem {array_type_index, array_elem_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_elem_index, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.test" => if let Operator::RefTestNonNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.test_null" => if let Operator::RefTestNullable {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.cast" => if let Operator::RefCastNonNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.cast_null" => if let Operator::RefCastNullable {..} = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "br_on_cast" => if let Operator::BrOnCast {relative_depth, ..} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "br_on_cast_fail" => if let Operator::BrOnCastFail {relative_depth, ..} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "any.convert_extern" => if let Operator::AnyConvertExtern = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "extern.convert_any" => if let Operator::ExternConvertAny = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.i31" => if let Operator::RefI31 = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i31.get_s" => if let Operator::I31GetS = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i31.get_u" => if let Operator::I31GetU = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_sat_f32_s" => if let Operator::I32TruncSatF32S = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_sat_f32_u" => if let Operator::I32TruncSatF32U = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_sat_f64_s" => if let Operator::I32TruncSatF64S = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.trunc_sat_f64_u" => if let Operator::I32TruncSatF64U = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.trunc_sat_f32_s" => if let Operator::I64TruncSatF32S = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.trunc_sat_f32_u" => if let Operator::I64TruncSatF32U = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.trunc_sat_f64_s" => if let Operator::I64TruncSatF64S = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.trunc_sat_f64_u" => if let Operator::I64TruncSatF64U = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.init" => if let Operator::MemoryInit {data_index, mem} = instr {
            define_imm0_u32_imm1_u32(*data_index, *mem, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.copy" => if let Operator::MemoryCopy {dst_mem, src_mem} = instr {
            define_imm0_u32_imm1_u32(*dst_mem, *src_mem, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.fill" => if let Operator::MemoryFill {mem} = instr {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "data.drop" => if let Operator::DataDrop {data_index} = instr {
            define_imm0::<u32>(*data_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "elem.drop" => if let Operator::ElemDrop {elem_index} = instr {
            define_imm0::<u32>(*elem_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "table.copy" => if let Operator::TableCopy {dst_table, src_table} = instr {
            define_imm0_u32_imm1_u32(*dst_table, *src_table, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "table.init" => if let Operator::TableInit {elem_index, table} = instr {
            define_imm0_u32_imm1_u32(*elem_index, *table, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "table.fill" => if let Operator::TableFill {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "table.get" => if let Operator::TableGet {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "table.set" => if let Operator::TableSet {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "table.grow" => if let Operator::TableGrow {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "table.size" => if let Operator::TableSize {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.atomic_notify" => if let Operator::MemoryAtomicNotify {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.atomic_wait32" => if let Operator::MemoryAtomicWait32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "memory.atomic_wait64" => if let Operator::MemoryAtomicWait64 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "atomic.fence" => if let Operator::AtomicFence = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_load" => if let Operator::I32AtomicLoad {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_load" => if let Operator::I64AtomicLoad {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_load8_u" => if let Operator::I32AtomicLoad8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_load16_u" => if let Operator::I32AtomicLoad16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_load8_u" => if let Operator::I64AtomicLoad8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_load16_u" => if let Operator::I64AtomicLoad16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_load32_u" => if let Operator::I64AtomicLoad32U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_store" => if let Operator::I32AtomicStore {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_store8" => if let Operator::I32AtomicStore8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_store16" => if let Operator::I32AtomicStore16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_store" => if let Operator::I64AtomicStore {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_store8" => if let Operator::I64AtomicStore8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_store16" => if let Operator::I64AtomicStore16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_store32" => if let Operator::I64AtomicStore32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw_add" => if let Operator::I32AtomicRmwAdd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw8_add_u" => if let Operator::I32AtomicRmw8AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw16_add_u" => if let Operator::I32AtomicRmw16AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw_add" => if let Operator::I64AtomicRmwAdd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw8_add_u" => if let Operator::I64AtomicRmw8AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw16_add_u" => if let Operator::I64AtomicRmw16AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw32_add_u" => if let Operator::I64AtomicRmw32AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw_sub" => if let Operator::I32AtomicRmwSub {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw8_sub_u" => if let Operator::I32AtomicRmw8SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw16_sub_u" => if let Operator::I32AtomicRmw16SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw_sub" => if let Operator::I64AtomicRmwSub {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw8_sub_u" => if let Operator::I64AtomicRmw8SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw16_sub_u" => if let Operator::I64AtomicRmw16SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw32_sub_u" => if let Operator::I64AtomicRmw32SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw_and" => if let Operator::I32AtomicRmwAnd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw8_and_u" => if let Operator::I32AtomicRmw8AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw16_and_u" => if let Operator::I32AtomicRmw16AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw_and" => if let Operator::I64AtomicRmwAnd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw8_and_u" => if let Operator::I64AtomicRmw8AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw16_and_u" => if let Operator::I64AtomicRmw16AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw32_and_u" => if let Operator::I64AtomicRmw32AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw_or" => if let Operator::I32AtomicRmwOr {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw8_or_u" => if let Operator::I32AtomicRmw8OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw16_or_u" => if let Operator::I32AtomicRmw16OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw_or" => if let Operator::I64AtomicRmwOr {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw8_or_u" => if let Operator::I64AtomicRmw8OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw16_or_u" => if let Operator::I64AtomicRmw16OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw32_or_u" => if let Operator::I64AtomicRmw32OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw_xor" => if let Operator::I32AtomicRmwXor {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw8_xor_u" => if let Operator::I32AtomicRmw8XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw16_xor_u" => if let Operator::I32AtomicRmw16XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw_xor" => if let Operator::I64AtomicRmwXor {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw8_xor_u" => if let Operator::I64AtomicRmw8XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw16_xor_u" => if let Operator::I64AtomicRmw16XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw32_xor_u" => if let Operator::I64AtomicRmw32XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw_xchg" => if let Operator::I32AtomicRmwXchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw8_xchg_u" => if let Operator::I32AtomicRmw8XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw16_xchg_u" => if let Operator::I32AtomicRmw16XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw_xchg" => if let Operator::I64AtomicRmwXchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw8_xchg_u" => if let Operator::I64AtomicRmw8XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw16_xchg_u" => if let Operator::I64AtomicRmw16XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw32_xchg_u" => if let Operator::I64AtomicRmw32XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw_cmpxchg" => if let Operator::I32AtomicRmwCmpxchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw8_cmpxchg_u" => if let Operator::I32AtomicRmw8CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i32.atomic_rmw16_cmpxchg_u" => if let Operator::I32AtomicRmw16CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw_cmpxchg" => if let Operator::I64AtomicRmwCmpxchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw8_cmpxchg_u" => if let Operator::I64AtomicRmw8CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw16_cmpxchg_u" => if let Operator::I64AtomicRmw16CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "i64.atomic_rmw32_cmpxchg_u" => if let Operator::I64AtomicRmw32CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "call_ref" => if let Operator::CallRef {type_index} = instr {
            define_imm0::<u32>(*type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "return_call_ref" => if let Operator::ReturnCallRef {type_index} = instr {
            define_imm0::<u32>(*type_index, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "ref.as_non_null" => if let Operator::RefAsNonNull = instr {
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "br_on_null" => if let Operator::BrOnNull {relative_depth} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        "br_on_non_null" => if let Operator::BrOnNonNull {relative_depth} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, &all_params);
            loc_info.add_probes(probe_rule.clone(), &probes);
        },
        _ => unimplemented!("Have not supported wasm:opcode:{event}")
    }

    let (all_args, ..) = get_ty_info_for_instr(app_wasm, fid, instr);

    // figure out which args are requested based on matched probes
    let mut probes_to_remove = vec![];
    for (i, (_, probe)) in loc_info.probes.iter_mut().enumerate() {
        if probe.metadata.body_args.req_args.matches(all_args.len()) {
            req_args.combine(&probe.metadata.body_args.req_args);
        } else {
            // remove probe!
            probes_to_remove.push(i);
            continue;
        }
        if probe.metadata.pred_args.req_args.matches(all_args.len()) {
            req_args.combine(&probe.metadata.pred_args.req_args);
        } else {
            // remove probe!
            probes_to_remove.push(i);
            continue;
        }
    }
    for i in probes_to_remove.iter() {
        loc_info.probes.remove(*i);
    }

    if req_args.is_some() {
        loc_info.args = req_args.of(all_args);
    }

    loc_info.is_prog_exit = is_prog_exit_call(instr, app_wasm);
    if loc_info.has_match() || loc_info.is_prog_exit {
        Some(loc_info)
    } else {
        None
    }
}

fn define_imm0_u32_imm1_u32(value0: u32, value1: u32, loc_info: &mut LocInfo, all_params: &HashSet<&WhammParam>) {
    for param in all_params {
        if let WhammParam::Imm { n, ty } = param {
            assert!(matches!(ty, DataType::U32));
            if *n == 0 {
                define_imm_n(*n, Some(Value::gen_u32(value0)), loc_info);
            } else if *n == 1 {
                define_imm_n(*n, Some(Value::gen_u32(value1)), loc_info);
            } else {
                panic!("WhammParam not available for opcode: {}", param);
            }
        }
    }
}

fn define_imm0<T>(value: T, _dt: DataType, gen: &dyn Fn(T) -> Value, loc_info: &mut LocInfo, all_params: &HashSet<&WhammParam>) {
    for param in all_params {
        if let WhammParam::Imm { n, ty } = param {
            assert_eq!(*n, 0);
            assert!(matches!(ty, _dt));

            define_imm_n(0, Some(gen(value)), loc_info);
            return;
        }
    }
}

fn define_imm_n(n: u32, val: Option<Value>, loc_info: &mut LocInfo) {
    loc_info.static_data.insert(format!("imm{n}"), val);
}

fn bind_vars_memarg(
    align: u8,
    offset: u64,
    memory: u32,
    loc_info: &mut LocInfo,
    all_params: &HashSet<&WhammParam>) {
    for param in all_params {
        match param {
            WhammParam::Align => {
                loc_info
                    .static_data
                    .insert("align".to_string(), Some(Value::gen_u32(align as u32)));
            }
            WhammParam::Offset => {
                loc_info
                    .static_data
                    .insert("offset".to_string(), Some(Value::gen_u64(offset)));
            }
            WhammParam::Memory => {
                loc_info
                    .static_data
                    .insert("memory".to_string(), Some(Value::gen_u32(memory)));
            }
            _ => {}
        }
    }
}
fn bind_vars_br_table(targets: &BrTable, loc_info: &mut LocInfo, all_params: &HashSet<&WhammParam>) -> Option<()> {
    for param in all_params {
        match param {
            WhammParam::NumTargets => {
                loc_info.static_data.insert(
                    WhammParam::NumTargets.to_string(),
                    Some(Value::gen_u32(targets.len())),
                );
            }
            WhammParam::DefaultTarget => {
                loc_info.static_data.insert(
                    WhammParam::DefaultTarget.to_string(),
                    Some(Value::gen_u32(targets.default())),
                );
            }
            WhammParam::Targets => {
                let mut target_map = HashMap::new();
                for (i, target) in targets.targets().enumerate() {
                    if let Ok(target) = target {
                        target_map.insert(i as u32, target);
                    }
                }
                loc_info.add_dynamic_value(
                    WhammParam::Targets.to_string(),
                    Value::U32U32Map {
                        val: Box::new(target_map),
                    },
                );
            }
            WhammParam::Imm { n, ty } => {
                if *n > targets.len() {
                    // this location doesn't match since the immN is out of bound
                    // of the immN's available
                    return None;
                }
                assert!(matches!(ty, DataType::U32));

                if *n == targets.len() {
                    // requesting the default value!
                    define_imm_n(
                        *n,
                        Some(Value::gen_u32(targets.default())),
                        loc_info,
                    );
                }

                for (i, target) in targets.targets().enumerate() {
                    if let Ok(target) = target {
                        if *n == i as u32 {
                            define_imm_n(
                                i as u32,
                                Some(Value::gen_u32(target)),
                                loc_info,
                            );
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Some(())
}

fn bind_vars_call(loc_info: &mut LocInfo, all_params: &HashSet<&WhammParam>, fid: u32, app_wasm: &Module) {
    let func_info = match app_wasm.functions.get_kind(FunctionID(fid)) {
        FuncKind::Import(ImportedFunction { import_id, .. }) => {
            let import = app_wasm.imports.get(*import_id);
            FuncInfo {
                func_kind: "import".to_string(),
                module: import.module.to_string(),
                name: import.name.to_string(),
            }
        }
        FuncKind::Local(LocalFunction { func_id, .. }) => FuncInfo {
            func_kind: "local".to_string(),
            module: match &app_wasm.module_name {
                Some(name) => name.clone(),
                None => "".to_string(),
            },
            name: match &app_wasm.functions.get_name(*func_id) {
                Some(name) => name.clone(),
                None => "".to_string(),
            },
        },
    };

    for param in all_params {
        match param {
            WhammParam::TargetFnName => {
                loc_info.static_data.insert(
                    "target_fn_name".to_string(),
                    Some(Value::Str {
                        val: func_info.name.to_string(),
                    }),
                );
            }
            WhammParam::TargetFnType => {
                loc_info.static_data.insert(
                    "target_fn_type".to_string(),
                    Some(Value::Str {
                        val: func_info.func_kind.to_string(),
                    }),
                );
            }
            WhammParam::TargetImpModule => {
                loc_info.static_data.insert(
                    "target_imp_module".to_string(),
                    Some(Value::Str {
                        val: func_info.module.to_string(),
                    }),
                );
            }
            WhammParam::Imm { n, ty } => {
                assert_eq!(*n, 0);
                assert!(matches!(ty, DataType::U32));

                define_imm_n(0, Some(Value::gen_u32(fid)), loc_info);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct FuncInfo {
    func_kind: String,
    module: String,
    name: String,
}

pub fn get_ty_info_for_instr(
    app_wasm: &Module,
    curr_fid: &FunctionID,
    instr: &Operator,
) -> (Vec<Arg>, Option<u32>) {
    // TODO -- how to make this less manual?
    let (ty_list, ty_id): (Vec<Option<OrcaType>>, Option<u32>) = match instr {
        Operator::Call {
            function_index: fid,
        } => {
            match app_wasm.functions.get_kind(FunctionID(*fid)) {
                FuncKind::Import(ImportedFunction { ty_id, .. })
                | FuncKind::Local(LocalFunction { ty_id, .. }) => {
                    if let Some(ty) = app_wasm.types.get(*ty_id) {
                        let mut res = vec![];
                        for t in ty.params().iter().rev() {
                            res.push(Some(*t));
                        }
                        (res, Some(**ty_id))
                    } else {
                        // no type info found!!
                        warn!("No type information found for import with FID {fid}");
                        (vec![], None)
                    }
                }
            }
        }
        Operator::If { .. } | Operator::BrIf { .. } | Operator::BrTable { .. } => {
            (vec![Some(OrcaType::I32)], None)
        }
        Operator::Block {
            blockty: wasmparser::BlockType::FuncType(ty_id),
        }
        | Operator::Loop {
            blockty: wasmparser::BlockType::FuncType(ty_id),
        } => {
            if let Some(ty) = app_wasm.types.get(TypeID(*ty_id)) {
                let mut res = vec![];
                for t in ty.params().iter() {
                    res.push(Some(*t));
                }
                (res, Some(*ty_id))
            } else {
                // no type info found!!
                warn!("No type information found for opcode");
                (vec![], None)
            }
        }
        Operator::CallIndirect { type_index, .. } => {
            if let Some(ty) = app_wasm.types.get(TypeID(*type_index)) {
                let mut res = vec![];
                for t in ty.params().iter().rev() {
                    res.push(Some(*t));
                }
                (res, Some(*type_index))
            } else {
                // no type info found!!
                warn!("No type information found for CallIndirect");
                (vec![], None)
            }
        }
        Operator::Drop => {
            // TODO -- how to express an unknown type?
            //     Lookup in the symbol table! We've placed type bounds in there during verification
            //     HOWEVER, we will need to keep a virtual stack to check if this match site is in fact
            //     a match based on the type bounds. (if they don't match up, not a match, don't emit)
            // e.g. [unknown]
            (vec![None], None)
        }
        Operator::Select => {
            // TODO -- how to express an unknown type?
            //     Lookup in the symbol table! We've placed type bounds in there during verification
            //     HOWEVER, we will need to keep a virtual stack to check if this match site is in fact
            //     a match based on the type bounds. (if they don't match up, not a match, don't emit)
            // e.g. [unknown, unknown, i32]
            (vec![None, None, Some(OrcaType::I32)], None)
        }
        Operator::LocalSet { local_index } | Operator::LocalTee { local_index } => {
            if let FuncKind::Local(LocalFunction { body, .. }) =
                app_wasm.functions.get_kind(*curr_fid)
            {
                if let Some((_, ty)) = body.locals.get(*local_index as usize) {
                    (vec![Some(*ty)], None)
                } else {
                    (vec![], None) // ignore
                }
            } else {
                (vec![], None) // ignore
            }
        }
        Operator::GlobalSet { global_index } => {
            let ty = match app_wasm.globals.get_kind(GlobalID(*global_index)) {
                GlobalKind::Import(ImportedGlobal {
                                       ty: GlobalType { content_type, .. },
                                       ..
                                   })
                | GlobalKind::Local(LocalGlobal {
                                        ty: GlobalType { content_type, .. },
                                        ..
                                    }) => OrcaType::from(*content_type),
            };
            (vec![Some(ty)], None)
        }
        Operator::I32Load { .. }
        | Operator::I64Load { .. }
        | Operator::F32Load { .. }
        | Operator::F64Load { .. }
        | Operator::I32Load8S { .. }
        | Operator::I32Load8U { .. }
        | Operator::I32Load16S { .. }
        | Operator::I32Load16U { .. }
        | Operator::I64Load8S { .. }
        | Operator::I64Load8U { .. }
        | Operator::I64Load16S { .. }
        | Operator::I64Load16U { .. }
        | Operator::I64Load32S { .. }
        | Operator::I64Load32U { .. } => (vec![Some(OrcaType::I32)], None),

        Operator::I32Store { .. }
        | Operator::I32Store8 { .. }
        | Operator::I32Store16 { .. } => (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None),
        Operator::I64Store { .. }
        | Operator::I64Store8 { .. }
        | Operator::I64Store16 { .. }
        | Operator::I64Store32 { .. } => (vec![Some(OrcaType::I64), Some(OrcaType::I32)], None),
        Operator::F32Store { .. } => (vec![Some(OrcaType::F32), Some(OrcaType::I32)], None),
        Operator::F64Store { .. } => (vec![Some(OrcaType::F64), Some(OrcaType::I32)], None),
        Operator::MemoryGrow { .. } => (vec![Some(OrcaType::I32)], None),

        Operator::I32Eqz => (vec![Some(OrcaType::I32)], None),
        Operator::I32Ne
        | Operator::I32Eq
        | Operator::I32LtS
        | Operator::I32LtU
        | Operator::I32GtS
        | Operator::I32GtU
        | Operator::I32LeS
        | Operator::I32LeU
        | Operator::I32GeS
        | Operator::I32GeU => (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None),

        Operator::I32Clz | Operator::I32Ctz | Operator::I32Popcnt => {
            (vec![Some(OrcaType::I32)], None)
        }

        Operator::I32Add
        | Operator::I32Sub
        | Operator::I32Mul
        | Operator::I32DivS
        | Operator::I32DivU
        | Operator::I32RemS
        | Operator::I32RemU
        | Operator::I32And
        | Operator::I32Or
        | Operator::I32Xor
        | Operator::I32Shl
        | Operator::I32ShrS
        | Operator::I32ShrU
        | Operator::I32Rotl
        | Operator::I32Rotr => (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None),

        Operator::I64Eqz => (vec![Some(OrcaType::I64)], None),
        Operator::I64Eq
        | Operator::I64Ne
        | Operator::I64LtS
        | Operator::I64LtU
        | Operator::I64GtS
        | Operator::I64GtU
        | Operator::I64LeS
        | Operator::I64LeU
        | Operator::I64GeS
        | Operator::I64GeU => (vec![Some(OrcaType::I64), Some(OrcaType::I64)], None),

        Operator::I64Clz | Operator::I64Ctz | Operator::I64Popcnt => {
            (vec![Some(OrcaType::I64)], None)
        }
        Operator::I64Add
        | Operator::I64Sub
        | Operator::I64Mul
        | Operator::I64DivS
        | Operator::I64DivU
        | Operator::I64RemS
        | Operator::I64RemU
        | Operator::I64And
        | Operator::I64Or
        | Operator::I64Xor
        | Operator::I64Shl
        | Operator::I64ShrS
        | Operator::I64ShrU
        | Operator::I64Rotl
        | Operator::I64Rotr => (vec![Some(OrcaType::I64), Some(OrcaType::I64)], None),

        Operator::F32Eq
        | Operator::F32Ne
        | Operator::F32Lt
        | Operator::F32Gt
        | Operator::F32Le
        | Operator::F32Ge => (vec![Some(OrcaType::F32), Some(OrcaType::F32)], None),

        Operator::F32Abs
        | Operator::F32Neg
        | Operator::F32Ceil
        | Operator::F32Floor
        | Operator::F32Trunc
        | Operator::F32Nearest
        | Operator::F32Sqrt => (vec![Some(OrcaType::F32)], None),
        Operator::F32Add
        | Operator::F32Sub
        | Operator::F32Mul
        | Operator::F32Div
        | Operator::F32Min
        | Operator::F32Max
        | Operator::F32Copysign => (vec![Some(OrcaType::F32), Some(OrcaType::F32)], None),

        Operator::F64Eq
        | Operator::F64Ne
        | Operator::F64Lt
        | Operator::F64Gt
        | Operator::F64Le
        | Operator::F64Ge => (vec![Some(OrcaType::F64), Some(OrcaType::F64)], None),

        Operator::F64Abs
        | Operator::F64Neg
        | Operator::F64Ceil
        | Operator::F64Floor
        | Operator::F64Trunc
        | Operator::F64Nearest
        | Operator::F64Sqrt => (vec![Some(OrcaType::F32)], None),
        Operator::F64Add
        | Operator::F64Sub
        | Operator::F64Mul
        | Operator::F64Div
        | Operator::F64Min
        | Operator::F64Max
        | Operator::F64Copysign => (vec![Some(OrcaType::F64), Some(OrcaType::F64)], None),

        Operator::I32WrapI64
        | Operator::F32ConvertI64S
        | Operator::F32ConvertI64U
        | Operator::F64ConvertI64S
        | Operator::F64ConvertI64U
        | Operator::F64ReinterpretI64
        | Operator::I64Extend8S
        | Operator::I64Extend16S
        | Operator::I64Extend32S => (vec![Some(OrcaType::I64)], None),
        Operator::I32TruncF32S | Operator::I32TruncF32U => (vec![Some(OrcaType::F32)], None),
        Operator::I32TruncF64S
        | Operator::I32TruncF64U
        | Operator::I64TruncF64S
        | Operator::I64TruncF64U
        | Operator::F32DemoteF64
        | Operator::I64ReinterpretF64
        | Operator::I32TruncSatF64S
        | Operator::I32TruncSatF64U
        | Operator::I64TruncSatF64S
        | Operator::I64TruncSatF64U => (vec![Some(OrcaType::F64)], None),
        Operator::I64ExtendI32S
        | Operator::I64ExtendI32U
        | Operator::F32ConvertI32S
        | Operator::F32ConvertI32U
        | Operator::F64ConvertI32S
        | Operator::F64ConvertI32U
        | Operator::F32ReinterpretI32
        | Operator::I32Extend8S
        | Operator::I32Extend16S => (vec![Some(OrcaType::I32)], None),
        Operator::I64TruncF32S
        | Operator::I64TruncF32U
        | Operator::F64PromoteF32
        | Operator::I32ReinterpretF32
        | Operator::I32TruncSatF32S
        | Operator::I32TruncSatF32U
        | Operator::I64TruncSatF32S
        | Operator::I64TruncSatF32U => (vec![Some(OrcaType::F32)], None),

        Operator::MemoryCopy { .. }
        | Operator::MemoryFill { .. }
        | Operator::TableInit { .. }
        | Operator::TableCopy { .. } => (
            vec![
                Some(OrcaType::I32),
                Some(OrcaType::I32),
                Some(OrcaType::I32),
            ],
            None,
        ),

        Operator::TableGet { .. } => (vec![Some(OrcaType::I32)], None),

        Operator::MemoryAtomicNotify { .. } => {
            (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None)
        }
        Operator::MemoryAtomicWait32 { .. } => (
            vec![
                Some(OrcaType::I32),
                Some(OrcaType::I32),
                Some(OrcaType::I64),
            ],
            None,
        ),
        Operator::MemoryAtomicWait64 { .. } => (
            vec![
                Some(OrcaType::I32),
                Some(OrcaType::I64),
                Some(OrcaType::I64),
            ],
            None,
        ),

        Operator::I32AtomicLoad { .. }
        | Operator::I64AtomicLoad { .. }
        | Operator::I32AtomicLoad8U { .. }
        | Operator::I32AtomicLoad16U { .. }
        | Operator::I64AtomicLoad8U { .. }
        | Operator::I64AtomicLoad16U { .. }
        | Operator::I64AtomicLoad32U { .. } => (vec![Some(OrcaType::I32)], None),

        Operator::I32AtomicStore { .. }
        | Operator::I32AtomicStore8 { .. }
        | Operator::I32AtomicStore16 { .. } => (vec![Some(OrcaType::I32)], None),

        Operator::I64AtomicStore { .. }
        | Operator::I64AtomicStore8 { .. }
        | Operator::I64AtomicStore16 { .. }
        | Operator::I64AtomicStore32 { .. } => (vec![Some(OrcaType::I32)], None),

        Operator::I32AtomicRmwAdd { .. }
        | Operator::I32AtomicRmw8AddU { .. }
        | Operator::I32AtomicRmw16AddU { .. }
        | Operator::I32AtomicRmwSub { .. }
        | Operator::I32AtomicRmw8SubU { .. }
        | Operator::I32AtomicRmw16SubU { .. }
        | Operator::I32AtomicRmwAnd { .. }
        | Operator::I32AtomicRmw8AndU { .. }
        | Operator::I32AtomicRmw16AndU { .. }
        | Operator::I32AtomicRmwOr { .. }
        | Operator::I32AtomicRmw8OrU { .. }
        | Operator::I32AtomicRmw16OrU { .. }
        | Operator::I32AtomicRmwXor { .. }
        | Operator::I32AtomicRmw8XorU { .. }
        | Operator::I32AtomicRmw16XorU { .. }
        | Operator::I32AtomicRmwXchg { .. }
        | Operator::I32AtomicRmw8XchgU { .. }
        | Operator::I32AtomicRmw16XchgU { .. }
        | Operator::I32AtomicRmwCmpxchg { .. }
        | Operator::I32AtomicRmw8CmpxchgU { .. }
        | Operator::I32AtomicRmw16CmpxchgU { .. } => {
            (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None)
        }

        Operator::I64AtomicRmwAdd { .. }
        | Operator::I64AtomicRmw8AddU { .. }
        | Operator::I64AtomicRmw16AddU { .. }
        | Operator::I64AtomicRmw32AddU { .. }
        | Operator::I64AtomicRmwSub { .. }
        | Operator::I64AtomicRmw8SubU { .. }
        | Operator::I64AtomicRmw16SubU { .. }
        | Operator::I64AtomicRmw32SubU { .. }
        | Operator::I64AtomicRmwAnd { .. }
        | Operator::I64AtomicRmw8AndU { .. }
        | Operator::I64AtomicRmw16AndU { .. }
        | Operator::I64AtomicRmw32AndU { .. }
        | Operator::I64AtomicRmwOr { .. }
        | Operator::I64AtomicRmw8OrU { .. }
        | Operator::I64AtomicRmw16OrU { .. }
        | Operator::I64AtomicRmw32OrU { .. }
        | Operator::I64AtomicRmwXor { .. }
        | Operator::I64AtomicRmw8XorU { .. }
        | Operator::I64AtomicRmw16XorU { .. }
        | Operator::I64AtomicRmw32XorU { .. }
        | Operator::I64AtomicRmwXchg { .. }
        | Operator::I64AtomicRmw8XchgU { .. }
        | Operator::I64AtomicRmw16XchgU { .. }
        | Operator::I64AtomicRmw32XchgU { .. }
        | Operator::I64AtomicRmwCmpxchg { .. }
        | Operator::I64AtomicRmw8CmpxchgU { .. }
        | Operator::I64AtomicRmw16CmpxchgU { .. }
        | Operator::I64AtomicRmw32CmpxchgU { .. } => {
            (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None)
        }

        Operator::Unreachable
        | Operator::Nop
        | Operator::Else
        | Operator::End
        | Operator::Br { .. }
        | Operator::Return
        | Operator::LocalGet { .. }
        | Operator::GlobalGet { .. }
        | Operator::MemorySize { .. }
        | Operator::I32Const { .. }
        | Operator::I64Const { .. }
        | Operator::F32Const { .. }
        | Operator::F64Const { .. }
        | Operator::StructNewDefault { .. }
        | Operator::MemoryInit { .. }
        | Operator::DataDrop { .. }
        | Operator::ElemDrop { .. }
        | Operator::RefNull { .. }
        | Operator::RefFunc { .. }
        | Operator::TableSize { .. }
        | Operator::AtomicFence => (vec![], None),

        _ => (vec![], None), // ignore other opcodes
    };

    let mut args = vec![];
    for (idx, ty) in ty_list.iter().enumerate() {
        args.push(Arg::new(format!("arg{}", idx), ty.to_owned()));
    }

    (args, ty_id)
}


/// A function that can be used to generate these emitter rule types
/// from the SimpleAstProbes type created by the behavior tree builder.
/// See the documentation for this type for why this works when retaining
/// composable instrumentation ordering.
/// The design decision for generating emitter types from this new AST representation
/// is motivated by the constraints of the Rust type system. The following is other
/// designs that were considered and why they are not possible in the Rust PL.
///
/// 1. Add `From` trait to parser Provider/Package/Event/Mode types to translate to emitter variations
///    - Will not work since the `From` implementation would be tied to the structs implementing the
///      underlying Provider/Package/Event/Mode traits. From the AST perspective, all we know is that
///      we have a `dyn Provider|Package|Event|Mode`, not a specific implementation of it.
/// 2. Explicitly visit the AST to generate corresponding emitter variations
///    - This is the same problem as #1.
/// 3. Match on the `*Kind` enum variants instead of String names
///    - This isn't doable since we have specific `*Kind` enums per Provider/Package/Event/Mode trait.
///      So, we can't add a new function `get_kind(&self) -> *Kind` to the trait since we can't tie
///      the return type to a specific `*Kind` enum.
/// 4. Add `ProcessLoc` trait directly to parser Provider/Package/Event/Mode types
///    - This is the same issue as #1.
///
/// All this being said, the best design we have here is to basically create a new factory pattern
/// that iterates over the SimpleAstProbes built by the behavior tree builder to match Provider/Package/Event/Mode
/// names to the corresponding emitter variation.
/// This will keep the ordering guarantees for composable instrumentation by construction of the type and
/// enable us to work around the annoying Rust type system constraints. This will also keep the emitter logic
/// separate from the parser/verifier/behavior tree logic and keep this emitter logic specific to the bytecode
/// rewriting injection strategy.
pub fn provider_factory<P: Provider + FromStr>(ast: &SimpleAstProbes) -> Vec<Box<P>> {
    // Track the added provider hierarchies.
    // When visiting the next provider hierarchy it will be added (if not already there)
    // OR the new hierarchy will be appended within its respectful location in the already-existing one.
    // This is relevant when considering multiple scripts!
    let mut providers: Vec<Box<P>> = vec![];
    ast.iter().for_each(|(provider_name, packages)| {
        let mut provider = P::from_str(provider_name);
        provider.add_packages(packages);

        providers.push(Box::new(provider));
    });

    providers
}
/// Splits out the logic to add new packages to a provider
fn package_factory<P: Package + FromStr + 'static>(
    ast_packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>,
) -> Vec<Box<dyn Package>> {
    let mut packages: Vec<Box<dyn Package>> = vec![];
    ast_packages.iter().for_each(|(package_name, events)| {
        let mut package = P::from_str(package_name);
        package.add_events(events);

        packages.push(Box::new(package));
    });
    packages
}
/// Splits out the logic to add new events to a package
fn event_factory<E: Event + FromStr + 'static>(
    ast_events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>,
) -> Vec<Box<dyn Event>> {
    let mut events: Vec<Box<dyn Event>> = vec![];
    ast_events.iter().for_each(|(event_name, probes)| {
        let mut event = E::from_str(event_name);
        event.add_probes(probes);

        events.push(Box::new(event));
    });
    events
}
fn probe_factory(
    ast_probes: &HashMap<WhammModeKind, Vec<Probe>>,
) -> HashMap<WhammModeKind, Vec<Probe>> {
    ast_probes
        .iter()
        .map(|(name, probe_list)| {
            // it would be nice to not have to do this iteration, but I don't know of another way...
            let mut new_list = vec![];
            probe_list.iter().for_each(|probe| {
                new_list.push(probe.to_owned());
            });

            (name.clone(), new_list)
        })
        .collect()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Arg {
    pub name: String,
    pub ty: Option<OrcaType>,
}
impl Arg {
    fn new(name: String, ty: Option<OrcaType>) -> Self {
        Self { name, ty }
    }
}

#[derive(Clone, Debug)]
pub struct ProbeRule {
    pub provider: Option<RulePart>,
    pub package: Option<RulePart>,
    pub event: Option<RulePart>,
    pub mode: Option<WhammModeKind>,
}
impl Display for ProbeRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let curr_provider = match &self.provider {
            Some(provider) => provider.name.clone(),
            None => "".to_string(),
        };
        let curr_package = match &self.package {
            Some(package) => package.name.clone(),
            None => "".to_string(),
        };
        let curr_event = match &self.event {
            Some(event) => event.name.clone(),
            None => "".to_string(),
        };
        let curr_mode = match &self.mode {
            Some(mode) => mode.name().clone(),
            None => "".to_string(),
        };
        write!(
            f,
            "{}:{}:{}:{}",
            curr_provider, curr_package, curr_event, curr_mode
        )
    }
}

#[derive(Default, Debug)]
pub struct LocInfo {
    /// Whether this location calls something that exits the program
    pub is_prog_exit: bool,
    /// static information to be saved in symbol table
    pub static_data: HashMap<String, Option<Value>>,
    /// dynamic information to be defined at the probe location
    pub dynamic_data: HashMap<String, Block>,
    /// dynamic information corresponding to the operands of this location
    pub(crate) args: Vec<Arg>,
    pub num_alt_probes: usize,
    /// the probes that were matched for this instruction
    /// note the Script ID is contained in Probe
    pub probes: Vec<(ProbeRule, Probe)>,
}
impl LocInfo {
    fn new() -> Self {
        Self::default()
    }
    fn has_match(&self) -> bool {
        !self.probes.is_empty()
    }
    fn add_probes(&mut self, base_rule: ProbeRule, probes: &HashMap<WhammModeKind, Vec<Probe>>) {
        probes.iter().for_each(|(probe_mode, probes)| {
            let mut rule = base_rule.clone();
            rule.mode = Some(probe_mode.clone());

            if matches!(probe_mode, WhammModeKind::Alt) {
                // this is an alt probe, mark it with the number!
                self.num_alt_probes += probes.len();
            }
            probes.iter().for_each(|probe| {
                // TODO -- remove this probe.clone()...works for now though...
                self.probes.push((rule.clone(), probe.clone()));
            });
        })
    }
    fn add_dynamic_value(&mut self, name: String, val: Value) {
        let var_id = Expr::VarId {
            definition: Definition::CompilerDynamic,
            name: name.clone(),
            loc: None,
        };
        match &val {
            Value::Number {
                val: NumLit::U8 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_u8(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I8 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_i8(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::U16 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_u16(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I16 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_i16(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::U32 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U32,
                Expr::Primitive {
                    val: Value::gen_u32(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I32 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::I32,
                Expr::Primitive {
                    val: Value::gen_i32(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::F32 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::F32,
                Expr::Primitive {
                    val: Value::gen_f32(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::U64 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::U64,
                Expr::Primitive {
                    val: Value::gen_u64(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::I64 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::I64,
                Expr::Primitive {
                    val: Value::gen_i64(*val),
                    loc: None,
                },
            ),
            Value::Number {
                val: NumLit::F64 { val },
                ..
            } => self.add_dynamic_assign(
                name,
                DataType::I64,
                Expr::Primitive {
                    val: Value::gen_f64(*val),
                    loc: None,
                },
            ),
            Value::Boolean { val, .. } => self.add_dynamic_assign(
                name,
                DataType::Boolean,
                Expr::Primitive {
                    val: Value::Boolean { val: *val },
                    loc: None,
                },
            ),
            Value::Str { val, .. } => self.add_dynamic_assign(
                name,
                DataType::Str,
                Expr::Primitive {
                    val: Value::Str { val: val.clone() },
                    loc: None,
                },
            ),
            Value::Tuple { vals, ty } => self.add_dynamic_assign(
                name,
                ty.clone(),
                Expr::Primitive {
                    val: Value::Tuple {
                        ty: ty.clone(),
                        vals: vals.clone(),
                    },
                    loc: None,
                },
            ),
            Value::U32U32Map { val: map_val } => {
                // create a declaration
                let decl = Statement::Decl {
                    ty: val.ty(),
                    var_id: var_id.clone(),
                    loc: None,
                };
                // create assignments
                let mut stmts = vec![decl];
                for (key, val) in map_val.iter() {
                    stmts.push(Statement::SetMap {
                        map: var_id.clone(),
                        key: Expr::Primitive {
                            val: Value::gen_u32(*key),
                            loc: None,
                        },
                        val: Expr::Primitive {
                            val: Value::gen_u32(*val),
                            loc: None,
                        },
                        loc: None,
                    });
                }
                self.add_dynamic_block(
                    name,
                    Block {
                        stmts,
                        results: None,
                        loc: None,
                    },
                );
            }
        };
    }
    fn add_dynamic_assign(&mut self, name: String, ty: DataType, expr: Expr) {
        let var_id = Expr::VarId {
            definition: Definition::CompilerDynamic,
            name: name.clone(),
            loc: None,
        };

        // create a declaration
        let decl = Statement::Decl {
            ty,
            var_id: var_id.clone(),
            loc: None,
        };
        // create an assignment
        let assign = Statement::Assign {
            var_id: var_id.clone(),
            expr,
            loc: None,
        };

        self.add_dynamic_block(
            name,
            Block {
                stmts: vec![decl, assign],
                results: None,
                loc: None,
            },
        );
    }
    fn add_dynamic_block(&mut self, name: String, block: Block) {
        self.dynamic_data.insert(name, block);
    }
    fn append(&mut self, other: &mut Self) {
        // handle static_data
        self.static_data.extend(other.static_data.to_owned());

        // handle dynamic_data
        self.dynamic_data.extend(other.dynamic_data.to_owned());

        // handle args
        if !self.args.is_empty() {
            if !other.args.is_empty() {
                // assert that args are equivalent
                if !self.args.iter().all(|item| other.args.contains(item)) {
                    panic!("Emitter rules found different values for instruction args, please report this bug!");
                }
            }
            // just keep self args the way it is (other clearly doesn't populate them)
        } else {
            // just set to the other's args
            self.args = other.args.to_owned()
        }

        // handle num_alt_probes
        self.num_alt_probes += other.num_alt_probes;

        // handle function end
        self.is_prog_exit = self.is_prog_exit || other.is_prog_exit;

        // handle probes
        self.probes.append(&mut other.probes);
    }
}

pub trait Provider {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info(&self, app_wasm: &Module, loc: Location, instr: &Operator) -> Option<LocInfo>;
    fn add_packages(
        &mut self,
        ast_packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>,
    );
}
pub trait Package {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info(
        &self,
        app_wasm: &Module,
        curr_fid: &FunctionID,
        instr: &Operator,
    ) -> Option<LocInfo>;
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>);
}
pub trait Event {
    /// Pass some location to the provider and get back two types of data:
    fn get_loc_info(
        &self,
        app_wasm: &Module,
        fid: &FunctionID,
        instr: &Operator,
    ) -> Option<LocInfo>;
    fn add_probes(&mut self, ast_probes: &HashMap<WhammModeKind, Vec<Probe>>);
}

pub struct WhammProvider {
    kind: WhammProviderKind,
    /// The packages of the probes that have been used in the Script.
    pub packages: Vec<Box<dyn Package>>,
}
impl FromStr for WhammProvider {
    fn from_str(name: &str) -> Self {
        match name {
            "core" => Self::core(),
            "wasm" => Self::wasm(),
            _ => panic!("unsupported WhammProvider: {name}"),
        }
    }
}
impl WhammProvider {
    fn core() -> Self {
        Self {
            kind: WhammProviderKind::Core,
            packages: vec![],
        }
    }
    fn wasm() -> Self {
        Self {
            kind: WhammProviderKind::Wasm,
            packages: vec![],
        }
    }
}
impl Provider for WhammProvider {
    fn get_loc_info(&self, app_wasm: &Module, loc: Location, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();
        let (fid, pc, fname) = match loc {
            Location::Module {
                func_idx,
                instr_idx,
            }
            | Location::Component {
                func_idx,
                instr_idx,
                ..
            } => {
                let mut fname = String::default();
                let name = app_wasm.functions.get_name(func_idx).as_ref();
                if let Some(name) = name {
                    fname = name.clone();
                }
                (func_idx, instr_idx, fname)
            }
        };

        match self.kind {
            WhammProviderKind::Wasm => {
                // if *fid == 30 {
                //     println!("we're here!!")
                // }
                loc_info
                    .static_data
                    .insert("fid".to_string(), Some(Value::gen_u32(*fid)));

                loc_info
                    .static_data
                    .insert("fname".to_string(), Some(Value::Str { val: fname.clone() }));

                // Don't think we need this right now...
                // loc_info.static_data.insert(
                //     "wasm_bytecode_loc".to_string(),
                //     Some(Value::U32 {
                //         ty: DataType::U32,
                //         val: pc,
                //     }),
                // );

                loc_info
                    .static_data
                    .insert("pc".to_string(), Some(Value::gen_u32(pc as u32)));
            }
            WhammProviderKind::Core => {
                // nothing to add
            }
        }

        // Get location info from the rest of the configured rules
        self.packages.iter().for_each(|package| {
            if let Some(mut other_loc_info) = package.get_loc_info(app_wasm, &fid, instr) {
                loc_info.append(&mut other_loc_info);
            }
        });

        if loc_info.has_match() || loc_info.is_prog_exit {
            Some(loc_info)
        } else {
            None
        }
    }
    fn add_packages(
        &mut self,
        ast_packages: &HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>>,
    ) {
        let packages = match self.kind {
            WhammProviderKind::Core => package_factory::<CorePackage>(ast_packages),
            WhammProviderKind::Wasm => package_factory::<WasmPackage>(ast_packages),
        };
        self.packages = packages;
    }
}

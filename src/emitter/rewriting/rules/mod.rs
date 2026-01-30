use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::generator::ast::{Probe, StackReq, WhammParam};
use crate::generator::rewriting::simple_ast::{SimpleAST, SimpleEvt, SimplePkg, SimpleProv};
use crate::parser::provider_handler::ModeKind;
use crate::parser::types::{Block, DataType, Definition, Expr, NumLit, RulePart, Statement, Value};
use crate::verifier::types::VarAddr;
use log::warn;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use wirm::Location;
use wirm::ir::id::{FunctionID, GlobalID, TypeID};
use wirm::ir::module::Module;
use wirm::ir::module::module_functions::{FuncKind, ImportedFunction};
use wirm::ir::module::module_globals::{GlobalKind, ImportedGlobal, LocalGlobal};
use wirm::ir::module::module_types::Types;
use wirm::ir::types::{DataType as WirmType, InstrumentationMode};
use wirm::wasmparser::{BlockType, BrTable, GlobalType, MemArg, Operator};

pub fn get_loc_info_for_active_probes(
    app_wasm: &Module,
    state: &mut MatchState,
    loc: Location,
    at_func_end: bool,
    instr: &Operator,
    ast: &mut SimpleAST,
) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    for (provider, packages) in ast.provs.iter_mut() {
        if let Some(mut tmp) =
            handle_provider(app_wasm, state, loc, at_func_end, instr, provider, packages)
        {
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
    state: &mut MatchState,
    loc: Location,
    at_func_end: bool,
    instr: &Operator,
    provider: &str,
    prov: &mut SimpleProv,
) -> Option<LocInfo> {
    match provider {
        "wasm" => handle_wasm(app_wasm, state, loc, at_func_end, instr, prov),
        _ => panic!("Provider not available: {provider}"),
    }
}

fn handle_wasm(
    app_wasm: &Module,
    state: &mut MatchState,
    loc: Location,
    at_func_end: bool,
    instr: &Operator,
    prov: &mut SimpleProv,
) -> Option<LocInfo> {
    let mut loc_info = LocInfo::new();
    let (fid, opidx, pc, fname) = match loc {
        Location::Module {
            func_idx,
            instr_idx,
        }
        | Location::Component {
            func_idx,
            instr_idx,
            ..
        } => {
            let fname = app_wasm
                .functions
                .get_name(func_idx)
                .clone()
                .unwrap_or_default();
            let pc = VisitingEmitter::lookup_pc_offset_for(app_wasm, &loc);
            (func_idx, instr_idx, pc, fname)
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
        .insert("opidx".to_string(), Some(Value::gen_u32(opidx as u32)));

    loc_info
        .static_data
        .insert("pc".to_string(), Some(Value::gen_u32(pc)));

    loc_info.static_data.insert(
        "at_func_end".to_string(),
        Some(Value::Boolean { val: at_func_end }),
    );

    for param in prov.all_params() {
        if let Some(n) = param.n_for("local") {
            let func = app_wasm.functions.get(fid).unwrap_local();

            let wasm_ty = if n < func.args.len() as u32 {
                // referring to a function argument
                if let Some(Types::FuncType { params, .. }) = app_wasm.types.get(func.ty_id) {
                    params.get(n as usize)?
                } else {
                    panic!(
                        "Unable to lookup the function type with ID: {}",
                        *func.ty_id
                    );
                }
            } else {
                // referring to a function local variable
                if let Some((_, wasm_ty)) = func.body.locals.get(n as usize) {
                    wasm_ty
                } else {
                    // no match! not correct local var context in this function
                    return None;
                }
            };

            if param
                .ty
                .is_compatible_with(&DataType::from_wasm_type(wasm_ty))
            {
                loc_info
                    .dynamic_alias
                    .insert(format!("local{n}"), (*wasm_ty, VarAddr::Local { addr: n }));
                continue;
            } else {
                // no match! not correct local var context in this function
                return None;
            }
        }
    }

    let mut res: Option<LocInfo> = Some(loc_info);
    for (package, pkg) in prov.pkgs.iter_mut() {
        if let Some(mut tmp) = handle_wasm_packages(
            app_wasm,
            state,
            at_func_end,
            &fid,
            opidx,
            instr,
            package,
            pkg,
        ) {
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
    state: &mut MatchState,
    at_func_end: bool,
    fid: &FunctionID,
    opidx: usize,
    instr: &Operator,
    package: &str,
    pkg: &mut SimplePkg,
) -> Option<LocInfo> {
    match package {
        "opcode" => handle_opcode(app_wasm, fid, instr, pkg),
        "func" => handle_func(app_wasm, fid, opidx, instr, pkg),
        "block" => handle_block(app_wasm, state, at_func_end, fid, opidx, instr, pkg),
        "begin" | "end" => unimplemented!("Have not implemented the package yet: {package}"),
        "report" => None, // not handled here
        _ => panic!("Package not available: 'wasm:{package}'"),
    }
}

fn handle_opcode(
    app_wasm: &Module,
    fid: &FunctionID,
    instr: &Operator,
    pkg: &mut SimplePkg,
) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    for (package, evt) in pkg.evts.iter_mut() {
        // See OpcodeEvent.get_loc_info
        if let Some(mut tmp) = handle_opcode_events(app_wasm, fid, instr, package, evt) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }
    res
}

#[rustfmt::skip]
fn handle_opcode_events(
    app_wasm: &Module,
    fid: &FunctionID,
    instr: &Operator,
    event: &String,
    evt: &mut SimpleEvt,
) -> Option<LocInfo> {
    let mut loc_info = LocInfo::new();

    loc_info
        .static_data
        .insert("opname".to_string(), Some(Value::Str{val: event.clone()}));

    // create a combination of WhammParams for all probes here
    let all_params = evt.all_params();
    let mut req_args = StackReq::None;
    let mut req_results = StackReq::None;
    let probe_rule = ProbeRule {
        provider: Some(RulePart::new("wasm".to_string(), None)),
        package: Some(RulePart::new("opcode".to_string(), None)),
        event: Some(RulePart::new(event.clone(), None)),
        mode: None,
    };

    match event.as_str() {
        "unreachable" => if let Operator::Unreachable = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "nop" => if let Operator::Nop = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "block" => if let Operator::Block {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "loop" => if let Operator::Loop {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "if" => if let Operator::If {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "else" => if let Operator::Else {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "try_table" => if let Operator::TryTable {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "throw" => if let Operator::Throw { tag_index } = instr {
            define_imm0::<u32>(*tag_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "throw_ref" => if let Operator::ThrowRef {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "end" => if let Operator::End {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "br" => if let Operator::Br { relative_depth } = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "br_if" => if let Operator::BrIf { relative_depth } = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "br_table" => if let Operator::BrTable { targets } = instr {
            bind_vars_br_table(targets, &mut loc_info, all_params)?;
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "return" => if let Operator::Return {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "call" => if let Operator::Call {function_index} = instr {
            if bind_vars_call(&mut loc_info, all_params, *function_index, app_wasm).is_ok() {
                loc_info.add_probes(probe_rule.clone(), evt, None);
            }
        },
        "call_indirect" => if let Operator::CallIndirect {type_index,
            table_index,} = instr {
            define_imm0_u32_imm1_u32(*type_index, *table_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "return_call" => if let Operator::ReturnCall {function_index} = instr {
            if bind_vars_call(&mut loc_info, all_params, *function_index, app_wasm).is_ok() {
                loc_info.add_probes(probe_rule.clone(), evt, None);
            }
        },
        "return_call_indirect" => if let Operator::ReturnCallIndirect {type_index, table_index } = instr {
            define_imm0_u32_imm1_u32(*type_index, *table_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "drop" => if let Operator::Drop = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "select" => if let Operator::Select = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "typed_select" => if let Operator::TypedSelect {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "local.get" => if let Operator::LocalGet {local_index} = instr {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "local.set" => if let Operator::LocalSet {local_index} = instr {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "local.tee" => if let Operator::LocalTee {local_index} = instr {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "global.get" => if let Operator::GlobalGet {global_index} = instr {
            define_imm0::<u32>(*global_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "global.set" => if let Operator::GlobalSet {global_index} = instr {
            define_imm0::<u32>(*global_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.load" => if let Operator::I32Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.load" => if let Operator::I64Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.load" => if let Operator::F32Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.load" => if let Operator::F64Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.load8_s" => if let Operator::I32Load8S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.load8_u" => if let Operator::I32Load8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.load16_s" => if let Operator::I32Load16S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.load16_u" => if let Operator::I32Load16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.load8_s" => if let Operator::I64Load8S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.load8_u" => if let Operator::I64Load8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.load16_s" => if let Operator::I64Load16S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.load16_u" => if let Operator::I64Load16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.load32_s" => if let Operator::I64Load32S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.load32_u" => if let Operator::I64Load32U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.store" => if let Operator::I32Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.store" => if let Operator::I64Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.store" => if let Operator::F32Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.store" => if let Operator::F64Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.store8" => if let Operator::I32Store8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.store16" => if let Operator::I32Store16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.store8" => if let Operator::I64Store8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.store16" => if let Operator::I64Store16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.store32" => if let Operator::I64Store32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.size" => if let Operator::MemorySize {mem} = instr {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.grow" => if let Operator::MemoryGrow {mem} = instr {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.const" => if let Operator::I32Const {value} = instr {
            define_imm0::<i32>(*value, DataType::I32, &Value::gen_i32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.const" => if let Operator::I64Const {value} = instr {
            define_imm0::<i64>(*value, DataType::I64, &Value::gen_i64, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.const" => if let Operator::F32Const {value} = instr {
            define_imm0::<f32>(f32::from(*value), DataType::F32, &Value::gen_f32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.const" => if let Operator::F64Const {value} = instr {
            define_imm0::<f64>(f64::from(*value), DataType::F64, &Value::gen_f64, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.null" => if let Operator::RefNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.is_null" => if let Operator::RefIsNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.func" => if let Operator::RefFunc {function_index} = instr {
            define_imm0::<u32>(*function_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.eq" => if let Operator::RefEq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.eqz" => if let Operator::I32Eqz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.eq" => if let Operator::I32Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.ne" => if let Operator::I32Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.lt_s" => if let Operator::I32LtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.lt_u" => if let Operator::I32LtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.gt_s" => if let Operator::I32GtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.gt_u" => if let Operator::I32GtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.le_s" => if let Operator::I32LeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.le_u" => if let Operator::I32LeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.ge_s" => if let Operator::I32GeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.ge_u" => if let Operator::I32GeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.eqz" => if let Operator::I64Eqz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.eq" => if let Operator::I64Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.ne" => if let Operator::I64Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.lt_s" => if let Operator::I64LtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.lt_u" => if let Operator::I64LtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.gt_s" => if let Operator::I64GtS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.gt_u" => if let Operator::I64GtU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.le_s" => if let Operator::I64LeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.le_u" => if let Operator::I64LeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.ge_s" => if let Operator::I64GeS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.ge_u" => if let Operator::I64GeU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.eq" => if let Operator::F32Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.ne" => if let Operator::F32Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.lt" => if let Operator::F32Lt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.gt" => if let Operator::F32Gt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.le" => if let Operator::F32Le {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.ge" => if let Operator::F32Ge {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.eq" => if let Operator::F64Eq {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.ne" => if let Operator::F64Ne {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.lt" => if let Operator::F64Lt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.gt" => if let Operator::F64Gt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.le" => if let Operator::F64Le {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.ge" => if let Operator::F64Ge {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.clz" => if let Operator::I32Clz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.ctz" => if let Operator::I32Ctz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.popcnt" => if let Operator::I32Popcnt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.add" => if let Operator::I32Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.sub" => if let Operator::I32Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.mul" => if let Operator::I32Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.div_s" => if let Operator::I32DivS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.div_u" => if let Operator::I32DivU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.rem_s" => if let Operator::I32RemS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.rem_u" => if let Operator::I32RemU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.and" => if let Operator::I32And {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.or" => if let Operator::I32Or {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.xor" => if let Operator::I32Xor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.shl" => if let Operator::I32Shl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.shr_s" => if let Operator::I32ShrS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.shr_u" => if let Operator::I32ShrU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.rotl" => if let Operator::I32Rotl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.rotr" => if let Operator::I32Rotr {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.clz" => if let Operator::I64Clz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.ctz" => if let Operator::I64Ctz {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.popcnt" => if let Operator::I64Popcnt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.add" => if let Operator::I64Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.sub" => if let Operator::I64Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.mul" => if let Operator::I64Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.div_s" => if let Operator::I64DivS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.div_u" => if let Operator::I64DivU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.rem_s" => if let Operator::I64RemS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.rem_u" => if let Operator::I64RemU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.and" => if let Operator::I64And {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.or" => if let Operator::I64Or {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.xor" => if let Operator::I64Xor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.shl" => if let Operator::I64Shl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.shr_s" => if let Operator::I64ShrS {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.shr_u" => if let Operator::I64ShrU {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.rotl" => if let Operator::I64Rotl {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.rotr" => if let Operator::I64Rotr {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.abs" => if let Operator::F32Abs {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.neg" => if let Operator::F32Neg {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.ceil" => if let Operator::F32Ceil {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.floor" => if let Operator::F32Floor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.trunc" => if let Operator::F32Trunc {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.nearest" => if let Operator::F32Nearest {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.sqrt" => if let Operator::F32Sqrt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.add" => if let Operator::F32Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.sub" => if let Operator::F32Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.mul" => if let Operator::F32Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.div" => if let Operator::F32Div {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.min" => if let Operator::F32Min {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.max" => if let Operator::F32Max {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.copysign" => if let Operator::F32Copysign {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.abs" => if let Operator::F64Abs {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.neg" => if let Operator::F64Neg {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.ceil" => if let Operator::F64Ceil {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.floor" => if let Operator::F64Floor {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.trunc" => if let Operator::F64Trunc {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.nearest" => if let Operator::F64Nearest {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.sqrt" => if let Operator::F64Sqrt {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.add" => if let Operator::F64Add {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.sub" => if let Operator::F64Sub {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.mul" => if let Operator::F64Mul {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.div" => if let Operator::F64Div {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.min" => if let Operator::F64Min {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.max" => if let Operator::F64Max {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.copysign" => if let Operator::F64Copysign {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.wrap_i64" => if let Operator::I32WrapI64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_f32_s" => if let Operator::I32TruncF32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_f32_u" => if let Operator::I32TruncF32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_f64_s" => if let Operator::I32TruncF64S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_f64_u" => if let Operator::I32TruncF64U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.extend_i32_s" => if let Operator::I64ExtendI32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.extend_i32_u" => if let Operator::I64ExtendI32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_f32_s" => if let Operator::I64TruncF32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_f32_u" => if let Operator::I64TruncF32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_f64_s" => if let Operator::I64TruncF64S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_f64_u" => if let Operator::I64TruncF64U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.convert_i32_s" => if let Operator::F32ConvertI32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.convert_i32_u" => if let Operator::F32ConvertI32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.convert_i64_s" => if let Operator::F32ConvertI64S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.convert_i64_u" => if let Operator::F32ConvertI64U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.demote_f64" => if let Operator::F32DemoteF64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.convert_i32_s" => if let Operator::F64ConvertI32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.convert_i32_u" => if let Operator::F64ConvertI32U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.convert_i64_s" => if let Operator::F64ConvertI64S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.convert_i64_u" => if let Operator::F64ConvertI64U {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.promote_f32" => if let Operator::F64PromoteF32 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.reinterpret_f32" => if let Operator::I32ReinterpretF32 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.reinterpret_f64" => if let Operator::I64ReinterpretF64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f32.reinterpret_i32" => if let Operator::F32ReinterpretI32 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "f64.reinterpret_i64" => if let Operator::F64ReinterpretI64 {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.extend8_s" => if let Operator::I32Extend8S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.extend16_s" => if let Operator::I32Extend16S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.extend8_s" => if let Operator::I64Extend8S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.extend16_s" => if let Operator::I64Extend16S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.extend32_s" => if let Operator::I64Extend32S {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "struct.new" => if let Operator::StructNew {struct_type_index} = instr {
            define_imm0::<u32>(*struct_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "struct.new_default" => if let Operator::StructNewDefault {struct_type_index} = instr {
            define_imm0::<u32>(*struct_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "struct.get" => if let Operator::StructGet {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "struct.get_s" => if let Operator::StructGetS {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "struct.get_u" => if let Operator::StructGetU {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "struct.set" => if let Operator::StructSet {struct_type_index, field_index} = instr {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.new" => if let Operator::ArrayNew {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.new_default" => if let Operator::ArrayNewDefault {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.new_fixed" => if let Operator::ArrayNewFixed {array_type_index, array_size} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_size, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.new_data" => if let Operator::ArrayNewData {array_type_index, array_data_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_data_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.new_elem" => if let Operator::ArrayNewElem {array_type_index, array_elem_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_elem_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.get" => if let Operator::ArrayGet {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.get_s" => if let Operator::ArrayGetS {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.get_u" => if let Operator::ArrayGetU {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.set" => if let Operator::ArraySet {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.len" => if let Operator::ArrayLen = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.fill" => if let Operator::ArrayFill {array_type_index} = instr {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.copy" => if let Operator::ArrayCopy {array_type_index_dst, array_type_index_src} = instr {
            define_imm0_u32_imm1_u32(*array_type_index_dst, *array_type_index_src, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.init_data" => if let Operator::ArrayInitData {array_type_index, array_data_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_data_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "array.init_elem" => if let Operator::ArrayInitElem {array_type_index, array_elem_index} = instr {
            define_imm0_u32_imm1_u32(*array_type_index, *array_elem_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.test" => if let Operator::RefTestNonNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.test_null" => if let Operator::RefTestNullable {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.cast" => if let Operator::RefCastNonNull {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.cast_null" => if let Operator::RefCastNullable {..} = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "br_on_cast" => if let Operator::BrOnCast {relative_depth, ..} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "br_on_cast_fail" => if let Operator::BrOnCastFail {relative_depth, ..} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "any.convert_extern" => if let Operator::AnyConvertExtern = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "extern.convert_any" => if let Operator::ExternConvertAny = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.i31" => if let Operator::RefI31 = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i31.get_s" => if let Operator::I31GetS = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i31.get_u" => if let Operator::I31GetU = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_sat_f32_s" => if let Operator::I32TruncSatF32S = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_sat_f32_u" => if let Operator::I32TruncSatF32U = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_sat_f64_s" => if let Operator::I32TruncSatF64S = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.trunc_sat_f64_u" => if let Operator::I32TruncSatF64U = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_sat_f32_s" => if let Operator::I64TruncSatF32S = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_sat_f32_u" => if let Operator::I64TruncSatF32U = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_sat_f64_s" => if let Operator::I64TruncSatF64S = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.trunc_sat_f64_u" => if let Operator::I64TruncSatF64U = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.init" => if let Operator::MemoryInit {data_index, mem} = instr {
            define_imm0_u32_imm1_u32(*data_index, *mem, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.copy" => if let Operator::MemoryCopy {dst_mem, src_mem} = instr {
            define_imm0_u32_imm1_u32(*dst_mem, *src_mem, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.fill" => if let Operator::MemoryFill {mem} = instr {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "data.drop" => if let Operator::DataDrop {data_index} = instr {
            define_imm0::<u32>(*data_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "elem.drop" => if let Operator::ElemDrop {elem_index} = instr {
            define_imm0::<u32>(*elem_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "table.copy" => if let Operator::TableCopy {dst_table, src_table} = instr {
            define_imm0_u32_imm1_u32(*dst_table, *src_table, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "table.init" => if let Operator::TableInit {elem_index, table} = instr {
            define_imm0_u32_imm1_u32(*elem_index, *table, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "table.fill" => if let Operator::TableFill {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "table.get" => if let Operator::TableGet {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "table.set" => if let Operator::TableSet {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "table.grow" => if let Operator::TableGrow {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "table.size" => if let Operator::TableSize {table} = instr {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.atomic_notify" => if let Operator::MemoryAtomicNotify {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.atomic_wait32" => if let Operator::MemoryAtomicWait32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "memory.atomic_wait64" => if let Operator::MemoryAtomicWait64 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "atomic.fence" => if let Operator::AtomicFence = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_load" => if let Operator::I32AtomicLoad {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_load" => if let Operator::I64AtomicLoad {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_load8_u" => if let Operator::I32AtomicLoad8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_load16_u" => if let Operator::I32AtomicLoad16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_load8_u" => if let Operator::I64AtomicLoad8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_load16_u" => if let Operator::I64AtomicLoad16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_load32_u" => if let Operator::I64AtomicLoad32U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_store" => if let Operator::I32AtomicStore {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_store8" => if let Operator::I32AtomicStore8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_store16" => if let Operator::I32AtomicStore16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_store" => if let Operator::I64AtomicStore {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_store8" => if let Operator::I64AtomicStore8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_store16" => if let Operator::I64AtomicStore16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_store32" => if let Operator::I64AtomicStore32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw_add" => if let Operator::I32AtomicRmwAdd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw8_add_u" => if let Operator::I32AtomicRmw8AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw16_add_u" => if let Operator::I32AtomicRmw16AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw_add" => if let Operator::I64AtomicRmwAdd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw8_add_u" => if let Operator::I64AtomicRmw8AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw16_add_u" => if let Operator::I64AtomicRmw16AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw32_add_u" => if let Operator::I64AtomicRmw32AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw_sub" => if let Operator::I32AtomicRmwSub {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw8_sub_u" => if let Operator::I32AtomicRmw8SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw16_sub_u" => if let Operator::I32AtomicRmw16SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw_sub" => if let Operator::I64AtomicRmwSub {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw8_sub_u" => if let Operator::I64AtomicRmw8SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw16_sub_u" => if let Operator::I64AtomicRmw16SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw32_sub_u" => if let Operator::I64AtomicRmw32SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw_and" => if let Operator::I32AtomicRmwAnd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw8_and_u" => if let Operator::I32AtomicRmw8AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw16_and_u" => if let Operator::I32AtomicRmw16AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw_and" => if let Operator::I64AtomicRmwAnd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw8_and_u" => if let Operator::I64AtomicRmw8AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw16_and_u" => if let Operator::I64AtomicRmw16AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw32_and_u" => if let Operator::I64AtomicRmw32AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw_or" => if let Operator::I32AtomicRmwOr {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw8_or_u" => if let Operator::I32AtomicRmw8OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw16_or_u" => if let Operator::I32AtomicRmw16OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw_or" => if let Operator::I64AtomicRmwOr {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw8_or_u" => if let Operator::I64AtomicRmw8OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw16_or_u" => if let Operator::I64AtomicRmw16OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw32_or_u" => if let Operator::I64AtomicRmw32OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw_xor" => if let Operator::I32AtomicRmwXor {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw8_xor_u" => if let Operator::I32AtomicRmw8XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw16_xor_u" => if let Operator::I32AtomicRmw16XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw_xor" => if let Operator::I64AtomicRmwXor {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw8_xor_u" => if let Operator::I64AtomicRmw8XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw16_xor_u" => if let Operator::I64AtomicRmw16XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw32_xor_u" => if let Operator::I64AtomicRmw32XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw_xchg" => if let Operator::I32AtomicRmwXchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw8_xchg_u" => if let Operator::I32AtomicRmw8XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw16_xchg_u" => if let Operator::I32AtomicRmw16XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw_xchg" => if let Operator::I64AtomicRmwXchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw8_xchg_u" => if let Operator::I64AtomicRmw8XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw16_xchg_u" => if let Operator::I64AtomicRmw16XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw32_xchg_u" => if let Operator::I64AtomicRmw32XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw_cmpxchg" => if let Operator::I32AtomicRmwCmpxchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw8_cmpxchg_u" => if let Operator::I32AtomicRmw8CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i32.atomic_rmw16_cmpxchg_u" => if let Operator::I32AtomicRmw16CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw_cmpxchg" => if let Operator::I64AtomicRmwCmpxchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw8_cmpxchg_u" => if let Operator::I64AtomicRmw8CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw16_cmpxchg_u" => if let Operator::I64AtomicRmw16CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "i64.atomic_rmw32_cmpxchg_u" => if let Operator::I64AtomicRmw32CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} = instr {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "call_ref" => if let Operator::CallRef {type_index} = instr {
            define_imm0::<u32>(*type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "return_call_ref" => if let Operator::ReturnCallRef {type_index} = instr {
            define_imm0::<u32>(*type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "ref.as_non_null" => if let Operator::RefAsNonNull = instr {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "br_on_null" => if let Operator::BrOnNull {relative_depth} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        "br_on_non_null" => if let Operator::BrOnNonNull {relative_depth} = instr {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        _ => panic!("Event not available: 'wasm:opcode:{event}'"),
    }

    let (all_args, all_results, ..) = get_ty_info_for_instr(app_wasm, fid, instr);

    // figure out which args are requested based on matched probes
    // (we don't have a match if the requested argument or result is beyond the
    // length of what's possible)
    let mut probes_to_remove = vec![];
    let max_arg_req = all_args.len();
    let max_res_req = all_results.len();
    for (i, (_, probe, _)) in loc_info.probes.iter_mut().enumerate() {
        let body_args = &probe.metadata.body_args;
        let pred_params = &probe.metadata.pred_args;
        if !check_match(&body_args.req_args, max_arg_req, &mut req_args, i, &mut probes_to_remove) {
            continue;
        }
        if !check_match(&body_args.req_results, max_res_req, &mut req_results, i, &mut probes_to_remove) {
            continue;
        }
        if !check_match(&pred_params.req_args, max_arg_req, &mut req_args, i, &mut probes_to_remove) {
            continue;
        }
        if !check_match(&pred_params.req_results, max_res_req, &mut req_results, i, &mut probes_to_remove) {
            continue;
        }
        fn check_match(to_check: &StackReq, max_req: usize, all_reqs: &mut StackReq, curr_probe: usize, to_remove: &mut Vec<usize>) -> bool {
            if to_check.matches(max_req) {
                all_reqs.combine(to_check);
                true
            } else {
                // remove probe!
                to_remove.push(curr_probe);
                false
            }
        }
    }
    for i in probes_to_remove.iter() {
        loc_info.probes.remove(*i);
    }

    loc_info.configure_stack_reqs(req_args, all_args, req_results, all_results);

    loc_info.is_prog_exit = is_prog_exit_call(instr, app_wasm);
    if loc_info.has_match() || loc_info.is_prog_exit {
        Some(loc_info)
    } else {
        None
    }
}

fn define_imm0_u32_imm1_u32(
    value0: u32,
    value1: u32,
    loc_info: &mut LocInfo,
    all_params: &[WhammParam],
) {
    for param in all_params.iter() {
        if let Some(n) = param.n_for("imm") {
            assert!(matches!(param.ty, DataType::U32));
            if n == 0 {
                define_imm_n(n, Some(Value::gen_u32(value0)), loc_info);
            } else if n == 1 {
                define_imm_n(n, Some(Value::gen_u32(value1)), loc_info);
            } else {
                panic!("WhammParam not available for opcode: {}", param.name);
            }
        }
    }
}

fn define_imm0<T>(
    value: T,
    _dt: DataType,
    r#gen: &dyn Fn(T) -> Value,
    loc_info: &mut LocInfo,
    all_params: &[WhammParam],
) {
    for param in all_params.iter() {
        if let Some(n) = param.n_for("imm") {
            assert_eq!(n, 0);
            assert!(matches!(&param.ty, _dt));

            define_imm_n(0, Some(r#gen(value)), loc_info);
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
    all_params: &[WhammParam],
) {
    for param in all_params.iter() {
        match param.name.as_str() {
            "align" => loc_info
                .static_data
                .insert(param.name.clone(), Some(Value::gen_u32(align as u32))),
            "offset" => loc_info
                .static_data
                .insert(param.name.clone(), Some(Value::gen_u64(offset))),
            "memory" => loc_info
                .static_data
                .insert(param.name.clone(), Some(Value::gen_u32(memory))),
            _ => None,
        };
    }
}
fn bind_vars_br_table(
    targets: &BrTable,
    loc_info: &mut LocInfo,
    all_params: &[WhammParam],
) -> Option<()> {
    for param in all_params.iter() {
        if let Some(n) = param.n_for("imm") {
            if n > targets.len() {
                // this location doesn't match since the immN is out of bound
                // of the immN's available
                return None;
            }
            assert!(matches!(param.ty, DataType::U32));

            if n == targets.len() {
                // requesting the default value!
                define_imm_n(n, Some(Value::gen_u32(targets.default())), loc_info);
            }

            for (i, target) in targets.targets().enumerate() {
                if let Ok(target) = target {
                    if n == i as u32 {
                        define_imm_n(i as u32, Some(Value::gen_u32(target)), loc_info);
                        break;
                    }
                }
            }
        } else {
            match param.name.as_str() {
                "num_targets" => {
                    loc_info
                        .static_data
                        .insert(param.name.clone(), Some(Value::gen_u32(targets.len())));
                }
                "default_target" => {
                    loc_info
                        .static_data
                        .insert(param.name.clone(), Some(Value::gen_u32(targets.default())));
                }
                "targets" => {
                    let mut target_map = HashMap::new();
                    for (i, target) in targets.targets().enumerate() {
                        if let Ok(target) = target {
                            target_map.insert(i as u32, target);
                        }
                    }
                    loc_info
                        .add_dynamic_value(param.name.clone(), Value::U32U32Map { val: target_map })
                }
                _ => {}
            };
        }
    }
    Some(())
}

fn bind_vars_call(
    loc_info: &mut LocInfo,
    all_params: &[WhammParam],
    fid: u32,
    app_wasm: &Module,
) -> Result<(), ()> {
    let func_info = match app_wasm.functions.get_kind(FunctionID(fid)) {
        FuncKind::Import(ImportedFunction {
            import_id, ty_id, ..
        }) => {
            let import = app_wasm.imports.get(*import_id);
            FuncInfo {
                func_kind: "import".to_string(),
                module: import.module.to_string(),
                name: import.name.to_string(),
                ty_id: *ty_id,
            }
        }
        FuncKind::Local(func) => FuncInfo {
            func_kind: "local".to_string(),
            module: match &app_wasm.module_name {
                Some(name) => name.clone(),
                None => "".to_string(),
            },
            name: match &app_wasm.functions.get_name(func.func_id) {
                Some(name) => name.clone(),
                None => "".to_string(),
            },
            ty_id: func.ty_id,
        },
    };

    let func_params =
        if let Some(Types::FuncType { params, .. }) = app_wasm.types.get(func_info.ty_id) {
            params.clone()
        } else {
            panic!(
                "Unable to lookup the function type with ID: {}",
                *func_info.ty_id
            );
        };

    for param in all_params.iter() {
        if let Some(n) = param.n_for("arg") {
            // check that the types match!
            if n as usize >= func_params.len() {
                // Doesn't have this argument, no match!
                return Err(());
            }
            if let Some(ty) = func_params.get(func_params.len() - (n as usize + 1)) {
                if *param.ty.to_wasm_type().first().unwrap() != *ty {
                    // types don't match, no match for this location!
                    return Err(());
                }
            } else {
                // Doesn't have this argument, no match!
                return Err(());
            }
            // else we have a match for this location!
        } else if let Some(n) = param.n_for("imm") {
            assert_eq!(n, 0);
            assert!(
                matches!(param.ty, DataType::U32),
                "wrong type: {}",
                param.ty
            );

            define_imm_n(0, Some(Value::gen_u32(fid)), loc_info);
        } else {
            match param.name.as_str() {
                "target_fn_name" => loc_info.static_data.insert(
                    "target_fn_name".to_string(),
                    Some(Value::Str {
                        val: func_info.name.to_string(),
                    }),
                ),
                "target_fn_type" => loc_info.static_data.insert(
                    "target_fn_type".to_string(),
                    Some(Value::Str {
                        val: func_info.func_kind.to_string(),
                    }),
                ),
                "target_imp_module" => loc_info.static_data.insert(
                    "target_imp_module".to_string(),
                    Some(Value::Str {
                        val: func_info.module.to_string(),
                    }),
                ),
                _ => None,
            };
        }
    }

    Ok(())
}

#[derive(Debug)]
struct FuncInfo {
    func_kind: String,
    module: String,
    name: String,
    ty_id: TypeID,
}

pub fn get_ty_info_for_instr(
    app_wasm: &Module,
    curr_fid: &FunctionID,
    instr: &Operator,
) -> (Vec<StackVal>, Vec<StackVal>, Option<u32>) {
    // TODO -- how to make this less manual?
    let (arg_list, res_list, ty_id): (Vec<Option<WirmType>>, Vec<Option<WirmType>>, Option<u32>) =
        match instr {
            Operator::If { .. } | Operator::BrIf { .. } | Operator::BrTable { .. } => {
                (vec![Some(WirmType::I32)], vec![], None)
            }
            Operator::Call {
                function_index: fid,
            } => {
                let ty_id = match app_wasm.functions.get_kind(FunctionID(*fid)) {
                    FuncKind::Import(ImportedFunction { ty_id, .. }) => *ty_id,
                    FuncKind::Local(func) => func.ty_id,
                };
                if let Some(ty) = app_wasm.types.get(ty_id) {
                    let mut args = vec![];
                    for t in ty.params().iter().rev() {
                        args.push(Some(*t));
                    }
                    let mut results = vec![];
                    for t in ty.results().iter().rev() {
                        results.push(Some(*t));
                    }
                    (args, results, Some(*ty_id))
                } else {
                    // no type info found!!
                    warn!("No type information found for import with FID {fid}");
                    (vec![], vec![], None)
                }
            }
            Operator::Block {
                blockty: BlockType::FuncType(ty_id),
            }
            | Operator::Loop {
                blockty: BlockType::FuncType(ty_id),
            }
            | Operator::CallIndirect {
                type_index: ty_id, ..
            } => {
                if let Some(ty) = app_wasm.types.get(TypeID(*ty_id)) {
                    let mut args = vec![];
                    for t in ty.params().iter().rev() {
                        args.push(Some(*t));
                    }
                    let mut results = vec![];
                    for t in ty.results().iter().rev() {
                        results.push(Some(*t));
                    }
                    (args, results, Some(*ty_id))
                } else {
                    // no type info found!!
                    warn!("No type information found for opcode");
                    (vec![], vec![], None)
                }
            }
            Operator::Drop => {
                // TODO -- how to express an unknown type?
                //     Lookup in the symbol table! We've placed type bounds in there during verification
                //     HOWEVER, we will need to keep a virtual stack to check if this match site is in fact
                //     a match based on the type bounds. (if they don't match up, not a match, don't emit)
                // e.g. [unknown]
                (vec![None], vec![], None)
            }
            Operator::Select => {
                // TODO -- how to express an unknown type?
                //     Lookup in the symbol table! We've placed type bounds in there during verification
                //     HOWEVER, we will need to keep a virtual stack to check if this match site is in fact
                //     a match based on the type bounds. (if they don't match up, not a match, don't emit)
                // e.g. [unknown, unknown, i32]
                (vec![None, None, Some(WirmType::I32)], vec![None], None)
            }
            Operator::LocalSet { local_index } | Operator::LocalTee { local_index } => {
                if let FuncKind::Local(func) = app_wasm.functions.get_kind(*curr_fid) {
                    if let Some((_, ty)) = func.body.locals.get(*local_index as usize) {
                        (vec![Some(*ty)], vec![], None)
                    } else {
                        (vec![], vec![], None) // ignore
                    }
                } else {
                    (vec![], vec![], None) // ignore
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
                    }) => WirmType::from(*content_type),
                };
                (vec![Some(ty)], vec![], None)
            }

            Operator::I32Load { .. }
            | Operator::I32Load8S { .. }
            | Operator::I32Load8U { .. }
            | Operator::I32Load16S { .. }
            | Operator::I32Load16U { .. } => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I32)], None)
            }
            Operator::I64Load { .. }
            | Operator::I64Load8S { .. }
            | Operator::I64Load8U { .. }
            | Operator::I64Load16S { .. }
            | Operator::I64Load16U { .. }
            | Operator::I64Load32S { .. }
            | Operator::I64Load32U { .. } => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I64)], None)
            }
            Operator::F32Load { .. } => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::F32)], None)
            }
            Operator::F64Load { .. } => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::F64)], None)
            }

            Operator::I32Store { .. }
            | Operator::I32Store8 { .. }
            | Operator::I32Store16 { .. } => {
                (vec![Some(WirmType::I32), Some(WirmType::I32)], vec![], None)
            }
            Operator::I64Store { .. }
            | Operator::I64Store8 { .. }
            | Operator::I64Store16 { .. }
            | Operator::I64Store32 { .. } => {
                (vec![Some(WirmType::I64), Some(WirmType::I32)], vec![], None)
            }
            Operator::F32Store { .. } => {
                (vec![Some(WirmType::F32), Some(WirmType::I32)], vec![], None)
            }
            Operator::F64Store { .. } => {
                (vec![Some(WirmType::F64), Some(WirmType::I32)], vec![], None)
            }
            Operator::MemoryGrow { .. } => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I32)], None)
            }

            Operator::I32Eqz => (vec![Some(WirmType::I32)], vec![Some(WirmType::I32)], None),
            Operator::I32Ne
            | Operator::I32Eq
            | Operator::I32LtS
            | Operator::I32LtU
            | Operator::I32GtS
            | Operator::I32GtU
            | Operator::I32LeS
            | Operator::I32LeU
            | Operator::I32GeS
            | Operator::I32GeU => (
                vec![Some(WirmType::I32), Some(WirmType::I32)],
                vec![Some(WirmType::I32)],
                None,
            ),

            Operator::I32Clz | Operator::I32Ctz | Operator::I32Popcnt => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I32)], None)
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
            | Operator::I32Rotr => (
                vec![Some(WirmType::I32), Some(WirmType::I32)],
                vec![Some(WirmType::I32)],
                None,
            ),

            Operator::I64Eqz => (vec![Some(WirmType::I64)], vec![Some(WirmType::I32)], None),
            Operator::I64Eq
            | Operator::I64Ne
            | Operator::I64LtS
            | Operator::I64LtU
            | Operator::I64GtS
            | Operator::I64GtU
            | Operator::I64LeS
            | Operator::I64LeU
            | Operator::I64GeS
            | Operator::I64GeU => (
                vec![Some(WirmType::I64), Some(WirmType::I64)],
                vec![Some(WirmType::I32)],
                None,
            ),

            Operator::I64Clz | Operator::I64Ctz | Operator::I64Popcnt => {
                (vec![Some(WirmType::I64)], vec![Some(WirmType::I64)], None)
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
            | Operator::I64Rotr => (
                vec![Some(WirmType::I64), Some(WirmType::I64)],
                vec![Some(WirmType::I64)],
                None,
            ),

            Operator::F32Eq
            | Operator::F32Ne
            | Operator::F32Lt
            | Operator::F32Gt
            | Operator::F32Le
            | Operator::F32Ge => (
                vec![Some(WirmType::F32), Some(WirmType::F32)],
                vec![Some(WirmType::I32)],
                None,
            ),

            Operator::F32Abs
            | Operator::F32Neg
            | Operator::F32Ceil
            | Operator::F32Floor
            | Operator::F32Trunc
            | Operator::F32Nearest
            | Operator::F32Sqrt => (vec![Some(WirmType::F32)], vec![Some(WirmType::F32)], None),
            Operator::F32Add
            | Operator::F32Sub
            | Operator::F32Mul
            | Operator::F32Div
            | Operator::F32Min
            | Operator::F32Max
            | Operator::F32Copysign => (
                vec![Some(WirmType::F32), Some(WirmType::F32)],
                vec![Some(WirmType::F32)],
                None,
            ),

            Operator::F64Eq
            | Operator::F64Ne
            | Operator::F64Lt
            | Operator::F64Gt
            | Operator::F64Le
            | Operator::F64Ge => (
                vec![Some(WirmType::F64), Some(WirmType::F64)],
                vec![Some(WirmType::I32)],
                None,
            ),

            Operator::F64Abs
            | Operator::F64Neg
            | Operator::F64Ceil
            | Operator::F64Floor
            | Operator::F64Trunc
            | Operator::F64Nearest
            | Operator::F64Sqrt => (vec![Some(WirmType::F32)], vec![Some(WirmType::F64)], None),
            Operator::F64Add
            | Operator::F64Sub
            | Operator::F64Mul
            | Operator::F64Div
            | Operator::F64Min
            | Operator::F64Max
            | Operator::F64Copysign => (
                vec![Some(WirmType::F64), Some(WirmType::F64)],
                vec![Some(WirmType::F64)],
                None,
            ),

            Operator::I32WrapI64 => (vec![Some(WirmType::I64)], vec![Some(WirmType::I32)], None),
            Operator::F32ConvertI64S | Operator::F32ConvertI64U => {
                (vec![Some(WirmType::I64)], vec![Some(WirmType::F32)], None)
            }
            Operator::F64ConvertI64S | Operator::F64ConvertI64U | Operator::F64ReinterpretI64 => {
                (vec![Some(WirmType::I64)], vec![Some(WirmType::F64)], None)
            }
            Operator::I64Extend8S | Operator::I64Extend16S | Operator::I64Extend32S => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I64)], None)
            }
            Operator::I32TruncF32S | Operator::I32TruncF32U => {
                (vec![Some(WirmType::F32)], vec![Some(WirmType::I32)], None)
            }
            Operator::I32TruncF64S
            | Operator::I32TruncF64U
            | Operator::I32TruncSatF64S
            | Operator::I32TruncSatF64U => {
                (vec![Some(WirmType::F64)], vec![Some(WirmType::I32)], None)
            }
            Operator::I64TruncF64S
            | Operator::I64TruncF64U
            | Operator::I64ReinterpretF64
            | Operator::I64TruncSatF64S
            | Operator::I64TruncSatF64U => {
                (vec![Some(WirmType::F64)], vec![Some(WirmType::I64)], None)
            }
            Operator::F32DemoteF64 => (vec![Some(WirmType::F64)], vec![Some(WirmType::F32)], None),
            Operator::I32Extend8S | Operator::I32Extend16S => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I32)], None)
            }
            Operator::I64ExtendI32S | Operator::I64ExtendI32U => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I64)], None)
            }
            Operator::F32ConvertI32S | Operator::F32ConvertI32U | Operator::F32ReinterpretI32 => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::F32)], None)
            }
            Operator::F64ConvertI32S | Operator::F64ConvertI32U => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::F64)], None)
            }
            Operator::I32ReinterpretF32 | Operator::I32TruncSatF32S | Operator::I32TruncSatF32U => {
                (vec![Some(WirmType::F32)], vec![Some(WirmType::I32)], None)
            }
            Operator::I64TruncF32S
            | Operator::I64TruncF32U
            | Operator::I64TruncSatF32S
            | Operator::I64TruncSatF32U => {
                (vec![Some(WirmType::F32)], vec![Some(WirmType::I64)], None)
            }
            Operator::F64PromoteF32 => (vec![Some(WirmType::F32)], vec![Some(WirmType::F64)], None),

            Operator::MemoryCopy { .. }
            | Operator::MemoryFill { .. }
            | Operator::MemoryInit { .. }
            | Operator::TableInit { .. }
            | Operator::TableCopy { .. } => (
                vec![
                    Some(WirmType::I32),
                    Some(WirmType::I32),
                    Some(WirmType::I32),
                ],
                vec![],
                None,
            ),

            Operator::TableGet { .. } => (vec![Some(WirmType::I32)], vec![None], None),

            Operator::MemoryAtomicNotify { .. } => (
                vec![Some(WirmType::I32), Some(WirmType::I32)],
                vec![Some(WirmType::I32)],
                None,
            ),
            Operator::MemoryAtomicWait32 { .. } => (
                vec![
                    Some(WirmType::I32),
                    Some(WirmType::I32),
                    Some(WirmType::I64),
                ],
                vec![Some(WirmType::I32)],
                None,
            ),
            Operator::MemoryAtomicWait64 { .. } => (
                vec![
                    Some(WirmType::I32),
                    Some(WirmType::I64),
                    Some(WirmType::I64),
                ],
                vec![Some(WirmType::I32)],
                None,
            ),

            Operator::I32AtomicLoad { .. }
            | Operator::I32AtomicLoad8U { .. }
            | Operator::I32AtomicLoad16U { .. } => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I32)], None)
            }
            Operator::I64AtomicLoad { .. }
            | Operator::I64AtomicLoad8U { .. }
            | Operator::I64AtomicLoad16U { .. }
            | Operator::I64AtomicLoad32U { .. } => {
                (vec![Some(WirmType::I32)], vec![Some(WirmType::I64)], None)
            }

            Operator::I32AtomicStore { .. }
            | Operator::I32AtomicStore8 { .. }
            | Operator::I32AtomicStore16 { .. } => {
                (vec![Some(WirmType::I32), Some(WirmType::I32)], vec![], None)
            }

            Operator::I64AtomicStore { .. }
            | Operator::I64AtomicStore8 { .. }
            | Operator::I64AtomicStore16 { .. }
            | Operator::I64AtomicStore32 { .. } => {
                (vec![Some(WirmType::I64), Some(WirmType::I32)], vec![], None)
            }

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
            | Operator::I32AtomicRmw16CmpxchgU { .. } => (
                vec![Some(WirmType::I32), Some(WirmType::I32)],
                vec![Some(WirmType::I32)],
                None,
            ),

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
            | Operator::I64AtomicRmw32CmpxchgU { .. } => (
                vec![Some(WirmType::I32), Some(WirmType::I64)],
                vec![Some(WirmType::I64)],
                None,
            ),

            Operator::Unreachable
            | Operator::Nop
            | Operator::End
            | Operator::Br { .. }
            | Operator::Else
            | Operator::DataDrop { .. }
            | Operator::ElemDrop { .. }
            | Operator::Return
            | Operator::AtomicFence => (vec![], vec![], None),
            Operator::LocalGet { .. } | Operator::GlobalGet { .. } => (vec![], vec![None], None),
            Operator::I32Const { .. }
            | Operator::MemorySize { .. }
            | Operator::TableSize { .. } => (vec![], vec![Some(WirmType::I32)], None),
            Operator::I64Const { .. } => (vec![], vec![Some(WirmType::I64)], None),
            Operator::F32Const { .. } => (vec![], vec![Some(WirmType::F32)], None),
            Operator::F64Const { .. } => (vec![], vec![Some(WirmType::F64)], None),

            // TODO -- support all opcodes!
            // Primarily what's left is v128 and GC opcodes
            _ => (vec![], vec![], None),
        };

    let mut args = vec![];
    let mut results = vec![];
    let push_val = |prefix: &str, idx: usize, ty: &Option<WirmType>, vals: &mut Vec<StackVal>| {
        vals.push(StackVal::new(format!("{prefix}{idx}"), ty.to_owned()));
    };
    arg_list.iter().enumerate().for_each(|(idx, ty)| {
        push_val("arg", idx, ty, &mut args);
    });
    res_list.iter().enumerate().for_each(|(idx, ty)| {
        push_val("res", idx, ty, &mut results);
    });

    (args, results, ty_id)
}

fn handle_block(
    app_wasm: &Module,
    state: &mut MatchState,
    at_func_end: bool,
    fid: &FunctionID,
    opidx: usize,
    instr: &Operator,
    pkg: &SimplePkg,
) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    let mut handle_evt = |name: &str, evt: &SimpleEvt| {
        // See OpcodeEvent.get_loc_info
        if let Some(mut tmp) = handle_block_events(
            app_wasm,
            state,
            at_func_end,
            fid,
            opidx,
            instr,
            &name.to_string(),
            evt,
        ) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    };

    // Retain the following order for semantics
    // FIRST, exit
    // SECOND, entry
    if let Some(evt) = pkg.evts.get("end") {
        handle_evt("end", evt);
    }
    if let Some(evt) = pkg.evts.get("start") {
        handle_evt("start", evt);
    }
    res
}

#[derive(Default)]
pub struct MatchState {
    basic_blocks: BasicBlockState,
}

// State to encode the start and end opcode index of a basic block.
// TODO: track whether ends are branched to using a control stack.
//       If this end has a branch to it, end the previous block, if there was one.
#[derive(Default)]
struct BasicBlockState {
    start: usize,
    end: usize,
}
impl BasicBlockState {
    fn reset(&mut self) {
        self.start = 0;
        self.end = 0;
    }
    fn continue_block(&mut self) {
        self.end += 1;
    }
    fn end_block_here(&mut self) {
        self.start = self.end;
    }
    fn get_instr_cnt(&self) -> usize {
        self.end - self.start
    }
}

#[rustfmt::skip]
fn handle_block_events(
    app_wasm: &Module,
    state: &mut MatchState,
    at_func_end: bool,
    _fid: &FunctionID,
    opidx: usize,
    instr: &Operator,
    event: &String,
    evt: &SimpleEvt,
) -> Option<LocInfo> {
    let mut loc_info = LocInfo::new();

    let probe_rule = ProbeRule {
        provider: Some(RulePart::new("wasm".to_string(), None)),
        package: Some(RulePart::new("block".to_string(), None)),
        event: Some(RulePart::new(event.clone(), None)),
        mode: None,
    };

    let block_state = &mut state.basic_blocks;
    // reset the state if we've entered a new function!
    if opidx == 0 { block_state.reset() }

    let is_prog_exit = is_prog_exit_call(instr, app_wasm);

    // See for implementation:
    // https://github.com/titzer/wizard-engine/blob/master/src/util/BasicBlockIterator.v3
    let mode = match instr {
        Operator::Loop {..} |
        // TODO (for End): track whether ends are branched to using a control stack.
        //       If this end has a branch to it, end the previous block, if there was one.
        Operator::End => {
            if block_state.start != opidx {
                define_block_data(event.as_str(), block_state, &mut loc_info);
                block_state.end_block_here();
                match event.as_str() {
                    "start" |
                    "end" => if matches!(instr, Operator::End) && !at_func_end {
                        // semantics of End requires that this be injected AFTER it to execute!
                        Some(InstrumentationMode::After)
                    } else {
                        Some(InstrumentationMode::Before)
                    },
                    _ => panic!("Event not available: 'wasm:block:{event}'"),
                }
            } else if opidx == 0 && event == "start" {
                // if we're at the start of the function, we want to insert basic block entry probes
                define_block_data(event.as_str(), block_state, &mut loc_info);
                block_state.continue_block();
                Some(InstrumentationMode::Before)
            } else {
                None
            }
        },

        // Bytecodes that end the current block after this instruction.
        Operator::If {..} |
        Operator::Else |
        Operator::Catch {..} |
        Operator::CatchAll |
        Operator::Throw {..} |
        Operator::Rethrow {..} |
        Operator::Return |
        Operator::Unreachable |
        Operator::Br {..} |
        Operator::BrTable {..} |
        Operator::BrIf {..} |
        Operator::BrOnCast {..} |
        Operator::BrOnCastFail {..} |
        Operator::BrOnNull {..} |
        Operator::BrOnNonNull {..} => {
            // End the current block after this instruction.
            block_state.continue_block();
            define_block_data(event.as_str(), block_state, &mut loc_info);
            block_state.end_block_here();

            match event.as_str() {
                // exit | : before this instruction (to ensure it executes)
                "start" |
                "end" => Some(InstrumentationMode::Before),
                _ => panic!("Event not available: 'wasm:block:{event}'"),
            }
        },
        _ => {
            block_state.continue_block();
            // handle block:entry at the top of a function!
            if (opidx == 0 && event == "start")
                // handle block:exit if this is program exit call
                || (is_prog_exit && event == "end") {
                Some(InstrumentationMode::Before)
            } else {
                None
            }
        }
    };

    if mode.is_some() {
        loc_info.add_probes(probe_rule.clone(), evt, mode);
    }

    loc_info.is_prog_exit = is_prog_exit;
    if loc_info.has_match() || is_prog_exit {
        Some(loc_info)
    } else {
        None
    }
}

fn define_block_data(evt: &str, block_state: &BasicBlockState, loc_info: &mut LocInfo) {
    if evt == "end" {
        loc_info.static_data.insert(
            "instr_count".to_string(),
            Some(Value::gen_u32(block_state.get_instr_cnt() as u32)),
        );
    }
}

fn handle_func(
    app_wasm: &Module,
    fid: &FunctionID,
    opidx: usize,
    instr: &Operator,
    pkg: &SimplePkg,
) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    for (package, evt) in pkg.evts.iter() {
        // See OpcodeEvent.get_loc_info
        if let Some(mut tmp) = handle_func_events(app_wasm, fid, opidx, instr, package, evt) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }
    res
}

#[rustfmt::skip]
fn handle_func_events(
    app_wasm: &Module,
    _fid: &FunctionID,
    opidx: usize,
    instr: &Operator,
    event: &String,
    evt: &SimpleEvt,
) -> Option<LocInfo> {
    let mut loc_info = LocInfo::new();

    let probe_rule = ProbeRule {
        provider: Some(RulePart::new("wasm".to_string(), None)),
        package: Some(RulePart::new("func".to_string(), None)),
        event: Some(RulePart::new(event.clone(), None)),
        mode: None,
    };

    // if this is program exit, we want to inject the function exit logic!
    let is_prog_exit = is_prog_exit_call(instr, app_wasm);

    match event.as_str() {
        "exit" => if is_prog_exit || opidx == 0 {
            // if this is program exit, we want to inject the function exit logic (as opcode:before)!
            // we're at the start of the function, inject both of these types of special events!
            // we only want to inject entry/exit events once.
            loc_info.add_probes(probe_rule.clone(), evt, None);
        }
        "entry" => if opidx == 0 {
            // override the `pc` value to be 0
        loc_info
                .static_data
                .insert("pc".to_string(), Some(Value::gen_u32(0)));
            // we're at the start of the function, inject both of these types of special events!
            // we only want to inject entry/exit events once.
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        _ => panic!("Event not available: 'wasm:func:{event}'"),
    }

    loc_info.is_prog_exit = is_prog_exit;
    if loc_info.has_match() || is_prog_exit {
        Some(loc_info)
    } else {
        None
    }
}

pub fn is_prog_exit_call(opcode: &Operator, wasm: &Module) -> bool {
    if let Operator::Call {
        function_index: fid,
    }
    | Operator::ReturnCall {
        function_index: fid,
    } = opcode
    {
        let target = match wasm.functions.get_kind(FunctionID(*fid)) {
            FuncKind::Import(ImportedFunction { import_id, .. }) => {
                let import = wasm.imports.get(*import_id);
                let mod_name = import.module.to_string();
                let func_name = import.name.to_string();
                format!("{mod_name}:{func_name}")
            }
            FuncKind::Local(func) => {
                let mod_name = match &wasm.module_name {
                    Some(name) => name.clone(),
                    None => "".to_string(),
                };
                let func_name = match &wasm.functions.get_name(func.func_id) {
                    Some(name) => name.clone(),
                    None => "".to_string(),
                };
                format!("{mod_name}:{func_name}")
            }
        };
        let exiting_call = HashSet::from(["wasi_snapshot_preview1:proc_exit".to_string()]);
        exiting_call.contains(&target)
    } else {
        false
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct StackVal {
    pub name: String,
    pub ty: Option<WirmType>,
}
impl StackVal {
    fn new(name: String, ty: Option<WirmType>) -> Self {
        Self { name, ty }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProbeRule {
    pub provider: Option<RulePart>,
    pub package: Option<RulePart>,
    pub event: Option<RulePart>,
    pub mode: Option<ModeKind>,
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
    pub(crate) dynamic_alias: HashMap<String, (WirmType, VarAddr)>,

    /// dynamic information corresponding to the operands of this location
    pub(crate) args: Vec<StackVal>,
    /// dynamic information corresponding to the results of this location
    pub(crate) results: Vec<StackVal>,

    pub num_alt_probes: usize,
    /// the probes that were matched for this instruction
    /// note the Script ID is contained in Probe
    pub probes: Vec<(ProbeRule, Probe, Option<InstrumentationMode>)>,
}
impl LocInfo {
    fn new() -> Self {
        Self::default()
    }
    fn has_match(&self) -> bool {
        !self.probes.is_empty()
    }
    fn configure_stack_reqs(
        &mut self,
        req_args: StackReq,
        all_args: Vec<StackVal>,
        req_res: StackReq,
        all_res: Vec<StackVal>,
    ) {
        if req_args.is_some() {
            self.args = req_args.of(all_args);
        }
        if req_res.is_some() {
            self.results = req_res.of(all_res);
        }
    }
    fn add_probes(
        &mut self,
        base_rule: ProbeRule,
        probes: &SimpleEvt,
        mode: Option<InstrumentationMode>,
    ) {
        probes.modes.iter().for_each(|(probe_mode, probes)| {
            let mut rule = base_rule.clone();
            rule.mode = Some(probe_mode.clone());

            if matches!(probe_mode, ModeKind::Alt) {
                // this is an alt probe, mark it with the number!
                self.num_alt_probes += probes.len();
            }
            probes.iter().for_each(|probe| {
                // TODO -- remove this probe.clone()...works for now though...
                self.probes.push((rule.clone(), probe.clone(), mode));
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
                    panic!(
                        "Emitter rules found different values for instruction args, please report this bug!"
                    );
                }
            }
            // just keep self args the way it is (other clearly doesn't populate them)
        } else {
            // just set to the other's args
            self.args = other.args.to_owned()
        }
        // TODO -- factor logic
        // handle results
        if !self.results.is_empty() {
            if !other.results.is_empty() {
                // assert that results are equivalent
                if !self.results.iter().all(|item| other.results.contains(item)) {
                    panic!(
                        "Emitter rules found different values for instruction results, please report this bug!"
                    );
                }
            }
            // just keep self results the way it is (other clearly doesn't populate them)
        } else {
            // just set to the other's results
            self.results = other.results.to_owned()
        }

        // handle num_alt_probes
        self.num_alt_probes += other.num_alt_probes;

        // handle function end
        self.is_prog_exit = self.is_prog_exit || other.is_prog_exit;

        // handle probes
        self.probes.append(&mut other.probes);
    }
}

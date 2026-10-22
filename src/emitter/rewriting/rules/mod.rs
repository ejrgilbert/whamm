use crate::common::error::ErrorGen;
use crate::emitter::rewriting::mono;
use crate::emitter::rewriting::rules::data_segments::get_first_local_mem_id;
use crate::emitter::rewriting::visiting_emitter::{VisitingEmitter, UNKNOWN_FID};
use crate::generator::ast::{Probe, StackReq, WhammParam, WhammParams};
use crate::generator::rewriting::simple_ast::{SimpleAST, SimpleEvt, SimplePkg, SimpleProv};
use crate::parser::provider_handler::ModeKind;
use crate::parser::types::{
    Block, DataType, DeclModifiers, Definition, Expr, NumFmt, NumLit, RulePart, Statement, Value,
};
use crate::verifier::types::VarAddr;
use log::warn;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use wirm::ir::id::{FunctionID, GlobalID, MemoryID, TypeID};
use wirm::ir::module::module_functions::{FuncKind, ImportedFunction};
use wirm::ir::module::module_globals::{GlobalKind, ImportedGlobal, LocalGlobal};
use wirm::ir::module::module_types::Types;
use wirm::ir::module::Module;
use wirm::ir::types::{DataType as WirmType, InstrumentationMode};
use wirm::wasmparser::{BlockType, BrTable, GlobalType, MemArg, Operator};
use wirm::Location;

pub(crate) mod data_segments;

pub fn get_loc_info_for_active_probes(
    app_wasm: &Module,
    state: &mut MatchState,
    loc: Location,
    at_func_end: bool,
    instr: &Operator,
    ast: &mut SimpleAST,
    err: &mut ErrorGen,
) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    for (provider, packages) in ast.provs.iter_mut() {
        if let Some(mut tmp) = handle_provider(
            app_wasm,
            state,
            loc,
            at_func_end,
            instr,
            provider,
            packages,
            err,
        ) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }
    if let Some(r) = &mut res {
        r.static_data.extend(init_whamm_bound_vars(app_wasm, err));
        reject_invalid_opcode_var_deps(r, err);
    }
    res
}

/// Drop probes that reference a var the matched opcode does not provide.
fn reject_invalid_opcode_var_deps(loc_info: &mut LocInfo, err: &mut ErrorGen) {
    // Only opcode locations carry a matched opcode; func/block events have none.
    let Some(event) = loc_info.opcode_name.clone() else {
        return;
    };

    let mut reported_invalid_vars = HashSet::new();
    let mut to_remove = vec![];
    for (i, (rule, probe, _)) in loc_info.probes.iter().enumerate() {
        let event_is_explicit = rule
            .event
            .as_ref()
            .map(|e| e.name == event)
            .unwrap_or(false);
        let bad_var = param_items(&probe.metadata.body_args)
            .chain(param_items(&probe.metadata.pred_args))
            .chain(type_bound_items(&probe.type_bounds))
            .map(|(name, _)| name)
            .find(|name| !loc_info.provides_var(name));
        if let Some(name) = bad_var {
            if event_is_explicit {
                // The probe named this exact opcode (no glob), user error instead of non-match
                let key = format!("{}_{}_{event}_{name}", probe.script_id, probe.probe_number);
                if reported_invalid_vars.insert(key) {
                    err.add_instr_error_at(
                        &format!("`{name}` is not a valid bound var for `{event}`"),
                        &probe.loc,
                    );
                }
            }
            to_remove.push(i);
        }
    }
    for i in to_remove.into_iter().rev() {
        loc_info.probes.remove(i);
    }
}

pub fn init_whamm_bound_vars(
    app_wasm: &Module,
    err: &mut ErrorGen,
) -> HashMap<String, Option<Value>> {
    let mut vars = HashMap::new();

    let memid = get_first_local_mem_id(app_wasm)
        .map_err(|e| err.add_instr_error(&e))
        .unwrap_or(MemoryID(0));
    vars.insert("APP_MEMID".to_string(), Some(Value::gen_u32(*memid)));

    vars
}

fn handle_provider(
    app_wasm: &Module,
    state: &mut MatchState,
    loc: Location,
    at_func_end: bool,
    instr: &Operator,
    provider: &str,
    prov: &mut SimpleProv,
    err: &mut ErrorGen,
) -> Option<LocInfo> {
    match provider {
        "wasm" => handle_wasm(app_wasm, state, loc, at_func_end, instr, prov, err),
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
    err: &mut ErrorGen,
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
            err,
        ) {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }

    if let Some(ref mut matches) = res {
        let all_locals = all_locals(app_wasm, &fid);
        // Represent `all_locals` as `StackVal`s for code reuse
        let local_stack: Vec<StackVal> = all_locals
            .iter()
            .map(|ty| StackVal::new(String::new(), Some(*ty)))
            .collect();
        // Check that every `local`is in range and is type-compatible.
        matches.filter_probes(|probe| {
            let body = &probe.metadata.body_args;
            let pred = &probe.metadata.pred_args;
            check_var_types(&local_stack, param_items(body), "local")
                && check_var_types(&local_stack, param_items(pred), "local")
                && check_var_types(&local_stack, type_bound_items(&probe.type_bounds), "local")
        });

        // Monomorphize any generic probes for this site: bind each type parameter to the
        // concrete operand type here and substitute it into the per-site probe clone.
        // Probes whose type vars can't be bound at this site (missing/unadmitted/
        // inconsistent operand types) are dropped.
        if matches
            .probes
            .iter()
            .any(|(_, p, _)| !p.type_params.is_empty())
        {
            let (all_args, all_results, ..) = get_ty_info_for_instr(app_wasm, &fid, instr);
            let mut kept = Vec::with_capacity(matches.probes.len());
            for (rule, mut probe, mode) in matches.probes.drain(..) {
                if probe.type_params.is_empty() {
                    kept.push((rule, probe, mode));
                    continue;
                }
                if let Some(binding) =
                    mono::bind_type_params_at_site(&probe, &all_args, &all_results, &local_stack)
                {
                    mono::monomorphize_probe(&mut probe, &binding);
                    kept.push((rule, probe, mode));
                }
                // else: no binding at this site -> drop the probe
            }
            matches.probes = kept;
        }

        let probes = &matches.probes;
        let dynamic_alias = &mut matches.dynamic_alias;
        for (_, probe, _) in probes.iter() {
            for params in [&probe.metadata.body_args, &probe.metadata.pred_args] {
                for (name, _) in param_items(params) {
                    let Some(n) = nth_prefixed(name, "local") else {
                        continue;
                    };
                    let Some(ty) = all_locals.get(n as usize) else {
                        continue;
                    };
                    dynamic_alias.insert(format!("local{n}"), (*ty, VarAddr::Local { addr: n }));
                }
            }
        }
    }

    res
}

fn all_locals(app_wasm: &Module, fid: &FunctionID) -> Vec<WirmType> {
    let func = app_wasm.functions.get(*fid).unwrap_local().unwrap();

    let mut locals = vec![];
    for arg in func.args.iter() {
        let wasm_ty = if let Some(Types::FuncType { params, .. }) = app_wasm.types.get(func.ty_id) {
            params.get(**arg as usize).unwrap()
        } else {
            panic!(
                "Unable to lookup the function type with ID: {}",
                *func.ty_id
            );
        };
        locals.push(*wasm_ty);
    }

    for (_, wasm_ty) in func.body.locals.iter() {
        locals.push(*wasm_ty);
    }

    locals
}

pub(crate) fn nth_prefixed(name: &str, prefix: &str) -> Option<u32> {
    name.strip_prefix(prefix)
        .and_then(|rest| rest.parse::<u32>().ok())
}

fn check_var_types<'a>(
    stack_vals: &[StackVal],
    items: impl IntoIterator<Item = (&'a str, &'a DataType)>,
    prefix: &str,
) -> bool {
    for (name, declared_ty) in items {
        // Type-variable operands are admitted/bound per site by mono::bind_type_params_at_site,
        // not by exact compatibility here — skip them so generic probes aren't filtered out.
        if matches!(declared_ty, DataType::TypeParam(_)) {
            continue;
        }
        let Some(n) = nth_prefixed(name, prefix) else {
            continue;
        };
        let Some(target_val) = stack_vals.get(n as usize) else {
            // index beyond what this instruction provides -> not a match
            return false;
        };
        if let Some(target_ty) = target_val.ty {
            if !declared_ty.is_compatible_with(&DataType::from_wasm_type(&target_ty)) {
                // wrong type for this position -> not a match
                return false;
            }
        }
        // else: no type info for this stack value, assume compatible
    }
    true
}

enum BoundMatch {
    /// type is known satisfied
    Match,
    /// type is known, but not satisfied (or out of range)
    Mismatch,
    /// type could not be determined
    Unresolved,
}
fn check_arg_type_bound(stack_vals: &[StackVal], name: &str, declared_ty: &DataType) -> BoundMatch {
    let Some(n) = nth_prefixed(name, "arg") else {
        // it's not an arg
        return BoundMatch::Match;
    };
    let Some(target_val) = stack_vals.get(n as usize) else {
        // index out of bound
        return BoundMatch::Mismatch;
    };
    match target_val.ty {
        Some(target_ty) => {
            if declared_ty.is_compatible_with(&DataType::from_wasm_type(&target_ty)) {
                BoundMatch::Match
            } else {
                BoundMatch::Mismatch
            }
        }
        None => BoundMatch::Unresolved,
    }
}

fn param_items(params: &WhammParams) -> impl Iterator<Item = (&str, &DataType)> {
    params.params.iter().map(|p| (p.name.as_str(), &p.ty))
}
fn type_bound_items(type_bounds: &[(Expr, DataType)]) -> impl Iterator<Item = (&str, &DataType)> {
    type_bounds.iter().filter_map(|(var, ty)| match var {
        // Type-variable operands are matched/bound separately (see mono::bind_type_params_at_site);
        // only concrete bounds participate in the exact-compatibility filter here.
        Expr::VarId { name, .. } if !matches!(ty, DataType::TypeParam(_)) => {
            Some((name.as_str(), ty))
        }
        _ => None,
    })
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
    err: &mut ErrorGen,
) -> Option<LocInfo> {
    match package {
        "opcode" => handle_opcode(app_wasm, state, fid, opidx, instr, pkg, err),
        "func" => handle_func(app_wasm, fid, opidx, instr, pkg),
        "block" => handle_block(app_wasm, state, at_func_end, fid, opidx, instr, pkg),
        "begin" | "end" => unimplemented!("Have not implemented the package yet: {package}"),
        "report" => None, // not handled here
        _ => panic!("Package not available: 'wasm:{package}'"),
    }
}

fn handle_opcode(
    app_wasm: &Module,
    state: &mut MatchState,
    fid: &FunctionID,
    opidx: usize,
    instr: &Operator,
    pkg: &mut SimplePkg,
    err: &mut ErrorGen,
) -> Option<LocInfo> {
    let mut res: Option<LocInfo> = None;
    // Whether whamm has an event for this concrete opcode at all
    let mut covered = false;
    for (package, evt) in pkg.evts.iter_mut() {
        // See OpcodeEvent.get_loc_info
        let (tmp, is_covered) =
            handle_opcode_events(app_wasm, state, fid, opidx, instr, package, evt, err);
        covered |= is_covered;
        if let Some(mut tmp) = tmp {
            if let Some(r) = &mut res {
                r.append(&mut tmp);
            } else {
                res = Some(tmp);
            }
        }
    }

    // Warn if a glob match misses some possible Wasm opcode (because Whamm doesn't support it yet)
    if pkg.opcode_wildcard && !covered {
        let name = uncovered_opcode_name(instr);
        if state.warned_uncovered_opcodes.insert(name.clone()) {
            err.add_probe_warn(
                &format!(
                    "your program would match a `{name}` opcode, but Whamm cannot instrument that yet. \
                     Please open GH issue if this is a blocker for your use case."
                ),
                &None,
            );
        }
    }

    res
}

fn uncovered_opcode_name(instr: &Operator) -> String {
    let dbg = format!("{instr:?}");
    dbg.split([' ', '{', '('])
        .next()
        .unwrap_or(&dbg)
        .to_string()
}

#[rustfmt::skip]
fn handle_opcode_events(
    app_wasm: &Module,
    state: &mut MatchState,
    fid: &FunctionID,
    opidx: usize,
    instr: &Operator,
    event: &String,
    evt: &mut SimpleEvt,
    err: &mut ErrorGen,
) -> (Option<LocInfo>, bool) {
    let mut loc_info = LocInfo::new();
    loc_info.opcode_name = Some(event.clone());
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

    let mut covered = true;
    match instr {
        Operator::Unreachable => if event == "unreachable" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Nop => if event == "nop" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Block {..} => if event == "block" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Loop {..} => if event == "loop" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::If {..} => if event == "if" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Else {..} => if event == "else" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TryTable {..} => if event == "try_table" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Throw { tag_index } => if event == "throw" {
            define_imm0::<u32>(*tag_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ThrowRef {..} => if event == "throw_ref" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::End {..} => if event == "end" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Br { relative_depth } => if event == "br" {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::BrIf { relative_depth } => if event == "br_if" {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::BrTable { targets } => if event == "br_table" {
            if bind_vars_br_table(targets, &mut loc_info, all_params).is_none() {
                return (None, true);
            }
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Return {..} => if event == "return" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Call {function_index} => if event == "call" {
            bind_vars_call(&mut loc_info, all_params, *function_index, app_wasm);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::CallIndirect {type_index,
            table_index,} => if event == "call_indirect" {
            define_imm0_u32_imm1_u32(*type_index, *table_index, &mut loc_info, all_params);
            loc_info.funcref_table_idx = Some(*table_index);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ReturnCall {function_index} => if event == "return_call" {
            bind_vars_call(&mut loc_info, all_params, *function_index, app_wasm);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ReturnCallIndirect {type_index, table_index } => if event == "return_call_indirect" {
            define_imm0_u32_imm1_u32(*type_index, *table_index, &mut loc_info, all_params);
            loc_info.funcref_table_idx = Some(*table_index);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Drop => if event == "drop" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::Select => if event == "select" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TypedSelect {..} => if event == "typed_select" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::LocalGet {local_index} => if event == "local.get" {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::LocalSet {local_index} => if event == "local.set" {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::LocalTee {local_index} => if event == "local.tee" {
            define_imm0::<u32>(*local_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::GlobalGet {global_index} => if event == "global.get" {
            define_imm0::<u32>(*global_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::GlobalSet {global_index} => if event == "global.set" {
            define_imm0::<u32>(*global_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.load" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.load" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "f32.load" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Load {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "f64.load" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Load8S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.load8_s" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Load8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.load8_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Load16S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.load16_s" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Load16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.load16_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Load8S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.load8_s" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Load8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.load8_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Load16S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.load16_s" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Load16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.load16_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Load32S {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.load32_s" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Load32U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.load32_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.store" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.store" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "f32.store" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Store {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "f64.store" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Store8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.store8" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Store16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.store16" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Store8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.store8" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Store16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.store16" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Store32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.store32" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemorySize {mem} => if event == "memory.size" {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemoryGrow {mem} => if event == "memory.grow" {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Const {value} => if event == "i32.const" {
            define_imm0::<i32>(*value, DataType::I32, &Value::gen_i32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Const {value} => if event == "i64.const" {
            define_imm0::<i64>(*value, DataType::I64, &Value::gen_i64, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Const {value} => if event == "f32.const" {
            define_imm0::<f32>(f32::from(*value), DataType::F32, &Value::gen_f32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Const {value} => if event == "f64.const" {
            define_imm0::<f64>(f64::from(*value), DataType::F64, &Value::gen_f64, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefNull {..} => if event == "ref.null" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefIsNull {..} => if event == "ref.is_null" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefFunc {function_index} => if event == "ref.func" {
            define_imm0::<u32>(*function_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefEq {..} => if event == "ref.eq" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Eqz {..} => if event == "i32.eqz" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Eq {..} => if event == "i32.eq" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Ne {..} => if event == "i32.ne" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32LtS {..} => if event == "i32.lt_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32LtU {..} => if event == "i32.lt_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32GtS {..} => if event == "i32.gt_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32GtU {..} => if event == "i32.gt_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32LeS {..} => if event == "i32.le_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32LeU {..} => if event == "i32.le_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32GeS {..} => if event == "i32.ge_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32GeU {..} => if event == "i32.ge_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Eqz {..} => if event == "i64.eqz" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Eq {..} => if event == "i64.eq" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Ne {..} => if event == "i64.ne" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64LtS {..} => if event == "i64.lt_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64LtU {..} => if event == "i64.lt_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64GtS {..} => if event == "i64.gt_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64GtU {..} => if event == "i64.gt_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64LeS {..} => if event == "i64.le_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64LeU {..} => if event == "i64.le_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64GeS {..} => if event == "i64.ge_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64GeU {..} => if event == "i64.ge_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Eq {..} => if event == "f32.eq" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Ne {..} => if event == "f32.ne" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Lt {..} => if event == "f32.lt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Gt {..} => if event == "f32.gt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Le {..} => if event == "f32.le" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Ge {..} => if event == "f32.ge" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Eq {..} => if event == "f64.eq" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Ne {..} => if event == "f64.ne" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Lt {..} => if event == "f64.lt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Gt {..} => if event == "f64.gt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Le {..} => if event == "f64.le" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Ge {..} => if event == "f64.ge" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Clz {..} => if event == "i32.clz" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Ctz {..} => if event == "i32.ctz" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Popcnt {..} => if event == "i32.popcnt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Add {..} => if event == "i32.add" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Sub {..} => if event == "i32.sub" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Mul {..} => if event == "i32.mul" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32DivS {..} => if event == "i32.div_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32DivU {..} => if event == "i32.div_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32RemS {..} => if event == "i32.rem_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32RemU {..} => if event == "i32.rem_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32And {..} => if event == "i32.and" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Or {..} => if event == "i32.or" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Xor {..} => if event == "i32.xor" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Shl {..} => if event == "i32.shl" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32ShrS {..} => if event == "i32.shr_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32ShrU {..} => if event == "i32.shr_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Rotl {..} => if event == "i32.rotl" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Rotr {..} => if event == "i32.rotr" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Clz {..} => if event == "i64.clz" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Ctz {..} => if event == "i64.ctz" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Popcnt {..} => if event == "i64.popcnt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Add {..} => if event == "i64.add" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Sub {..} => if event == "i64.sub" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Mul {..} => if event == "i64.mul" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64DivS {..} => if event == "i64.div_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64DivU {..} => if event == "i64.div_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64RemS {..} => if event == "i64.rem_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64RemU {..} => if event == "i64.rem_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64And {..} => if event == "i64.and" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Or {..} => if event == "i64.or" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Xor {..} => if event == "i64.xor" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Shl {..} => if event == "i64.shl" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64ShrS {..} => if event == "i64.shr_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64ShrU {..} => if event == "i64.shr_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Rotl {..} => if event == "i64.rotl" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Rotr {..} => if event == "i64.rotr" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Abs {..} => if event == "f32.abs" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Neg {..} => if event == "f32.neg" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Ceil {..} => if event == "f32.ceil" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Floor {..} => if event == "f32.floor" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Trunc {..} => if event == "f32.trunc" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Nearest {..} => if event == "f32.nearest" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Sqrt {..} => if event == "f32.sqrt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Add {..} => if event == "f32.add" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Sub {..} => if event == "f32.sub" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Mul {..} => if event == "f32.mul" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Div {..} => if event == "f32.div" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Min {..} => if event == "f32.min" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Max {..} => if event == "f32.max" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32Copysign {..} => if event == "f32.copysign" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Abs {..} => if event == "f64.abs" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Neg {..} => if event == "f64.neg" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Ceil {..} => if event == "f64.ceil" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Floor {..} => if event == "f64.floor" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Trunc {..} => if event == "f64.trunc" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Nearest {..} => if event == "f64.nearest" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Sqrt {..} => if event == "f64.sqrt" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Add {..} => if event == "f64.add" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Sub {..} => if event == "f64.sub" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Mul {..} => if event == "f64.mul" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Div {..} => if event == "f64.div" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Min {..} => if event == "f64.min" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Max {..} => if event == "f64.max" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64Copysign {..} => if event == "f64.copysign" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32WrapI64 {..} => if event == "i32.wrap_i64" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncF32S {..} => if event == "i32.trunc_f32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncF32U {..} => if event == "i32.trunc_f32_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncF64S {..} => if event == "i32.trunc_f64_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncF64U {..} => if event == "i32.trunc_f64_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64ExtendI32S {..} => if event == "i64.extend_i32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64ExtendI32U {..} => if event == "i64.extend_i32_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncF32S {..} => if event == "i64.trunc_f32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncF32U {..} => if event == "i64.trunc_f32_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncF64S {..} => if event == "i64.trunc_f64_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncF64U {..} => if event == "i64.trunc_f64_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32ConvertI32S {..} => if event == "f32.convert_i32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32ConvertI32U {..} => if event == "f32.convert_i32_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32ConvertI64S {..} => if event == "f32.convert_i64_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32ConvertI64U {..} => if event == "f32.convert_i64_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32DemoteF64 {..} => if event == "f32.demote_f64" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64ConvertI32S {..} => if event == "f64.convert_i32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64ConvertI32U {..} => if event == "f64.convert_i32_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64ConvertI64S {..} => if event == "f64.convert_i64_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64ConvertI64U {..} => if event == "f64.convert_i64_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64PromoteF32 {..} => if event == "f64.promote_f32" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32ReinterpretF32 {..} => if event == "i32.reinterpret_f32" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64ReinterpretF64 {..} => if event == "i64.reinterpret_f64" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F32ReinterpretI32 {..} => if event == "f32.reinterpret_i32" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::F64ReinterpretI64 {..} => if event == "f64.reinterpret_i64" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Extend8S {..} => if event == "i32.extend8_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32Extend16S {..} => if event == "i32.extend16_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Extend8S {..} => if event == "i64.extend8_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Extend16S {..} => if event == "i64.extend16_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64Extend32S {..} => if event == "i64.extend32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::StructNew {struct_type_index} => if event == "struct.new" {
            define_imm0::<u32>(*struct_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::StructNewDefault {struct_type_index} => if event == "struct.new_default" {
            define_imm0::<u32>(*struct_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::StructGet {struct_type_index, field_index} => if event == "struct.get" {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::StructGetS {struct_type_index, field_index} => if event == "struct.get_s" {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::StructGetU {struct_type_index, field_index} => if event == "struct.get_u" {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::StructSet {struct_type_index, field_index} => if event == "struct.set" {
            define_imm0_u32_imm1_u32(*struct_type_index, *field_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayNew {array_type_index} => if event == "array.new" {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayNewDefault {array_type_index} => if event == "array.new_default" {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayNewFixed {array_type_index, array_size} => if event == "array.new_fixed" {
            define_imm0_u32_imm1_u32(*array_type_index, *array_size, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayNewData {array_type_index, array_data_index} => if event == "array.new_data" {
            define_imm0_u32_imm1_u32(*array_type_index, *array_data_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayNewElem {array_type_index, array_elem_index} => if event == "array.new_elem" {
            define_imm0_u32_imm1_u32(*array_type_index, *array_elem_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayGet {array_type_index} => if event == "array.get" {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayGetS {array_type_index} => if event == "array.get_s" {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayGetU {array_type_index} => if event == "array.get_u" {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArraySet {array_type_index} => if event == "array.set" {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayLen => if event == "array.len" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayFill {array_type_index} => if event == "array.fill" {
            define_imm0::<u32>(*array_type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayCopy {array_type_index_dst, array_type_index_src} => if event == "array.copy" {
            define_imm0_u32_imm1_u32(*array_type_index_dst, *array_type_index_src, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayInitData {array_type_index, array_data_index} => if event == "array.init_data" {
            define_imm0_u32_imm1_u32(*array_type_index, *array_data_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ArrayInitElem {array_type_index, array_elem_index} => if event == "array.init_elem" {
            define_imm0_u32_imm1_u32(*array_type_index, *array_elem_index, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefTestNonNull {..} => if event == "ref.test" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefTestNullable {..} => if event == "ref.test_null" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefCastNonNull {..} => if event == "ref.cast" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefCastNullable {..} => if event == "ref.cast_null" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::BrOnCast {relative_depth, ..} => if event == "br_on_cast" {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::BrOnCastFail {relative_depth, ..} => if event == "br_on_cast_fail" {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::AnyConvertExtern => if event == "any.convert_extern" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ExternConvertAny => if event == "extern.convert_any" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefI31 => if event == "ref.i31" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I31GetS => if event == "i31.get_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I31GetU => if event == "i31.get_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncSatF32S => if event == "i32.trunc_sat_f32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncSatF32U => if event == "i32.trunc_sat_f32_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncSatF64S => if event == "i32.trunc_sat_f64_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32TruncSatF64U => if event == "i32.trunc_sat_f64_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncSatF32S => if event == "i64.trunc_sat_f32_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncSatF32U => if event == "i64.trunc_sat_f32_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncSatF64S => if event == "i64.trunc_sat_f64_s" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64TruncSatF64U => if event == "i64.trunc_sat_f64_u" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemoryInit {data_index, mem} => if event == "memory.init" {
            define_imm0_u32_imm1_u32(*data_index, *mem, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemoryCopy {dst_mem, src_mem} => if event == "memory.copy" {
            define_imm0_u32_imm1_u32(*dst_mem, *src_mem, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemoryFill {mem} => if event == "memory.fill" {
            define_imm0::<u32>(*mem, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::DataDrop {data_index} => if event == "data.drop" {
            define_imm0::<u32>(*data_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ElemDrop {elem_index} => if event == "elem.drop" {
            define_imm0::<u32>(*elem_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TableCopy {dst_table, src_table} => if event == "table.copy" {
            define_imm0_u32_imm1_u32(*dst_table, *src_table, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TableInit {elem_index, table} => if event == "table.init" {
            define_imm0_u32_imm1_u32(*elem_index, *table, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TableFill {table} => if event == "table.fill" {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TableGet {table} => if event == "table.get" {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.funcref_table_idx = Some(*table);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TableSet {table} => if event == "table.set" {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            define_resolved_fid_via_b1_lookback(app_wasm, fid, opidx, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TableGrow {table} => if event == "table.grow" {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::TableSize {table} => if event == "table.size" {
            define_imm0::<u32>(*table, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemoryAtomicNotify {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "memory.atomic.notify" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemoryAtomicWait32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "memory.atomic.wait32" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::MemoryAtomicWait64 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "memory.atomic.wait64" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::AtomicFence => if event == "atomic.fence" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicLoad {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.load" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicLoad {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.load" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicLoad8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.load8_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicLoad16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.load16_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicLoad8U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.load8_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicLoad16U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.load16_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicLoad32U {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.load32_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicStore {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.store" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicStore8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.store8" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicStore16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.store16" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicStore {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.store" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicStore8 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.store8" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicStore16 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.store16" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicStore32 {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.store32" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmwAdd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw.add" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw8AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw8.add_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw16AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw16.add_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmwAdd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw.add" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw8AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw8.add_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw16AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw16.add_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw32AddU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw32.add_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmwSub {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw.sub" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw8SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw8.sub_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw16SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw16.sub_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmwSub {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw.sub" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw8SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw8.sub_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw16SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw16.sub_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw32SubU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw32.sub_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmwAnd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw.and" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw8AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw8.and_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw16AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw16.and_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmwAnd {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw.and" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw8AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw8.and_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw16AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw16.and_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw32AndU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw32.and_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmwOr {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw.or" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw8OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw8.or_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw16OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw16.or_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmwOr {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw.or" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw8OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw8.or_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw16OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw16.or_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw32OrU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw32.or_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmwXor {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw.xor" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw8XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw8.xor_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw16XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw16.xor_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmwXor {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw.xor" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw8XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw8.xor_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw16XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw16.xor_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw32XorU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw32.xor_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmwXchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw.xchg" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw8XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw8.xchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw16XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw16.xchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmwXchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw.xchg" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw8XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw8.xchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw16XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw16.xchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw32XchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw32.xchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmwCmpxchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw.cmpxchg" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw8CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw8.cmpxchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I32AtomicRmw16CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i32.atomic.rmw16.cmpxchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmwCmpxchg {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw.cmpxchg" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw8CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw8.cmpxchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw16CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw16.cmpxchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::I64AtomicRmw32CmpxchgU {memarg: MemArg {
            align,
            offset,
            memory,
            ..
        }} => if event == "i64.atomic.rmw32.cmpxchg_u" {
            bind_vars_memarg(*align, *offset, *memory, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::CallRef {type_index} => if event == "call_ref" {
            define_imm0::<u32>(*type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            define_resolved_fid_via_b1_lookback(app_wasm, fid, opidx, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::ReturnCallRef {type_index} => if event == "return_call_ref" {
            define_imm0::<u32>(*type_index, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            define_resolved_fid_via_b1_lookback(app_wasm, fid, opidx, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::RefAsNonNull => if event == "ref.as_non_null" {
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::BrOnNull {relative_depth} => if event == "br_on_null" {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        Operator::BrOnNonNull {relative_depth} => if event == "br_on_non_null" {
            define_imm0::<u32>(*relative_depth, DataType::U32, &Value::gen_u32, &mut loc_info, all_params);
            loc_info.add_probes(probe_rule.clone(), evt, None);
        },
        _ => covered = false,
    }

    let (all_args, all_results, ..) = get_ty_info_for_instr(app_wasm, fid, instr);

    // warn on unknowable operand args (til we land the abstract interpreter)
    for (_, probe, _) in loc_info.probes.iter() {
        let has_unresolved_bound = type_bound_items(&probe.type_bounds).any(|(name, ty)| {
            matches!(check_arg_type_bound(&all_args, name, ty), BoundMatch::Unresolved)
        });
        if has_unresolved_bound {
            let key = format!("{}_{}_{event}", probe.script_id, probe.probe_number);
            if state.warned_unresolved_operand_bounds.insert(key) {
                err.add_probe_warn(
                    &format!(
                        "cannot apply probe to the `{event}` since its operand types could \
                         not be determined. Supporting polymorphic opcodes is future work."
                    ),
                    &probe.loc,
                );
            }
        }
    }

    // figure out which args are requested based on matched probes
    // (we don't have a match if the requested argument or result is beyond the
    // length of what's possible)
    loc_info.filter_probes(|probe| {
        let body = &probe.metadata.body_args;
        let pred = &probe.metadata.pred_args;
        let bounds = &probe.type_bounds;
        check_var_types(&all_args, param_items(body), "arg")
            && check_var_types(&all_args, param_items(pred), "arg")
            && check_var_types(&all_results, param_items(body), "res")
            && check_var_types(&all_results, param_items(pred), "res")
            && type_bound_items(bounds)
                .all(|(name, ty)| matches!(check_arg_type_bound(&all_args, name, ty), BoundMatch::Match))
            && check_var_types(&all_results, type_bound_items(bounds), "res")

        // NOTE: we don't check other bound vars here, we do that after we've merged
        // all matches against the full operand set to user-error OR un-match
    });

    for (_, probe, _) in loc_info.probes.iter() {
        req_args.combine(&probe.metadata.body_args.req_args);
        req_args.combine(&probe.metadata.pred_args.req_args);

        req_results.combine(&probe.metadata.body_args.req_results);
        req_results.combine(&probe.metadata.pred_args.req_results);
    }

    loc_info.configure_stack_reqs(req_args, all_args, req_results, all_results);

    loc_info.is_prog_exit = is_prog_exit_call(instr, app_wasm);
    loc_info.is_trap_exit = is_unconditional_trap(instr);
    let result = if loc_info.has_match() || loc_info.is_prog_exit || loc_info.is_trap_exit {
        Some(loc_info)
    } else {
        None
    };
    (result, covered)
}

fn define_imm0_u32_imm1_u32(
    value0: u32,
    value1: u32,
    loc_info: &mut LocInfo,
    all_params: &[WhammParam],
) {
    for param in all_params.iter() {
        if let Some(n) = param.n_for("imm") {
            if n == 0 {
                define_imm_n(n, Some(Value::gen_u32(value0)), loc_info);
            } else if n == 1 {
                define_imm_n(n, Some(Value::gen_u32(value1)), loc_info);
            }
            // leave out of range immediates unbound, error at emit time
        }
    }
}

fn define_imm0<T>(
    value: T,
    _dt: DataType,
    gen: &dyn Fn(T) -> Value,
    loc_info: &mut LocInfo,
    all_params: &[WhammParam],
) {
    for param in all_params.iter() {
        if let Some(n) = param.n_for("imm") {
            if n != 0 {
                // leave out of range immediates unbound, error at emit time
                continue;
            }
            define_imm_n(0, Some(gen(value)), loc_info);
            return;
        }
    }
}

fn define_imm_n(n: u32, val: Option<Value>, loc_info: &mut LocInfo) {
    loc_info.static_data.insert(format!("imm{n}"), val);
}

/// If the instruction at `opidx - 1` in the body of `fid` is `ref.func $N`,
/// return `Some(N)`. Otherwise `None`.
fn prev_ref_func_fid(app_wasm: &Module, fid: &FunctionID, opidx: usize) -> Option<u32> {
    if opidx == 0 {
        return None;
    }
    let local_func = app_wasm.functions.get(*fid).unwrap_local().ok()?;
    if let Operator::RefFunc { function_index } =
        local_func.body.instructions.get_ops().get(opidx - 1)?
    {
        Some(*function_index)
    } else {
        None
    }
}

/// Bind `resolved_fid` for events whose funcref came in on the stack
/// (`table.set`, `call_ref`, `return_call_ref`). Recovers the fid via
/// single-instruction lookback for a preceding `ref.func`; otherwise binds the
/// `UNKNOWN_FID` sentinel. No-ops if no probe references `resolved_fid`.
fn define_resolved_fid_via_b1_lookback(
    app_wasm: &Module,
    fid: &FunctionID,
    opidx: usize,
    loc_info: &mut LocInfo,
    all_params: &[WhammParam],
) {
    if !all_params.iter().any(|p| p.name == "resolved_fid") {
        return;
    }
    let val = prev_ref_func_fid(app_wasm, fid, opidx)
        .map(|n| n as i32)
        .unwrap_or(UNKNOWN_FID);
    loc_info.static_data.insert(
        "resolved_fid".to_string(),
        Some(Value::Number {
            val: NumLit::I32 { val },
            ty: DataType::I32,
            token: String::new(),
            fmt: NumFmt::NA,
        }),
    );
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
            if n > targets.len() || !matches!(param.ty, DataType::U32) {
                // this location doesn't match since the immN is out of bound
                // of the immN's available
                return None;
            }

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

/// Binds the call-site's static data (`target_fn_name`, `target_fn_type`,
/// `target_imp_module`, and `imm0`=fid) for the given event.
///
/// `argN` arity/type checks are intentionally NOT done here — the generic
/// per-probe filter in `handle_wasm_packages` (see the `filter_probes` call
/// driven by `get_ty_info_for_instr`) handles them. Checking them here
/// would reject the whole event on any probe's mismatch, dropping sibling
/// probes that DO fit the call's signature.
fn bind_vars_call(loc_info: &mut LocInfo, all_params: &[WhammParam], fid: u32, app_wasm: &Module) {
    let func_info = match app_wasm.functions.get_kind(FunctionID(fid)) {
        FuncKind::Import(ImportedFunction { import_id, .. }) => {
            let import = app_wasm.imports.get(*import_id);
            FuncInfo {
                func_kind: "import".to_string(),
                module: import.module.to_string(),
                name: import.name.to_string(),
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
        },
    };

    for param in all_params.iter() {
        if param.n_for("arg").is_some() {
            // arg arity/type is validated per-probe in the generic filter.
        } else if let Some(n) = param.n_for("imm") {
            if n != 0 || !matches!(param.ty, DataType::U32) {
                // leave out of range immediates unbound, error at emit time
                continue;
            }
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
                    for t in ty.params().unwrap().iter().rev() {
                        args.push(Some(*t));
                    }
                    let mut results = vec![];
                    for t in ty.results().unwrap().iter().rev() {
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
            } => {
                if let Some(ty) = app_wasm.types.get(TypeID(*ty_id)) {
                    let mut args = vec![];
                    for t in ty.params().unwrap().iter().rev() {
                        args.push(Some(*t));
                    }
                    let mut results = vec![];
                    for t in ty.results().unwrap().iter().rev() {
                        results.push(Some(*t));
                    }
                    (args, results, Some(*ty_id))
                } else {
                    warn!("No type information found for opcode");
                    (vec![], vec![], None)
                }
            }
            Operator::CallIndirect {
                type_index: ty_id, ..
            }
            | Operator::ReturnCallIndirect {
                type_index: ty_id, ..
            } => {
                if let Some(ty) = app_wasm.types.get(TypeID(*ty_id)) {
                    let mut args = vec![];
                    // The table entry index (i32) is on top of the stack,
                    // popped first by call_indirect. It must be first in
                    // the args vec so local_set ordering matches.
                    args.push(Some(WirmType::I32));
                    for t in ty.params().unwrap().iter().rev() {
                        args.push(Some(*t));
                    }
                    let mut results = vec![];
                    for t in ty.results().unwrap().iter().rev() {
                        results.push(Some(*t));
                    }
                    (args, results, Some(*ty_id))
                } else {
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
            | Operator::F64Sqrt => (vec![Some(WirmType::F64)], vec![Some(WirmType::F64)], None),
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
                (vec![Some(WirmType::I64)], vec![Some(WirmType::I64)], None)
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
                    Some(WirmType::I64), // timeout
                    Some(WirmType::I32), // expected
                    Some(WirmType::I32), // addr
                ],
                vec![Some(WirmType::I32)],
                None,
            ),
            Operator::MemoryAtomicWait64 { .. } => (
                vec![
                    Some(WirmType::I64), // timeout
                    Some(WirmType::I64), // expected
                    Some(WirmType::I32), // addr
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
            | Operator::I32AtomicRmw16XchgU { .. } => (
                vec![Some(WirmType::I32), Some(WirmType::I32)],
                vec![Some(WirmType::I32)],
                None,
            ),

            // [addr, expected, replacement] -> [old]
            Operator::I32AtomicRmwCmpxchg { .. }
            | Operator::I32AtomicRmw8CmpxchgU { .. }
            | Operator::I32AtomicRmw16CmpxchgU { .. } => (
                vec![
                    Some(WirmType::I32),
                    Some(WirmType::I32),
                    Some(WirmType::I32),
                ],
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
            | Operator::I64AtomicRmw32XchgU { .. } => (
                vec![Some(WirmType::I64), Some(WirmType::I32)],
                vec![Some(WirmType::I64)],
                None,
            ),

            // [addr, expected, replacement] -> [old]
            Operator::I64AtomicRmwCmpxchg { .. }
            | Operator::I64AtomicRmw8CmpxchgU { .. }
            | Operator::I64AtomicRmw16CmpxchgU { .. }
            | Operator::I64AtomicRmw32CmpxchgU { .. } => (
                vec![
                    Some(WirmType::I64),
                    Some(WirmType::I64),
                    Some(WirmType::I32),
                ],
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
    warned_uncovered_opcodes: HashSet<String>,
    warned_unresolved_operand_bounds: HashSet<String>,
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
                define_block_data(event.as_str(), block_state, &mut loc_info);
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
    loc_info.is_trap_exit = is_unconditional_trap(instr);
    if loc_info.has_match() || is_prog_exit || loc_info.is_trap_exit {
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
    loc_info.is_trap_exit = is_unconditional_trap(instr);
    if loc_info.has_match() || is_prog_exit || loc_info.is_trap_exit {
        Some(loc_info)
    } else {
        None
    }
}

/// Opcodes that always trap (and thus terminates the program) when reached.
pub fn is_unconditional_trap(opcode: &Operator) -> bool {
    matches!(opcode, Operator::Unreachable)
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
impl From<&crate::generator::ast::ProbeRule> for ProbeRule {
    fn from(rule: &crate::generator::ast::ProbeRule) -> Self {
        let crate::generator::ast::ProbeRule {
            provider,
            package,
            event,
            mode,
        } = rule;

        Self {
            provider: (!provider.name.is_empty()).then(|| provider.clone()),
            package: (!package.name.is_empty()).then(|| package.clone()),
            event: (!event.name.is_empty()).then(|| event.clone()),
            mode: (!mode.name.is_empty()).then(|| ModeKind::from(mode.name.clone())),
        }
    }
}

#[derive(Default, Debug)]
pub struct LocInfo {
    /// Whether this location calls something that exits the program
    pub is_prog_exit: bool,
    /// Whether this location is an unconditional trap
    pub is_trap_exit: bool,
    /// static information to be saved in symbol table
    pub static_data: HashMap<String, Option<Value>>,
    /// dynamic information to be defined at the probe location
    pub dynamic_data: HashMap<String, Block>,
    pub(crate) dynamic_alias: HashMap<String, (WirmType, VarAddr)>,

    /// dynamic information corresponding to the operands of this location
    pub(crate) args: Vec<StackVal>,
    /// dynamic information corresponding to the results of this location
    pub(crate) results: Vec<StackVal>,

    /// If set, indicates this is a call_indirect/return_call_indirect site.
    /// The value is the table index (imm1) used to derive target_funcref via table.get.
    pub funcref_table_idx: Option<u32>,

    /// The concrete opcode matched at this location, if this is an opcode event.
    pub(crate) opcode_name: Option<String>,

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
    fn provides_var(&self, name: &str) -> bool {
        if self.static_data.contains_key(name)
            || self.dynamic_data.contains_key(name)
            || self.dynamic_alias.contains_key(name)
        {
            return true;
        }
        // We know whether this will be derived _later_ based on
        // whether we have table indices now
        if name == "target_funcref" {
            return self.funcref_table_idx.is_some();
        }
        // already validated previously, accept these
        nth_prefixed(name, "arg").is_some()
            || nth_prefixed(name, "res").is_some()
            || nth_prefixed(name, "local").is_some()
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
    fn filter_probes<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut Probe) -> bool,
    {
        let mut to_remove = vec![];
        for (i, (_, probe, _)) in self.probes.iter_mut().enumerate() {
            if !keep(probe) {
                to_remove.push(i);
            }
        }
        to_remove.sort();
        to_remove.reverse();
        for i in to_remove {
            self.probes.remove(i);
        }
    }
    fn add_dynamic_value(&mut self, name: String, val: Value) {
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
            Value::NumericLiteral { .. } => {
                unreachable!("NumericLiteral must be resolved by the type checker before emission")
            }
            Value::U32U32Map { val: map_val } => {
                // create a declaration
                let decl = Statement::VarDecl {
                    name: name.clone(),
                    ty: val.ty(),
                    definition: Definition::CompilerDynamic,
                    modifiers: DeclModifiers::default(),
                    init: None,
                    loc: None,
                };
                // create assignments
                let mut stmts = vec![decl];
                for (key, val) in map_val.iter() {
                    stmts.push(Statement::SetMap {
                        map: name.clone(),
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
        // create a declaration
        let decl = Statement::VarDecl {
            name: name.clone(),
            ty,
            definition: Definition::CompilerDynamic,
            modifiers: DeclModifiers::default(),
            init: None,
            loc: None,
        };
        // create an assignment
        let assign = Statement::Assign {
            var_id: Expr::VarId {
                definition: Definition::CompilerDynamic,
                name: name.clone(),
                loc: None,
            },
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
        // TODO -- factor logic
        // handle results
        if !self.results.is_empty() {
            if !other.results.is_empty() {
                // assert that results are equivalent
                if !self.results.iter().all(|item| other.results.contains(item)) {
                    panic!("Emitter rules found different values for instruction results, please report this bug!");
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
        self.is_trap_exit = self.is_trap_exit || other.is_trap_exit;

        // handle funcref_table_idx
        if self.funcref_table_idx.is_none() {
            self.funcref_table_idx = other.funcref_table_idx;
        }

        // handle opcode_name (all merged events share one instruction)
        if self.opcode_name.is_none() {
            self.opcode_name = other.opcode_name.take();
        }

        // handle probes
        self.probes.append(&mut other.probes);
    }
}

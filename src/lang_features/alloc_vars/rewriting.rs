#![allow(clippy::too_many_arguments)]

use crate::common::error::ErrorGen;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use std::collections::HashMap;

#[derive(Default)]
pub struct UnsharedVarHandler {
    pub available_gids: HashMap<DataType, Vec<u32>>,
}
impl UnsharedVarHandler {
    pub fn add_available_gid(&mut self, gid: u32, ty: &DataType) {
        self.available_gids
            .entry(ty.clone())
            .and_modify(|list| {
                list.push(gid);
            })
            .or_insert(vec![gid]);
    }
    pub fn use_available_gid(
        &mut self,
        ty: &DataType,
        err_msg: &str,
        err: &mut ErrorGen,
    ) -> Option<u32> {
        if let Some(list) = self.available_gids.get_mut(ty) {
            if !list.is_empty() {
                return Some(list.remove(0));
            }
        }
        err.unexpected_error(
            true,
            Some(format!(
                "{err_msg} No available global {ty}s for unshared vars"
            )),
            None,
        );
        None
    }
    pub fn allocate_var(
        &mut self,
        var_name: &str,
        ty: &DataType,
        is_report: bool,
        addr: &mut Option<VarAddr>,
        report_vars: &mut ReportVars,
        err_msg: &str,
        err: &mut ErrorGen,
    ) -> bool {
        if let DataType::Map { .. } = ty {
            // should already be handled!
            // See VisitingEmitter::emit_body
            return true;
        }
        match addr {
            Some(VarAddr::Global { .. }) | None => {
                if let Some(id) = self.use_available_gid(ty, err_msg, err) {
                    if is_report {
                        report_vars.put_local_metadata(id, var_name.to_string(), ty.clone(), err);
                    }

                    *addr = Some(VarAddr::Global { addr: id });
                    true
                } else {
                    false
                }
            }
            Some(VarAddr::Local { .. })
            | Some(VarAddr::MapId { .. })
            | Some(VarAddr::MemLoc { .. }) => {
                //this shouldn't happen for unshared vars - need to err
                err.unexpected_error(
                    true,
                    Some(format!("{err_msg} Expected Global VarAddr.")),
                    None,
                );
                false
            }
        }
    }
}

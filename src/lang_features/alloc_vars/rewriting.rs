#![allow(clippy::too_many_arguments)]
use crate::common::error::ErrorGen;
use crate::emitter::report_var_metadata::ReportVarMetadata;
use crate::emitter::utils::whamm_type_to_wasm_type;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::parser::types::DataType;
use crate::verifier::types::VarAddr;
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::MacroOpcode;
use orca_wasm::Opcode;

#[derive(Default)]
pub struct UnsharedVarHandler {
    pub used_i32_gids: Vec<u32>,
    pub available_i32_gids: Vec<u32>,
}
impl UnsharedVarHandler {
    pub fn use_available_gid(&mut self, err_msg: &str, err: &mut ErrorGen) -> Option<u32> {
        if self.available_i32_gids.is_empty() {
            err.unexpected_error(
                true,
                Some(format!(
                    "{err_msg} No available global I32s for unshared vars"
                )),
                None,
            );
            return None;
        }
        let id = self.available_i32_gids.remove(0);
        self.used_i32_gids.push(id);

        Some(id)
    }
    pub fn allocate_var<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
        &mut self,
        var_name: &str,
        ty: &DataType,
        is_report: bool,
        addr: &mut Option<VarAddr>,
        injector: &mut T,
        map_lib_adapter: &mut MapLibAdapter,
        report_var_metadata: &mut ReportVarMetadata,
        err_msg: &str,
        err: &mut ErrorGen,
    ) -> bool {
        if let DataType::Map { .. } = ty {
            let map_id = if is_report {
                map_lib_adapter.map_create_report(
                    var_name.to_string(),
                    ty.clone(),
                    injector,
                    report_var_metadata,
                    true,
                    err,
                )
            } else {
                map_lib_adapter.map_create(ty.clone(), injector, err)
            };
            *addr = Some(VarAddr::MapId { addr: map_id });
            return true;
        }
        match addr {
            Some(VarAddr::Global { .. }) | None => {
                let wasm_ty = whamm_type_to_wasm_type(ty);
                if let Some(id) = self.use_available_gid(err_msg, err) {
                    if is_report {
                        report_var_metadata.put_local_metadata(
                            id,
                            var_name.to_string(),
                            wasm_ty,
                            err,
                        );
                    }

                    *addr = Some(VarAddr::Global { addr: id });
                    true
                } else {
                    false
                }
            }
            Some(VarAddr::Local { .. }) | Some(VarAddr::MapId { .. }) => {
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

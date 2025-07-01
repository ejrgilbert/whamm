use std::collections::HashMap;
use wirm::module_builder::AddLocal;
use wirm::{DataType, Opcode};

#[derive(Default)]
pub struct LocalsTracker {
    available: HashMap<DataType, Vec<u32>>,
    in_use: HashMap<DataType, Vec<u32>>,
}
impl LocalsTracker {
    pub fn use_local<T: AddLocal>(&mut self, ty: DataType, injector: &mut T) -> u32 {
        let id = if let Some(list) = self.available.get_mut(&ty) {
            if let Some(id) = list.pop() {
                id
            } else {
                *injector.add_local(ty)
            }
        } else {
            *injector.add_local(ty)
        };

        self.add_in_use(ty, id);
        id
    }
    fn add_in_use(&mut self, ty: DataType, id: u32) {
        self.in_use
            .entry(ty)
            .and_modify(|list| {
                // insert at the beginning so that lower IDs are at the top
                // (for `extend` to keep them there)
                list.insert(0, id);
            })
            .or_insert(vec![id]);
    }
    pub fn reset_probe<'a, T: Opcode<'a>>(&mut self, _injector: &mut T) {
        self.available.extend(self.in_use.to_owned());
        self.in_use.clear();
    }
    pub fn reset_function(&mut self) {
        self.available.clear();
        self.in_use.clear();
    }
}

// fn gen_null_const<'a, T: Opcode<'a>>(ty: &DataType, injector: &mut T) {
//     match ty {
//         DataType::I8 => injector.i32_const(0),
//         DataType::I16 => injector.i32_const(0),
//         DataType::I32 => injector.i32_const(0),
//         DataType::I64 => injector.i64_const(0),
//         DataType::F32 => injector.f32_const(0f32),
//         DataType::F64 => injector.f64_const(0f64),
//         DataType::V128
//         | DataType::FuncRef
//         | DataType::FuncRefNull
//         | DataType::ExternRef
//         | DataType::ExternRefNull
//         | DataType::Any
//         | DataType::AnyNull
//         | DataType::None
//         | DataType::NoExtern
//         | DataType::NoFunc
//         | DataType::Eq
//         | DataType::EqNull
//         | DataType::Struct
//         | DataType::StructNull
//         | DataType::Array
//         | DataType::ArrayNull
//         | DataType::I31
//         | DataType::I31Null
//         | DataType::Exn
//         | DataType::NoExn
//         | DataType::Module { .. }
//         | DataType::RecGroup(_)
//         | DataType::CoreTypeId(_)
//         | DataType::Cont
//         | DataType::NoCont => todo!(),
//     };
// }

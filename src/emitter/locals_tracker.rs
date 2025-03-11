use std::collections::HashMap;
use orca_wasm::DataType;
use orca_wasm::module_builder::AddLocal;

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
            .entry(ty.clone())
            .and_modify(|list| {
                // insert at the beginning so that lower IDs are at the top
                // (for `extend` to keep them there)
                list.insert(0, id);
            })
            .or_insert(vec![id]);
    }
    pub fn add(&mut self, ty: DataType, id: u32) {
        self.available
            .entry(ty.clone())
            .and_modify(|list| {
                // insert at the beginning so that lower IDs are at the top
                // (for `pop`)
                list.insert(0, id);
            })
            .or_insert(vec![id]);
    }
    pub fn reset_probe(&mut self) {
        self.available.extend(self.in_use.to_owned());
        // self.in_use.clear();
    }
    pub fn reset_function(&mut self) {
        self.available.clear();
        self.in_use.clear();
    }
}
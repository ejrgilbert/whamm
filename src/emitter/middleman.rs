//this is the code that knows which functions to call in lib.rs based on what is in the AST
use crate::parser::types::{Whamm, WhammVisitor};
use crate::common::error::{ErrorGen, WhammError};

pub fn map_create(name:String, key: DataType, value: DataType, err: &mut ErrorGen){

}

pub fn map_insert(name: String, map: DataType, err: &mut ErrorGen) { //this "map" should have a key and value if its of DataType map
        
}

pub fn map_get(name: String, key: DataType, err: &mut ErrorGen) { //this "key" should be the key of the map
    //you can get the 
}
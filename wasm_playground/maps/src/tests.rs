//to test, use RUST_TEST_THREADS=1 cargo test
//Necessary because the mutex expects a single thread of acessors (or at least 1 thread per name in the map)

#[allow(unused_imports)]
use crate::*;
//testing map functionality
#[test]
fn test_i32_i32() {
    create_i32_i32("test".to_string());
    insert_i32_i32("test".to_string(), 1, 2);
    insert_i32_i32("test".to_string(), 2, 3);
    println!("{:?}", get_i32("test".to_string(), &1));
    let a = get_i32("test".to_string(), &1);
    println!("{:?}", get_i32("test".to_string(), &a.unwrap()));
    assert!(a == Some(2));
}
#[test]
fn test_string_bool() {
    create_string_bool("test".to_string());
    insert_string_bool("test".to_string(), "hello".to_string(), true);
    insert_string_bool("test".to_string(), "world".to_string(), false);
    println!("{:?}", get_bool("test".to_string(), &"hello".to_string()));
    let a = get_bool("test".to_string(), &"hello".to_string());
    println!("{:?}", get_bool("test".to_string(), &a.unwrap()));
    assert!(a == Some(true));
}
//test the ones including tuples and maps especially
#[test]
fn test_i32_tuple() {
    create_i32_tuple("test".to_string());
    insert_i32_tuple("test".to_string(), 1, TupleVariant::i32_i32(2, 3));
    insert_i32_tuple("test".to_string(), 2, TupleVariant::i32_i32(3, 4));
    println!("{:?}", get_tuple("test".to_string(), &1));
    let a = get_tuple("test".to_string(), &1);
    assert!(*(a.unwrap()) == TupleVariant::i32_i32(2, 3));
}
#[test]
fn test_i32_map() {
    create_i32_map("test".to_string());
    insert_i32_map("test".to_string(), 1, AnyMap::i32_i32_Map(HashMap::new()));
    //to change the stuff in the map, lock the mutex then get mut on that lock
    {
        let mut my_maps = MY_MAPS.lock().unwrap();
        let map = get_map_mut(&mut my_maps, "test".to_string(), &1).unwrap();
        map.insert(Box::new(2), Box::new(3));
    }
    let mut map = get_map("test".to_string(), &1).unwrap();
    //otherwise, you can just get the map for its contents -- Inserting on map from this get does not change the global map
    map.insert(Box::new(3), Box::new(4)); //does nothing
    println!("{:?}", get_map("test".to_string(), &1).unwrap().get_i32(&2));
    assert!(get_map("test".to_string(), &1).unwrap().get_i32(&2) == Some(3));
    assert!(get_map("test".to_string(), &1).unwrap().get_i32(&3) == None);
}
//tuple as key
#[test]
fn test_tuple_i32() {
    create_tuple_i32("test".to_string());
    insert_tuple_i32("test".to_string(), TupleVariant::i32_i32(1, 2), 3);
    insert_tuple_i32("test".to_string(), TupleVariant::i32_i32(2, 3), 4);
    println!(
        "{:?}",
        get_i32("test".to_string(), &Box::new(TupleVariant::i32_i32(1, 2)))
    );
    let a = get_i32("test".to_string(), &Box::new(TupleVariant::i32_i32(1, 2)));
    assert!(a == Some(3));
}
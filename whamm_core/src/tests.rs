//to test, use RUST_TEST_THREADS=1 cargo test
//Necessary because the mutex expects a single thread of acessors (or at least 1 thread per name in the map)

#[allow(unused_imports)]
use crate::*;
//testing map functionality
#[test]
fn test_i32_i32() {
    create_i32_i32(0);
    insert_i32_i32(0, 1, 2);
    insert_i32_i32(0, 2, 3);
    println!("{:?}", get_i32_optional(0, &1));
    let a = get_i32_optional(0, &1);
    println!("{:?}", get_i32_optional(0, &a.unwrap()));
    assert!(a == Some(2));
}
#[test]
fn test_string_bool() {
    create_string_bool(1);
    insert_string_bool(1, "hello".to_string(), true);
    insert_string_bool(1, "world".to_string(), false);
    println!("{:?}", get_bool_optional(1, &"hello".to_string()));
    let a = get_bool_optional(1, &"hello".to_string());
    println!("{:?}", get_bool_optional(1, &a.unwrap()));
    assert!(a == Some(true));
}
//test the ones including tuples and maps especially
#[test]
fn test_i32_tuple() {
    create_i32_tuple(2);
    insert_i32_tuple(2, 1, TupleVariant::i32_i32(2, 3));
    insert_i32_tuple(2, 2, TupleVariant::i32_i32(3, 4));
    println!("{:?}", get_tuple_optional(2, &1));
    let a = get_tuple_optional(2, &1);
    assert!(*(a.unwrap()) == TupleVariant::i32_i32(2, 3));
}
#[test]
fn test_i32_map() {
    create_i32_map(3);
    insert_i32_map(3, 1, AnyMap::i32_i32_Map(HashMap::new()));
    //to change the stuff in the map, lock the mutex then get mut on that lock
    {
        let mut my_maps = MY_MAPS.lock().unwrap();
        let map = get_map_mut(&mut my_maps, 3, &1).unwrap();
        map.insert(Box::new(2), Box::new(3));
    }
    let mut map = get_map_optional(3, &1).unwrap();
    //otherwise, you can just get the map for its contents -- Inserting on map from this get does not change the global map
    map.insert(Box::new(3), Box::new(4)); //does nothing
    println!("{:?}", get_map_optional(3, &1).unwrap().get_i32(&2));
    assert!(get_map_optional(3, &1).unwrap().get_i32(&2) == Some(3));
    assert!(get_map_optional(3, &1).unwrap().get_i32(&3) == None);
}
//tuple as key
#[test]
fn test_tuple_i32() {
    create_tuple_i32(4);
    insert_tuple_i32(4, TupleVariant::i32_i32(1, 2), 3);
    insert_tuple_i32(4, TupleVariant::i32_i32(2, 3), 4);
    println!(
        "{:?}",
        get_i32_optional(4, &Box::new(TupleVariant::i32_i32(1, 2)))
    );
    let a = get_i32_optional(4, &Box::new(TupleVariant::i32_i32(1, 2)));
    assert!(a == Some(3));
}

//testing new non-optional getters
#[test]
fn test_i32_i32_notopt() {
    create_i32_i32(5);
    insert_i32_i32(5, 1, 2);
    insert_i32_i32(5, 2, 3);
    println!("{:?}", get_i32(5, &1));
    let a = get_i32(5, &1);
    println!("{:?}", get_i32(5, &a));
    assert!(a == 2);
}

#[test]
fn i32i32i32_i32case() {
    let a = 6;
    let b = a;
    create_map_i32i32i32tuple_i32(a);
    insert_map_i32i32i32tuple_i32(b, 1, 2, 3, 4);
    let c = get_i32_from_i32i32i32tuple(b, 1, 2, 3);
    println!("{}", c);
}

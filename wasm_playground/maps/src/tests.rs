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


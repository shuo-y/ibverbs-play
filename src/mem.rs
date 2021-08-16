// See https://doc.rust-lang.org/std/primitive.array.html

use std::collections::HashMap;

fn check_mem(array: &[i32; 10]) {
    for i in 0..10 {
        assert_eq!(array[i], 1);
    }
}

fn main() {
    let array : [i32; 10] = [1; 10]; 
    println!("array {:?} ", array);
    check_mem(&array);

    // See https://doc.rust-lang.org/book/ch08-03-hash-maps.html
    let keys = vec![12];
    let values = vec!["hello".to_string()];
    let mut table: HashMap<_, _> =
        keys.into_iter().zip(values.into_iter()).collect();

    println!("{:?}", table);
}

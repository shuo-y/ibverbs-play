// See https://doc.rust-lang.org/std/primitive.array.html

use std::collections::HashMap;

// About tree see https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370

fn main() {
    // See https://doc.rust-lang.org/book/ch08-03-hash-maps.html
    let keys = vec![12];
    let values = vec!["hello".to_string()];
    let mut table: HashMap<_, _> =
        keys.into_iter().zip(values.into_iter()).collect();
    table.insert(11, "w".to_string());

    println!("{:?}", table);
}

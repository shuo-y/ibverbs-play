// See https://doc.rust-lang.org/std/primitive.array.html

fn check_mem(array: &[i32; 10]) {
    for i in 0..10 {
        assert_eq!(array[i], 1);
    }
}

fn main() {
    let array : [i32; 10] = [1; 10]; 
    println!("array {:?} ", array);
    check_mem(&array);
}

// Based on https://github.com/jonhoo/rust-ibverbs/blob/master/examples/loopback.rs
// and https://docs.rs/ibverbs/0.5.0/src/ibverbs/lib.rs.html#141
// Refer to https://github.com/jonhoo/rust-ibverbs/commit/cfbb93771fd180c63ca0ff89d80121674239be9f
// for the start

fn main() {
    println!("Hello, world!");
    
    let dev_num = ibverbs::devices().unwrap().len();

    println!("Number of devices {} ", dev_num);

    let dev = ibverbs::devices();

    // Check the Result type
    // See https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
    match dev {
        Ok(n) => println!("numbers is {} ", n.len()),
        Err(e) => println!("No device found {}", e),
    }
    
    // Check the type https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
    // See also https://doc.rust-lang.org/std/any/fn.type_name.html
}

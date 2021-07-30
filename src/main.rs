// Based on https://github.com/jonhoo/rust-ibverbs/blob/master/examples/loopback.rs
// and https://docs.rs/ibverbs/0.5.0/src/ibverbs/lib.rs.html#141
// Refer to https://github.com/jonhoo/rust-ibverbs/commit/cfbb93771fd180c63ca0ff89d80121674239be9f
// for the start

fn main() {
    println!("Hello, world!");
    
    let dev = ibverbs::devices().unwrap().len();

    println!("Number of devices {} ", dev);

}

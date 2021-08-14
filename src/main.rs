// See https://github.com/jonhoo/rust-ibverbs/blob/master/examples/loopback.rs
// See https://docs.rs/ibverbs/0.5.0/src/ibverbs/lib.rs.html
// See https://docs.rs/ibverbs/0.6.0/src/ibverbs/lib.rs.html
// Refer to https://github.com/jonhoo/rust-ibverbs/commit/cfbb93771fd180c63ca0ff89d80121674239be9f
// for the start
// See https://doc.rust-lang.org/book/ch05-02-example-structs.html about #[derive(Debug)]

use ibverbs::Device;
use std::process::Command;

fn proc_dev(dev: Option<Device>) {
    println!("Do something");
    match dev {
        Some(d) => {
            let guid_num = d.guid().unwrap();
            println!("The guid {} ", guid_num);
            let ctx = d.open().expect("cannot create ctx"); // Similar to unwrap
            println!("Check the ctx ");
            // create_cq  number of entries is 4 id is 1
            let cq = ctx.create_cq(4, 1).unwrap();
            let mut comp = [ibverbs::ibv_wc::default(); 1];
            // See https://github.com/jonhoo/rust-ibverbs/blob/master/examples/loopback.rs from github
            cq.poll(&mut comp);
            println!("Try to poll");
            let _pd = ctx.alloc_pd().unwrap();
        },
        None =>  println!("no device"),
    }
}

fn main() {
    let devices = ibverbs::devices();

    // Check the Result type
    // See https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
    match devices {
        Ok(n) => {
            println!("numbers is {} ", n.len());
            proc_dev(n.get(0))
        },
        Err(e) => println!("No device found {}", e),
    }
    
    // Check the type https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
    // See also https://doc.rust-lang.org/std/any/fn.type_name.html
    // Pick 1st device

    // https://doc.rust-lang.org/std/process/struct.Command.html
    let ret = Command::new("sh")
        .arg("-c")
        .arg("ls -lah .")
        .output()
        .expect("Run failed");

    let dev_guid = ibverbs::devices()
        .unwrap()
        .get(0)
        .unwrap()
        .guid()
        .unwrap();

    println!("Dev guid {} ", dev_guid);

}

// See https://github.com/jonhoo/rust-ibverbs/blob/master/examples/loopback.rs
// See https://docs.rs/ibverbs/0.5.0/src/ibverbs/lib.rs.html
// See https://docs.rs/ibverbs/0.6.0/src/ibverbs/lib.rs.html
// Refer to https://github.com/jonhoo/rust-ibverbs/commit/cfbb93771fd180c63ca0ff89d80121674239be9f
// for the start
// See https://doc.rust-lang.org/book/ch05-02-example-structs.html about #[derive(Debug)]

// use std::process::Command;
// use std::env;
// use std::ffi::OsString;

fn main() {
    // Result type see https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
    // Check the type https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
    // See https://doc.rust-lang.org/std/any/fn.type_name.html
    // See https://doc.rust-lang.org/std/process/struct.Command.html
    // See rustc --explain E0515
    // See https://doc.rust-lang.org/book/ch13-01-closures.html
    let devices = ibverbs::devices().unwrap();
    let get_dev = |x| {
        devices.get(x)
    };
    let get_dev_num = |x: &ibverbs::DeviceList| {
        x.len()
    };
    println!("{}", get_dev_num(&devices));
    let dev = get_dev(0).unwrap();
    let dev_guid = dev.guid().unwrap();

    println!("Dev guid {} ", dev_guid);
    let ctx = dev.open().unwrap();
    let wr_num = 1024;
    let send_cq = ctx.create_cq(wr_num, 0).unwrap();
    let recv_cq = ctx.create_cq(wr_num, 1).unwrap();

    let pd0 = ctx.alloc_pd().unwrap();
    let pqp0 = pd0
        .create_qp(&send_cq, &recv_cq, ibverbs::ibv_qp_type::IBV_QPT_RC)
        .build()
        .unwrap();
    let endpoint0 = pqp0.endpoint();
    // See https://doc.rust-lang.org/stable/std/mem
    // println!("Size of {} ", std::mem::size_of::<QueuePairEndpoint>());
    let endpoint0_bytes = unsafe {
        std::mem::transmute::<ibverbs::QueuePairEndpoint, [u8; 24]>(endpoint0)
    };
    // QueuePairEndpoint 192 bits
    let pqp1 = pd0
        .create_qp(&send_cq, &recv_cq, ibverbs::ibv_qp_type::IBV_QPT_RC)
        .build()
        .unwrap();
    let endpoint1 = pqp1.endpoint();
    let qp0 = pqp0.handshake(endpoint1);
    let qp1 = pqp1.handshake(endpoint0);
}


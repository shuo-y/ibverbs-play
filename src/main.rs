// See https://github.com/jonhoo/rust-ibverbs/blob/master/examples/loopback.rs
// See https://docs.rs/ibverbs/0.5.0/src/ibverbs/lib.rs.html
// See https://docs.rs/ibverbs/0.6.0/src/ibverbs/lib.rs.html
// Refer to https://github.com/jonhoo/rust-ibverbs/commit/cfbb93771fd180c63ca0ff89d80121674239be9f for the start
// See https://doc.rust-lang.org/book/ch05-02-example-structs.html about #[derive(Debug)]

use std::io::Read;
use std::io::Write;
use std::convert::TryInto;

fn tcp_exchange_server(local_endpoint: ibverbs::QueuePairEndpoint) -> ibverbs::QueuePairEndpoint {
    let tcp_listen = std::net::TcpListener::bind("127.0.0.1:12345").unwrap();
    // See https://doc.rust-lang.org/std/net/struct.TcpListener.html
    let (mut stream, address) = tcp_listen.accept().unwrap();
    println!("connect with {:?}", address);
    // A buffer for QueuePairEndpoint
    let mut remote_endpoint_buffer: [u8; 24] = [0; 24];
    stream.read(&mut remote_endpoint_buffer).unwrap();
    let remote_endpoint = unsafe {
        std::mem::transmute::<[u8; 24], ibverbs::QueuePairEndpoint>(remote_endpoint_buffer)
    };
    let mut local_endpoint_buffer = unsafe {
        std::mem::transmute::<ibverbs::QueuePairEndpoint, [u8; 24]>(local_endpoint)
    };
    stream.write(&mut local_endpoint_buffer).unwrap();
    return remote_endpoint;
}

fn tcp_exchange_client(local_endpoint: ibverbs::QueuePairEndpoint, ip: &str) -> ibverbs::QueuePairEndpoint {
    loop {
        // See https://doc.rust-lang.org/std/string/struct.String.html
        let mut addr = String::from("");
        addr.push_str(ip);
        addr.push_str(":12345");
        // See https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect
        if let Ok(mut stream) = std::net::TcpStream::connect(addr) {
            let mut local_endpoint_buffer = unsafe {
                std::mem::transmute::<ibverbs::QueuePairEndpoint, [u8; 24]>(local_endpoint)
            };
            stream.write(&mut local_endpoint_buffer).unwrap();
            let mut receive_buffer: [u8; 24] = [0; 24];
            stream.read(&mut receive_buffer).unwrap();
            let remote_endpoint = unsafe {
                std::mem::transmute::<[u8; 24], ibverbs::QueuePairEndpoint>(receive_buffer)
            };
            return remote_endpoint;
        }
    }
}

fn tcp_exchange(machine: i32, local_endpoint: ibverbs::QueuePairEndpoint, ip: &str) -> ibverbs::QueuePairEndpoint {
    if machine == 0 {
        return tcp_exchange_server(local_endpoint);
    } else {
        return tcp_exchange_client(local_endpoint, ip);
    }
}

fn server_func(qp: &mut ibverbs::QueuePair,
               mr: &mut ibverbs::MemoryRegion<u8>,
               send_cq: &ibverbs::CompletionQueue,
               recv_cq: &ibverbs::CompletionQueue) {
    unsafe {
        qp.post_receive(mr, ..1024, 0)
    }.unwrap();
    let mut complete = [ibverbs::ibv_wc::default(); 1024];
    loop {
        let completed = recv_cq.poll(&mut complete).unwrap();
        if completed.len() > 0 {
        let data = unsafe {
            std::mem::transmute::<[u8; 8], u64>(mr[..8]
            .try_into()
            .expect("Convert error"))
        };
        // See https://stackoverflow.com/questions/25428920/how-to-get-a-slice-as-an-array-in-rust
        println!("receive {}", data);
        break;
        }
    }
}

fn client_func(qp: &mut ibverbs::QueuePair,
               mr: &mut ibverbs::MemoryRegion<u8>,
               send_cq: &ibverbs::CompletionQueue,
               recv_cq: &ibverbs::CompletionQueue) {
    mr[0] = 222;
    unsafe {
        qp.post_send(mr, ..1024, 0)
    }.unwrap();
    let mut complete = [ibverbs::ibv_wc::default(); 1024];
    loop {
        let completed = send_cq.poll(&mut complete).unwrap();
        if completed.len() > 0 {
            break;
        }
    }
}

fn main() {
    // 0 for server 1 for client
    let mut machine = 0;
    let mut ip = "";
    // Check arguments see https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        assert!(args.len() >= 3);
        assert_eq!(&args[1], "--server");
        ip = &args[2];
        machine = 1;
    }
    // Result type see https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
    // Check the type https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
    // See https://doc.rust-lang.org/std/any/fn.type_name.html
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

    let pd = ctx.alloc_pd().unwrap();
    let pqp = pd
        .create_qp(&send_cq, &recv_cq, ibverbs::ibv_qp_type::IBV_QPT_RC)
        .build()
        .unwrap();
    let local_endpoint = pqp.endpoint();
    // See https://doc.rust-lang.org/stable/std/mem
    // println!("Size of {} ", std::mem::size_of::<QueuePairEndpoint>());
    let remote_endpoint = tcp_exchange(machine, local_endpoint, ip);
    let mut qp = pqp.handshake(remote_endpoint).unwrap();
    // QueuePairEndpoint 192 bits
    let mut mr = pd.allocate::<u8>(4096).unwrap();
    if machine == 0 {
        server_func(&mut qp, &mut mr, &send_cq, &recv_cq);
    } else {
        client_func(&mut qp, &mut mr, &send_cq, &recv_cq);
    }
}


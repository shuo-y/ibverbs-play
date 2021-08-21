// Source https://doc.rust-lang.org/std/net/struct.TcpStream.html
// Source https://doc.rust-lang.org/std/str/fn.from_utf8.html

use std::net::TcpStream;
use std::io::{Write, Read};
use rand::Rng;
use std::convert::TryInto;

fn main() {

    loop {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
            // Use mut stream see E00596
            println!("my addr {} ", stream.local_addr().unwrap());
            println!("remote addr {} ", stream.peer_addr().unwrap());
            let val = rand::thread_rng().gen::<u64>();
            println!("client val {} ", val);
            let inp = val.to_be_bytes();
            stream.write(&inp).unwrap();
            stream.flush().unwrap();

            let mut buffer = [0; 32];
            stream.read(&mut buffer).unwrap();
            let recv = &buffer[..8];
            // See https://doc.rust-lang.org/std/primitive.u64.html#method.from_be_bytes
            let val = u64::from_be_bytes(recv.try_into().unwrap());
            println!("Receive {}", val);
            break
        }
    }
}

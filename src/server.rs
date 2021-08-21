// Source https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// Source https://doc.rust-lang.org/std/thread/fn.sleep.html
// Source https://doc.rust-lang.org/stable/std/time/struct.Duration.html
// Source https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// Source https://doc.rust-lang.org/std/net/struct.TcpListener.html
// Source https://doc.rust-lang.org/std/str/fn.from_utf8.html

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use rand::Rng;
//use std::ptr;
use std::convert::TryInto;

fn handle_con(mut stream: TcpStream) {
    // See https://www.reddit.com/r/rust/comments/8bak8k/help_getting_ip_address_client_connected_to_with/
    // See https://doc.rust-lang.org/std/net/struct.TcpStream.html
    println!("Try to response {} ", stream.peer_addr().unwrap());
    let mut buffer = [0; 32];

    // See https://docs.rs/rand/0.6.2/rand/trait.Rng.html#method.gen
    let val = rand::thread_rng().gen::<u64>();
    println!("server val {} ", val);

    let inp = val.to_be_bytes(); // See https://doc.rust-lang.org/std/primitive.u64.html#method.to_be_bytes

    stream.read(&mut buffer).unwrap();
    let recv = &buffer[..8];
    // See https://doc.rust-lang.org/std/primitive.u64.html#method.from_be_bytes
    // Convert &[u8] to [u8; 8]
    let client_val = u64::from_be_bytes(recv.try_into().unwrap());
    println!("Receive {}", client_val);

    stream.write(&inp).unwrap();
}

fn main() {
    let listen = TcpListener::bind("127.0.0.1:8888").unwrap();
    
    //use std::thread;
    //use std::time;

    for stream in listen.incoming() {
        handle_con(stream.unwrap());
    }
}

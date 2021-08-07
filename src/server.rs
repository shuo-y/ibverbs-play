// Source https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// Source https://doc.rust-lang.org/std/thread/fn.sleep.html
// Source https://doc.rust-lang.org/stable/std/time/struct.Duration.html
// Source https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// Source https://doc.rust-lang.org/std/net/struct.TcpListener.html

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;

fn handle_con(mut stream: TcpStream) {
    // See https://www.reddit.com/r/rust/comments/8bak8k/help_getting_ip_address_client_connected_to_with/
    // See https://doc.rust-lang.org/std/net/struct.TcpStream.html
    println!("Try to response {} ", stream.peer_addr().unwrap());
}

fn main() {
    let listen = TcpListener::bind("127.0.0.1:8888").unwrap();
    
    //use std::thread;
    //use std::time;

    for stream in listen.incoming() {
        match stream {
            Ok(stream) => { handle_con(stream) }
            Err(e) => { println!("Error {} ", e) }
        }
    }
}

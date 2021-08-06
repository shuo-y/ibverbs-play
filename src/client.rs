// Source https://doc.rust-lang.org/std/net/struct.TcpStream.html

use std::net::TcpStream;

fn main() {

    loop {
        if let Ok(stream) = TcpStream::connect("127.0.0.1:8888") {
            println!("Connect set up!");
            break
        }
    }
}

// Source https://doc.rust-lang.org/std/net/struct.TcpStream.html
// Source https://doc.rust-lang.org/std/str/fn.from_utf8.html

use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

fn main() {

    loop {
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
            // Use mut stream see E00596
            println!("Connect set up!");
            println!("my addr {} ", stream.local_addr().unwrap());
            println!("remote addr {} ", stream.peer_addr().unwrap());
            let request = "Msg from client";
            stream.write(request.as_bytes()).unwrap();
            stream.flush().unwrap(); // Why

            let mut buffer = [0; 32];
            stream.read(&mut buffer).unwrap();
            let msg = std::str::from_utf8(&buffer).unwrap();
            println!("Receive: {} ", msg);
            break
        }
    }
}

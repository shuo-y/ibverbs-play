// Source https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// Source https://doc.rust-lang.org/std/thread/fn.sleep.html
// Source https://doc.rust-lang.org/stable/std/time/struct.Duration.html

use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listen = TcpListener::bind("127.0.0.1:8888").unwrap();
    
    use std::thread;
    use std::time;

    let wait_time = time::Duration::new(10, 0);
    thread::sleep(wait_time);

}

use std::io::{stdin, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let server_address = "127.0.0.1:8888";

    let mut socket = TcpStream::connect(server_address)
        .expect("cannot establish connection to server");
    socket.set_nonblocking(true).expect("unavailable");
    println!("connect to {}", server_address); 

    start_thread(socket.try_clone().unwrap());

    let user = input("whats your name?");
    println!("{}, Input message here", user);
    loop {
        let msg = input("");
        let msg = format!("{}> {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                println!("[Receive] {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
fn input(msg: &str) -> String {
    if msg != "" { println!("{}", msg); }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("standard output error");
    String::from(buf.trim())
}

#[warn(unused_variables)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for _stream in listener.incoming() {
        let stream = _stream.unwrap();

        println!("Connection established!");
    }
}

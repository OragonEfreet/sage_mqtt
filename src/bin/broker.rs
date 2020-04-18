use std::net::{TcpListener, TcpStream};
use sage_mqtt::{Decode, Connect};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let connect = Connect::decode(&mut stream);
    println!("{:?}", connect);
}

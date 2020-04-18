use std::io::prelude::*;
use std::net::TcpStream;

use sage_mqtt::{Connect, Encode};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    let connect = Connect::default();
    let mut encoded = Vec::new();
    connect.encode(&mut encoded).unwrap();

    stream.write(&encoded)?;
    Ok(())
} // the stream is closed here

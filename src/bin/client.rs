use std::io::prelude::*;
use std::net::TcpStream;

use sage_mqtt::{ControlPacket, Encode};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    let mut encoded = Vec::new();
    ControlPacket::Connect(Default::default())
        .encode(&mut encoded)
        .unwrap();

    stream.write(&encoded)?;
    Ok(())
} // the stream is closed here

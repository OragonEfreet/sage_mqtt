use std::io::prelude::*;
use std::net::TcpStream;

use sage_mqtt::ControlPacket;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    let mut encoded = Vec::new();
    ControlPacket::Connect(Default::default())
        .encode(&mut encoded)
        .await
        .unwrap();

    stream.write_all(&encoded)?;
    Ok(())
} // the stream is closed here

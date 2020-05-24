use sage_mqtt::*;

#[async_std::main]
async fn main() {
    let mut encoded = Vec::new();

    ControlPacket::Publish(Default::default())
        .encode(&mut encoded)
        .await
        .unwrap();

    //println!("{:?}", encoded);
    //     let mut reader = Cursor::new(encoded);

    //     let _ = ControlPacket::decode(&mut reader).unwrap();

    //     // stream.write(&encoded)?;
} // the stream is closed here

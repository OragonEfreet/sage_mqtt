use sage_mqtt::*;

#[async_std::main]
async fn main() {
    let mut encoded = Vec::new();

    Packet::Publish(Publish::with_message("".into(), ""))
        .encode(&mut encoded)
        .await
        .unwrap();

    //     let mut reader = Cursor::new(encoded);

    //     let _ = Packet::decode(&mut reader).unwrap();

    //     // stream.write(&encoded)?;
} // the stream is closed here

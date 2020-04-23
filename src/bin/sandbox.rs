use sage_mqtt::*;

fn main() {
    let mut encoded = Vec::new();

    ControlPacket::UnSubscribe(UnSubscribe {
        packet_identifier: 1337,
        user_properties: vec![("Mogwaï".into(), "Cat".into())],
        subscriptions: vec![
            "harder".into(),
            "better".into(),
            "faster".into(),
            "stronger".into(),
        ],
    })
    .encode(&mut encoded)
    .unwrap();

    println!("{:?}", encoded);
    //     let mut reader = Cursor::new(encoded);

    //     let _ = ControlPacket::decode(&mut reader).unwrap();

    //     // stream.write(&encoded)?;
} // the stream is closed here

use sage_mqtt::*;

fn main() {
    let mut encoded = Vec::new();

    ControlPacket::Connect(Connect {
        keep_alive: 10,
        clean_start: true,
        session_expiry_interval: 10,
        user_name: Some("Willow".into()),
        password: Some("Jaden".into()),
        will: Some(Will {
            qos: QoS::AtLeastOnce,
            ..Default::default()
        }),
        ..Default::default()
    })
    .encode(&mut encoded)
    .unwrap();

    println!("{:?}", encoded);
    //     let mut reader = Cursor::new(encoded);

    //     let _ = ControlPacket::decode(&mut reader).unwrap();

    //     // stream.write(&encoded)?;
} // the stream is closed here

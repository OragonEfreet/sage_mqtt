// use std::io::Cursor;

// use sage_mqtt::{Connect, ControlPacket, Decode, Encode};
// fn connect_decoded() -> Connect {
//     let keep_alive = 10;

//     let session_expiry_interval = 10;

//     let mut c = Connect {
//         keep_alive,
//         session_expiry_interval,
//         ..Default::default()
//     };

//     c.receive_maximum = 42;
//     c.maximum_packet_size = Some(16);
//     c.topic_alias_maximum = Some(2);
//     c.request_response_information = Some(true);
//     c.user_properties.push(("Jarod".into(), "Jaden".into()));
//     c.authentication_method = Some("Willow".into());
//     c
// }

fn main() {
    //     // let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    //     let mut encoded = Vec::new();

    //     ControlPacket::Connect(connect_decoded())
    //         .encode(&mut encoded)
    //         .unwrap();

    //     println!("{:?}", encoded);

    //     let mut reader = Cursor::new(encoded);

    //     let _ = ControlPacket::decode(&mut reader).unwrap();

    //     // stream.write(&encoded)?;
} // the stream is closed here

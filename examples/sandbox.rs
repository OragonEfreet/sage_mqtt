use sage_mqtt::*;

#[async_std::main]
async fn main() {
    let mut encoded = Vec::new();

    ControlPacket::ConnAck(ConnAck {
        session_present: true,
        reason_code: ReasonCode::Banned,
        session_expiry_interval: Some(1337),
        receive_maximum: 30,
        maximum_qos: QoS::AtLeastOnce,
        retain_available: false,
        maximum_packet_size: Some(256),
        assigned_client_id: Some("WalkThisWay".into()),
        topic_alias_maximum: 10,
        reason_string: "RUN-DMC".into(),
        user_properties: vec![("Mogwaï".into(), "Cat".into())],
        wildcard_subscription_available: false,
        subscription_identifiers_available: true,
        shared_subscription_available: false,
        keep_alive: Some(17),
        response_information: "Aerosmith".into(),
        reference: Some("Paint It Black".into()),
        authentication: Some(Authentication {
            method: "Willow".into(),
            data: vec![0x0D, 0x15, 0xEA, 0x5E],
        }),
    })
    .encode(&mut encoded)
    .await
    .unwrap();

    println!("{:?}", encoded);
    //     let mut reader = Cursor::new(encoded);

    //     let _ = ControlPacket::decode(&mut reader).unwrap();

    //     // stream.write(&encoded)?;
} // the stream is closed here

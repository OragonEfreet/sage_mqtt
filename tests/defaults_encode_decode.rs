use async_std::io::Cursor;
use sage_mqtt::{
    Auth, ConnAck, Connect, Disconnect, Error, Packet, PubAck, PubComp, PubRec, PubRel, ReasonCode,
    SubAck, Subscribe, UnSubAck, UnSubscribe,
};

#[async_std::test]
async fn default_connect() {
    let mut encoded = Vec::new();
    let send_packet: Packet = Connect::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Connect packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode Connect");
    if let Packet::Connect(receive_packet) = receive_result {
        assert_eq!(receive_packet, Connect::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn connect_with_default_auth() {
    let mut encoded = Vec::new();
    let send_packet: Packet = Connect {
        authentication: Some(Default::default()),
        ..Default::default()
    }
    .into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Connect packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode Connect");
    if let Packet::Connect(receive_packet) = receive_result {
        assert_eq!(
            receive_packet,
            Connect {
                authentication: Some(Default::default()),
                ..Default::default()
            }
        );
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_connack() {
    let mut encoded = Vec::new();
    let send_packet: Packet = ConnAck::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode ConnAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode ConnAck");
    if let Packet::ConnAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, ConnAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_puback() {
    let mut encoded = Vec::new();
    let send_packet: Packet = PubAck::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode PubAck");
    if let Packet::PubAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pubrec() {
    let mut encoded = Vec::new();
    let send_packet: Packet = PubRec::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubRec packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode PubRec");
    if let Packet::PubRec(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubRec::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pubrel() {
    let mut encoded = Vec::new();
    let send_packet: Packet = PubRel::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubRel packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode PubRel");
    if let Packet::PubRel(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubRel::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pubcomp() {
    let mut encoded = Vec::new();
    let send_packet: Packet = PubComp::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubComp packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode PubComp");
    if let Packet::PubComp(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubComp::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_subscribe() {
    let mut encoded = Vec::new();
    let send_packet: Packet = Subscribe::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Subscribe packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor).await;
    assert!(matches!(
        receive_result,
        Err(Error::Reason(ReasonCode::ProtocolError))
    ));
}

#[async_std::test]
async fn default_suback() {
    let mut encoded = Vec::new();
    let send_packet: Packet = SubAck::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode SubAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode SubAck");
    if let Packet::SubAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, SubAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_unsubscribe() {
    let mut encoded = Vec::new();
    let send_packet: Packet = UnSubscribe::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode UnSubscribe packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor).await;
    assert!(matches!(
        receive_result,
        Err(Error::Reason(ReasonCode::ProtocolError))
    ));
}

#[async_std::test]
async fn default_unsuback() {
    let mut encoded = Vec::new();
    let send_packet: Packet = UnSubAck::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode UnSubAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode UnSubAck");
    if let Packet::UnSubAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, UnSubAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pingreq() {
    let mut encoded = Vec::new();
    let send_size = Packet::PingReq
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PingReq packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode PingReq");
    assert!(matches!(receive_result, Packet::PingReq));
}

#[async_std::test]
async fn default_pingresp() {
    let mut encoded = Vec::new();
    let send_size = Packet::PingResp
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PingResp packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode PingResp");
    assert!(matches!(receive_result, Packet::PingResp));
}

#[async_std::test]
async fn default_disconnect() {
    let mut encoded = Vec::new();
    let send_packet: Packet = Disconnect::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Disconnect packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode Disconnect");
    if let Packet::Disconnect(receive_packet) = receive_result {
        assert_eq!(receive_packet, Disconnect::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_auth() {
    let mut encoded = Vec::new();
    let send_packet: Packet = Auth::default().into();
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Auth packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = Packet::decode(&mut cursor)
        .await
        .expect("Cannot decode Auth");
    if let Packet::Auth(receive_packet) = receive_result {
        assert_eq!(receive_packet, Auth::default());
    } else {
        panic!("Incorrect packet type");
    }
}

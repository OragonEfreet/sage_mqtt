use async_std::io::Cursor;
use sage_mqtt::{
    Auth, ConnAck, Connect, ControlPacket, Disconnect, PubAck, PubComp, PubRec, PubRel, Publish,
    SubAck, Subscribe, UnSubAck, UnSubscribe,
};

#[async_std::test]
async fn default_connect() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::Connect(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Connect packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode Connect");
    if let ControlPacket::Connect(receive_packet) = receive_result {
        assert_eq!(receive_packet, Connect::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_connack() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::ConnAck(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode ConnAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode ConnAck");
    if let ControlPacket::ConnAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, ConnAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_publish() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::Publish(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Publish packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode Publish");
    if let ControlPacket::Publish(receive_packet) = receive_result {
        assert_eq!(receive_packet, Publish::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_puback() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::PubAck(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode PubAck");
    if let ControlPacket::PubAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pubrec() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::PubRec(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubRec packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode PubRec");
    if let ControlPacket::PubRec(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubRec::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pubrel() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::PubRel(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubRel packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode PubRel");
    if let ControlPacket::PubRel(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubRel::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pubcomp() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::PubComp(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PubComp packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode PubComp");
    if let ControlPacket::PubComp(receive_packet) = receive_result {
        assert_eq!(receive_packet, PubComp::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_subscribe() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::Subscribe(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Subscribe packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode Subscribe");
    if let ControlPacket::Subscribe(receive_packet) = receive_result {
        assert_eq!(receive_packet, Subscribe::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_suback() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::SubAck(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode SubAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode SubAck");
    if let ControlPacket::SubAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, SubAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_unsubscribe() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::UnSubscribe(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode UnSubscribe packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode UnSubscribe");
    if let ControlPacket::UnSubscribe(receive_packet) = receive_result {
        assert_eq!(receive_packet, UnSubscribe::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_unsuback() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::UnSubAck(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode UnSubAck packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode UnSubAck");
    if let ControlPacket::UnSubAck(receive_packet) = receive_result {
        assert_eq!(receive_packet, UnSubAck::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_pingreq() {
    let mut encoded = Vec::new();
    let send_size = ControlPacket::PingReq
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PingReq packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode PingReq");
    assert!(matches!(receive_result, ControlPacket::PingReq));
}

#[async_std::test]
async fn default_pingresp() {
    let mut encoded = Vec::new();
    let send_size = ControlPacket::PingResp
        .encode(&mut encoded)
        .await
        .expect("Cannot encode PingResp packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode PingResp");
    assert!(matches!(receive_result, ControlPacket::PingResp));
}

#[async_std::test]
async fn default_disconnect() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::Disconnect(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Disconnect packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode Disconnect");
    if let ControlPacket::Disconnect(receive_packet) = receive_result {
        assert_eq!(receive_packet, Disconnect::default());
    } else {
        panic!("Incorrect packet type");
    }
}

#[async_std::test]
async fn default_auth() {
    let mut encoded = Vec::new();
    let send_packet = ControlPacket::Auth(Default::default());
    let send_size = send_packet
        .encode(&mut encoded)
        .await
        .expect("Cannot encode Auth packet");
    assert!(send_size > 0);

    let mut cursor = Cursor::new(encoded);
    let receive_result = ControlPacket::decode(&mut cursor)
        .await
        .expect("Cannot decode Auth");
    if let ControlPacket::Auth(receive_packet) = receive_result {
        assert_eq!(receive_packet, Auth::default());
    } else {
        panic!("Incorrect packet type");
    }
}

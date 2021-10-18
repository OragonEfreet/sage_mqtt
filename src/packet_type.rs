use crate::QoS;

/// The control packet type is present as the first element of the fixed header
/// in an MQTT paquet. It is encoded in a 8bit flag set where the 4 most
/// significant bits represent the type of the paquet and the 4 least are flags
/// where values depend on the type.
#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Reserved,
    Connect,
    ConnAck,
    Publish {
        duplicate: bool,
        qos: QoS,
        retain: bool,
    },
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    UnSubscribe,
    UnSubAck,
    PingReq,
    PingResp,
    Disconnect,
    Auth,
}

enum PayloadRequirements {
    None,
    Required,
    Optional,
}

impl From<PacketType> for PayloadRequirements {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::Publish { .. } => PayloadRequirements::Optional,
            PacketType::Connect
            | PacketType::Subscribe
            | PacketType::SubAck
            | PacketType::UnSubscribe
            | PacketType::UnSubAck => PayloadRequirements::Required,
            _ => PayloadRequirements::None,
        }
    }
}
